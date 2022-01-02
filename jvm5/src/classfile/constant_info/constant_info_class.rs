use crate::classfile::class_reader::ClassReader;
use std::sync::{Arc, RwLock};
use crate::classfile::constant_pool::ConstantPool;

pub struct ConstantClassInfo{
    name_index: u16,
    cp: Arc<RwLock<ConstantPool>>,
}

impl ConstantClassInfo{
    pub(crate) fn new(reader: &mut ClassReader, cp: Arc<RwLock<ConstantPool>>) -> Self {
        let name_index = reader.read_u16();
        Self { name_index, cp }
    }

    pub fn name(&self) -> Arc<String> {
        self.cp.read().unwrap().get_utf8(self.name_index)
    }
}

