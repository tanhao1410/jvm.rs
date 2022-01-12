use std::sync::{Arc, RwLock};

use crate::classfile::constant_info::ConstantInfo;
use crate::rtda::heap::class::Class;
use crate::rtda::heap::constant_pool::class_ref::ClassRef;
use crate::rtda::heap::constant_pool::constant::Constant::Empty;
use crate::rtda::heap::constant_pool::constant_pool::ConstantPool;
use crate::rtda::heap::constant_pool::field_ref::FieldRef;
use crate::rtda::heap::constant_pool::interface_method_ref::InterfaceMethodRef;
use crate::rtda::heap::constant_pool::method_ref::MethodRef;

pub enum Constant {
    Empty,
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(ClassRef),
    String(String),
    FieldRef(FieldRef),
    MethodRef(MethodRef),
    InterfaceMethodRef(InterfaceMethodRef),
    NameAndType(),
    MethodHandle(),
    MethodType(),
    Dynamic(),
    InvokeDynamic(),
}

impl Constant {
    /// 将class_file中的ConstantInfo 转换成Constant
    pub fn new(class: Arc<RwLock<Class>>,cp: Arc<RwLock<ConstantPool>>,info: &ConstantInfo) -> Self {

        match info {
            ConstantInfo::Empty => Empty,
            ConstantInfo::Integer(val) => Constant::Integer(val.value()),
            ConstantInfo::String(v) => Constant::String(v.string().to_string()),
            ConstantInfo::Utf8(v) => Constant::Utf8(v.string().to_string()),
            ConstantInfo::Float(v) => Constant::Float(v.value()),
            ConstantInfo::Long(v) => Constant::Long(v.value()),
            ConstantInfo::Class(v) => Constant::Class(ClassRef::new(cp,v)),
            ConstantInfo::FieldRef(v) => Constant::FieldRef(FieldRef::new(cp,v)),
            ConstantInfo::MethodRef(v) => Constant::MethodRef(MethodRef::new(cp,v)),
            ConstantInfo::InterfaceMethodRef(v) => Constant::InterfaceMethodRef(InterfaceMethodRef::new(cp,v)),
            ConstantInfo::NameAndType(v) => Constant::NameAndType(),
            ConstantInfo::MethodHandle(v) => Constant::MethodHandle(),
            ConstantInfo::MethodType(v) => Constant::MethodType(),
            ConstantInfo::InvokeDynamic(v) => Constant::InvokeDynamic(),
            ConstantInfo::Double(v) => Constant::Double(v.value())
        }
    }

    pub fn get_class_ref_mut(&mut self) -> &mut ClassRef {
        match self {
            Constant::Class(class_ref) => {
                class_ref.resolve_class();
                class_ref
            },
            _ => panic!("impossible.")
        }
    }

    pub fn get_field_ref(&self) -> &FieldRef {
        match self {
            Constant::FieldRef(field_ref) => field_ref,
            _ => panic!("impossible.")
        }
    }
    pub fn get_field_ref_mut(&mut self) -> &mut FieldRef {
        match self {
            Constant::FieldRef(field_ref) => field_ref,
            _ => panic!("impossible.")
        }
    }

    pub fn get_method_ref(&self) -> &MethodRef {
        match self {
            Constant::MethodRef(method_ref) => method_ref,
            _ => panic!("impossible.")
        }
    }

    pub fn get_method_ref_mut(&mut self) -> &mut MethodRef {
        match self {
            Constant::MethodRef(method_ref) => method_ref,
            _=> panic!("impossible.")
        }
    }
    pub fn get_interface_method_ref_mut(&mut self) -> &mut InterfaceMethodRef {
        match self {
            Constant::InterfaceMethodRef(method_ref) => method_ref,
            _=>panic!()
        }
    }

}