pub mod constants;
pub mod loads;
pub mod stores;
pub mod stack;
pub mod math;
pub mod conversions;


pub mod comparisons;
pub mod control;
pub mod extended;
pub mod references;
pub mod method_invoke_logic;

use comparisons::*;
use constants::*;
use control::*;
use conversions::*;
use extended::*;
use loads::*;
use math::*;
use stack::*;
use stores::*;

use crate::rtda::frame::Frame;
use std::fmt::Debug;
use std::sync::{RwLock, Arc};
use crate::instructions::references::{PUT_FIELD, GET_FIELD, GET_STATIC, PUT_STATIC, NEW, CHECK_CAST, INSTANCE_OF, INVOKE_SPECIAL, INVOKE_VIRTUAL, INVOKE_STATIC, NEW_ARRAY, ANEW_ARRAY, ARRAY_LENGTH, MULTI_ANEW_ARRAY};
use crate::rtda::thread::Thread;
use std::cell::RefMut;

include!("bytecode_reader.rs");
include!("instruction.rs");
include!("branch_instruction.rs");