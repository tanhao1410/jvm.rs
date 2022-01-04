use std::sync::{Arc, RwLock};
use crate::rtda::heap::class::Class;
use crate::rtda::heap::constant_pool::constant::Constant;
use crate::classfile::constant_pool::ConstantPool as ClassFileConstantPool;

///运行时常量池，主要存放两类信息，字面量与 符合引用（类符合引用，字段符号引用，方法符号引用，接口方法符号引用）
pub struct ConstantPool {
    class: Arc<RwLock<Class>>,
    consts: Vec<Constant>,
}

impl ConstantPool {
    /// 把class文件中的常量池转换成运行时常量池
    pub fn new(class: Arc<RwLock<Class>>, cf_cp: Arc<RwLock<ClassFileConstantPool>>) -> Arc<RwLock<ConstantPool>> {

        let mut constant_pool = Arc::new(RwLock::new(Self{ class:class.clone(), consts: vec![] }));

        let count = cf_cp.read().unwrap().constants_count();
        let temp = cf_cp.read().unwrap();
        let infos = temp.constant_infos();
        let mut i = 1;
        let mut consts = vec![Constant::Empty];
        while i < count {
            let constant = Constant::new(class.clone(),constant_pool.clone(),&infos[i]);
            match &constant {
                Constant::Long(_) => {
                    consts.push(Constant::Empty);
                    i += 1;
                }
                Constant::Double(_) => {
                    consts.push(Constant::Empty);
                    i += 1;
                }
                _ => {}
            }
            consts.push(constant);
            i += 1;
        }
        constant_pool.write().unwrap().consts = consts;
        constant_pool
    }

    /// 根据序号 获取常量值
    pub fn get_constant(&self, index: usize) -> &Constant {
        &self.consts[index]
    }

    pub fn get_constant_mut(&mut self,index :usize)->&mut Constant{
        &mut self.consts[index]
    }

    pub fn get_class(&self)->Arc<RwLock<Class>>{
        self.class.clone()
    }
}