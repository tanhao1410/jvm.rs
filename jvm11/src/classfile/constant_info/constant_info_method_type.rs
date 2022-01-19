use crate::classfile::class_reader::ClassReader;

#[allow(dead_code)]
pub struct ConstantMethodTypeInfo {
    descriptor_index: u16
}

impl ConstantMethodTypeInfo {
    pub(crate) fn new(reader: &mut ClassReader) -> ConstantMethodTypeInfo {
        Self { descriptor_index: reader.read_u16() }
    }
}

