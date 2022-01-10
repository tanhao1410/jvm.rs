use crate::rtda::heap::access_flags::AccessFlags::{ACC_ABSTRACT, ACC_FINAL, ACC_INTERFACE, ACC_NATIVE, ACC_PRIVATE, ACC_PROTEDTED, ACC_PUBLIC, ACC_STATIC, ACC_SYNTHETIC};

#[allow(non_camel_case_types)]
/// 字段和方法的访问标志
pub enum AccessFlags {
    // class field method
    ACC_PUBLIC = 0x0001,
    //       field method
    ACC_PRIVATE = 0x0002,
    //       field method
    ACC_PROTEDTED = 0x0004,
    //       field method
    ACC_STATIC = 0x0008,
    // class field method
    ACC_FINAL = 0x0010,
    // class
    //ACC_SUPER = 0x0020,
    //             method
    ACC_SYNCHRONIZED_OR_ACC_SUPER = 0x0020,
    //       field
    //ACC_VOLATILE = 0x0040,
    //             method
    ACC_BRIDGE_OR_ACC_VOLATILE = 0x0040,
    //       field
    //ACC_TRANSIENT = 0x0080,
    //             method
    ACC_VARARGS_OR_ACC_TRANSIENT = 0x0080,
    //             method
    ACC_NATIVE = 0x0100,
    // class
    ACC_INTERFACE = 0x0200,
    // class
    ACC_ABSTRACT = 0x0400,
    //             method
    ACC_STRICT = 0x0800,
    // class field method
    ACC_SYNTHETIC = 0x1000,
    // class
    ACC_ANNOTATION = 0x2000,
    // class field
    ACC_ENUM = 0x4000,

}

impl AccessFlags {
    pub(crate) fn is_public(access_flags: u16) -> bool {
        access_flags & ACC_PUBLIC as u16 != 0
    }
    pub fn is_static(access_flags: u16) -> bool {
        access_flags & ACC_STATIC as u16 != 0
    }

    pub fn is_private(access_flags: u16) -> bool {
        access_flags & ACC_PRIVATE as u16 != 0
    }

    pub fn is_protected(access_flags: u16) -> bool {
        access_flags & ACC_PROTEDTED as u16 != 0
    }

    pub fn is_synthetic(access_flags: u16) -> bool {
        access_flags & ACC_SYNTHETIC as u16 != 0
    }

    pub fn is_final(access_flags: u16) -> bool { access_flags & ACC_FINAL as u16 != 0 }

    pub fn is_interface(access_flags: u16) -> bool {
        access_flags & ACC_INTERFACE as u16 != 0
    }
    pub fn is_abstract(access_flags: u16) -> bool {
        access_flags & ACC_ABSTRACT as u16 != 0
    }

    pub fn is_native(access_flags: u16) -> bool { access_flags & ACC_NATIVE as u16 != 0 }
}



