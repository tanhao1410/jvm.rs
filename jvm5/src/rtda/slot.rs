use crate::rtda::object::Object;
use crate::rtda::slot::Slot::Num;
use std::sync::{RwLock, Arc};

/// 代表操作数栈或局部变量表中的 数，要么是一个数，要么是一个引用
pub enum Slot{
    Nil(),
    Num(u32),
    //Ref(Object)
    Ref(Arc<RwLock<Object>>)
}

impl Clone for Slot{
    fn clone(&self) -> Self {
        match self {
            Slot::Nil()=>Slot::Nil(),
            Slot::Num(n) => Slot::Num(*n),
            Slot::Ref(rc_obj) => Slot::Ref(rc_obj.clone()),
        }
    }
}