use crate::classfile::class_reader::ClassReader;

pub struct ConstantIntegerInfo{
    val : i32
}

impl ConstantIntegerInfo{
    pub fn new(reader:&mut ClassReader) ->Self{
        ConstantIntegerInfo{val:reader.read_u32() as i32}
    }

    pub fn value(&self) -> i32 {
        self.val
    }
}

