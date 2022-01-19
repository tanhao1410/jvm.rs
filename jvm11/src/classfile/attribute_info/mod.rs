mod deprecated_attribute;
mod unparsed_attribute;
pub mod code_attribute;
mod constant_value_attribute;
mod exceptions_attribute;
pub mod line_num_table_attribute;
mod local_var_table_attribute;
mod source_file_attribute;
mod synthetic_attribute;

use crate::classfile::class_reader::ClassReader;
use std::sync::{RwLock, Arc};
use crate::classfile::constant_pool::ConstantPool;
use crate::classfile::attribute_info::unparsed_attribute::UnparsedAttribute;
use crate::classfile::attribute_info::code_attribute::CodeAttribute;
use crate::classfile::attribute_info::constant_value_attribute::ConstantValueAttribute;
use crate::classfile::attribute_info::deprecated_attribute::DeprecatedAttribute;
use crate::classfile::attribute_info::exceptions_attribute::ExceptionsAttribute;
use crate::classfile::attribute_info::line_num_table_attribute::LineNumTableAttribute;
use crate::classfile::attribute_info::local_var_table_attribute::LocalVarTableAttribute;
use crate::classfile::attribute_info::source_file_attribute::SourceFileAttribute;
use crate::classfile::attribute_info::synthetic_attribute::SyntheticAttribute;

pub enum AttributeInfo{
    CodeAttr(CodeAttribute),
    ConstantValueAttr(ConstantValueAttribute),
    DeprecatedAttr(DeprecatedAttribute),
    ExceptionsAttr(ExceptionsAttribute),
    LineNumTableAttr(LineNumTableAttribute),
    LocalVarTableAttr(LocalVarTableAttribute),
    SourceFileAttr(SourceFileAttribute),
    SyntheticAttr(SyntheticAttribute),
    UnparsedAttr(UnparsedAttribute)
}

impl AttributeInfo {
    fn new(name_index: u16, name: &str, length: u32, reader: &mut ClassReader, cp: Arc<RwLock<ConstantPool>>) -> AttributeInfo {
        match name {
            "Code" => Self::CodeAttr(CodeAttribute::new(reader, cp)),
            "ConstantValue" => Self::ConstantValueAttr(ConstantValueAttribute::new(reader)),
            "Deprecated" => Self::DeprecatedAttr(DeprecatedAttribute {}),
            "Exceptions" => Self::ExceptionsAttr(ExceptionsAttribute::new(reader)),
            "LineNumberTable" => Self::LineNumTableAttr(LineNumTableAttribute::new(reader)),
            "LocalVariableTable" => Self::LocalVarTableAttr(LocalVarTableAttribute::new(reader)),
            "SourceFile" => Self::SourceFileAttr(SourceFileAttribute::new(reader)),
            "Synthetic" => Self::SyntheticAttr(SyntheticAttribute {}),
            _ => Self::UnparsedAttr(UnparsedAttribute::new(name_index, length, reader)),
        }
    }

    pub fn read_attributes(reader: &mut ClassReader, cp: Arc<RwLock<ConstantPool>>) -> Vec<AttributeInfo> {
        let count = reader.read_u16();
        let mut attributes = Vec::new();
        for _ in 0..count {
            attributes.push(Self::read_attribute(reader, cp.clone()));
        }
        attributes
    }

    /// 读取一个属性，每一个属性都是先一个名称的index（u16)，然后接下来一个u32 的长度，后面是数据
    fn read_attribute(reader:&mut ClassReader, cp:Arc<RwLock<ConstantPool>>) ->AttributeInfo{
        //先根据第一个index读取看是哪一个属性
        let name_index = reader.read_u16();
        let attr_name = cp.read().unwrap().get_utf8(name_index);
        let attr_len = reader.read_u32();
        AttributeInfo::new(name_index, attr_name.as_str(), attr_len, reader, cp)
    }
}