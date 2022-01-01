use crate::classfile::class_reader::ClassReader;

pub struct ConstantDoubleInfo{
    val : f64
}

impl ConstantDoubleInfo{
    pub fn new(reader:&mut ClassReader) ->Self{
        //ConstantDoubleInfo{val: f64::from_be_bytes(reader.read_u64().to_be_bytes()) }
        Self { val: f64::from_bits(reader.read_u64()) }
    }

    pub fn value(&self) -> f64 {
        self.val
    }
}

