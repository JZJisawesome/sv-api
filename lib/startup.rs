/*
 * File:    startup.rs
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

use std::thread::ThreadId;

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

#[macro_export]
macro_rules! vlog_startup_routines {
    //($($arg:tt),*) => {
    ($($arg:ident),*) => {//TODO support closures, functions in another module or part of a trait (with::), etc
        #[doc(hidden)]
        mod ___sv_api_vlog_startup_routines___ {
            extern "C" fn ___sv_api_call_vlog_startup_routines___() {
                //SAFETY: This is the only place we allow this function to be called without
                //warning that the caller would violate safety by doing so.
                unsafe { ::sv_api::startup::___startup_routines_started___(); }

                $(
                    super::$arg();
                )*

                //SAFETY: This is the only place we allow this function to be called without
                //warning that the caller would violate safety by doing so.
                //This function allows the other dpi/pli/vpi abstraction functions to be called
                //without panicing, so it must be called in order for user code to be able to
                //use those functions. Those functions can only be used AFTER the startup routines
                //have finished, so we only call this function after the above.
                unsafe { ::sv_api::startup::___startup_routines_finished___(); }
            }

            //SAFETY: We must end vlog_startup_routines with a null pointer, so we do so with None
            //Although there's no unsafe {} here, the simulator which will load the library
            //and reference this symbol will expect this to be upheld
            #[no_mangle]
            #[used]
            static vlog_startup_routines: [Option<extern "C" fn()>; 2usize] = [
                Some(___sv_api_call_vlog_startup_routines___),
                None
            ];
        }
    };
}

macro_rules! this_fn_str {
    () => {{
        //Thanks https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust
        fn ___subfunction___() {}
        fn ___type_name_of___<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        ___type_name_of___(___subfunction___)
            .strip_suffix("::___subfunction___")
            .expect("Suffix should exist!")
    }};
}
pub(crate) use this_fn_str;

macro_rules! panic_if_in_startup_routine {
    () => {{
        if crate::startup::in_startup_routine() {
            panic!(
                "{}() cannot be called during a startup routine!",
                crate::startup::this_fn_str!()
            );
        }
    }};
}
pub(crate) use panic_if_in_startup_routine;

macro_rules! panic_if_not_main_thread {
    () => {{
        if !crate::startup::is_main_thread() {
            panic!(
                "{}() cannot be called by any thread other than the main thread!",
                crate::startup::this_fn_str!()
            );
        }
    }};
}
pub(crate) use panic_if_not_main_thread;

/* ------------------------------------------------------------------------------------------------
 * Static Variables
 * --------------------------------------------------------------------------------------------- */

///Used to uphold invariants the C interface requires
static INIT_FINISHED: std::sync::OnceLock<()> = std::sync::OnceLock::new();

///Used to ensure thread safety since the C interface is not thread safe
static MAIN_THREAD_ID: std::sync::OnceLock<ThreadId> = std::sync::OnceLock::new();

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

///Should only be called by the vlog_startup_routines! macro, any other use is undefined behaviour
#[doc(hidden)]
pub unsafe fn ___startup_routines_started___() {
    let this_thread = std::thread::current();
    let this_thread_id = this_thread.id();
    MAIN_THREAD_ID
        .set(this_thread_id)
        .expect("___startup_routines_started___() was called manually at some point!");
}

///Should only be called by the vlog_startup_routines! macro, any other use is undefined behaviour
#[doc(hidden)]
pub unsafe fn ___startup_routines_finished___() {
    INIT_FINISHED
        .set(())
        .expect("___startup_routines_finished___() was called manually at some point!");
}

///Returns true if we are currently in a startup routine (and thus most SV interfaces are unavailable)
pub fn in_startup_routine() -> bool {
    INIT_FINISHED.get().is_none()
}

///Returns true if we are currently in the main thread
///
///Useful to double-check you aren't in another thread (otherwise no SV interfaces would be
///available)
pub fn is_main_thread() -> bool {
    let this_thread = std::thread::current();
    let this_thread_id = this_thread.id();
    let main_thread_id = MAIN_THREAD_ID
        .get()
        .expect("We should already know the main thread id");
    this_thread_id == *main_thread_id
}

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
