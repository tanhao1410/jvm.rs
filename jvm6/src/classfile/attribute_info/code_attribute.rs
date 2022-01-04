use crate::classfile::attribute_info::AttributeInfo;
use crate::classfile::class_reader::ClassReader;
use std::sync::{RwLock, Arc};
use crate::classfile::constant_pool::ConstantPool;

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
}

pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
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