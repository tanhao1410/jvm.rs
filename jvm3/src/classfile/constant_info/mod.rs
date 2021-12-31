mod constant_info_utf8;
mod constant_info_integer;
mod constant_info_class;
mod constant_info_string;
mod constant_info_double;
mod constant_info_fieldref;
mod constant_info_memberref;
mod constant_info_float;
mod constant_info_interface_methodref;
mod constant_info_invoke_dynamic;
mod constant_info_long;
mod constant_info_method_handle;
mod constant_info_method_type;
mod constant_info_methodref;
mod constant_info_name_and_type;

use crate::classfile::class_reader::ClassReader;
use std::sync::{RwLock, Arc};
use crate::classfile::constant_pool::ConstantPool;
use crate::classfile::constant_info::constant_info_utf8::ConstantUtf8Info;
use crate::classfile::constant_info::constant_info_integer::ConstantIntegerInfo;
use crate::classfile::constant_info::constant_info_float::ConstantFloatInfo;
use crate::classfile::constant_info::constant_info_long::ConstantLongInfo;
use crate::classfile::constant_info::constant_info_double::ConstantDoubleInfo;
use crate::classfile::constant_info::constant_info_class::ConstantClassInfo;
use crate::classfile::constant_info::constant_info_string::ConstantStringInfo;
use crate::classfile::constant_info::constant_info_methodref::ConstantMethodrefInfo;
use crate::classfile::constant_info::constant_info_interface_methodref::ConstantInterfaceMethodrefInfo;
use crate::classfile::constant_info::constant_info_name_and_type::ConstantNameAndTypeInfo;
use crate::classfile::constant_info::constant_info_method_handle::ConstantMethodHandleInfo;
use crate::classfile::constant_info::constant_info_method_type::ConstantMethodTypeInfo;
use crate::classfile::constant_info::constant_info_invoke_dynamic::ConstantInvokeDynamicInfo;
use crate::classfile::constant_info::constant_info_fieldref::ConstantFiledrefInfo;


const CONSTANT_CLASS_______________: u8 = 7;
const CONSTANT_FIELD_REF___________: u8 = 9;
const CONSTANT_METHOD_REF__________: u8 = 10;
const CONSTANT_INTERFACE_METHOD_REF: u8 = 11;
const CONSTANT_STRING______________: u8 = 8;
const CONSTANT_INTEGER_____________: u8 = 3;
const CONSTANT_FLOAT_______________: u8 = 4;
const CONSTANT_LONG________________: u8 = 5;
const CONSTANT_DOUBLE______________: u8 = 6;
const CONSTANT_NAME_AND_TYPE_______: u8 = 12;
const CONSTANT_UTF8________________: u8 = 1;
const CONSTANT_METHOD_HANDLE_______: u8 = 15;
const CONSTANT_METHOD_TYPE_________: u8 = 16;
const CONSTANT_INVOKE_DYNAMIC______: u8 = 18;


pub enum ConstantInfo {
    Empty,
    Utf8(ConstantUtf8Info),
    Integer(ConstantIntegerInfo),

    Float(ConstantFloatInfo),
    Long(ConstantLongInfo),
    Double(ConstantDoubleInfo),
    Class(ConstantClassInfo),
    String(ConstantStringInfo),
    FieldRef(ConstantFiledrefInfo),
    MethodRef(ConstantMethodrefInfo),
    InterfaceMethodRef(ConstantInterfaceMethodrefInfo),
    NameAndType(ConstantNameAndTypeInfo),


    MethodHandle(ConstantMethodHandleInfo),
    MethodType(ConstantMethodTypeInfo),
    InvokeDynamic(ConstantInvokeDynamicInfo),

}

impl ConstantInfo {
    //传入ConstantPool的目的是：在常量类型中，类或接口的信息，name_index 指向的是常量池索引，需要从常量池中找到该信息。
    pub(crate) fn read_constant_info(reader: &mut ClassReader, cp: Arc<RwLock<ConstantPool>>) -> ConstantInfo {
        let tag = reader.read_u8();
        match tag {
            CONSTANT_UTF8________________ => Self::Utf8(ConstantUtf8Info::new(reader)),
            CONSTANT_INTEGER_____________ => Self::Integer(ConstantIntegerInfo::new(reader)),
            CONSTANT_FLOAT_______________ => Self::Float(ConstantFloatInfo::new(reader)),
            CONSTANT_LONG________________ => Self::Long(ConstantLongInfo::new(reader)),
            CONSTANT_DOUBLE______________ => Self::Double(ConstantDoubleInfo::new(reader)),
            CONSTANT_CLASS_______________ => Self::Class(ConstantClassInfo::new(reader, cp.clone())),
            CONSTANT_STRING______________ => Self::String(ConstantStringInfo::new(reader, cp.clone())),
            CONSTANT_FIELD_REF___________ => Self::FieldRef(ConstantFiledrefInfo::new(reader, cp.clone())),
            CONSTANT_METHOD_REF__________ => Self::MethodRef(ConstantMethodrefInfo::new(reader, cp.clone())),
            CONSTANT_INTERFACE_METHOD_REF => Self::InterfaceMethodRef(ConstantInterfaceMethodrefInfo::new(reader, cp.clone())),
            CONSTANT_NAME_AND_TYPE_______ => Self::NameAndType(ConstantNameAndTypeInfo::new(reader)),
            CONSTANT_METHOD_HANDLE_______ => Self::MethodHandle(ConstantMethodHandleInfo::new(reader)),
            CONSTANT_METHOD_TYPE_________ => Self::MethodType(ConstantMethodTypeInfo::new(reader)),
            CONSTANT_INVOKE_DYNAMIC______ => Self::InvokeDynamic(ConstantInvokeDynamicInfo::new(reader)),
            _ => panic!("java.lang.ClassFormatError: constant pool tag {}", tag),
        }
    }
}