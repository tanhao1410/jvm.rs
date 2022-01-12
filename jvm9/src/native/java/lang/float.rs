use crate::native::registry::register;
use crate::rtda::heap::array_datas::ArrayDatas;
use crate::rtda::local_vars::LocalVars;
use crate::rtda::slot::{Slot, Slots};

pub fn init() {
    register("java/lang/Float",
             "floatToRawIntBits",
             "(F)I"
             , float_to_raw_int_bits);
}

fn float_to_raw_int_bits(local_vars: &LocalVars) -> Option<Slots> {
    let value = local_vars.get_float(0);
    Some(Slots::from_one_slot(Slot::Num(value.to_bits())))
}