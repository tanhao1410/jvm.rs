use crate::classfile::class_reader::ClassReader;
use std::sync::{Arc, RwLock};
use crate::classfile::constant_pool::ConstantPool;

pub struct ConstantStringInfo{
    string_index: u16,
    cp: Arc<RwLock<ConstantPool>>,
}

impl ConstantStringInfo{
    pub fn new(reader:&mut ClassReader,cp:Arc<RwLock<ConstantPool>>) ->Self{
        ConstantStringInfo{string_index:reader.read_u16(),cp:cp}
    }

    pub fn string(&self) -> Arc<String> {
        self.cp.read().unwrap().get_utf8(self.string_index)
    }
}

