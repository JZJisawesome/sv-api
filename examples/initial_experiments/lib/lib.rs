/*
 * File:    lib.rs
 * Brief:   Example DPI/PLI/VPI shared object module.
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

use sv_api::*;

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

vlog_startup_routines!(hello_world, setup_callback);

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

//TODO includes "type"-defs, structs, enums, unions, etc

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

fn hello_world() {
    println!("Hello simulator from Rust!");
    //sim_println!("Hello, world from SystemVerilog!");//Not allowed during a startup routine
}

fn setup_callback() {
    let time = callbacks::Time::SimTime { high: 1, low: 2 };
    callbacks::CallbackBuilder::new()
        .call(start_of_simulation_callback)
        .register();
}

fn start_of_simulation_callback() {
    sim_println!("Now we can do more stuff!");
    use std::fmt::Write as _;
    let mut printer = print::SimulatorPrinter::new();
    writeln!(printer, "Multiple writes to the simulator's output...").unwrap();
    writeln!(printer, "...but done using a Writer!").unwrap();
    printer.flush().unwrap();
    sim_println!("Alrighty onto more interesting things!");

    std::thread::spawn(|| {
        //sim_println!("Hello from a thread!");//This would panic too
    });

    sim_println!("Simulator Product: \"{}\"", info::product_name().unwrap());
    sim_println!("Simulator Version: \"{}\"", info::version().unwrap());
    sim_println!("Simulator Command Line Arguments: {:?}", info::arguments().unwrap());
    //sim_println!("DPI Version: \"{}\"", info::dpi_version().unwrap());

    //TODO
}

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
