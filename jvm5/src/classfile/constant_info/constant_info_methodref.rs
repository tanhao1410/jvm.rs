use crate::classfile::constant_info::constant_info_memberref::ConstantMemberrefInfo;
use crate::classfile::constant_pool::ConstantPool;
use std::sync::{Arc, RwLock};
use crate::classfile::class_reader::ClassReader;

pub struct ConstantMethodrefInfo{
    member : ConstantMemberrefInfo
}

impl ConstantMethodrefInfo{
    pub fn new(reader:&mut ClassReader,cp:Arc<RwLock<ConstantPool>>) ->Self{

        ConstantMethodrefInfo{member:ConstantMemberrefInfo::new(reader,cp.clone())}
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

