use crate::rtda::heap::class::Class;
use std::sync::{RwLock, Arc};
use crate::rtda::heap::constant_pool::constant_pool::ConstantPool;
use crate::rtda::heap::method::Method;
use crate::classfile::constant_info::constant_info_methodref::ConstantMethodrefInfo;
use crate::rtda::heap::class_loader::ClassLoader;
#[allow(dead_code)]
pub struct MethodRef{
    cp: Arc<RwLock<ConstantPool>>,
    class_name: Arc<String>,
    pub(crate) class: Option<Arc<RwLock<Class>>>,
    //当前的类
    pub cur_class:Arc<RwLock<Class>>,

    //相比类符号引用多了的
    pub name: Arc<String>,
    pub descriptor: Arc<String>,
    pub method:Option<Arc<Method>>
}

impl MethodRef{
    pub fn new(cp:Arc<RwLock<ConstantPool>>,info: &ConstantMethodrefInfo)->Self{
        Self{
            cur_class:cp.clone().read().unwrap().get_class().clone(),
            cp,
            class_name: info.class_name(),
            class:None,
            name: info.member().name_and_descriptor().0,
            descriptor: info.member().name_and_descriptor().1,
            method: None
        }
    }

    /// 非接口方法引用解析
    pub fn resolve_method(&mut self)->Arc<Method>{
        if self.method.is_none(){
            self.resolve_method_ref();
        }
        self.method.as_ref().unwrap().clone()
    }

    pub fn resolve_method_ref(&mut self){
        let cur_class = self.cur_class.clone();

        let ref_class = ClassLoader::load_class(cur_class.read().unwrap().loader.clone(),
                                                self.class_name.as_str());

        self.class = Some(ref_class.clone());

        if ref_class.read().unwrap().is_interface(){
            panic!("java.lang.IncompatiibleClassChangeError");
        }

        let method = ref_class.read().unwrap().lookup_method(self.name.as_str(),self.descriptor.as_str());
        if method.is_none(){
            panic!("java.lang.NoSuchMethodError");
        }

        if !method.as_ref().unwrap().is_accessible_to(ref_class){
            panic!("java.lang.IllegalAccessError");
        }

        self.method = method;
    }
}