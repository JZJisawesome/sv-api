/*
 * File:    object.rs
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

use crate::result;
use crate::result::Error;
use crate::result::Result;
use crate::startup::panic_if_in_startup_routine;
use crate::startup::panic_if_not_main_thread;

use std::ffi::{CStr, CString};
use std::ptr::{self, NonNull};

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

/* ------------------------------------------------------------------------------------------------
 * Types
 * --------------------------------------------------------------------------------------------- */

#[derive(Debug)]
#[repr(transparent)]//To help with the null pointer optimization
pub struct ObjectHandle(NonNull<sv_bindings::PLI_UINT32>);

#[derive(Debug)]
pub struct ObjectChildrenIterator {
    iterator_handle: Option<ObjectHandle>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
#[non_exhaustive]
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
    Attribute = sv_bindings::vpiAttribute, // attribute of an object
    BitSelect = sv_bindings::vpiBitSelect, // Bit-select of parameter, var select
    Callback = sv_bindings::vpiCallback,   // callback object
    DelayTerm = sv_bindings::vpiDelayTerm, // Delay term which is a load or driver
    DelayDevice = sv_bindings::vpiDelayDevice, // Delay object within a net
    Frame = sv_bindings::vpiFrame,         // reentrant task/func frame
    GateArray = sv_bindings::vpiGateArray, // gate instance array
    ModuleArray = sv_bindings::vpiModuleArray, // module instance array
    PrimitiveArray = sv_bindings::vpiPrimitiveArray, // vpiprimitiveArray type
    NetArray = sv_bindings::vpiNetArray,   // multidimensional net
    Range = sv_bindings::vpiRange,         // range declaration
    RegArray = sv_bindings::vpiRegArray,   // multidimensional reg
    SwitchArray = sv_bindings::vpiSwitchArray, // switch instance array
    UdpArray = sv_bindings::vpiUdpArray,   // UDP instance array
    ContAssignBit = sv_bindings::vpiContAssignBit, // Bit of a vector continuous assignment
    NamedEventArray = sv_bindings::vpiNamedEventArray, // multidimensional named event

    // Object types added with 1364-2005
    IndexedPartSelect = sv_bindings::vpiIndexedPartSelect, // Indexed part-select object
    GenScopeArray = sv_bindings::vpiGenScopeArray,         // array of generated scopes
    GenScope = sv_bindings::vpiGenScope,                   // A generated scope
    GenVar = sv_bindings::vpiGenVar,                       // Object used to instantiate gen scopes
}

//FIX numbers get reused here...
#[derive(Clone, Copy, Debug)]
#[repr(i32)]
#[non_exhaustive]
//Note we cannot do = sv_bindings::... like before since some of these reuse numbers
pub enum ObjectProperty {
    Undefined,        // undefined property
    Type,             // type of object
    Name,             // local name of object
    FullName,         // full hierarchical name
    Size,             // size of gate, net, port, etc.
    File,             // File name in which the object is used
    LineNo,           // line number where the object is used
    TopModule,        // top-level module (Boolean)
    CellInstance,     // cell (Boolean)
    DefName,          // module definition name
    Protected,        // source protected module (Boolean)
    TimeUnit,         // module time unit
    TimePrecision,    // module time precision
    DefNetType,       // default net type
    UnconnDrive,      // unconnected port drive strength
    HighZ,            // No default drive given
    Pull1,            // default pull1 drive
    Pull0,            // default pull0 drive
    DefFile,          // File name where the module is defined
    DefLineNo,        // line number for module definition
    DefDelayMode,     // Default delay mode for a module
    DelayModeNone,    // no delay mode specified
    DelayModePath,    // path delay mode
    DelayModeDistrib, // distributed delay mode
    DelayModeUnit,    // unit delay mode
    DelayModeZero,    // zero delay mode
    DelayModeMTM,     // min:typ:max delay mode
    DefDecayTime,     // Default decay time for a module
    Scalar,           // scalar (Boolean)
    Vector,           // vector (Boolean)
    ExplicitName,     // port is explicitly named
    Direction,        // direction of port:
    Input,            // input
    Output,           // output
    Inout,            // inout
    MixedIO,          // mixed input-output
    NoDirection,      // no direction
    ConnByName,       // connected by name (Boolean)
    NetType,          // net subtypes:
    Wire,             // wire net
    Wand,             // wire-and net
    Wor,              // wire-or net
    Tri,              // tri net
    Tri0,             // pull-down net
    Tri1,             // pull-up net
    TriReg,           // three-state reg net
    TriAnd,           // three-state wire-and net
    TriOr,            // three-state wire-or net
    Supply1,          // supply-1 net
    Supply0,          // supply-0 net
    None,             // no default net type (1364-2001)
    Uwire,            // unresolved wire net (1364-2005)
    ExplicitScalared, // explicitly scalared (Boolean)
    ExplicitVectored, // explicitly vectored (Boolean)
    Expanded,         // expanded vector net (Boolean)
    ImplicitDecl,     // implicitly declared net (Boolean)
    ChargeStrength,   // charge decay strength of net
    Array,            // variable array (Boolean)
    PortIndex,        // Port index
    TermIndex,        // Index of a primitive terminal
    Strength0,        // 0-strength of net or gate
    Strength1,        // 1-strength of net or gate
    PrimType,         // primitive subtypes:
    AndPrim,          // and gate
    NandPrim,         // nand gate
    NorPrim,          // nor gate
    OrPrim,           // or gate
    XorPrim,          // xor gate
    XnorPrim,         // xnor gate
    BufPrim,          // buffer
    NotPrim,          // not gate
    Bufif0Prim,       // zero-enabled buffer
    Bufif1Prim,       // one-enabled buffer
    Notif0Prim,       // zero-enabled not gate
    Notif1Prim,       // one-enabled not gate
    NmosPrim,         // nmos switch
    PmosPrim,         // pmos switch
    CmosPrim,         // cmos switch
    RnmosPrim,        // resistive nmos switch
    RpmosPrim,        // resistive pmos switch
    RcmosPrim,        // resistive cmos switch
    RtranPrim,        // resistive bidirectional
    Rtranif0Prim,     // zero-enable resistive bidirectional
    Rtranif1Prim,     // one-enable resistive bidirectional
    TranPrim,         // bidirectional
    Tranif0Prim,      // zero-enabled bidirectional
    Tranif1Prim,      // one-enabled bidirectional
    PullupPrim,       // pullup
    PulldownPrim,     // pulldown
    SeqPrim,          // sequential UDP
    CombPrim,         // combinational UDP
    Polarity,         // polarity of module path...
    DataPolarity,     // ...or data path:
    Positive,         // positive
    Negative,         // negative
    Unknown,          // unknown (unspecified)
    Edge,             // edge type of module path:
    NoEdge,           // no edge
    Edge01,           // 0 -> 1
    Edge10,           // 1 -> 0
    Edge0x,           // 0 -> x
    Edgex1,           // x -> 1
    Edge1x,           // 1 -> x
    Edgex0,           // x -> 0
    Posedge,          // posedge
    Negedge,          // negedge
    AnyEdge,          // any edge
    PathType,         // path delay connection subtypes:
    PathFull,         // ( a *> b )
    PathParallel,     // ( a => b )
    TchkType,         // timing check subtypes:
    Setup,            // $setup
    Hold,             // $hold
    Period,           // $period
    Width,            // $width
    Skew,             // $skew
    Recovery,         // $recovery
    NoChange,         // $nochange
    SetupHold,        // $setuphold
    Fullskew,         // $fullskew -- added for 1364-2001
    Recrem,           // $recrem -- added for 1364-2001
    Removal,          // $removal -- added for 1364-2001
    Timeskew,         // $timeskew -- added for 1364-2001
    OpType,           // operation subtypes:
    MinusOp,          // unary minus
    PlusOp,           // unary plus
    NotOp,            // unary not
    BitNegOp,         // bitwise negation
    UnaryAndOp,       // bitwise reduction AND
    UnaryNandOp,      // bitwise reduction NAND
    UnaryOrOp,        // bitwise reduction OR
    UnaryNorOp,       // bitwise reduction NOR
    UnaryXorOp,       // bitwise reduction XOR
    UnaryXNorOp,      // bitwise reduction XNOR
    SubOp,            // binary subtraction
    DivOp,            // binary division
    ModOp,            // binary modulus
    EqOp,             // binary equality
    NeqOp,            // binary inequality
    CaseEqOp,         // case (x and z) equality
    CaseNeqOp,        // case inequality
    GtOp,             // binary greater than
    GeOp,             // binary greater than or equal
    LtOp,             // binary less than
    LeOp,             // binary less than or equal
    LShiftOp,         // binary left shift
    RShiftOp,         // binary right shift
    AddOp,            // binary addition
    MultOp,           // binary multiplication
    LogAndOp,         // binary logical AND
    LogOrOp,          // binary logical OR
    BitAndOp,         // binary bitwise AND
    BitOrOp,          // binary bitwise OR
    BitXorOp,         // binary bitwise XOR
    BitXNorOp,        // binary bitwise XNOR
    ConditionOp,      // ternary conditional
    ConcatOp,         // n-ary concatenation
    MultiConcatOp,    // repeated concatenation
    EventOrOp,        // event OR
    NullOp,           // null operation
    ListOp,           // list of expressions
    MinTypMaxOp,      // min:typ:max: delay expression
    PosedgeOp,        // posedge
    NegedgeOp,        // negedge
    ArithLShiftOp,    // arithmetic left shift (1364-2001)
    ArithRShiftOp,    // arithmetic right shift (1364-2001)
    PowerOp,          // arithmetic power op (1364-2001)
    ConstType,        // constant subtypes:
    DecConst,         // decimal integer
    RealConst,        // real
    BinaryConst,      // binary integer
    OctConst,         // octal integer
    HexConst,         // hexadecimal integer
    StringConst,      // string literal
    IntConst,         // integer constant (1364-2001)
    TimeConst,        // time constant
    Blocking,         // blocking assignment (Boolean)
    CaseType,         // case statement subtypes:
    CaseExact,        // exact match
    CaseX,            // ignore X's
    CaseZ,            // ignore Z's
    NetDeclAssign,    // assign part of decl (Boolean)
    FuncType,         // function & system function type
    IntFunc,          // returns integer
    RealFunc,         // returns real
    TimeFunc,         // returns time
    SizedFunc,        // returns an arbitrary size
    SizedSignedFunc,  // returns sized signed value
    SysFuncType,      // alias 1364-1995 system function subtypes to 1364-2001 function subtypes
    SysFuncInt,
    SysFuncReal,
    SysFuncTime,
    SysFuncSized,
    UserDefn,         // user-defined system task/func(Boolean)
    Scheduled,        // object still scheduled (Boolean)
    Active,           // reentrant task/func frame is active
    Automatic,        // task/func obj is automatic
    Cell,             // configuration cell
    Config,           // configuration config file
    ConstantSelect,   // (Boolean) bit-select or part-select indices are constant expressions
    Decompile,        // decompile the object
    DefAttribute,     // Attribute defined for the obj
    DelayType,        // delay subtype
    ModPathDelay,     // module path delay
    InterModPathDelay, // intermodule path delay
    MIPDelay,         // module input port delay
    IteratorType,     // object type of an iterator
    Library,          // configuration library
    Offset,           // offset from LSB
    ResolvedNetType,  // net subtype after resolution, returns same subtypes as vpiNetType
    SaveRestartID,    // unique ID for save/restart data
    SaveRestartLocation, // name of save/restart data file
    Valid,             // reentrant task/func frame or automatic variable is valid
    ValidFalse,
    ValidTrue,
    Signed, // TRUE for vpiIODecl and any object in the expression class if the object has the signed attribute
    LocalParam,       // TRUE when a param is declared as a localparam
    ModPathHasIfNone, // Mod path has an ifnone statement
    IndexedPartSelectType, // Indexed part-select type
    PosIndexed,           // +:
    NegIndexed,           // -:
    IsMemory,             // TRUE for a one-dimensional reg array
    IsProtected,          // TRUE for protected design information
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Scalar {
    Zero = sv_bindings::vpi0,
    One = sv_bindings::vpi1,
    X = sv_bindings::vpiX,
    Z = sv_bindings::vpiZ,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Value {
    BinStr(CString),
    OctStr(CString),
    DecStr(CString),
    HexStr(CString),
    Scalar(Scalar),
    Integer(i32),
    Real(f64),
    String(CString),
    Vector(Vec<Scalar>),
    Strength,//TODO contents
    Suppress,
    Time,//TODO contents
    ObjType,//TODO contents
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
#[non_exhaustive]
pub enum ValueKind {
    BinStr = sv_bindings::vpiBinStrVal,
    OctStr = sv_bindings::vpiOctStrVal,
    DecStr = sv_bindings::vpiDecStrVal,
    HexStr = sv_bindings::vpiHexStrVal,
    Scalar = sv_bindings::vpiScalarVal,
    Integer = sv_bindings::vpiIntVal,
    Real = sv_bindings::vpiRealVal,
    String = sv_bindings::vpiStringVal,
    Vector = sv_bindings::vpiVectorVal,
    Strength = sv_bindings::vpiStrengthVal,
    Suppress = sv_bindings::vpiSuppressVal,
    Time = sv_bindings::vpiTimeVal,
    ObjType = sv_bindings::vpiObjTypeVal,
}

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

impl ObjectHandle {
    pub fn get_property_bool(&mut self, property: ObjectProperty) -> Result<bool> {
        panic_if_in_startup_routine!();
        panic_if_not_main_thread!();

        let property_integer = self.get_property_i32(property)?;
        
        match property_integer {
            0 => Ok(false),
            1 => Ok(true),
            _ => Error::UnknownSimulatorError.into(),//Happens if this isn't a boolean property
        }
    }

    pub fn get_property_i32(&mut self, property: ObjectProperty) -> Result<i32> {
        panic_if_in_startup_routine!();
        panic_if_not_main_thread!();

        //FIXME justify safety
        let result = unsafe { sv_bindings::vpi_get(property.into(), self.0.as_ptr()) };

        result::from_last_vpi_call()?;

        Ok(result)
    }

    pub fn get_property_i64(&mut self, property: ObjectProperty) -> Result<i64> {
        panic_if_in_startup_routine!();
        panic_if_not_main_thread!();

        //FIXME justify safety
        let result = unsafe { sv_bindings::vpi_get64(property.into(), self.0.as_ptr()) };

        result::from_last_vpi_call()?;

        Ok(result)
    }

    pub fn get_property_string(&mut self, property: ObjectProperty) -> Result<String> {
        panic_if_in_startup_routine!();
        panic_if_not_main_thread!();

        Ok(self.get_property_cstring(property)?.into_string().map_err(|e| Error::other(e))?)
    }

    pub fn get_property_cstring(&mut self, property: ObjectProperty) -> Result<CString> {
        panic_if_in_startup_routine!();
        panic_if_not_main_thread!();

        //FIXME justify safety
        let cstring = unsafe { sv_bindings::vpi_get_str(property.into(), self.0.as_ptr()) };

        result::from_last_vpi_call()?;

        if cstring.is_null() {
            //Some simulators return null for non-existent properties/if the property isn't a string
            Error::UnknownSimulatorError.into()
        } else {
            //FIXME justify safety
            //LRM says explictly we need to make a copy here
            Ok(unsafe { CStr::from_ptr(cstring) }.to_owned() )
        }
    }

    pub fn get_value(&mut self, value_kind: ValueKind) -> Result<Value> {
        panic_if_in_startup_routine!();
        panic_if_not_main_thread!();

        todo!()//TODO
    }

    //TODO provide function to cleanup callback

    //Shouldn't be null and should be a valid handle pointer
    pub(crate) unsafe fn from_raw(raw: sv_bindings::vpiHandle) -> Self {
        Self(NonNull::new(raw).expect("Null pointer shouldn't be passed to ObjectHandle::from_raw"))
    }
}

impl ObjectChildrenIterator {
    pub fn of_root(object_type: ObjectType) -> Result<ObjectChildrenIterator> {
        panic_if_in_startup_routine!();
        panic_if_not_main_thread!();

        //SAFETY: It is legal to pass vpi_iterate a null pointer, and object_type is valid
        let raw_handle = unsafe { sv_bindings::vpi_iterate(object_type as i32, ptr::null_mut()) };

        result::from_last_vpi_call()?;

        let iterator_handle = if raw_handle.is_null() {//No children with the given type
            None
        } else {
            //SAFETY: The handle is not null and is valid since it was returned by vpi_iterate
            Some(unsafe { ObjectHandle::from_raw(raw_handle) })
        };

        Ok(ObjectChildrenIterator {
            iterator_handle: iterator_handle,
        })
    }

    pub fn of(parent: &mut ObjectHandle, object_type: ObjectType) -> Result<ObjectChildrenIterator> {
        panic_if_in_startup_routine!();
        panic_if_not_main_thread!();

        //SAFETY: We assume parent and object_type are valid
        let raw_handle = unsafe { sv_bindings::vpi_iterate(object_type as i32, parent.0.as_ptr()) };

        result::from_last_vpi_call()?;

        let iterator_handle = if raw_handle.is_null() {//No children with the given type
            None
        } else {
            //SAFETY: The handle is not null and is valid since it was returned by vpi_iterate
            Some(unsafe { ObjectHandle::from_raw(raw_handle) })
        };

        Ok(ObjectChildrenIterator {
            iterator_handle: iterator_handle,
        })
    }
}

impl Value {
    pub const fn kind(&self) -> ValueKind {
        match self {
            Value::BinStr(_) => ValueKind::BinStr,
            Value::OctStr(_) => ValueKind::OctStr,
            Value::DecStr(_) => ValueKind::DecStr,
            Value::HexStr(_) => ValueKind::HexStr,
            Value::Scalar(_) => ValueKind::Scalar,
            Value::Integer(_) => ValueKind::Integer,
            Value::Real(_) => ValueKind::Real,
            Value::String(_) => ValueKind::String,
            Value::Vector(_) => ValueKind::Vector,
            Value::Strength => ValueKind::Strength,//TODO contents
            Value::Suppress => ValueKind::Suppress,
            Value::Time => ValueKind::Time,//TODO contents
            Value::ObjType => ValueKind::ObjType,//TODO contents
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

impl PartialEq for ObjectHandle {
    fn eq(&self, other: &ObjectHandle) -> bool {
        panic_if_in_startup_routine!();
        panic_if_not_main_thread!();

        //SAFETY: We assume self and other are valid
        let result = unsafe { sv_bindings::vpi_compare_objects(self.0.as_ptr(), other.0.as_ptr()) };

        result::from_last_vpi_call().unwrap();//TODO how to propagate this error?

        result == 1
    }
}
impl Eq for ObjectHandle {}

impl Drop for ObjectHandle {
    fn drop(&mut self) {
        panic_if_in_startup_routine!();
        panic_if_not_main_thread!();
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

impl Iterator for ObjectChildrenIterator {
    type Item = ObjectHandle;

    fn next(&mut self) -> Option<ObjectHandle> {
        panic_if_in_startup_routine!();
        panic_if_not_main_thread!();
        let unwrapped_iterator_handle = self.iterator_handle.as_mut()?;

        //FIXME justify safety
        let raw_handle_from_scan = unsafe { sv_bindings::vpi_scan(unwrapped_iterator_handle.0.as_ptr()) };

        result::from_last_vpi_call().ok()?;

        if raw_handle_from_scan.is_null() {
            //FIXME I actually don't think we should release the iterator handle in this case (scan
            //being null already takes care of that)
            self.iterator_handle = None; //Iterator handle is now invalid (this drops it)
            None
        } else {
            //SAFETY: The handle is not null and is valid since it was returned by vpi_iterate
            Some(unsafe { ObjectHandle::from_raw(raw_handle_from_scan) })
        }
    }
}

impl From<ObjectProperty> for i32 {
    fn from(prop: ObjectProperty) -> i32 {
        match prop {
            ObjectProperty::Undefined => sv_bindings::vpiUndefined,
            ObjectProperty::Type => sv_bindings::vpiType,
            ObjectProperty::Name => sv_bindings::vpiName,
            ObjectProperty::FullName => sv_bindings::vpiFullName,
            ObjectProperty::Size => sv_bindings::vpiSize,
            ObjectProperty::File => sv_bindings::vpiFile,
            ObjectProperty::LineNo => sv_bindings::vpiLineNo,
            ObjectProperty::TopModule => sv_bindings::vpiTopModule,
            ObjectProperty::CellInstance => sv_bindings::vpiCellInstance,
            ObjectProperty::DefName => sv_bindings::vpiDefName,
            ObjectProperty::Protected => sv_bindings::vpiProtected,
            ObjectProperty::TimeUnit => sv_bindings::vpiTimeUnit,
            ObjectProperty::TimePrecision => sv_bindings::vpiTimePrecision,
            ObjectProperty::DefNetType => sv_bindings::vpiDefNetType,
            ObjectProperty::UnconnDrive => sv_bindings::vpiUnconnDrive,
            ObjectProperty::HighZ => sv_bindings::vpiHighZ,
            ObjectProperty::Pull1 => sv_bindings::vpiPull1,
            ObjectProperty::Pull0 => sv_bindings::vpiPull0,
            ObjectProperty::DefFile => sv_bindings::vpiDefFile,
            ObjectProperty::DefLineNo => sv_bindings::vpiDefLineNo,
            ObjectProperty::DefDelayMode => sv_bindings::vpiDefDelayMode,
            ObjectProperty::DelayModeNone => sv_bindings::vpiDelayModeNone,
            ObjectProperty::DelayModePath => sv_bindings::vpiDelayModePath,
            ObjectProperty::DelayModeDistrib => sv_bindings::vpiDelayModeDistrib,
            ObjectProperty::DelayModeUnit => sv_bindings::vpiDelayModeUnit,
            ObjectProperty::DelayModeZero => sv_bindings::vpiDelayModeZero,
            ObjectProperty::DelayModeMTM => sv_bindings::vpiDelayModeMTM,
            ObjectProperty::DefDecayTime => sv_bindings::vpiDefDecayTime,
            ObjectProperty::Scalar => sv_bindings::vpiScalar,
            ObjectProperty::Vector => sv_bindings::vpiVector,
            ObjectProperty::ExplicitName => sv_bindings::vpiExplicitName,
            ObjectProperty::Direction => sv_bindings::vpiDirection,
            ObjectProperty::Input => sv_bindings::vpiInput,
            ObjectProperty::Output => sv_bindings::vpiOutput,
            ObjectProperty::Inout => sv_bindings::vpiInout,
            ObjectProperty::MixedIO => sv_bindings::vpiMixedIO,
            ObjectProperty::NoDirection => sv_bindings::vpiNoDirection,
            ObjectProperty::ConnByName => sv_bindings::vpiConnByName,
            ObjectProperty::NetType => sv_bindings::vpiNetType,
            ObjectProperty::Wire => sv_bindings::vpiWire,
            ObjectProperty::Wand => sv_bindings::vpiWand,
            ObjectProperty::Wor => sv_bindings::vpiWor,
            ObjectProperty::Tri => sv_bindings::vpiTri,
            ObjectProperty::Tri0 => sv_bindings::vpiTri0,
            ObjectProperty::Tri1 => sv_bindings::vpiTri1,
            ObjectProperty::TriReg => sv_bindings::vpiTriReg,
            ObjectProperty::TriAnd => sv_bindings::vpiTriAnd,
            ObjectProperty::TriOr => sv_bindings::vpiTriOr,
            ObjectProperty::Supply1 => sv_bindings::vpiSupply1,
            ObjectProperty::Supply0 => sv_bindings::vpiSupply0,
            ObjectProperty::None => sv_bindings::vpiNone,
            ObjectProperty::Uwire => sv_bindings::vpiUwire,
            ObjectProperty::ExplicitScalared => sv_bindings::vpiExplicitScalared,
            ObjectProperty::ExplicitVectored => sv_bindings::vpiExplicitVectored,
            ObjectProperty::Expanded => sv_bindings::vpiExpanded,
            ObjectProperty::ImplicitDecl => sv_bindings::vpiImplicitDecl,
            ObjectProperty::ChargeStrength => sv_bindings::vpiChargeStrength,
            ObjectProperty::Array => sv_bindings::vpiArray,
            ObjectProperty::PortIndex => sv_bindings::vpiPortIndex,
            ObjectProperty::TermIndex => sv_bindings::vpiTermIndex,
            ObjectProperty::Strength0 => sv_bindings::vpiStrength0,
            ObjectProperty::Strength1 => sv_bindings::vpiStrength1,
            ObjectProperty::PrimType => sv_bindings::vpiPrimType,
            ObjectProperty::AndPrim => sv_bindings::vpiAndPrim,
            ObjectProperty::NandPrim => sv_bindings::vpiNandPrim,
            ObjectProperty::NorPrim => sv_bindings::vpiNorPrim,
            ObjectProperty::OrPrim => sv_bindings::vpiOrPrim,
            ObjectProperty::XorPrim => sv_bindings::vpiXorPrim,
            ObjectProperty::XnorPrim => sv_bindings::vpiXnorPrim,
            ObjectProperty::BufPrim => sv_bindings::vpiBufPrim,
            ObjectProperty::NotPrim => sv_bindings::vpiNotPrim,
            ObjectProperty::Bufif0Prim => sv_bindings::vpiBufif0Prim,
            ObjectProperty::Bufif1Prim => sv_bindings::vpiBufif1Prim,
            ObjectProperty::Notif0Prim => sv_bindings::vpiNotif0Prim,
            ObjectProperty::Notif1Prim => sv_bindings::vpiNotif1Prim,
            ObjectProperty::NmosPrim => sv_bindings::vpiNmosPrim,
            ObjectProperty::PmosPrim => sv_bindings::vpiPmosPrim,
            ObjectProperty::CmosPrim => sv_bindings::vpiCmosPrim,
            ObjectProperty::RnmosPrim => sv_bindings::vpiRnmosPrim,
            ObjectProperty::RpmosPrim => sv_bindings::vpiRpmosPrim,
            ObjectProperty::RcmosPrim => sv_bindings::vpiRcmosPrim,
            ObjectProperty::RtranPrim => sv_bindings::vpiRtranPrim,
            ObjectProperty::Rtranif0Prim => sv_bindings::vpiRtranif0Prim,
            ObjectProperty::Rtranif1Prim => sv_bindings::vpiRtranif1Prim,
            ObjectProperty::TranPrim => sv_bindings::vpiTranPrim,
            ObjectProperty::Tranif0Prim => sv_bindings::vpiTranif0Prim,
            ObjectProperty::Tranif1Prim => sv_bindings::vpiTranif1Prim,
            ObjectProperty::PullupPrim => sv_bindings::vpiPullupPrim,
            ObjectProperty::PulldownPrim => sv_bindings::vpiPulldownPrim,
            ObjectProperty::SeqPrim => sv_bindings::vpiSeqPrim,
            ObjectProperty::CombPrim => sv_bindings::vpiCombPrim,
            ObjectProperty::Polarity => sv_bindings::vpiPolarity,
            ObjectProperty::DataPolarity => sv_bindings::vpiDataPolarity,
            ObjectProperty::Positive => sv_bindings::vpiPositive,
            ObjectProperty::Negative => sv_bindings::vpiNegative,
            ObjectProperty::Unknown => sv_bindings::vpiUnknown,
            ObjectProperty::Edge => sv_bindings::vpiEdge,
            ObjectProperty::NoEdge => sv_bindings::vpiNoEdge,
            ObjectProperty::Edge01 => sv_bindings::vpiEdge01,
            ObjectProperty::Edge10 => sv_bindings::vpiEdge10,
            ObjectProperty::Edge0x => sv_bindings::vpiEdge0x,
            ObjectProperty::Edgex1 => sv_bindings::vpiEdgex1,
            ObjectProperty::Edge1x => sv_bindings::vpiEdge1x,
            ObjectProperty::Edgex0 => sv_bindings::vpiEdgex0,
            ObjectProperty::Posedge => sv_bindings::vpiPosedge,
            ObjectProperty::Negedge => sv_bindings::vpiNegedge,
            ObjectProperty::AnyEdge => sv_bindings::vpiAnyEdge,
            ObjectProperty::PathType => sv_bindings::vpiPathType,
            ObjectProperty::PathFull => sv_bindings::vpiPathFull,
            ObjectProperty::PathParallel => sv_bindings::vpiPathParallel,
            ObjectProperty::TchkType => sv_bindings::vpiTchkType,
            ObjectProperty::Setup => sv_bindings::vpiSetup,
            ObjectProperty::Hold => sv_bindings::vpiHold,
            ObjectProperty::Period => sv_bindings::vpiPeriod,
            ObjectProperty::Width => sv_bindings::vpiWidth,
            ObjectProperty::Skew => sv_bindings::vpiSkew,
            ObjectProperty::Recovery => sv_bindings::vpiRecovery,
            ObjectProperty::NoChange => sv_bindings::vpiNoChange,
            ObjectProperty::SetupHold => sv_bindings::vpiSetupHold,
            ObjectProperty::Fullskew => sv_bindings::vpiFullskew,
            ObjectProperty::Recrem => sv_bindings::vpiRecrem,
            ObjectProperty::Removal => sv_bindings::vpiRemoval,
            ObjectProperty::Timeskew => sv_bindings::vpiTimeskew,
            ObjectProperty::OpType => sv_bindings::vpiOpType,
            ObjectProperty::MinusOp => sv_bindings::vpiMinusOp,
            ObjectProperty::PlusOp => sv_bindings::vpiPlusOp,
            ObjectProperty::NotOp => sv_bindings::vpiNotOp,
            ObjectProperty::BitNegOp => sv_bindings::vpiBitNegOp,
            ObjectProperty::UnaryAndOp => sv_bindings::vpiUnaryAndOp,
            ObjectProperty::UnaryNandOp => sv_bindings::vpiUnaryNandOp,
            ObjectProperty::UnaryOrOp => sv_bindings::vpiUnaryOrOp,
            ObjectProperty::UnaryNorOp => sv_bindings::vpiUnaryNorOp,
            ObjectProperty::UnaryXorOp => sv_bindings::vpiUnaryXorOp,
            ObjectProperty::UnaryXNorOp => sv_bindings::vpiUnaryXNorOp,
            ObjectProperty::SubOp => sv_bindings::vpiSubOp,
            ObjectProperty::DivOp => sv_bindings::vpiDivOp,
            ObjectProperty::ModOp => sv_bindings::vpiModOp,
            ObjectProperty::EqOp => sv_bindings::vpiEqOp,
            ObjectProperty::NeqOp => sv_bindings::vpiNeqOp,
            ObjectProperty::CaseEqOp => sv_bindings::vpiCaseEqOp,
            ObjectProperty::CaseNeqOp => sv_bindings::vpiCaseNeqOp,
            ObjectProperty::GtOp => sv_bindings::vpiGtOp,
            ObjectProperty::GeOp => sv_bindings::vpiGeOp,
            ObjectProperty::LtOp => sv_bindings::vpiLtOp,
            ObjectProperty::LeOp => sv_bindings::vpiLeOp,
            ObjectProperty::LShiftOp => sv_bindings::vpiLShiftOp,
            ObjectProperty::RShiftOp => sv_bindings::vpiRShiftOp,
            ObjectProperty::AddOp => sv_bindings::vpiAddOp,
            ObjectProperty::MultOp => sv_bindings::vpiMultOp,
            ObjectProperty::LogAndOp => sv_bindings::vpiLogAndOp,
            ObjectProperty::LogOrOp => sv_bindings::vpiLogOrOp,
            ObjectProperty::BitAndOp => sv_bindings::vpiBitAndOp,
            ObjectProperty::BitOrOp => sv_bindings::vpiBitOrOp,
            ObjectProperty::BitXorOp => sv_bindings::vpiBitXorOp,
            ObjectProperty::BitXNorOp => sv_bindings::vpiBitXNorOp,
            ObjectProperty::ConditionOp => sv_bindings::vpiConditionOp,
            ObjectProperty::ConcatOp => sv_bindings::vpiConcatOp,
            ObjectProperty::MultiConcatOp => sv_bindings::vpiMultiConcatOp,
            ObjectProperty::EventOrOp => sv_bindings::vpiEventOrOp,
            ObjectProperty::NullOp => sv_bindings::vpiNullOp,
            ObjectProperty::ListOp => sv_bindings::vpiListOp,
            ObjectProperty::MinTypMaxOp => sv_bindings::vpiMinTypMaxOp,
            ObjectProperty::PosedgeOp => sv_bindings::vpiPosedgeOp,
            ObjectProperty::NegedgeOp => sv_bindings::vpiNegedgeOp,
            ObjectProperty::ArithLShiftOp => sv_bindings::vpiArithLShiftOp,
            ObjectProperty::ArithRShiftOp => sv_bindings::vpiArithRShiftOp,
            ObjectProperty::PowerOp => sv_bindings::vpiPowerOp,
            ObjectProperty::ConstType => sv_bindings::vpiConstType,
            ObjectProperty::DecConst => sv_bindings::vpiDecConst,
            ObjectProperty::RealConst => sv_bindings::vpiRealConst,
            ObjectProperty::BinaryConst => sv_bindings::vpiBinaryConst,
            ObjectProperty::OctConst => sv_bindings::vpiOctConst,
            ObjectProperty::HexConst => sv_bindings::vpiHexConst,
            ObjectProperty::StringConst => sv_bindings::vpiStringConst,
            ObjectProperty::IntConst => sv_bindings::vpiIntConst,
            ObjectProperty::TimeConst => sv_bindings::vpiTimeConst,
            ObjectProperty::Blocking => sv_bindings::vpiBlocking,
            ObjectProperty::CaseType => sv_bindings::vpiCaseType,
            ObjectProperty::CaseExact => sv_bindings::vpiCaseExact,
            ObjectProperty::CaseX => sv_bindings::vpiCaseX,
            ObjectProperty::CaseZ => sv_bindings::vpiCaseZ,
            ObjectProperty::NetDeclAssign => sv_bindings::vpiNetDeclAssign,
            ObjectProperty::FuncType => sv_bindings::vpiFuncType,
            ObjectProperty::IntFunc => sv_bindings::vpiIntFunc,
            ObjectProperty::RealFunc => sv_bindings::vpiRealFunc,
            ObjectProperty::TimeFunc => sv_bindings::vpiTimeFunc,
            ObjectProperty::SizedFunc => sv_bindings::vpiSizedFunc,
            ObjectProperty::SizedSignedFunc => sv_bindings::vpiSizedSignedFunc,
            ObjectProperty::SysFuncType => sv_bindings::vpiSysFuncType,
            ObjectProperty::SysFuncInt => sv_bindings::vpiSysFuncInt,
            ObjectProperty::SysFuncReal => sv_bindings::vpiSysFuncReal,
            ObjectProperty::SysFuncTime => sv_bindings::vpiSysFuncTime,
            ObjectProperty::SysFuncSized => sv_bindings::vpiSysFuncSized,
            ObjectProperty::UserDefn => sv_bindings::vpiUserDefn,
            ObjectProperty::Scheduled => sv_bindings::vpiScheduled,
            ObjectProperty::Active => sv_bindings::vpiActive,
            ObjectProperty::Automatic => sv_bindings::vpiAutomatic,
            ObjectProperty::Cell => sv_bindings::vpiCell,
            ObjectProperty::Config => sv_bindings::vpiConfig,
            ObjectProperty::ConstantSelect => sv_bindings::vpiConstantSelect,
            ObjectProperty::Decompile => sv_bindings::vpiDecompile,
            ObjectProperty::DefAttribute => sv_bindings::vpiDefAttribute,
            ObjectProperty::DelayType => sv_bindings::vpiDelayType,
            ObjectProperty::ModPathDelay => sv_bindings::vpiModPathDelay,
            ObjectProperty::InterModPathDelay => sv_bindings::vpiInterModPathDelay,
            ObjectProperty::MIPDelay => sv_bindings::vpiMIPDelay,
            ObjectProperty::IteratorType => sv_bindings::vpiIteratorType,
            ObjectProperty::Library => sv_bindings::vpiLibrary,
            ObjectProperty::Offset => sv_bindings::vpiOffset,
            ObjectProperty::ResolvedNetType => sv_bindings::vpiResolvedNetType,
            ObjectProperty::SaveRestartID => sv_bindings::vpiSaveRestartID,
            ObjectProperty::SaveRestartLocation => sv_bindings::vpiSaveRestartLocation,
            ObjectProperty::Valid => sv_bindings::vpiValid,
            ObjectProperty::ValidFalse => sv_bindings::vpiValidFalse,
            ObjectProperty::ValidTrue => sv_bindings::vpiValidTrue,
            ObjectProperty::Signed => sv_bindings::vpiSigned,
            ObjectProperty::LocalParam => sv_bindings::vpiLocalParam,
            ObjectProperty::ModPathHasIfNone => sv_bindings::vpiModPathHasIfNone,
            ObjectProperty::IndexedPartSelectType => sv_bindings::vpiIndexedPartSelectType,
            ObjectProperty::PosIndexed => sv_bindings::vpiPosIndexed,
            ObjectProperty::NegIndexed => sv_bindings::vpiNegIndexed,
            ObjectProperty::IsMemory => sv_bindings::vpiIsMemory,
            ObjectProperty::IsProtected => sv_bindings::vpiIsProtected,
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
