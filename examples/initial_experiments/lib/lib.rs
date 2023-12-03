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

    let product_name = info::product_name().unwrap();
    let version = info::version().unwrap();
    let arguments = info::arguments().unwrap();
    //let dpi_version = info::dpi_version().unwrap();

    //Alrighty, let's traverse the hierarchy!
    for mut handle in object::ObjectChildrenIterator::of_root(object::ObjectType::Module).unwrap() {
        print_module_handle_properties(&mut handle, 0);

        traverse_module_hierarchy_recursively(&mut handle, 1);
    }

    sim_println!("Simulator Product: \"{}\"", product_name);
    sim_println!("Simulator Version: \"{}\"", version);
    sim_println!("Simulator Command Line Arguments: {:?}", arguments);
    //sim_println!("DPI Version: \"{}\"", dpi_version);
}

fn traverse_module_hierarchy_recursively(handle: &mut object::ObjectHandle, indent: usize) {
    for mut handle in object::ObjectChildrenIterator::of(handle, object::ObjectType::Module).unwrap() {
        print_module_handle_properties(&mut handle, indent);


        println!("{:indent$}Children:", "", indent = ((indent * 4) + 2));
        traverse_module_hierarchy_recursively(&mut handle, indent + 1);
    }
}

fn print_module_handle_properties(handle: &mut object::ObjectHandle, indent: usize) {
    println!("{:indent$}Object name: {}", "", handle.get_property_string(object::ObjectProperty::Name).unwrap(), indent = (indent * 4));

    println!("{:indent$}Object type: {}", "", handle.get_property_string(object::ObjectProperty::Type).unwrap(), indent = ((indent * 4) + 2));
    println!("{:indent$}Object full name: {}", "", handle.get_property_string(object::ObjectProperty::FullName).unwrap(), indent = ((indent * 4) + 2));
    //println!("{:indent$}Object size: {}", "", handle.get_property_string(object::ObjectProperty::Size).unwrap(), indent = ((indent * 4) + 2));//Panics since it's not a string
    println!("{:indent$}Object size: {}", "", handle.get_property_i32(object::ObjectProperty::Size).unwrap(), indent = ((indent * 4) + 2));
    println!("{:indent$}Object file: {}", "", handle.get_property_string(object::ObjectProperty::File).unwrap(), indent = ((indent * 4) + 2));
    println!("{:indent$}Object line #: {}", "", handle.get_property_i32(object::ObjectProperty::LineNo).unwrap(), indent = ((indent * 4) + 2));
        
    //Module-specific properties
    //println!("{:indent$}Top module: {}", "", handle.get_property_string(object::ObjectProperty::TopModule).unwrap(), indent = ((indent * 4) + 2);
    println!("{:indent$}Module def name: {}", "", handle.get_property_string(object::ObjectProperty::DefName).unwrap(), indent = ((indent * 4) + 2));
    println!("{:indent$}Module def file: {}", "", handle.get_property_string(object::ObjectProperty::DefFile).unwrap(), indent = ((indent * 4) + 2));

    print!("{:indent$}Ports: ", "", indent = ((indent * 4) + 2));
    list_child_names(handle, object::ObjectType::Port);
    println!();

    print!("{:indent$}Nets: ", "", indent = ((indent * 4) + 2));
    list_child_names(handle, object::ObjectType::Net);
    println!();

    print!("{:indent$}Registers: ", "", indent = ((indent * 4) + 2));
    list_child_names(handle, object::ObjectType::Reg);
    println!();
}

fn list_child_names(handle: &mut object::ObjectHandle, type_: object::ObjectType) {
    for mut handle in object::ObjectChildrenIterator::of(handle, type_).unwrap() {
        print!("{}, ", handle.get_property_string(object::ObjectProperty::Name).unwrap());
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
