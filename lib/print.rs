/*
 * File:    print.rs
 * Brief:   Utilities to print to the simulator's standard output.
 *
 * Copyright (C) 2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Uses vpi_printf() from IEEE 1800 to print to the simulator's standard output.
 *
*/

/*!
 *
 * Utilities to print to the simulator's standard output.
 *
 * [`sim_print!()`](crate::sim_print) and [`sim_println!()`](crate::sim_println) are analogous to the
 * standard [`print!()`] and [`println!()`] macros but print to the simulator's standard output instead
 * of the regular Rust standard output (though these are often the same).
 *
 * There is also a [`SimulatorPrinter`] similar to the standard [`Stdout`](std::io::Stdout) that
 * allows you to amortize the cost of locking the simulator's standard output over multiple print
 * calls (implementing the [Writer](std::fmt::Write) trait).
 *
 * Since these use `vpi_printf()` internally, and according to the IEEE 1800-2017 standard, __it is
 * not safe to call it from a startup routine__, if you do, a panic will occur or you'll
 * get an [Err] depending on exactly what you're working with.
 *
*/

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

use std::ffi::CString;
use std::fmt::{Error, Write};
use std::fmt::Result as FmtResult;
use std::sync::{Mutex, MutexGuard, PoisonError};

use crate::startup::in_startup_routine;

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

///Prints to the simulator output.
///
///This may or not be the same as printing to the regular Rust standard output.
///It is preferable if you want, for example, ensure you are logging to the same log file that the
///simulator itself is writing to.
///
///Equivalent to the [`sim_println!()`](crate::sim_println) macro except that a newline is not
///printed at the end of the message.
///
///This does make use of a temporary [`SimulatorPrinter`] object, so don't invoke this if you
///currently have one locked in the current thread to avoid a deadlock.
///
///Analagous to [`print!()`](std::print).
#[macro_export]
macro_rules! sim_print {
    ($($arg:tt)*) => {{
        use std::fmt::Write as _;
        write!(::sv_api::print::SimulatorPrinter::new(), $($arg)*)
            .expect("Failure writing to simulator output with sim_print!(), are you in a startup routine?");
    }};
}

///Prints to the simulator output, with a newline.
///
///This may or not be the same as printing to the regular Rust standard output.
///It is preferable if you want, for example, ensure you are logging to the same log file that the
///simulator itself is writing to.
///
///This does make use of a temporary [`SimulatorPrinter`] object, so don't invoke this if you
///currently have one locked in the current thread to avoid a deadlock.
///
///Analagous to [`println!()`](std::println).
#[macro_export]
macro_rules! sim_println {
    ($($arg:tt)*) => {{
        use std::fmt::Write as _;
        writeln!(::sv_api::print::SimulatorPrinter::new(), $($arg)*)
            .expect("Failure writing to simulator output with sim_println!(), are you in a startup routine?");
    }};
}

/* ------------------------------------------------------------------------------------------------
 * Static Variables
 * --------------------------------------------------------------------------------------------- */

//TODO does a mutex like this properly protect the simulator's output?
static PRINT_MUTEX: Mutex<()> = Mutex::new(());

/* ------------------------------------------------------------------------------------------------
 * Types
 * --------------------------------------------------------------------------------------------- */

///A handle to the simulator's output.
///
///Useful for amortizing the cost of locking the simulator's output over multiple writes.
///
///By default, the printer will lock on every write, but you can use the [`prelock()`](Self::prelock)
///method to re-use a lock over multiple writes. You can release this lock by either calling
///[`unlock()`](Self::unlock) or by dropping the struct (such as letting it go out of scope).
#[derive(Debug)]
pub struct SimulatorPrinter {
    guard: Option<MutexGuard<'static, ()>>
}

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

impl SimulatorPrinter {
    pub fn new() -> Self {
        Self {
            guard: None
        }
    }

    ///Locks the simulator's output.
    ///
    ///Useful for amortizing the cost of locking the simulator's output over multiple writes.
    ///
    ///To avoid deadlock, you shouldn't call [`prelock()`](Self::prelock) more than once (including on
    ///seperate instances of this struct) without calling [`unlock()`](Self::unlock) in between.
    ///You also should avoid using the [`sim_print!()`](crate::sim_print) or [`sim_println!()`](crate::sim_println)
    ///macros while the printer is prelocked.
    pub fn prelock(&mut self) -> Result<(), PoisonError<()>> {//To be more efficient
        if self.guard.is_some() {//Protect against locking more than once
            return Ok(());
        }
        self.guard = Some(PRINT_MUTEX.lock().map_err(|_| PoisonError::new(()))?);
        Ok(())
    }

    ///Unlocks the simulator's output.
    ///
    ///This is also called automatically when the struct is dropped.
    pub fn unlock(&mut self) {
        self.guard = None;
    }

    ///Returns `true` if the simulator's output is currently prelocked.
    pub fn is_prelocked(&self) -> bool {
        self.guard.is_some()
    }

    //TODO perhaps return a VpiError? using vpi_chk_error()?
    pub fn flush(&mut self) -> Result<(), ()> {
        //We can only call vpi_flush() after startup routines have finished, and if we are in the
        //main thread
        if in_startup_routine() || !is_main_thread() {
            return Err(());
        }

        //SAFETY: We're calling vpi_flush() from the main thread and after startup routines have finished
        if unsafe { sv_bindings::vpi_flush() } == 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

impl Write for SimulatorPrinter {
    fn write_str(&mut self, string: &str) -> FmtResult {
        //We can only call vpi_printf() after startup routines have finished
        if in_startup_routine() {
            return Err(Error);
        }

        //We can only print up to i32::MAX bytes since that's all we can check
        //to ensure that the string was printed successfully
        let num_bytes: i32 = string.len().try_into().map_err(|_| Error)?;

        //Need a null terminated string to actually print
        let cstring = CString::new(string).map_err(|_| Error)?;

        //Get the print mutex lock (just for this one print session) if we don't already have it
        let one_time_guard = if self.guard.is_none() {
            Some(PRINT_MUTEX.lock().map_err(|_| Error)?)
        } else {
            None
        };

        //SAFETY: It is safe to cast to *mut PLI_BYTE8 because vpi_printf does not modify the string.
        //We are also guaranteed that the string is null terminated because the pointer is from a
        //CString. We only actually print if we weren't in a startup routine, and finally
        //we're the only thread trying to print at once because of the mutex.
        let num_bytes_written = unsafe {
            sv_bindings::vpi_printf(cstring.as_ptr() as *mut sv_bindings::PLI_BYTE8)
        };

        if num_bytes_written == num_bytes {
            Ok(())
        } else {//EOF or more or less bytes written than expected
            Err(Error)
        }

        //If we did get a one time guard, it will be dropped here
    }
}

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
