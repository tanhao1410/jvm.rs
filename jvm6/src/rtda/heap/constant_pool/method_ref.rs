use crate::rtda::heap::class::Class;
use std::sync::{RwLock, Arc};
use crate::rtda::heap::constant_pool::constant_pool::ConstantPool;
use crate::rtda::heap::method::Method;
use crate::classfile::constant_info::constant_info_methodref::ConstantMethodrefInfo;
use crate::classfile::constant_pool::ConstantPool as ClassFileConstantPool;

pub struct MethodRef{
    cp: Arc<RwLock<ConstantPool>>,
    class_name: Arc<String>,
    class: Option<Arc<RwLock<Class>>>,

    //相比类符号引用多了的
    pub name: Arc<String>,
    pub descriptor: Arc<String>,
    method:Option<Arc<RwLock<Method>>>
}

impl MethodRef{
    pub fn new(cp:Arc<RwLock<ConstantPool>>,info: &ConstantMethodrefInfo)->Self{
        Self{
            cp,
            class_name: info.class_name(),
            class:None,
            name: info.member().name_and_descriptor().0,
            descriptor: info.member().name_and_descriptor().1,
            method: None
        }
    }
}