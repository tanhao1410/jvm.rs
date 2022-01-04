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
use crate::instructions::references::{PUT_FIELD, GET_FIELD, GET_STATIC, PUT_STATIC, NEW, CHECK_CAST, INSTANCE_OF, INVOKE_SPECIAL, INVOKE_VIRTUAL};

include!("bytecode_reader.rs");
include!("instruction.rs");
include!("branch_instruction.rs");