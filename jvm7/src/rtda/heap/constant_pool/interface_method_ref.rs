use crate::rtda::heap::class::Class;
use std::sync::{RwLock, Arc};
use crate::rtda::heap::constant_pool::constant_pool::ConstantPool;
use crate::rtda::heap::method::Method;
use crate::classfile::constant_info::constant_info_interface_methodref::ConstantInterfaceMethodrefInfo;
use crate::rtda::heap::class_loader::ClassLoader;

pub struct InterfaceMethodRef{

    cp: Arc<RwLock<ConstantPool>>,
    class_name: Arc<String>,
    class: Option<Arc<RwLock<Class>>>,

    //当前的类
    pub cur_class:Arc<RwLock<Class>>,

    //相比类符号引用多了的
    name: Arc<String>,
    descriptor: Arc<String>,
    method:Option<Arc<Method>>
}

impl InterfaceMethodRef{
    pub fn new(cp: Arc<RwLock<ConstantPool>>,info: &ConstantInterfaceMethodrefInfo)->Self{
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

    pub fn resolve_interface_method(&mut self)->Arc<Method>{
        if self.method.is_none(){
            self.resolve_iterface_method_ref();
        }
        self.method.as_ref().unwrap().clone()
    }

    fn resolve_iterface_method_ref(&mut self){
        let cur_class = self.cur_class.clone();

        let ref_class = ClassLoader::load_class(cur_class.read().unwrap().loader.clone(), self.class_name.clone());


        if !ref_class.read().unwrap().is_interface(){
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