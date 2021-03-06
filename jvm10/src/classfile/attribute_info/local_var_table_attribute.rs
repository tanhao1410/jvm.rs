use crate::classfile::class_reader::ClassReader;

#[allow(dead_code)]
pub struct LocalVarTableAttribute{
    local_variable_table: Vec<LocalVariableTableEntry>
}
#[allow(dead_code)]
struct LocalVariableTableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

impl LocalVarTableAttribute {
    pub(crate) fn new(reader: &mut ClassReader) -> LocalVarTableAttribute {
        let local_variable_table_length = reader.read_u16();
        let mut local_variable_table = Vec::new();
        for _ in 0..local_variable_table_length {
            let start_pc = reader.read_u16();
            let length = reader.read_u16();
            let name_index = reader.read_u16();
            let descriptor_index = reader.read_u16();
            let index = reader.read_u16();
            local_variable_table.push(LocalVariableTableEntry { start_pc, length, name_index, descriptor_index, index });
        }
        Self { local_variable_table }
    }
}