use crate::classfile::class_reader::ClassReader;
use std::sync::Arc;

#[allow(dead_code)]
pub struct LineNumTableAttribute {
    pub line_num_table : Arc<Vec<LineNumTableEntry>>
}

impl LineNumTableAttribute {
    pub fn new(reader:&mut ClassReader)->LineNumTableAttribute{
        let line_number_table_length = reader.read_u16();
        let mut line_num_table = Vec::new();
        for _ in 0..line_number_table_length {
            line_num_table.push(LineNumTableEntry::new(reader));
        }
        let line_num_table = Arc::new(line_num_table);
        Self { line_num_table }
    }
}

#[allow(dead_code)]
pub struct LineNumTableEntry {
    pub start_pc: u16,
    pub line_num: u16,
}

impl LineNumTableEntry{
    fn new(reader:&mut ClassReader)->LineNumTableEntry{
        let start_pc = reader.read_u16();
        let line_num = reader.read_u16();
        Self{start_pc,line_num}
    }
}