use std::sync::{RwLock, Arc};
use crate::classfile::constant_pool::ConstantPool;
use crate::classfile::attribute_info::AttributeInfo;
use crate::classfile::class_reader::ClassReader;
use crate::classfile::attribute_info::code_attribute::CodeAttribute;

///字段和方法的基本结构
pub struct MemberInfo {
    cp: Arc<RwLock<ConstantPool>>,
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

impl MemberInfo{
    pub fn read_members(reader:&mut ClassReader,cp:Arc<RwLock<ConstantPool>>)->Vec<Self>{
        let count = reader.read_u16();
        let mut members = vec![];
        for _ in 0..count{
            members.push(Self::read_member(reader,cp.clone()));
        }
        members
    }

    pub fn read_member(reader:&mut ClassReader,cp:Arc<RwLock<ConstantPool>>)->Self{
        let access_flags = reader.read_u16();
        let name_index = reader.read_u16();
        let descriptor_index = reader.read_u16();
        let attributes = AttributeInfo::read_attributes(reader,cp.clone());
        MemberInfo{ cp, access_flags, name_index,descriptor_index,attributes }
    }

    pub fn access_flgs(&self) -> u16 {
        self.access_flags
    }

    pub fn name(&self) -> Arc<String> {
        self.cp.read().unwrap().get_utf8(self.name_index)
    }

    pub fn descriptor(&self) -> Arc<String> {
        self.cp.read().unwrap().get_utf8(self.descriptor_index)
    }

    /// 返回类方法的 code属性,类方法可能没有该属性，本地方法
    pub fn code_attribute(&self)->Option<&CodeAttribute>{
        for attr in self.attributes.iter(){
            if let AttributeInfo::CodeAttr(attr) = attr{
                return Some(attr);
            }
        }
        //panic!("no code error")
        None
    }

    pub fn constant_value_index(&self)->usize {
        for attr in &self.attributes {
            if let AttributeInfo::ConstantValueAttr(value) = attr{
                return value.value_index() as usize;
            }
        }
        0
    }


}