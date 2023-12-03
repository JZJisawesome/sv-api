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
 * not safe to call it from a startup routine__, if you do, a panic will occur. This will also happen
 * if you call printing functions / invoke printing macros from a thread other than the main one,
 * which also isn't permissible.
 *
*/

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

use crate::result;
use crate::result::Error;
use crate::result::Result;
use crate::startup::panic_if_in_startup_routine;
use crate::startup::panic_if_not_main_thread;

use std::ffi::{CStr, CString};
use std::fmt::{self, Write};

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
            .expect("Failure writing to simulator output with sim_print!()");
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
            .expect("Failure writing to simulator output with sim_println!()");
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
    pub const fn new() -> Self {
        Self {}
    }

    pub fn flush(&mut self) -> Result<()> {
        panic_if_in_startup_routine!();
        panic_if_not_main_thread!();

        //SAFETY: We're calling vpi_flush() from the main thread and after startup routines have finished
        let result = unsafe { sv_bindings::vpi_flush() };

        result::from_last_vpi_call()?;

        if result == 0 {
            Ok(())
        } else {
            Error::UnknownSimulatorError.into()
        }
    }

    ///Write textual data to the simulator's output
    ///
    ///Since the backing `vpi_printf()` function used is not thread safe, ['write_str()`]
    ///will panic if you attempt to call it from a thread other than the main one.
    ///
    ///It will also panic if you attempt to call it during a start routine since
    ///`vpi_printf()` also doesn't support this.
    ///
    ///Unlike [`write_str()`](std::fmt::Write::write_str), this writes a C string rather
    ///than a Rust string, which can be useful if you need to write-non UTF-8 data.
    pub fn write_cstr(&mut self, cstr: &CStr) -> Result<()> {
        panic_if_in_startup_routine!();
        panic_if_not_main_thread!();

        //We can only print up to i32::MAX bytes since that's all we can check
        //to ensure that the string was printed successfully
        let num_bytes: i32 = cstr
            .to_bytes()
            .len()
            .try_into()
            .map_err(|e| Error::other(e))?;

        //Null-terminated format string
        const FORMAT_STRING_PTR: *const sv_bindings::PLI_BYTE8 = b"%s\0".as_ptr().cast();

        //SAFETY: It is safe to cast to *mut because vpi_printf does not modify the string.
        //We are also guaranteed that the 2nd string is null terminated because the pointer is from a
        //CStr. We only actually print if we weren't in a startup routine, and finally we only print
        //if we are in the main thread.
        //Additionally, the format string is properly formed and is null terminated.
        let num_bytes_written =
            unsafe { sv_bindings::vpi_printf(FORMAT_STRING_PTR as *mut _, cstr.as_ptr()) };

        result::from_last_vpi_call()?;

        if num_bytes_written == num_bytes {
            Ok(())
        } else {
            //EOF or more or less bytes written than expected
            Error::UnknownSimulatorError.into()
        }
    }
}

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

impl Write for SimulatorPrinter {
    ///Write textual data to the simulator's output
    ///
    ///Since the backing `vpi_printf()` function used is not thread safe, ['write_str()`]
    ///will panic if you attempt to call it from a thread other than the main one.
    ///
    ///It will also panic if you attempt to call it during a start routine since
    ///`vpi_printf()` also doesn't support this.
    ///
    ///Unlike [`write_cstr()`](SimulatorPrinter::write_cstr), this writes a Rust string
    ///rather than a C string.
    fn write_str(&mut self, str_: &str) -> fmt::Result {
        panic_if_in_startup_routine!();
        panic_if_not_main_thread!();

        //Need a null terminated string to actually print
        let cstring = CString::new(str_).map_err(|_| fmt::Error)?;

        self.write_cstr(&cstring).map_err(|_| fmt::Error)
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
