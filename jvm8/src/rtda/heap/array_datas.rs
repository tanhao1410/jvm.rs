use std::sync::{Arc, RwLock};
use crate::rtda::heap::object::Object;
use crate::rtda::heap::array_datas::ArrayDatas::Empty;
use crate::rtda::slot::Slots;

///数组对象内部存储的数据
pub enum ArrayDatas{
    Empty,
    Bytes(Vec<i8>),
    Shorts(Vec<i16>),
    Ints(Vec<i32>),
    Longs(Vec<i64>),
    Chars(Vec<u16>),
    Floats(Vec<f32>),
    Doubles(Vec<f64>),
    Refs(Slots) // 里面可能什么都没存，
}

impl Default for ArrayDatas{
    fn default() -> Self {
        Empty
    }
}