/*
 * File:    lib.rs
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

pub mod callbacks;
pub mod info;
pub mod object;
pub mod print;
pub mod result;
pub mod startup;

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

//TODO (includes "use ..." and "extern crate ...")

/* ------------------------------------------------------------------------------------------------
 * Types
 * --------------------------------------------------------------------------------------------- */

pub enum Diagnostic {
    Nothing = 0,
    TimeAndLocation = 1,
    TimeLocationAndResources = 2
}

/* ------------------------------------------------------------------------------------------------
 * Functions
 * --------------------------------------------------------------------------------------------- */

pub fn stop(diag: Diagnostic) {
    startup::panic_if_in_startup_routine!();
    startup::panic_if_not_main_thread!();
    //TODO justify safety
    unsafe {
        sv_bindings::vpi_control(sv_bindings::vpiStop, diag as i32);
    }
}

pub fn finish(diag: Diagnostic) {
    startup::panic_if_in_startup_routine!();
    startup::panic_if_not_main_thread!();
    //TODO justify safety
    unsafe {
        sv_bindings::vpi_control(sv_bindings::vpiFinish, diag as i32);
    }
}

//TODO reset() (more complicated)

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
