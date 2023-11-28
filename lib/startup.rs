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

#[macro_export]
macro_rules! vlog_startup_routines {
    //($($arg:tt),*) => {
    ($($arg:ident),*) => {//TODO support closures, functions in another module or part of a trait (with::), etc
        #[doc(hidden)]
        mod ___sv_api_vlog_startup_routines___ {
            extern "C" fn ___sv_api_call_vlog_startup_routines___() {
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

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

macro_rules! panic_if_in_startup_routine {
    () => {
        if crate::startup::in_startup_routine() {
            //Thanks https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust
            fn ___dummy___() {}
            fn ___type_name_of___<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let ___fn_name___ = ___type_name_of___(___dummy___)
                .strip_suffix("::___dummy___")
                .expect("Suffix should exist!");
            panic!("{}() cannot be called during a startup routine!", ___fn_name___);
        }
    };
}
pub(crate) use panic_if_in_startup_routine;

/* ------------------------------------------------------------------------------------------------
 * Constants
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Static Variables
 * --------------------------------------------------------------------------------------------- */

static INIT_FINISHED: std::sync::OnceLock<()> = std::sync::OnceLock::new();

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

//Should only be called by the vlog_startup_routines! macro, any other use is undefined behaviour
#[doc(hidden)]
pub unsafe fn ___startup_routines_finished___() {
    INIT_FINISHED.set(()).expect("___startup_routines_finished___() was called manually at some point!");
}

pub fn in_startup_routine() -> bool {
    INIT_FINISHED.get().is_none()
}

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
