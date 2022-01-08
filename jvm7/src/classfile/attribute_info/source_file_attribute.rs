use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_pool::ConstantPool;
use std::sync::Arc;

/// 出现在ClassFile结构中，用于指出源文件名
pub struct SourceFileAttribute{
    sourcefile_index : u16
}

impl SourceFileAttribute{
    pub fn new(reader:&mut ClassReader)->Self{
        let sourcefile_index = reader.read_u16();
        Self{sourcefile_index}
    }

    pub fn file_name(&self, cp: &ConstantPool) -> Arc<String> {
        cp.get_utf8(self.sourcefile_index)
    }
}