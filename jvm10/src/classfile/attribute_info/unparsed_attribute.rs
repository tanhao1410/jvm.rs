use crate::classfile::class_reader::ClassReader;

#[allow(dead_code)]
pub struct UnparsedAttribute {
    name_index: u16,
    length: u32,
    info: Vec<u8>,
}

impl UnparsedAttribute {
    pub(crate) fn new(name_index: u16, length: u32, reader: &mut ClassReader) -> UnparsedAttribute {
        let info = reader.read_bytes(length);
        Self { name_index, length, info }
    }
}