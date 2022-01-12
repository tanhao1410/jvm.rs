use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ops::Add;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

use crate::rtda::frame::Frame;
use crate::rtda::slot::Slots;
use crate::rtda::local_vars::LocalVars;

lazy_static! {
    static ref NATIVE_METHOD: Mutex<HashMap<String, fn(&LocalVars)-> Option<Slots> >> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}


/// 注册本地方法
pub fn register<T: AsRef<str>>(class_name: T, method_name: T, method_desc: T,
                               method: fn(&LocalVars) -> Option<Slots>) {
    let mut guard = NATIVE_METHOD.lock().unwrap();
    let native_method_map = guard.borrow_mut();
    let key = "".to_string()
        .add(class_name.as_ref())
        .add("~")
        .add(method_name.as_ref())
        .add("~")
        .add(method_desc.as_ref());
    native_method_map.insert(key, method);
}

pub fn find_native_method<T: AsRef<str>>(class_name: T, method_name: T, method_desc: T)
                                         -> Option<fn(&LocalVars) -> Option<Slots> > {
    let key = "".to_string()
        .add(class_name.as_ref())
        .add("~")
        .add(method_name.as_ref())
        .add("~")
        .add(method_desc.as_ref());

    if method_desc.as_ref() == "()V" && method_name.as_ref() == "registerNatives" {
        return Some(empty_native_method);
    }

    let mut guard = NATIVE_METHOD.lock().unwrap();
    let native_method_map = guard.borrow();
    if let Some(res) = native_method_map.get(&key) {
        Some(res.clone())
    } else {
        None
    }
}

/// Object类中的一个本地方法，在java中通过它来注册其它本地方法的，在本程序中不需要
fn empty_native_method(local_vars: &LocalVars) -> Option<Slots> { None }

