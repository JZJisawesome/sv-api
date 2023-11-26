/*
 * File:    lib.rs
 * Brief:   TODO
 *
 * Copyright (C) TODO John Jekel
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

//TODO

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

//TODO (includes "use ..." and "extern crate ...")

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

macro_rules! sv_print {
    ($($arg:tt)*) => {
        vpi_print_str(&format!($($arg)*));
    };
}

macro_rules! sv_println {
    ($($arg:tt)*) => {
        sv_print!($($arg)*);
        sv_print!("\n");
    };
}

/* ------------------------------------------------------------------------------------------------
 * Constants
 * --------------------------------------------------------------------------------------------- */

#[no_mangle]
pub static vlog_startup_routines: [Option<extern "C" fn()>; 2usize] = [Some(call_startup_routines), None];

/* ------------------------------------------------------------------------------------------------
 * Static Variables
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Types
 * --------------------------------------------------------------------------------------------- */

#[derive(Debug)]
struct VpiHandle {
    handle: Option<sv_bindings::vpiHandle>,
}

#[derive(Clone, Copy, Debug)]
enum ObjectType {
    Always,               // always procedure
    AssignStmt,           // quasi-continuous assignment
    Assignment,           // procedural assignment
    Begin,                // block statement
    Case,                 // case statement
    CaseItem,             // case statement item
    Constant,             // numerical constant or string literal
    ContAssign,           // continuous assignment
    Deassign,             // deassignment statement
    DefParam,             // defparam
    DelayControl,         // delay statement (e.g., #10)
    Disable,              // named block disable statement
    EventControl,         // wait on event, e.g., @e
    EventStmt,            // event trigger, e.g., ->e
    For,                  // for statement
    Force,                // force statement
    Forever,              // forever statement
    Fork,                 // fork-join block
    FuncCall,             // function call
    Function,             // function
    Gate,                 // primitive gate
    If,                   // if statement
    IfElse,               // if–else statement
    Initial,              // initial procedure
    IntegerVar,           // integer variable
    InterModPath,         // intermodule wire delay
    Iterator,             // iterator
    IODecl,               // input/output declaration
    Memory,               // behavioral memory
    MemoryWord,           // single word of memory
    ModPath,              // module path for path delays
    Module,               // module instance
    NamedBegin,           // named block statement
    NamedEvent,           // event variable
    NamedFork,            // named fork-join block
    Net,                  // scalar or vector net
    NetBit,               // bit of vector net
    NullStmt,             // a semicolon. Ie. #10 ;
    Operation,            // behavioral operation
    ParamAssign,          // module parameter assignment
    Parameter,            // module parameter
    PartSelect,           // part-select
    PathTerm,             // terminal of module path
    Port,                 // module port
    PortBit,              // bit of vector module port
    PrimTerm,             // primitive terminal
    RealVar,              // real variable
    Reg,                  // scalar or vector reg
    RegBit,               // bit of vector reg
    Release,              // release statement
    Repeat,               // repeat statement
    RepeatControl,        // repeat control in an assign stmt
    SchedEvent,           // vpi_put_value() event
    SpecParam,            // specparam
    Switch,               // transistor switch
    SysFuncCall,          // system function call
    SysTaskCall,          // system task call
    TableEntry,           // UDP state table entry
    Task,                 // task
    TaskCall,             // task call
    Tchk,                 // timing check
    TchkTerm,             // terminal of timing check
    TimeVar,              // time variable
    TimeQueue,            // simulation event queue
    Udp,                  // user-defined primitive
    UdpDefn,              // UDP definition
    UserSystf,            // user-defined system task/function
    VarSelect,            // variable array selection
    Wait,                 // wait statement
    While,                // while statement

    // Object types added with 1364-2001
    Attribute,            // attribute of an object
    BitSelect,            // Bit-select of parameter, var select
    Callback,             // callback object
    DelayTerm,            // Delay term which is a load or driver
    DelayDevice,          // Delay object within a net
    Frame,                // reentrant task/func frame
    GateArray,            // gate instance array
    ModuleArray,          // module instance array
    PrimitiveArray,       // vpiprimitiveArray type
    NetArray,             // multidimensional net
    Range,                // range declaration
    RegArray,             // multidimensional reg
    SwitchArray,          // switch instance array
    UdpArray,             // UDP instance array
    ContAssignBit,        // Bit of a vector continuous assignment
    NamedEventArray,      // multidimensional named event

    // Object types added with 1364-2005
    IndexedPartSelect,    // Indexed part-select object
    GenScopeArray,        // array of generated scopes
    GenScope,             // A generated scope
    GenVar,               // Object used to instantiate gen scopes
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

impl Drop for VpiHandle {
    fn drop(&mut self) {
        if let Some(handle) = self.handle {
            //FIXME test this on a simulator that supports it
            /*
            unsafe {
                //FIXME justify safety
                sv_bindings::vpi_release_handle(handle);
            }
            */
        }
    }
}

/* ------------------------------------------------------------------------------------------------
 * Functions
 * --------------------------------------------------------------------------------------------- */

fn register_startup_routines() {
    //TODO
}

extern "C" fn start_of_sim_callback(callback_data_ptr: *mut sv_bindings::t_cb_data) -> sv_bindings::PLI_INT32 {
    sv_println!("Start of simulation!");
    let iterate_handle = vpi_get_top_module_iterate_handle();
    sv_println!("Iterate handle dbg: {:?}", iterate_handle);

    let scan_handle = unsafe {
        sv_bindings::vpi_scan(
            iterate_handle.handle.unwrap()
        )
    };
    sv_println!("Scan handle dbg: {:?}", scan_handle);

    //TESTING
    unsafe {
        //Why is this null? There is a module...
        let top_module_handle = sv_bindings::vpi_handle(
            sv_bindings::vpiModule as i32,
            scan_handle
        );
        sv_println!("Top module handle dbg: {:?}", top_module_handle);

        //Get the name
        let name = std::ffi::CStr::from_ptr(sv_bindings::vpi_get_str(
            sv_bindings::vpiName as i32,
            top_module_handle
        )).to_string_lossy().into_owned();
        sv_println!("Top module name: {}", name);
    }

    0
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

pub extern "C" fn call_startup_routines() {
    sv_println!("Hello SystemVerilog from Rust 123!");

    //FIXME test this on a simulator that supports it
    //sv_println!("DPI Version: {}", sv_dpi_version_string());

    //Haha, I got an error since we can't do vpi_iterate before startup routines have finished
    //let top_module_handle = vpi_get_top_module_handle();

    //Let's try it in this function instead:
    unsafe {
        START_OF_SIM_CALLBACK_DATA.time = &mut VPI_TIME;//To overcome :(
        sv_bindings::vpi_register_cb(
            &mut START_OF_SIM_CALLBACK_DATA
        );
    }
}

fn vpi_get_top_module_iterate_handle() -> VpiHandle {
    let handle = unsafe {
        sv_bindings::vpi_iterate(
            sv_bindings::vpiModule as i32,
            std::ptr::null_mut()
        )
    };
    VpiHandle {
        handle: Some(handle),
    }
}

fn sv_dpi_version_string() -> String {
    sv_dpi_version_cstring().to_string_lossy().into_owned()
}

fn sv_dpi_version_cstring() -> &'static std::ffi::CStr {
    unsafe {
        //FIXME is the string pointer guaranteed to always be valid (or should we make a copy)?
        std::ffi::CStr::from_ptr(sv_bindings::svDpiVersion())
    }
}

fn vpi_print_str(string: &str) {
    let cstr = std::ffi::CString::new(string).unwrap();
    vpi_print_cstr(&cstr);
}

fn vpi_print_cstr(cstr: &std::ffi::CStr) {
    unsafe {
        //Safety: It is safe to cast to *mut PLI_BYTE8 because vpi_printf does not modify the string
        sv_bindings::vpi_printf(cstr.as_ptr() as *mut sv_bindings::PLI_BYTE8);
    }
}

//TESTING
/*
#[no_mangle]
pub static vlog_startup_routines: [Option<unsafe extern "C" fn()>; 2usize] = [Some(testing123_c), None];
pub fn testing123() {
    const TEST: &[u8] = b"Hi SystemVerilog From Rust!\n\0";
    unsafe {
        sv_bindings::vpi_printf(TEST.as_ptr() as *mut i8);
    }
}
pub extern "C" fn testing123_c() {
    testing123();
}
*/

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
