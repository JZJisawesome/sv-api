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
pub enum Error/*<'a>*/ {
    Unknown,
    //TODO others
    Known {
        state_when_error_occurred: State,
        severity: Severity,
        /*
        message: &'a CStr,
        product: &'a CStr,
        code: &'a CStr,
        file: &'a CStr,
        */
        line: i32,
    },
    Other(Box<dyn std::error::Error>)//A non sv-api error
}

//We box the Error to reduce the cost of the normal case when there are no errors
pub type Result<T> = std::result::Result<T, Box<Error>>;

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum State {
    Compile = sv_bindings::vpiCompile,
    PLI = sv_bindings::vpiPLI,
    Run = sv_bindings::vpiRun
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Severity {
    Notice = sv_bindings::vpiNotice,
    Warning = sv_bindings::vpiWarning,
    Error = sv_bindings::vpiError,
    System = sv_bindings::vpiSystem,
    Internal = sv_bindings::vpiInternal
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
            _ => None
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Unknown                  => write!(f, "Unknown or unclassified error"),
            Error::Known { .. }             => write!(f, "Known: {}", "???"),//TODO display this properly
            Error::Other(other_boxed_error) => write!(f, "Other: {}", other_boxed_error)
        }
    }
}

impl<T> From<Error> for Result<T> {
    fn from(error: Error) -> Self {
        Err(Box::new(error))
    }
}

/* ------------------------------------------------------------------------------------------------
 * Functions
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
