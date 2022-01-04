use crate::classfile::class_reader::ClassReader;

pub struct ConstantFloatInfo{
    val : f32
}

impl ConstantFloatInfo{
    pub fn new(reader:&mut ClassReader) ->Self{
        ConstantFloatInfo{val:f32::from_bits(reader.read_u32()) }
    }

    pub fn value(&self) -> f32 {
        self.val
    }
}

