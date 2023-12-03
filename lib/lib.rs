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

//TESTING
/*
static mut START_OF_SIM_CALLBACK_DATA: sv_bindings::t_cb_data = sv_bindings::t_cb_data {
    reason: sv_bindings::cbAtStartOfSimTime as i32,
    cb_rtn: None,//Some(start_of_sim_callback),
    obj: std::ptr::null_mut(),
    time: std::ptr::null_mut(),
    //time: unsafe { &mut VPI_TIME },//Doesn't work :(
    value: std::ptr::null_mut(),
    index: 0,
    user_data: std::ptr::null_mut()
};
vlog_startup_routines!(test123);
fn test123() {
    //sim_println!("Hello World!");//Illegal to do this
    unsafe {
        //START_OF_SIM_CALLBACK_DATA.time = &mut VPI_TIME;//To overcome :(

        //TODO add a callback wrapper to clean up the struct at the end/give it back to user code
        //(basically consume the box and pass the whole boxed START_OF_SIM_CALLBACK_DATA to the user
        //after removing the box to avoid memleaks)
        let time = Time::SimTime{high: 1, low: 2};
        let ctime: sv_bindings::t_vpi_time = time.into();
        let ctimebox = Box::new(ctime);
        START_OF_SIM_CALLBACK_DATA.time = Box::into_raw(ctimebox);

        //TODO in the wrapper around the registration callback panic if SupressTime or Time is NULL
        sv_bindings::vpi_register_cb(
            &mut START_OF_SIM_CALLBACK_DATA
        );
    }
}
static mut VPI_TIME: sv_bindings::t_vpi_time = sv_bindings::t_vpi_time {
    type_: sv_bindings::vpiSimTime as i32,
    low: 1,
    high: 2,
    real: 0.0,
};
extern "C" fn start_of_sim_callback(callback_data_ptr: *mut sv_bindings::t_cb_data) -> sv_bindings::PLI_INT32 {
    sim_println!("In start_of_sim_callback");
    for mut module_handle in ObjectIterator::new(ObjectType::Module) {

        //Get the name
        let name = unsafe { std::ffi::CStr::from_ptr(sv_bindings::vpi_get_str(
            sv_bindings::vpiName as i32,
            module_handle.handle.as_ptr()
        )) }.to_string_lossy().into_owned();
        sim_println!("Module \"{}\" discovered, handle: {:?}.", name, module_handle);

        sim_println!("Let's see if it contains any modules (only one level deep):");
        for submodule_handle in ObjectIterator::new_with_reference(ObjectType::Module, &mut module_handle) {
            let name = unsafe { std::ffi::CStr::from_ptr(sv_bindings::vpi_get_str(
                sv_bindings::vpiName as i32,
                submodule_handle.handle.as_ptr()
            )) }.to_string_lossy().into_owned();
            sim_println!("  Module \"{}\" discovered, handle: {:?}.", name, submodule_handle);
        }

        sim_println!("Let's see if it contains any registers (only one level deep):");
        for net_handle in ObjectIterator::new_with_reference(ObjectType::Reg, &mut module_handle) {
            let name = unsafe { std::ffi::CStr::from_ptr(sv_bindings::vpi_get_str(
                sv_bindings::vpiName as i32,
                net_handle.handle.as_ptr()
            )) }.to_string_lossy().into_owned();
            sim_println!("  Net \"{}\" discovered, handle: {:?}.", name, net_handle);
        }
    }
    sim_println!("Simulator info: {:?}", get_simulator_info());
    0
}
*/
//End of TESTING

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
