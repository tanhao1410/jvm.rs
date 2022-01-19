use std::ops::Deref;
use std::sync::{Arc, RwLock};

use crate::classfile::constant_info::constant_info_class::ConstantClassInfo;
use crate::rtda::heap::class::Class;
use crate::rtda::heap::class_loader::ClassLoader;
use crate::rtda::heap::constant_pool::constant_pool::ConstantPool;

#[allow(dead_code)]
pub struct ClassRef{
    cp : Arc<RwLock<ConstantPool>>,
    class_name:Arc<String>,
    pub class:Option<Arc<RwLock<Class>>>,

    //代表着当前类，即该类引用属于哪个类的运行时常量池
    pub cur_class:Arc<RwLock<Class>>
}

impl  ClassRef{
    pub fn new(cp: Arc<RwLock<ConstantPool>>,info: &ConstantClassInfo)->Self{
        Self{
            cur_class: cp.clone().read().unwrap().get_class().clone(),
            cp,
            class_name: info.name(),
            class:None,
        }
    }

    /// 类符号引用解析
    pub fn resolve_class(&mut self)->Arc<RwLock<Class>>{
        if self.class.is_none(){
            //得到当前的类
            let cur_class = self.cur_class.clone();
            // 获取类加载器
            let class_loader = cur_class.read().unwrap().class_loader().clone();
            // 用该class loader 加载这个引用类
            let ref_class = ClassLoader::load_class(class_loader, self.class_name.as_str());

            if !ref_class.read().unwrap().is_accessible_to(cur_class.read().unwrap().deref()){
                panic!("java.lang.IllegalAccessError");
            }
            self.class = Some(ref_class.clone());
        }
        self.class.as_ref().unwrap().clone()
    }

}