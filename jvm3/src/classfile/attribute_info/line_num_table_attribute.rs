use crate::classfile::class_reader::ClassReader;

pub struct LineNumTableAttribute {
    line_num_table : Vec<LineNumTableEntry>
}

impl LineNumTableAttribute {
    pub fn new(reader:&mut ClassReader)->LineNumTableAttribute{
        let line_number_table_length = reader.read_u16();
        let mut line_num_table = Vec::new();
        for _ in 0..line_number_table_length {
            line_num_table.push(LineNumTableEntry::new(reader));
        }
        Self { line_num_table }
    }
}

pub struct LineNumTableEntry {
    start_pc: u16,
    line_num: u16,
}

impl LineNumTableEntry{
    fn new(reader:&mut ClassReader)->LineNumTableEntry{
        let start_pc = reader.read_u16();
        let line_num = reader.read_u16();
        Self{start_pc,line_num}
    }
}