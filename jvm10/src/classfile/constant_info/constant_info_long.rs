use crate::classfile::class_reader::ClassReader;

pub struct ConstantLongInfo{
    val : i64
}

impl ConstantLongInfo{
    pub(crate) fn new(reader: &mut ClassReader) -> ConstantLongInfo {
        Self { val: reader.read_u64() as i64 }
    }

    pub fn value(&self) -> i64 {
        self.val
    }
}

