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

use crate::object::ObjectHandle;
use crate::result;
use crate::result::Error;
use crate::result::Result;
use crate::startup::panic_if_not_main_thread;

use std::marker::PhantomPinned;
use std::pin::Pin;

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
    //Self referential; raw_cb_data.user_data is made to be a pointer to the outer wrapper struct
    raw_cb_data: sv_bindings::t_cb_data,
    func: Box<dyn FnMut()>,
    _pin: PhantomPinned,
}

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

impl CallbackDataWrapper {
    unsafe extern "C" fn closure_callback_wrapper(cb_data: *mut sv_bindings::t_cb_data) -> i32 {
        let self_ptr: *mut CallbackDataWrapper = (*cb_data).user_data.cast();
        let self_ref: &mut CallbackDataWrapper = &mut *self_ptr;
        //TODO passthru callback reasons to the user
        //No need to pass user data thru since they could have just used a closure which will have
        //whatever data they desire associated with it
        (self_ref.func)();
        0
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

    pub fn register(mut self) -> Result<ObjectHandle> {
        panic_if_not_main_thread!();

        //TODO allow the user to specify most of this via the builder functions

        let time = Time::SimTime { high: 0, low: 1 };
        let ctime: sv_bindings::t_vpi_time = time.into();
        let ctimebox = Box::new(ctime);

        let callback_data_wrapper = CallbackDataWrapper {
            raw_cb_data: sv_bindings::t_cb_data {
                reason: sv_bindings::cbAtStartOfSimTime,
                cb_rtn: Some(CallbackDataWrapper::closure_callback_wrapper),
                obj: std::ptr::null_mut(),
                time: Box::into_raw(ctimebox),
                //time: unsafe { &mut VPI_TIME },//Doesn't work :(
                value: std::ptr::null_mut(),
                index: 0,
                user_data: std::ptr::null_mut(),
            },
            func: self.func.take().ok_or(Error::InvalidCallbackConfig)?,
            _pin: PhantomPinned,
        };

        let mut pinned_boxed_callback_data_wrapper = Box::pin(callback_data_wrapper);
        let pinned_callback_data_wrapper_ref: Pin<&mut CallbackDataWrapper> =
            Pin::as_mut(&mut pinned_boxed_callback_data_wrapper);

        //TODO justify safety
        let cb_handle = unsafe {
            //Safe to do as_mut since modifying a field of the wrapper doesn't move the whole thing
            let callback_data_wrapper_ref = pinned_callback_data_wrapper_ref.get_unchecked_mut();

            let callback_data_wrapper_ptr: *mut CallbackDataWrapper = callback_data_wrapper_ref;
            callback_data_wrapper_ref.raw_cb_data.user_data = callback_data_wrapper_ptr as *mut _;

            let raw_cb_handle_ptr = sv_bindings::vpi_register_cb(&mut callback_data_wrapper_ref.raw_cb_data);
            result::from_last_vpi_call()?;
            ObjectHandle::from_raw(raw_cb_handle_ptr)
        };

        //TODO avoid memory leaks, but still clean up things somehow
        std::mem::forget(pinned_boxed_callback_data_wrapper);

        Ok(cb_handle)
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
