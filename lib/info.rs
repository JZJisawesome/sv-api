/*
 * File:    info.rs
 * Brief:   Utilities to query information about the simulator.
 *
 * Copyright (C) 2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Wrappers around vpi_get_vlog_info and svDpiVersion
 *
*/

/*!
 * Utilities to query information about the simulator.
 *
 * If your code needs to know information about the simulator it is running on, command line /
 * option arguments, or the version of the DPI interface supported, this module is for you!
*/

/* ------------------------------------------------------------------------------------------------
 * Submodules
 * --------------------------------------------------------------------------------------------- */

//TODO (includes "mod ..." and "pub mod ...")

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

use crate::startup::panic_if_in_startup_routine;
use crate::startup::panic_if_not_main_thread;

use crate::result;
use crate::result::Error;
use crate::result::Result;

use std::ffi::CStr;

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
struct SafeVlogInfoWrapper<'a> {
    arguments: Vec<&'a CStr>,
    product_name: &'a CStr,
    version: &'a CStr,
}

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Traits And Default Implementations
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Functions
 * --------------------------------------------------------------------------------------------- */

//TODO is 'static a correct assumption? Do the string pointers we are passed last forever?
pub fn arguments() -> Result<Vec<&'static str>> {
    panic_if_in_startup_routine!();
    panic_if_not_main_thread!();

    arguments_cstr()?
        .iter()
        .map(
            |&cstr| cstr.to_str().map_err(|e| Box::new(Error::other(e))), //TODO why is box needed here but not elsewhere?
        )
        .collect()
}

//TODO is 'static a correct assumption? Do the string pointers we are passed last forever?
pub fn arguments_cstr() -> Result<Vec<&'static CStr>> {
    panic_if_in_startup_routine!();
    panic_if_not_main_thread!();

    Ok(vpi_get_safe_vlog_info_wrapper()?.arguments)
}

//TODO is 'static a correct assumption? Do the string pointers we are passed last forever?
pub fn product_name() -> Result<&'static str> {
    panic_if_in_startup_routine!();
    panic_if_not_main_thread!();

    Ok(product_name_cstr()?.to_str().map_err(|e| Error::other(e))?)
}

//TODO is 'static a correct assumption? Do the string pointers we are passed last forever?
pub fn product_name_cstr() -> Result<&'static CStr> {
    panic_if_in_startup_routine!();
    panic_if_not_main_thread!();

    Ok(vpi_get_safe_vlog_info_wrapper()?.product_name)
}

//TODO is 'static a correct assumption? Do the string pointers we are passed last forever?
pub fn version() -> Result<&'static str> {
    panic_if_in_startup_routine!();
    panic_if_not_main_thread!();

    Ok(version_cstr()?.to_str().map_err(|e| Error::other(e))?)
}

//TODO is 'static a correct assumption? Do the string pointers we are passed last forever?
pub fn version_cstr() -> Result<&'static CStr> {
    panic_if_in_startup_routine!();
    panic_if_not_main_thread!();

    Ok(vpi_get_safe_vlog_info_wrapper()?.version)
}

//TODO is 'static a correct assumption? Do the string pointers we are passed last forever?
pub fn dpi_version() -> Result<&'static str> {
    panic_if_in_startup_routine!();
    panic_if_not_main_thread!();

    Ok(dpi_version_cstr()?.to_str().map_err(|e| Error::other(e))?)
}

//TODO is 'static a correct assumption? Do the string pointers we are passed last forever?
pub fn dpi_version_cstr() -> Result<&'static CStr> {
    panic_if_in_startup_routine!();
    panic_if_not_main_thread!();

    //SAFETY: We assume svDpiVersion() returns a proper null-terminated string
    Ok(unsafe {
        let raw_dpi_version_str_ptr = sv_bindings::svDpiVersion();
        debug_assert!(!raw_dpi_version_str_ptr.is_null());
        result::from_last_vpi_call()?;
        std::ffi::CStr::from_ptr(sv_bindings::svDpiVersion())
    })
}

//TODO is 'static a correct assumption? Do the string pointers we are passed last forever?
fn vpi_get_safe_vlog_info_wrapper() -> Result<SafeVlogInfoWrapper<'static>> {
    panic_if_in_startup_routine!();
    panic_if_not_main_thread!();

    let mut raw_info = sv_bindings::t_vpi_vlog_info {
        argc: 0,
        argv: std::ptr::null_mut(),
        product: std::ptr::null_mut(),
        version: std::ptr::null_mut(),
    };

    //SAFETY: vpi_get_vlog_info() is safe to call from the main thread
    //and we are not in a startup routine so we're good.
    let result = unsafe { sv_bindings::vpi_get_vlog_info(&mut raw_info) };

    result::from_last_vpi_call()?;

    if result != 1 {
        return Error::UnknownSimulatorError.into();
    }

    //Package up command line arguments/arguments from an "options" file into a Vec
    let num_args = raw_info
        .argc
        .try_into()
        .expect("Simulator gave us a negative number of arguments even though it shouldn't!");
    let mut arguments = Vec::with_capacity(num_args);
    for i in 0..num_args {
        //SAFETY: We are only offsetting in argv by at most num_args - 1
        //which is the number of elements in argv guaranteed by the LRM.
        let arg_str_ptr = unsafe { *(raw_info.argv.add(i)) };

        debug_assert!(
            !arg_str_ptr.is_null(),
            "Got null pointer from simulator where one was not expected!"
        );

        //SAFETY: We should have been given a valid null-terminated string
        let arg_str = unsafe { std::ffi::CStr::from_ptr(arg_str_ptr) };

        arguments.push(arg_str);
    }

    //Package up the simulator's product name and version
    debug_assert!(
        !raw_info.product.is_null(),
        "Got null pointer from simulator where one was not expected!"
    );
    debug_assert!(
        !raw_info.version.is_null(),
        "Got null pointer from simulator where one was not expected!"
    );

    //SAFETY: We should have been given valid null-terminated strings
    let product_name = unsafe { std::ffi::CStr::from_ptr(raw_info.product) };
    let version = unsafe { std::ffi::CStr::from_ptr(raw_info.version) };

    Ok(SafeVlogInfoWrapper {
        arguments: arguments,
        product_name: product_name,
        version: version,
    })
}

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
