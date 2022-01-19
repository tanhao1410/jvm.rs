use crate::classfile::constant_pool::ConstantPool;
use crate::classfile::member_info::MemberInfo;
use crate::classfile::attribute_info::AttributeInfo;
use std::sync::{RwLock, Arc};
use crate::classfile::class_reader::ClassReader;
use std::ops::Deref;
#[allow(dead_code)]
pub struct ClassFile {
    minor_version: u16,
    major_version: u16,
    //constant_pool: ConstantPool,
    //常量池会出现共享的情况
    constant_pool: Arc<RwLock<ConstantPool>>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<MemberInfo>,
    methods: Vec<MemberInfo>,
    attributes: Vec<AttributeInfo>,
}

impl ClassFile{
    pub fn parse(data:Vec<u8>)->Self{
        let cr = ClassReader::new(data);
        Self::read(cr)
    }

    pub fn read(mut reader:ClassReader) ->Self{
        Self::read_magic_and_check(&mut reader);
        let (minor_version,major_version) = Self::read_and_check_version(&mut reader);

        //读取常量池，常量池里面的数据比较复杂，通过一个专门的常量池结构体来表示。由它去处理
        let contant_pool = ConstantPool::read_constant_pool(&mut reader);

        let access_flags = reader.read_u16();
        let this_class = reader.read_u16();
        let super_class = reader.read_u16();
        let interfaces = reader.read_u16s();

        let fields = MemberInfo::read_members(&mut reader, contant_pool.clone());
        let methods = MemberInfo::read_members(&mut reader, contant_pool.clone());

        let attributes = AttributeInfo::read_attributes(&mut reader, contant_pool.clone());

        Self { minor_version, major_version, constant_pool: contant_pool, access_flags, this_class, super_class,
            interfaces, fields, methods, attributes }
    }

    fn read_magic_and_check(reader : &mut ClassReader)->u32{
        let magic = reader.read_u32();
        if magic != 0xCAFEBABE {
            panic!("java.lang.ClassFormatError: magic!");
        }
        magic
    }

    fn read_and_check_version(reader: &mut ClassReader) -> (u16, u16) {
        let minor_version: u16 = reader.read_u16();
        let major_version: u16 = reader.read_u16();

        let res = (minor_version, major_version);

        match res {
            (_, 45) => res,
            (0, 46) => res,
            (0, 47) => res,
            (0, 48) => res,
            (0, 49) => res,
            (0, 50) => res,
            (0, 51) => res,
            (0, 52) => res,
            _ => panic!("java.lang.UnsupportedClassVersionError!")
        }
    }


    pub fn constant_pool(&self) -> Arc<RwLock<ConstantPool>> {
        self.constant_pool.clone()
    }
    pub fn access_flags(&self) -> u16 {
        self.access_flags
    }
    pub fn fields(&self) -> &Vec<MemberInfo> {
        &self.fields
    }
    pub fn methods(&self) -> &Vec<MemberInfo> {
        &self.methods
    }

    pub fn class_name(&self) -> Arc<String> {
        self.constant_pool.read().unwrap().deref().class_name(self.this_class)
    }
    pub fn super_class_name(&self) -> Arc<String> {
        if self.super_class > 0 {
            self.constant_pool.read().unwrap().class_name(self.super_class)
        } else {
            Arc::new("".to_string())
        }
    }
    pub fn interface_names(&self) -> Vec<Arc<String>> {
        let mut names = Vec::new();
        for i in &self.interfaces {
            names.push(self.constant_pool.read().unwrap().class_name(*i));
        }
        names
    }

    pub fn get_source_file(&self)->Arc<String>{
        let cp = &self.constant_pool;
        let cp = cp.read().unwrap();
        for attr in &self.attributes{
            match attr{
                AttributeInfo::SourceFileAttr(attr) => {
                    return attr.file_name(&*cp);
                },
                _=>{}
            }
        }
        Arc::new("Unknown".to_string())
    }
}