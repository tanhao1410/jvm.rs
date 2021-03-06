use crate::classfile::class_reader::ClassReader;

#[allow(dead_code)]
pub struct ConstantMethodHandleInfo {
    reference_kind: u8,
    reference_index: u16,
}

impl ConstantMethodHandleInfo {
    pub(crate) fn new(reader: &mut ClassReader) -> ConstantMethodHandleInfo {
        let reference_kind = reader.read_u8();
        let reference_index = reader.read_u16();
        Self { reference_kind, reference_index }
    }
}

