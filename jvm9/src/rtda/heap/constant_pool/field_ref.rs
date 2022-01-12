use std::sync::{Arc, RwLock};
use crate::rtda::heap::constant_pool::constant_pool::ConstantPool;
use crate::rtda::heap::class::Class;
use crate::classfile::constant_info::constant_info_fieldref::ConstantFiledrefInfo;
use crate::rtda::heap::field::Field;
use crate::rtda::heap::class_loader::ClassLoader;

pub struct FieldRef {
    pub cp: Arc<RwLock<ConstantPool>>,
    pub class_name: Arc<String>,
    pub class: Option<Arc<RwLock<Class>>>,

    //相比类符号引用多了的
    pub name: Arc<String>,
    pub descriptor: Arc<String>,
    pub field:Option<Arc<RwLock<Field>>>,

    pub cur_class:Arc<RwLock<Class>>,
}

impl FieldRef {
    pub fn new(cp: Arc<RwLock<ConstantPool>>,info: &ConstantFiledrefInfo)->Self{
        Self{
            cur_class:cp.clone().read().unwrap().get_class().clone(),
            cp,
            class_name: info.class_name(),
            class:None,
            name: info.member().name_and_descriptor().0,
            descriptor: info.member().name_and_descriptor().1,
            field: None
        }
    }

    pub fn resolve_field(&mut self)->Arc<RwLock<Field>>{
        if self.field.is_none(){
            let cur_class = self.cur_class.clone();
            //解析引用类
            // 获取类加载器
            let class_loader = cur_class.read().unwrap().class_loader().clone();
            // 用该class loader 加载这个引用类
            let ref_class = ClassLoader::load_class(class_loader, self.class_name.clone());

            let field = Self::lookup_field(ref_class.clone(),self.name.clone().as_str(),self.descriptor.clone().as_str());
            if field.is_none(){
                panic!("java.lang.NoSuchFieldError");
            }

            if !field.as_ref().unwrap().read().unwrap().is_accessible_to(cur_class){
                panic!("java.lang.IllegalAccessError");
            }

            self.class = Some(ref_class.clone());
            self.field = field;
        }
        self.field.as_ref().unwrap().clone()
    }


    fn lookup_field(class: Arc<RwLock<Class>>, name: &str, descriptor: &str) -> Option<Arc<RwLock<Field>>> {

        for field in &class.read().unwrap().fields {
            if field.read().unwrap().name.as_str().eq(name) && field.read().unwrap().descriptor.as_str().eq(descriptor) {
                return Some(field.clone());
            }
        }

        //从类实现的接口去找
        for iface in &class.read().unwrap().interfaces {
            let field = Self::lookup_field(iface.clone(), name, descriptor);
            match field {
                Some(f) => return Some(f),
                None => {}
            }
        }

        //从父类中去找
        match &class.read().unwrap().super_class {
            Some(cptr) => Self::lookup_field(cptr.clone(), name, descriptor),
            None => None
        }
    }

}

