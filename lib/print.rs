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
 * TODO rustdoc for this file here
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

#[macro_export]
macro_rules! sim_print {
    ($($arg:tt)*) => {{
        use std::fmt::Write as _;
        write!(::sv_api::print::SimulatorPrinter::new(), $($arg)*)
            .expect("Failure writing to simulator output with sim_print!(), are you in a startup routine?");
    }};
}

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

#[derive(Debug)]
pub struct SimulatorPrinter {
    guard: Option<MutexGuard<'static, ()>>
}

#[derive(Debug)]
pub struct SimulatorPrinterLock<'a> {
    sim_printer: &'a mut SimulatorPrinter
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

    pub fn lock(&mut self) -> Result<&mut Self, PoisonError<()>> {
        self.guard = Some(PRINT_MUTEX.lock().map_err(|_| PoisonError::new(()))?);
        Ok(self)
    }

    pub fn unlock(&mut self) {
        self.guard = None;
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
