use crate::classfile::class_reader::ClassReader;
use std::sync::Arc;

pub struct ConstantUtf8Info{
    //string : String
    //TODO 采用ARC
    string: Arc<String>,
}

impl ConstantUtf8Info{
    pub fn new(reader: &mut ClassReader) -> ConstantUtf8Info {
        let length = reader.read_u16();
        let bytes = reader.read_bytes(length as u32);
        let string = Arc::new(String::from_utf8(bytes).unwrap());
        Self { string }
    }

    pub fn string(&self) -> Arc<String> {
        self.string.clone()
    }
}