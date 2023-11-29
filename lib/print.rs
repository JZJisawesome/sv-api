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
 * is useful if you need a [`Writer`] corresponding to the simulator's output.
 *
 * Since these use `vpi_printf()` internally, and according to the IEEE 1800-2017 standard, __it is
 * not safe to call it from a startup routine__, if you do, a panic will occur or you'll
 * get an [Err] depending on exactly what you're working with. This will also happen if you call
 * printing functions / invoke printing macros from a thread other than the main one, which also
 * isn't permissible.
 *
*/

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

use std::ffi::CString;
use std::fmt::{Error, Write};
use std::fmt::Result as FmtResult;

use crate::startup::in_startup_routine;
use crate::startup::is_main_thread;

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
///This will panic if it is called from a thread other than the main thread or during a startup
///routine
///
///Analagous to [`print!()`](std::print).
#[macro_export]
macro_rules! sim_print {
    ($($arg:tt)*) => {{
        use std::fmt::Write as _;
        write!(::sv_api::print::SimulatorPrinter::new(), $($arg)*)
            .expect("Failure writing to simulator output with sim_print!(), are you in a startup routine or not in the main thread?");
    }};
}

///Prints to the simulator output, with a newline.
///
///This may or not be the same as printing to the regular Rust standard output.
///It is preferable if you want, for example, ensure you are logging to the same log file that the
///simulator itself is writing to.
///
///This will panic if it is called from a thread other than the main thread or during a startup
///routine
///
///Analagous to [`println!()`](std::println).
#[macro_export]
macro_rules! sim_println {
    ($($arg:tt)*) => {{
        use std::fmt::Write as _;
        writeln!(::sv_api::print::SimulatorPrinter::new(), $($arg)*)
            .expect("Failure writing to simulator output with sim_println!(), are you in a startup routine or not in the main thread?");
    }};
}

/* ------------------------------------------------------------------------------------------------
 * Types
 * --------------------------------------------------------------------------------------------- */

///A handle to the simulator's output.
///
///Useful for if you need a Writer struct corresponding to the simulator's output.
///
#[derive(Debug)]
pub struct SimulatorPrinter {}

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

impl SimulatorPrinter {
    pub fn new() -> Self {
        Self {}
    }
}

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

impl Write for SimulatorPrinter {
    ///Write formatted textual data to the simulator's output
    ///Since the backing `vpi_printf()` function used is not thread safe, write_str will return an
    ///[`Err`] if you attempt to call [`write_str()`] from a thread other than the main one.
    ///
    ///It will also [`Err`] out if you attempt to call it during a start routine since
    ///`vpi_printf()` also doesn't support this.
    fn write_str(&mut self, string: &str) -> FmtResult {
        //We can only call vpi_printf() after startup routines have finished, and if we are in the
        //main thread
        if in_startup_routine() || !is_main_thread() {
            return Err(Error);
        }

        //We can only print up to i32::MAX bytes since that's all we can check
        //to ensure that the string was printed successfully
        let num_bytes: i32 = string.len().try_into().map_err(|_| Error)?;

        //Need a null terminated string to actually print
        let cstring = CString::new(string).map_err(|_| Error)?;

        //SAFETY: It is safe to cast to *mut PLI_BYTE8 because vpi_printf does not modify the string.
        //We are also guaranteed that the string is null terminated because the pointer is from a
        //CString. We only actually print if we weren't in a startup routine, and finally
        //we only print if we are in the main thread.
        let num_bytes_written = unsafe {
            sv_bindings::vpi_printf(cstring.as_ptr() as *mut sv_bindings::PLI_BYTE8)
        };

        if num_bytes_written == num_bytes {
            Ok(())
        } else {//EOF or more or less bytes written than expected
            Err(Error)
        }
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
