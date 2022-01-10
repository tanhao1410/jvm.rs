
use super::*;
use std::fmt::{Formatter, Error, Debug};
use crate::rtda::slot::Slot;
use crate::rtda::heap::constant_pool::constant::Constant;
use crate::rtda::heap::string_pool::get_java_string;

include!("nop.rs");
include!("aconst_null.rs");
include!("dconst_0.rs");
include!("dconst_1.rs");
include!("fconst_0.rs");
include!("fconst_1.rs");
include!("fconst_2.rs");
include!("iconst_0.rs");
include!("iconst_1.rs");
include!("iconst_2.rs");
include!("iconst_3.rs");
include!("iconst_4.rs");
include!("iconst_5.rs");
include!("iconst_m1.rs");
include!("lconst_0.rs");
include!("lconst_1.rs");
include!("bipush.rs");
include!("sipush.rs");
include!("ldc.rs");
include!("ldc_w.rs");
include!("ldc2_w.rs");