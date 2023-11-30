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

//TODO (includes "mod ..." and "pub mod ...")

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

//TODO (includes "use ..." and "extern crate ...")

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

#[macro_export]
macro_rules! sv_print {
    ($($arg:tt)*) => {
        vpi_print_str(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! sv_println {
    ($($arg:tt)*) => {
        sv_print!($($arg)*);
        sv_print!("\n");
    };
}

#[macro_export]
macro_rules! vlog_startup_routines {
    //($($arg:tt),*) => {
    ($($arg:ident),*) => {//TODO support closures, functions in another module or part of a trait (with::), etc
        #[doc(hidden)]
        mod __sv_api_vlog_startup_routines {
            extern "C" fn __sv_api_call_vlog_startup_routines() {
                $(
                    super::$arg();
                )*

                //SAFETY: This is the only place we allow this function to be called without
                //warning that the caller would violate safety by doing so.
                //This function allows the other dpi/pli/vpi abstraction functions to be called
                //without panicing, so it must be called in order for user code to be able to
                //use those functions. Those functions can only be used AFTER the startup routines
                //have finished, so we only call this function after the above.
                unsafe { ::sv_api::startup_routines_finished(); }
            }

            //SAFETY: We must end vlog_startup_routines with a null pointer, so we do so with None
            //Although there's no unsafe {} here, the simulator which will load the library
            //and reference this symbol will expect this to be upheld
            #[no_mangle]
            #[used]
            static vlog_startup_routines: [Option<extern "C" fn()>; 2usize] = [
                Some(__sv_api_call_vlog_startup_routines),
                None
            ];
        }
    };
}

/* ------------------------------------------------------------------------------------------------
 * Constants
 * --------------------------------------------------------------------------------------------- */

//#[no_mangle]
//pub static vlog_startup_routines: [Option<extern "C" fn()>; 2usize] = [Some(call_startup_routines), None];

/* ------------------------------------------------------------------------------------------------
 * Static Variables
 * --------------------------------------------------------------------------------------------- */

static INIT_FINISHED: std::sync::OnceLock<()> = std::sync::OnceLock::new();

/* ------------------------------------------------------------------------------------------------
 * Types
 * --------------------------------------------------------------------------------------------- */

#[derive(Debug)]
#[repr(transparent)]
pub struct ObjectHandle {
    handle: std::ptr::NonNull<sv_bindings::PLI_UINT32>//We don't want a pointer to a pointer
}

#[derive(Debug)]
pub struct ObjectIterator {
    iterator_handle: Option<ObjectHandle>
}


#[derive(Clone, Copy, Debug)]
pub enum Time {
    ScaledRealTime(f64),
    SimTime{high: u32, low: u32},
    SuppressTime
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
pub enum ObjectType {
    Always = sv_bindings::vpiAlways,               // always procedure
    AssignStmt = sv_bindings::vpiAssignStmt,       // quasi-continuous assignment
    Assignment = sv_bindings::vpiAssignment,       // procedural assignment
    Begin = sv_bindings::vpiBegin,                 // block statement
    Case = sv_bindings::vpiCase,                   // case statement
    CaseItem = sv_bindings::vpiCaseItem,           // case statement item
    Constant = sv_bindings::vpiConstant,           // numerical constant or string literal
    ContAssign = sv_bindings::vpiContAssign,       // continuous assignment
    Deassign = sv_bindings::vpiDeassign,           // deassignment statement
    DefParam = sv_bindings::vpiDefParam,           // defparam
    DelayControl = sv_bindings::vpiDelayControl,   // delay statement (e.g., #10)
    Disable = sv_bindings::vpiDisable,             // named block disable statement
    EventControl = sv_bindings::vpiEventControl,   // wait on event, e.g., @e
    EventStmt = sv_bindings::vpiEventStmt,         // event trigger, e.g., ->e
    For = sv_bindings::vpiFor,                     // for statement
    Force = sv_bindings::vpiForce,                 // force statement
    Forever = sv_bindings::vpiForever,             // forever statement
    Fork = sv_bindings::vpiFork,                   // fork-join block
    FuncCall = sv_bindings::vpiFuncCall,           // function call
    Function = sv_bindings::vpiFunction,           // function
    Gate = sv_bindings::vpiGate,                   // primitive gate
    If = sv_bindings::vpiIf,                       // if statement
    IfElse = sv_bindings::vpiIfElse,               // if–else statement
    Initial = sv_bindings::vpiInitial,             // initial procedure
    IntegerVar = sv_bindings::vpiIntegerVar,       // integer variable
    InterModPath = sv_bindings::vpiInterModPath,   // intermodule wire delay
    Iterator = sv_bindings::vpiIterator,           // iterator
    IODecl = sv_bindings::vpiIODecl,               // input/output declaration
    Memory = sv_bindings::vpiMemory,               // behavioral memory
    MemoryWord = sv_bindings::vpiMemoryWord,       // single word of memory
    ModPath = sv_bindings::vpiModPath,             // module path for path delays
    Module = sv_bindings::vpiModule,               // module instance
    NamedBegin = sv_bindings::vpiNamedBegin,       // named block statement
    NamedEvent = sv_bindings::vpiNamedEvent,       // event variable
    NamedFork = sv_bindings::vpiNamedFork,         // named fork-join block
    Net = sv_bindings::vpiNet,                     // scalar or vector net
    NetBit = sv_bindings::vpiNetBit,               // bit of vector net
    NullStmt = sv_bindings::vpiNullStmt,           // a semicolon. Ie. #10 ;
    Operation = sv_bindings::vpiOperation,         // behavioral operation
    ParamAssign = sv_bindings::vpiParamAssign,     // module parameter assignment
    Parameter = sv_bindings::vpiParameter,         // module parameter
    PartSelect = sv_bindings::vpiPartSelect,       // part-select
    PathTerm = sv_bindings::vpiPathTerm,           // terminal of module path
    Port = sv_bindings::vpiPort,                   // module port
    PortBit = sv_bindings::vpiPortBit,             // bit of vector module port
    PrimTerm = sv_bindings::vpiPrimTerm,           // primitive terminal
    RealVar = sv_bindings::vpiRealVar,             // real variable
    Reg = sv_bindings::vpiReg,                     // scalar or vector reg
    RegBit = sv_bindings::vpiRegBit,               // bit of vector reg
    Release = sv_bindings::vpiRelease,             // release statement
    Repeat = sv_bindings::vpiRepeat,               // repeat statement
    RepeatControl = sv_bindings::vpiRepeatControl, // repeat control in an assign stmt
    SchedEvent = sv_bindings::vpiSchedEvent,       // vpi_put_value() event
    SpecParam = sv_bindings::vpiSpecParam,         // specparam
    Switch = sv_bindings::vpiSwitch,               // transistor switch
    SysFuncCall = sv_bindings::vpiSysFuncCall,     // system function call
    SysTaskCall = sv_bindings::vpiSysTaskCall,     // system task call
    TableEntry = sv_bindings::vpiTableEntry,       // UDP state table entry
    Task = sv_bindings::vpiTask,                   // task
    TaskCall = sv_bindings::vpiTaskCall,           // task call
    Tchk = sv_bindings::vpiTchk,                   // timing check
    TchkTerm = sv_bindings::vpiTchkTerm,           // terminal of timing check
    TimeVar = sv_bindings::vpiTimeVar,             // time variable
    TimeQueue = sv_bindings::vpiTimeQueue,         // simulation event queue
    Udp = sv_bindings::vpiUdp,                     // user-defined primitive
    UdpDefn = sv_bindings::vpiUdpDefn,             // UDP definition
    UserSystf = sv_bindings::vpiUserSystf,         // user-defined system task/function
    VarSelect = sv_bindings::vpiVarSelect,         // variable array selection
    Wait = sv_bindings::vpiWait,                   // wait statement
    While = sv_bindings::vpiWhile,                 // while statement

    // Object types added with 1364-2001
    Attribute = sv_bindings::vpiAttribute,              // attribute of an object
    BitSelect = sv_bindings::vpiBitSelect,              // Bit-select of parameter, var select
    Callback = sv_bindings::vpiCallback,                // callback object
    DelayTerm = sv_bindings::vpiDelayTerm,              // Delay term which is a load or driver
    DelayDevice = sv_bindings::vpiDelayDevice,          // Delay object within a net
    Frame = sv_bindings::vpiFrame,                      // reentrant task/func frame
    GateArray = sv_bindings::vpiGateArray,              // gate instance array
    ModuleArray = sv_bindings::vpiModuleArray,          // module instance array
    PrimitiveArray = sv_bindings::vpiPrimitiveArray,    // vpiprimitiveArray type
    NetArray = sv_bindings::vpiNetArray,                // multidimensional net
    Range = sv_bindings::vpiRange,                      // range declaration
    RegArray = sv_bindings::vpiRegArray,                // multidimensional reg
    SwitchArray = sv_bindings::vpiSwitchArray,          // switch instance array
    UdpArray = sv_bindings::vpiUdpArray,                // UDP instance array
    ContAssignBit = sv_bindings::vpiContAssignBit,      // Bit of a vector continuous assignment
    NamedEventArray = sv_bindings::vpiNamedEventArray,  // multidimensional named event

    // Object types added with 1364-2005
    IndexedPartSelect = sv_bindings::vpiIndexedPartSelect,  // Indexed part-select object
    GenScopeArray = sv_bindings::vpiGenScopeArray,          // array of generated scopes
    GenScope = sv_bindings::vpiGenScope,                    // A generated scope
    GenVar = sv_bindings::vpiGenVar,                        // Object used to instantiate gen scopes
}

#[derive(Debug)]
pub struct SimulatorInfo<'a> {
    //TODO provide argc and argv 
    product_name: &'a str,
    version: &'a str
}

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

impl ObjectIterator {
    fn new(object_type: ObjectType) -> ObjectIterator {
        //FIXME justify safety
        let raw_handle = unsafe { sv_bindings::vpi_iterate(object_type as i32, std::ptr::null_mut()) };

        let iterator_handle = if raw_handle.is_null() {
            None
        } else {
            Some(ObjectHandle { handle: std::ptr::NonNull::new(raw_handle).unwrap() })
        };

        ObjectIterator {
            iterator_handle: iterator_handle
        }
    }

    fn new_with_reference(object_type: ObjectType, reference: &mut ObjectHandle) -> ObjectIterator {
        //FIXME justify safety
        let raw_handle = unsafe { sv_bindings::vpi_iterate(object_type as i32, reference.handle.as_ptr()) };

        let iterator_handle = if raw_handle.is_null() {
            None
        } else {
            Some(ObjectHandle { handle: std::ptr::NonNull::new(raw_handle).unwrap() })
        };

        ObjectIterator {
            iterator_handle: iterator_handle
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

impl Drop for ObjectHandle {
    fn drop(&mut self) {
        //Guaranteed the handle is not null (it is a NonNull)
        //FIXME test this on a simulator that supports it
        /*
        unsafe {
            //FIXME justify safety
            sv_bindings::vpi_release_handle(handle.as_ptr());
        }
        */
    }
}

impl Iterator for ObjectIterator {
    type Item = ObjectHandle;

    fn next(&mut self) -> Option<ObjectHandle> {
        let unwrapped_iterator_handle = self.iterator_handle.as_mut()?;

        //FIXME justify safety
        let raw_handle_from_scan = unsafe {
            sv_bindings::vpi_scan(unwrapped_iterator_handle.handle.as_ptr())
        };

        if raw_handle_from_scan.is_null() {
            self.iterator_handle = None;//Iterator handle is now invalid (this drops it)
            None
        } else {
            Some(ObjectHandle { handle: std::ptr::NonNull::new(raw_handle_from_scan).unwrap() })
        }
    }
}

impl From<Time> for sv_bindings::t_vpi_time {
    fn from(time: Time) -> sv_bindings::t_vpi_time {
        match time {
            Time::ScaledRealTime(real) => sv_bindings::t_vpi_time {
                type_: TimeType::ScaledRealTime as i32,
                low: 0,
                high: 0,
                real: real
            },
            Time::SimTime{high, low} => sv_bindings::t_vpi_time {
                type_: TimeType::SimTime as i32,
                low: low,
                high: high,
                real: 0.0
            },
            Time::SuppressTime => sv_bindings::t_vpi_time {
                type_: TimeType::SuppressTime as i32,
                low: 0,
                high: 0,
                real: 0.0
            }
        }
    }
}

/* ------------------------------------------------------------------------------------------------
 * Functions
 * --------------------------------------------------------------------------------------------- */

//TESTING
/*
vlog_startup_routines!(test123);
fn test123() {
    //sv_println!("Hello World!");//Illegal to do this
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
static mut START_OF_SIM_CALLBACK_DATA: sv_bindings::t_cb_data = sv_bindings::t_cb_data {
    reason: sv_bindings::cbAtStartOfSimTime as i32,
    cb_rtn: Some(start_of_sim_callback),
    obj: std::ptr::null_mut(),
    time: std::ptr::null_mut(),
    //time: unsafe { &mut VPI_TIME },//Doesn't work :(
    value: std::ptr::null_mut(),
    index: 0,
    user_data: std::ptr::null_mut()
};
extern "C" fn start_of_sim_callback(callback_data_ptr: *mut sv_bindings::t_cb_data) -> sv_bindings::PLI_INT32 {
    sv_println!("In start_of_sim_callback");
    for mut module_handle in ObjectIterator::new(ObjectType::Module) {

        //Get the name
        let name = unsafe { std::ffi::CStr::from_ptr(sv_bindings::vpi_get_str(
            sv_bindings::vpiName as i32,
            module_handle.handle.as_ptr()
        )) }.to_string_lossy().into_owned();
        sv_println!("Module \"{}\" discovered, handle: {:?}.", name, module_handle);

        sv_println!("Let's see if it contains any modules (only one level deep):");
        for submodule_handle in ObjectIterator::new_with_reference(ObjectType::Module, &mut module_handle) {
            let name = unsafe { std::ffi::CStr::from_ptr(sv_bindings::vpi_get_str(
                sv_bindings::vpiName as i32,
                submodule_handle.handle.as_ptr()
            )) }.to_string_lossy().into_owned();
            sv_println!("  Module \"{}\" discovered, handle: {:?}.", name, submodule_handle);
        }

        sv_println!("Let's see if it contains any registers (only one level deep):");
        for net_handle in ObjectIterator::new_with_reference(ObjectType::Reg, &mut module_handle) {
            let name = unsafe { std::ffi::CStr::from_ptr(sv_bindings::vpi_get_str(
                sv_bindings::vpiName as i32,
                net_handle.handle.as_ptr()
            )) }.to_string_lossy().into_owned();
            sv_println!("  Net \"{}\" discovered, handle: {:?}.", name, net_handle);
        }
    }
    sv_println!("Simulator info: {:?}", get_simulator_info());
    0
}
*/
//End of TESTING

//Should only be called by the vlog_startup_routines! macro, any other use is undefined behaviour
pub unsafe fn startup_routines_finished() {
    INIT_FINISHED.set(()).expect("startup_routines_finished() was called manually at some point!");
}

pub fn get_dpi_version() -> &'static str {
    INIT_FINISHED.get().expect("Attempt to get DPI version during a startup routine!");
    unsafe {
        //FIXME is the string pointer guaranteed to always be valid (or should we make a copy)?
        std::ffi::CStr::from_ptr(sv_bindings::svDpiVersion())
    }.to_str().unwrap()
}

pub fn vpi_print_str(string: &str) {
    let cstr = std::ffi::CString::new(string).unwrap();
    vpi_print_cstr(&cstr);
}

pub fn vpi_print_cstr(cstr: &std::ffi::CStr) {
    INIT_FINISHED.get().expect("Attempt to print using the VPI during a startup routine!");
    unsafe {
        //Safety: It is safe to cast to *mut PLI_BYTE8 because vpi_printf does not modify the string
        sv_bindings::vpi_printf(cstr.as_ptr() as *mut sv_bindings::PLI_BYTE8);
    }
}

//TODO is 'static a correct assumption?
pub fn get_simulator_info() -> Option<SimulatorInfo<'static>> {
    INIT_FINISHED.get().expect("Attempt to get simulator info during a startup routine!");

    let mut raw_info = sv_bindings::t_vpi_vlog_info {
        argc: 0,
        argv: std::ptr::null_mut(),
        product: std::ptr::null_mut(),
        version: std::ptr::null_mut(),
    };
    //TODO justify safety
    unsafe {
        if sv_bindings::vpi_get_vlog_info(&mut raw_info) != 1 {
            return None;
        }
    }
    //TODO justify safety
    //TODO what if a string is null?
    Some(SimulatorInfo {
        product_name: unsafe { std::ffi::CStr::from_ptr(raw_info.product) }.to_str().ok()?,
        version: unsafe { std::ffi::CStr::from_ptr(raw_info.version) }.to_str().ok()?
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
