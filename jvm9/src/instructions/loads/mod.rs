use super::*;
use std::fmt::{Formatter, Debug};
use std::fmt::Error;
use crate::rtda::slot::{Slot, Slots};


include!("aload.rs");
include!("aload_0.rs");
include!("aload_1.rs");
include!("aload_2.rs");
include!("aload_3.rs");

include!("dload.rs");
include!("dload_0.rs");
include!("dload_1.rs");
include!("dload_2.rs");
include!("dload_3.rs");

include!("fload.rs");
include!("fload_0.rs");
include!("fload_1.rs");
include!("fload_2.rs");
include!("fload_3.rs");

include!("iload.rs");
include!("iload_0.rs");
include!("iload_1.rs");
include!("iload_2.rs");
include!("iload_3.rs");

include!("lload.rs");
include!("lload_0.rs");
include!("lload_1.rs");
include!("lload_2.rs");
include!("lload_3.rs");

//数组相关的
include!("aaload.rs");
include!("baload.rs");
include!("caload.rs");
include!("daload.rs");
include!("faload.rs");
include!("iaload.rs");
include!("laload.rs");
include!("saload.rs");
