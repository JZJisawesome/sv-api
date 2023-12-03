/*
 * File:    result.rs
 * Brief:   TODO
 *
 * Copyright (C) 2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * TODO longer description
 *
*/

/*!
 * TODO rustdoc for this file here
*/

/* ------------------------------------------------------------------------------------------------
 * Submodules
 * --------------------------------------------------------------------------------------------- */

//TODO (includes "mod ..." and "pub mod ...")

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

use std::ffi::CStr;
use std::fmt;
use std::fmt::Display;

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

//TODO (also pub(crate) use the_macro statements here too)

/* ------------------------------------------------------------------------------------------------
 * Constants
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Static Variables
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Types
 * --------------------------------------------------------------------------------------------- */

#[derive(Debug)]
#[non_exhaustive]
pub enum Error /*<'a>*/ {
    UnknownSimulatorError,
    //TODO others
    //TODO is 'static a correct assumption? Do the string pointers we are passed last forever?
    KnownSimulatorError {
        state_when_error_occurred: State,
        severity: Severity,
        /*
        message: &'a CStr,
        product: &'a CStr,
        code: &'a CStr,
        file: &'a CStr,
        */
        message: &'static CStr,
        product: &'static CStr,
        code: &'static CStr,
        file: &'static CStr,
        line: i32,
    },
    EnumConversion {
        from_int: i32,
        to_enum: &'static str,
    },
    Other(Box<dyn std::error::Error>), //A non sv-api error
}

//We box the Error to reduce the cost of the normal case when there are no errors
pub type Result<T> = std::result::Result<T, Box<Error>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum State {
    Compile = sv_bindings::vpiCompile,
    PLI = sv_bindings::vpiPLI,
    Run = sv_bindings::vpiRun,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Severity {
    Notice = sv_bindings::vpiNotice,
    Warning = sv_bindings::vpiWarning,
    Error = sv_bindings::vpiError,
    System = sv_bindings::vpiSystem,
    Internal = sv_bindings::vpiInternal,
}

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

impl Error {
    ///Useful for making error handling more concise
    pub fn other(error: impl std::error::Error + 'static) -> Self {
        Error::Other(Box::new(error))
    }
}

/* ------------------------------------------------------------------------------------------------
 * Traits And Default Implementations
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Other(other_boxed_error) => Some(other_boxed_error.as_ref()),
            _ => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnknownSimulatorError => {
                write!(f, "Unknown or unclassified error from the simulator")
            }
            Error::KnownSimulatorError {
                state_when_error_occurred,
                severity,
                message,
                product,
                code,
                file,
                line,
            } => {
                write!(
                    f,
                    "Simulator encountered an error!\n\
                     State when error occurred: {}\n\
                     Severity of error:         {}\n\
                     Message:                   {}\n\
                     Product:                   {}\n\
                     Code:                      {}\n\
                     File:                      {}\n\
                     Line:                      {}\n",
                    state_when_error_occurred,
                    severity,
                    message.to_string_lossy(),
                    product.to_string_lossy(),
                    code.to_string_lossy(),
                    file.to_string_lossy(),
                    line
                )
            }
            Error::EnumConversion { from_int, to_enum } => write!(
                f,
                "Could not convert {} (i32) to type {}",
                from_int, to_enum
            ),
            Error::Other(other_boxed_error) => write!(f, "Other: {}", other_boxed_error),
        }
    }
}

impl<T> From<Error> for Result<T> {
    fn from(error: Error) -> Self {
        Err(Box::new(error))
    }
}

impl TryFrom<i32> for State {
    type Error = Box<Error>;

    fn try_from(value: i32) -> Result<Self> {
        match value {
            sv_bindings::vpiCompile => Ok(State::Compile),
            sv_bindings::vpiPLI => Ok(State::PLI),
            sv_bindings::vpiRun => Ok(State::Run),
            _ => Error::EnumConversion {
                from_int: value,
                to_enum: "State",
            }
            .into(),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Compile => write!(f, "Compile"),
            State::PLI => write!(f, "PLI"),
            State::Run => write!(f, "Run"),
        }
    }
}

impl TryFrom<i32> for Severity {
    type Error = Box<Error>;

    fn try_from(value: i32) -> Result<Self> {
        match value {
            sv_bindings::vpiNotice => Ok(Severity::Notice),
            sv_bindings::vpiWarning => Ok(Severity::Warning),
            sv_bindings::vpiError => Ok(Severity::Error),
            sv_bindings::vpiSystem => Ok(Severity::System),
            sv_bindings::vpiInternal => Ok(Severity::Internal),
            _ => Error::EnumConversion {
                from_int: value,
                to_enum: "Severity",
            }
            .into(),
        }
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Notice => write!(f, "Notice"),
            Severity::Warning => write!(f, "Warning"),
            Severity::Error => write!(f, "Error"),
            Severity::System => write!(f, "System"),
            Severity::Internal => write!(f, "Internal"),
        }
    }
}

/* ------------------------------------------------------------------------------------------------
 * Functions
 * --------------------------------------------------------------------------------------------- */

///Returns Ok(()) if nothing went wrong
pub(crate) fn from_last_vpi_call() -> Result<()> {
    let mut raw_error_info = sv_bindings::s_vpi_error_info {
        state: 0,
        level: 0,
        message: std::ptr::null_mut(),
        product: std::ptr::null_mut(),
        code: std::ptr::null_mut(),
        file: std::ptr::null_mut(),
        line: 0,
    };

    //SAFETY: raw_error_info is valid, so vpi_check_error is free to modify it
    let result = unsafe { sv_bindings::vpi_chk_error(&mut raw_error_info) };

    if result == 0 {
        Ok(())
    } else {
        Error::KnownSimulatorError {
            state_when_error_occurred: raw_error_info
                .state
                .try_into()
                .expect("We should get a legal state from the simulator"),
            severity: raw_error_info
                .level
                .try_into()
                .expect("We should get a legal level from the simulator"),
            message: unsafe { CStr::from_ptr(raw_error_info.message) },
            product: unsafe { CStr::from_ptr(raw_error_info.product) },
            code: unsafe { CStr::from_ptr(raw_error_info.code) },
            file: unsafe { CStr::from_ptr(raw_error_info.file) },
            line: raw_error_info.line,
        }
        .into()
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
