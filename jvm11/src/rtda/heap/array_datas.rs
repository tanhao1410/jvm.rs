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

impl ArrayDatas{
    pub fn clone(&self)->Self{
        match self {
            ArrayDatas::Empty => {ArrayDatas::Empty}
            ArrayDatas::Bytes(vals) =>ArrayDatas::Bytes(vals.clone()),
            ArrayDatas::Shorts(vals) => {ArrayDatas::Shorts(vals.clone())}
            ArrayDatas::Ints(vals) => ArrayDatas::Ints(vals.clone()),
            ArrayDatas::Longs(vals) => ArrayDatas::Longs(vals.clone()),
            ArrayDatas::Chars(vals) => ArrayDatas::Chars(vals.clone()),
            ArrayDatas::Floats(vals) => ArrayDatas::Floats(vals.clone()),
            ArrayDatas::Doubles(vals) => ArrayDatas::Doubles(vals.clone()),
            ArrayDatas::Refs(vals) => ArrayDatas::Refs(vals.clone())
        }
    }
}

impl Default for ArrayDatas{
    fn default() -> Self {
        Empty
    }
}