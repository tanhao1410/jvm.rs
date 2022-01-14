use crate::native::registry::register;
use crate::rtda::local_vars::LocalVars;
use crate::rtda::slot::{ Slots};
use std::sync::RwLockWriteGuard;
use crate::rtda::thread::Thread;

pub fn init() {
    register("java/lang/Double",
             "doubleToRawLongBits",
             "(D)J"
             , doule_to_raw_long_bits);
    register("java/lang/Double",
             "longBitsToDouble",
             "(J)D"
             , doule_to_raw_long_bits);
}

fn doule_to_raw_long_bits(local_vars: &LocalVars,_:RwLockWriteGuard<Thread>) -> Option<Slots> {
    let value = local_vars.get_double(0);
    let mut slots = Slots::new(2);
    slots.set_long(0, value.to_bits() as i64);
    Some(slots)
}

#[allow(dead_code)]
fn long_bits_to_double(local_vars:&LocalVars,_:RwLockWriteGuard<Thread>) -> Option<Slots>{

    let value = local_vars.get_i64(0);
    let res = f64::from_bits(value as u64);
    let mut slots = Slots::new(2);
    slots.set_double(0,res);
    Some(slots)
}