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
pub mod result;
pub mod startup;
pub mod print;

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

//TODO (includes "use ..." and "extern crate ...")

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

//TODO

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

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

impl ObjectIterator {
    fn new(object_type: ObjectType) -> ObjectIterator {
        startup::panic_if_in_startup_routine!();
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
        startup::panic_if_in_startup_routine!();
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
            //FIXME I actually don't think we should release the iterator handle in this case (scan
            //being null already takes care of that)
            self.iterator_handle = None;//Iterator handle is now invalid (this drops it)
            None
        } else {
            Some(ObjectHandle { handle: std::ptr::NonNull::new(raw_handle_from_scan).unwrap() })
        }
    }
}

/* ------------------------------------------------------------------------------------------------
 * Functions
 * --------------------------------------------------------------------------------------------- */

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
