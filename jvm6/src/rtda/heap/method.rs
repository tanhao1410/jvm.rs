use std::sync::{RwLock, Arc};
use crate::rtda::heap::class::Class;
use crate::classfile::member_info::MemberInfo;
use crate::rtda::heap::access_flags::AccessFlags;


pub struct Method {
    access_flags: u16,
    pub name: Arc<String>,
    pub descriptor: Arc<String>,
    class: Arc<RwLock<Class>>,
    pub max_stack: usize,
    pub max_locals: usize,
    pub code: Arc<Vec<u8>>,
}

impl Method {
    fn new(class: Arc<RwLock<Class>>, meminfo: &MemberInfo) -> Arc<Method> {
        let code_attr = meminfo.code_attribute();
        if let Some(code_attr) = code_attr{
            Arc::new(Self {
                access_flags: meminfo.access_flgs(),
                name: meminfo.name(),
                descriptor: meminfo.descriptor(),
                class,
                max_stack: code_attr.max_stack() as usize,
                max_locals: code_attr.max_locals() as usize,
                code: code_attr.code().clone(),
            })
        }else{
            Arc::new(Self {
                access_flags: meminfo.access_flgs(),
                name: meminfo.name(),
                descriptor: meminfo.descriptor(),
                class,
                max_stack: 4,
                max_locals:4,
                code: Arc::new(vec![]),
            })
        }
    }

    pub fn new_methods(class: Arc<RwLock<Class>>, meminfos: &Vec<MemberInfo>) -> Vec<Arc<Method>> {
        meminfos.iter().map(|mem| {
            Self::new(class.clone(), mem)
        }).collect()
    }

    pub fn is_static(&self) -> bool {
        AccessFlags::is_static(self.access_flags)
    }
    pub fn is_private(&self) -> bool {
        AccessFlags::is_private(self.access_flags)
    }
    pub fn is_protected(&self) -> bool {
        AccessFlags::is_protected(self.access_flags)
    }
    pub fn is_public(&self) -> bool { AccessFlags::is_public(self.access_flags) }
    pub fn is_synthetic(&self) -> bool {
        AccessFlags::is_synthetic(self.access_flags)
    }
    pub fn is_final(&self) -> bool { AccessFlags::is_final(self.access_flags) }

    pub fn constant_pool(&self) -> Arc<RwLock<crate::rtda::heap::constant_pool::constant_pool::ConstantPool>> {
        self.class.read().unwrap().constant_pool.as_ref().unwrap().clone()
    }
}