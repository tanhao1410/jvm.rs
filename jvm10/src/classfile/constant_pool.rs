use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_info::ConstantInfo;
use std::sync::{Arc, RwLock};

///class 文件中的常量池
pub struct ConstantPool{
    constant_infos: Vec<ConstantInfo>
}

impl ConstantPool{
    //读取常量池
    pub fn read_constant_pool(reader : &mut ClassReader)->Arc<RwLock<ConstantPool>>{
        //先读取数量
        let count = reader.read_u16();

        let constant_pool = Arc::new(RwLock::new(ConstantPool{constant_infos:vec![]}));

        //注意事项：0是无效索引，表示不指向任何常量
        //long与double各占两个位置
        let mut constant_infos = Vec::with_capacity(count as usize);
        constant_infos.push(ConstantInfo::Empty);
        let mut i = 1;
        while i < count {
            let constant_info = ConstantInfo::read_constant_info(reader, constant_pool.clone());
            match &constant_info {
                ConstantInfo::Long(_) => {
                    constant_infos.push(constant_info);
                    constant_infos.push(ConstantInfo::Empty);
                    i += 1;
                }
                ConstantInfo::Double(_) => {
                    constant_infos.push(constant_info);
                    constant_infos.push(ConstantInfo::Empty);
                    i += 1;
                }
                _ => {
                    constant_infos.push(constant_info);
                }
            }
            i += 1;
        }
        constant_pool.write().unwrap().constant_infos = constant_infos;
        constant_pool
    }

    pub(crate) fn get_utf8(&self, index: u16) -> Arc<String> {
        let constant_info = self.get_constant_info(index);
        match constant_info {
            ConstantInfo::Utf8(info) => info.string(),
            _ => panic!("impossible.")
        }
    }

    pub(crate) fn name_and_type(&self, index: u16) -> (Arc<String>, Arc<String>) {
        let constant_info = self.get_constant_info(index);

        match constant_info {
            ConstantInfo::NameAndType(info) => {
                let _name = self.get_utf8(info.name_index());
                let _type = self.get_utf8(info.description_index());
                (_name, _type)
            }
            _ => panic!("impossible.")
        }
    }

    pub(crate) fn class_name(&self, class_index: u16) -> Arc<String> {
        let constant_info = self.get_constant_info(class_index);
        match constant_info {
            ConstantInfo::Class(info) => info.name(),
            _ => panic!("impossible.")
        }
    }

    pub fn constants_count(&self)->usize{
        self.constant_infos.len()
    }

    pub fn constant_infos(&self)->&Vec<ConstantInfo>{
        &self.constant_infos
    }

    fn get_constant_info(&self,index : u16)->&ConstantInfo{
        match self.constant_infos.get(index as usize) {
            Some(c) => c,
            None => panic!("impossible.")
        }
    }
}