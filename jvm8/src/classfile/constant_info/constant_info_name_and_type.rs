use crate::classfile::class_reader::ClassReader;

///字段或方法的名称和描述符
pub struct ConstantNameAndTypeInfo {
    name_index: u16,
    descriptor_index: u16,
}

impl ConstantNameAndTypeInfo {
    pub(crate) fn new(reader: &mut ClassReader) -> ConstantNameAndTypeInfo {
        let name_index = reader.read_u16();
        let descriptor_index = reader.read_u16();
        Self { name_index, descriptor_index }
    }

    pub(crate) fn name_index(&self)->u16{
        self.name_index
    }

    pub(crate) fn description_index(&self)->u16{
        self.descriptor_index
    }
}

