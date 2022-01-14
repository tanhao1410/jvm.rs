use crate::classfile::attribute_info::AttributeInfo;
use crate::classfile::class_reader::ClassReader;
use std::sync::{RwLock, Arc};
use crate::classfile::constant_pool::ConstantPool;
use crate::classfile::attribute_info::line_num_table_attribute::LineNumTableEntry;

#[allow(dead_code)]
pub struct CodeAttribute {
    max_stack: u16,
    max_locals: u16,
    code_length: u32,
    //code: Vec<u8>,
    code: Arc<Vec<u8>>,
    //exception_table_length: u16,
    exception_table: Vec<ExceptionTableEntry>,
    //attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

#[allow(dead_code)]
impl CodeAttribute {
    pub fn new(reader: &mut ClassReader, cp: Arc<RwLock<ConstantPool>>) -> Self {
        let max_stack = reader.read_u16();
        let max_locals = reader.read_u16();
        let code_length = reader.read_u32();
        let code = Arc::new(reader.read_bytes(code_length));
        let exception_table_length = reader.read_u16();
        let mut exception_table = vec![];
        for _ in 0..exception_table_length {
            exception_table.push(ExceptionTableEntry::new(reader));
        }
        let attributes = AttributeInfo::read_attributes(reader, cp);
        Self {
            max_stack,
            max_locals,
            code_length,
            code,
            exception_table,
            attributes,
        }
    }

    pub fn max_stack(&self) -> u16 { self.max_stack }
    pub fn max_locals(&self) -> u16 { self.max_locals }
    pub fn code(&self) -> Arc<Vec<u8>> { self.code.clone() }
    pub fn exception_table(&self) -> &Vec<ExceptionTableEntry> { &self.exception_table }
    pub fn attributes(&self) -> &Vec<AttributeInfo> { &self.attributes }

    pub fn line_number_table_attribute(&self) -> Arc<Vec<LineNumTableEntry>> {
        for attr in &self.attributes {
            match attr {
                AttributeInfo::LineNumTableAttr(attr) => {
                    return attr.line_num_table.clone();
                }
                _ => {}
            }
        }
        Arc::new(vec![])
    }
}

#[allow(dead_code)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl ExceptionTableEntry {
    pub fn new(reader: &mut ClassReader) -> Self {
        ExceptionTableEntry {
            start_pc: reader.read_u16(),
            end_pc: reader.read_u16(),
            handler_pc: reader.read_u16(),
            catch_type: reader.read_u16(),
        }
    }
}