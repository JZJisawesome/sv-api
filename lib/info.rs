/*
 * File:    info.rs
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

use crate::startup::panic_if_in_startup_routine;
use crate::startup::panic_if_not_main_thread;

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
pub struct SimulatorInfo<'a> {
    arguments: Vec<&'a str>,
    product_name: &'a str,
    version: &'a str
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
pub fn get_simulator_info() -> Option<SimulatorInfo<'static>> {
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
    unsafe {
        if sv_bindings::vpi_get_vlog_info(&mut raw_info) != 1 {
            return None;
        }
    }

    //Package up command line arguments/arguments from an "options" file into a Vec
    let num_args = raw_info.argc.try_into().ok()?;
    let mut arguments = Vec::with_capacity(num_args);
    for i in 0..num_args {
        //SAFETY: We are only offsetting in argv by at most num_args - 1
        //which is the number of elements in argv guaranteed by the LRM.
        let arg_str_ptr = unsafe { *(raw_info.argv.add(i)) };

        if arg_str_ptr.is_null() {
            return None;//This should never happen, but just in case it does...
        }

        //SAFETY: We should have been given a valid null-terminated string
        let arg_str = unsafe { std::ffi::CStr::from_ptr(arg_str_ptr) }.to_str().ok()?;

        arguments.push(arg_str);
    }

    //Package up the simulator's product name and version
    if raw_info.product.is_null() || raw_info.version.is_null() {
        return None;//This should never happen, but just in case it does...
    }

    //SAFETY: We should have been given a valid null-terminated string
    let product_name    = unsafe { std::ffi::CStr::from_ptr(raw_info.product) }.to_str().ok()?;
    let version         = unsafe { std::ffi::CStr::from_ptr(raw_info.version) }.to_str().ok()?;

    Some(SimulatorInfo {
        arguments:      arguments,
        product_name:   product_name,
        version:        version
    })
}

pub fn get_dpi_version() -> &'static str {
    panic_if_in_startup_routine!();
    panic_if_not_main_thread!();
    unsafe {
        //FIXME is the string pointer guaranteed to always be valid (or should we make a copy)?
        std::ffi::CStr::from_ptr(sv_bindings::svDpiVersion())
    }.to_str().unwrap()
}

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
