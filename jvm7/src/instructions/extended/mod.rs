use super::*;
use std::fmt::{Formatter, Debug};
use std::fmt::Error;
use std::ops::Deref;
use crate::rtda::slot::Slot;

include!("goto_w.rs");
include!("ifnonnull.rs");
include!("ifnull.rs");
// include!("jsr_w.rs");
// include!("multianewarray.rs");
include!("wide.rs");
