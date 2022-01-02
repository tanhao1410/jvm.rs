pub mod constants;
pub mod loads;
pub mod stores;
pub mod stack;
pub mod math;
pub mod conversions;


pub mod comparisons;
pub mod control;
pub mod extended;

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

include!("bytecode_reader.rs");
include!("instruction.rs");
include!("branch_instruction.rs");