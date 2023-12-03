/*
 * File:    callbacks.rs
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

//TODO (includes "use ..." and "extern crate ...")

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
//TESTING
static mut START_OF_SIM_CALLBACK_DATA: sv_bindings::t_cb_data = sv_bindings::t_cb_data {
    reason: sv_bindings::cbAtStartOfSimTime,
    cb_rtn: None, //Some(start_of_sim_callback),
    obj: std::ptr::null_mut(),
    time: std::ptr::null_mut(),
    //time: unsafe { &mut VPI_TIME },//Doesn't work :(
    value: std::ptr::null_mut(),
    index: 0,
    user_data: std::ptr::null_mut(),
};

/* ------------------------------------------------------------------------------------------------
 * Types
 * --------------------------------------------------------------------------------------------- */

pub struct CallbackBuilder {
    func: Option<Box<dyn FnMut()>>,
}

#[derive(Clone, Copy, Debug)]
pub enum Time {
    ScaledRealTime(f64),
    SimTime { high: u32, low: u32 },
    SuppressTime,
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum TimeType {
    ScaledRealTime = sv_bindings::vpiScaledRealTime,
    SimTime = sv_bindings::vpiSimTime,
    SuppressTime = sv_bindings::vpiSuppressTime,
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum CallbackReason {
    Todo, //TODO
}

struct CallbackDataWrapper {
    //Self referential
    raw_cb_data: sv_bindings::t_cb_data,
    func: Option<Box<dyn FnMut()>>,
}

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

impl CallbackDataWrapper {
    fn make_self_referential(&mut self) {
        //Only call this when the wrapper has been pinned in memory
        let self_ptr: *mut CallbackDataWrapper = self;
        self.raw_cb_data.user_data = self_ptr.cast();
    }
}

impl CallbackBuilder {
    pub const fn new() -> CallbackBuilder {
        CallbackBuilder { func: None }
    }

    pub fn call(mut self, func: impl FnMut() + 'static) -> CallbackBuilder {
        self.func = Some(Box::new(func));
        self
    }

    extern "C" fn closure_wrapper(cb_data: *mut sv_bindings::t_cb_data) -> i32 {
        //TODO justify safety
        unsafe {
            //https://users.rust-lang.org/t/why-cant-a-convert-a-mut-c-void-into-mut-dyn-std-read/95780/7
            //TODO try to make this more efficient in the future
            let thin_ptr: *mut *mut dyn FnMut() = (*cb_data).user_data.cast();
            let fat_ptr: *mut dyn FnMut() = *thin_ptr;
            (*fat_ptr)(); //TODO pass the closure extra info about what happened
                          //No need to re-box things since the closure may be re-called multiple times and we
                          //don't want to drop it
                          //FIXME how should we clean this up at the end?
                          //FIXME what if this is called from multiple threads?
        };

        0
    }

    pub fn register(mut self) {
        //TESTING
        unsafe {
            START_OF_SIM_CALLBACK_DATA.cb_rtn = Some(CallbackBuilder::closure_wrapper); //Some(self.func.unwrap());
            let time = Time::SimTime { high: 0, low: 1 };
            let ctime: sv_bindings::t_vpi_time = time.into();
            let ctimebox = Box::new(ctime);
            START_OF_SIM_CALLBACK_DATA.time = Box::into_raw(ctimebox);

            assert!(self.func.is_some());
            let boxed_closure = self.func.take().unwrap();
            //https://users.rust-lang.org/t/why-cant-a-convert-a-mut-c-void-into-mut-dyn-std-read/95780/7
            //TODO try to make this more efficient in the future
            let fat_ptr: *mut dyn FnMut() = Box::into_raw(boxed_closure);
            let boxed_fat_ptr = Box::new(fat_ptr);
            let thin_ptr: *mut *mut dyn FnMut() = Box::into_raw(boxed_fat_ptr);
            START_OF_SIM_CALLBACK_DATA.user_data = thin_ptr.cast();

            //TODO in the wrapper around the registration callback panic if SupressTime or Time is NULL
            sv_bindings::vpi_register_cb(&mut START_OF_SIM_CALLBACK_DATA);
        }
    }
}

/* ------------------------------------------------------------------------------------------------
 * Traits And Default Implementations
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

impl From<Time> for sv_bindings::t_vpi_time {
    fn from(time: Time) -> sv_bindings::t_vpi_time {
        match time {
            Time::ScaledRealTime(real) => sv_bindings::t_vpi_time {
                type_: TimeType::ScaledRealTime as i32,
                low: 0,
                high: 0,
                real: real,
            },
            Time::SimTime { high, low } => sv_bindings::t_vpi_time {
                type_: TimeType::SimTime as i32,
                low: low,
                high: high,
                real: 0.0,
            },
            Time::SuppressTime => sv_bindings::t_vpi_time {
                type_: TimeType::SuppressTime as i32,
                low: 0,
                high: 0,
                real: 0.0,
            },
        }
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
