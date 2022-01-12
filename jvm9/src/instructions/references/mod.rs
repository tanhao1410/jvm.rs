use std::fmt::{Debug, Formatter};
use std::fmt::Error;
use crate::instructions::{Instruction, BytecodeReader, method_invoke_logic};
use crate::rtda::frame::Frame;
use crate::rtda::slot::Slot;
use crate::rtda::heap::constant_pool::constant::Constant;
use crate::rtda::heap::class::Class;
use crate::instructions::method_invoke_logic::invoke_method;
use std::sync::{RwLock, Arc};
use crate::rtda::thread::Thread;
use std::rc::Rc;
use std::cell::RefCell;
use crate::rtda::heap::class_loader::ClassLoader;
use crate::rtda::heap::object::Object;
use crate::native::registry::find_native_method;
use crate::utils::string_utils::{get_string_from_slot, get_java_string};

include!("anewarray.rs");
include!("multianewarray.rs");
include!("arraylength.rs");
//include!("athrow.rs");
include!("checkcast.rs");
include!("getfield.rs");
include!("getstatic.rs");
include!("instanceof.rs");
// include!("invokedynamic.rs");
include!("invokeinterface.rs");
include!("invokespecial.rs");
include!("invokestatic.rs");
include!("invokevirtual.rs");
include!("invokenative.rs");
// include!("monitorenter.rs");
// include!("monitorexit.rs");
include!("new.rs");
include!("newarray.rs");
include!("putfield.rs");
include!("putstatic.rs");