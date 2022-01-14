use std::sync::{Arc, RwLock};

use crate::classfile::member_info::MemberInfo;
use crate::rtda::heap::access_flags::AccessFlags;
use crate::rtda::heap::class::Class;

pub struct Field {
    access_flags: u16,
    pub name: Arc<String>,
    pub descriptor: Arc<String>,
    pub class: Arc<RwLock<Class>>,
    //用于 知道静态变量和实例变量需要多少空间，给每一个变量编上号
    pub slot_id:usize,
    // 类常量中（final static 基本类型和string）有些存储在了常量池中，记录索引
    pub const_value_index:usize,
}
#[allow(dead_code)]
impl Field {
    fn new(class: Arc<RwLock<Class>>, meminfo: &MemberInfo) -> Arc<RwLock<Field>> {
        Arc::new(RwLock::new(Self{
            access_flags:meminfo.access_flgs(),
            name:meminfo.name().clone(),
            descriptor:meminfo.descriptor().clone(),
            class:class,
            slot_id: 0,
            const_value_index: meminfo.constant_value_index()
        }))
    }

    pub fn new_fields(class: Arc<RwLock<Class>>, meminfos: &Vec<MemberInfo>) -> Vec<Arc<RwLock<Field>>> {
        let mut res = vec![];
        for mem in meminfos.iter(){
            res.push(Self::new(class.clone(),mem));
        }
        res
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

    pub fn is_long_or_double(&self) -> bool {
        self.descriptor.as_str() == "J" || self.descriptor.as_str() == "D"
    }

    pub fn is_accessible_to(&self, other_class: Arc<RwLock<Class>>) -> bool {
        // public的情况下，可以
        if self.is_public() {
            true
        } else {
            let cur_class = self.class.clone();
            if self.is_protected() {
                other_class.read().unwrap().name.as_str() == cur_class.read().unwrap().name.as_str() //是同一个类
                    || other_class.read().unwrap().is_sub_class_of(cur_class.clone()) //other 是当前类的子类
                    || cur_class.read().unwrap().package_name().eq(other_class.read().unwrap().package_name()) //两个类处在同一个包中
            } else if !self.is_private() {
                cur_class.read().unwrap().package_name().eq(other_class.read().unwrap().package_name()) //非私有，非保护，看包名是否相同
            } else {
                other_class.read().unwrap().name.as_str() == cur_class.read().unwrap().name.as_str() // 看是否是同一个类
            }
        }
    }
}