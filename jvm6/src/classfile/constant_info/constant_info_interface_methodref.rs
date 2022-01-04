use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_info::constant_info_memberref::ConstantMemberrefInfo;
use crate::classfile::constant_pool::ConstantPool;
use std::sync::{Arc, RwLock};

pub struct ConstantInterfaceMethodrefInfo{
    member : ConstantMemberrefInfo
}

impl ConstantInterfaceMethodrefInfo{
    pub fn new(reader:&mut ClassReader,cp:Arc<RwLock<ConstantPool>>) ->Self{

        ConstantInterfaceMethodrefInfo{member:ConstantMemberrefInfo::new(reader,cp.clone())}
    }

    pub fn class_name(&self) -> Arc<String> {
        self.member.class_name()
    }
    pub fn name_and_descriptor(&self) -> (Arc<String>, Arc<String>) {
        self.member.name_and_descriptor()
    }
    pub fn member(&self) -> &ConstantMemberrefInfo {
        &self.member
    }
}

