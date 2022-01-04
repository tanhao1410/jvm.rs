use std::sync::{Arc, RwLock};
use crate::classfile::class_file::ClassFile;
use crate::rtda::heap::method::Method;
use crate::rtda::heap::field::Field;
use crate::rtda::slot::{Slot, Slots};
use crate::rtda::heap::class_loader::ClassLoader;
use crate::rtda::heap::constant_pool::constant_pool::ConstantPool;
use crate::rtda::heap::access_flags::AccessFlags;
use crate::rtda::heap::object::Object;
use std::ops::Deref;

/// 要放进方法区中的类，
pub struct Class {
    pub access_flags: u16,
    pub name: Arc<String>,
    pub super_class_name: Arc<String>,
    pub interface_names: Vec<Arc<String>>,

    pub constant_pool: Option<Arc<RwLock<ConstantPool>>>, // 存在还没有的情况

    pub fields: Vec<Arc<RwLock<Field>>>,
    pub methods: Vec<Arc<Method>>,
    pub loader: Arc<RwLock<ClassLoader>>,
    pub super_class: Option<Arc<RwLock<Class>>>,
    //可能不存在父类
    pub interfaces: Vec<Arc<RwLock<Class>>>,
    pub instance_slot_count: u32,
    pub static_slot_count: u32,
    pub static_vars: Slots,
}

impl Class {
    /// 从classFile创建一个Class
    pub fn new(cf: &ClassFile, loader: Arc<RwLock<ClassLoader>>) -> Arc<RwLock<Self>> {
        let access_flags = cf.access_flags();
        let name = cf.class_name();
        let super_class_name = cf.super_class_name();
        let interface_names = cf.interface_names();

        let mut res = Arc::new(RwLock::new(Self {
            access_flags,
            name,
            super_class_name,
            interface_names,
            constant_pool: None,
            fields: vec![],
            methods: vec![],
            loader,
            super_class: None,
            interfaces: vec![],
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: Slots::new(0),
        }));

        let constant_pool = ConstantPool::new(res.clone(), cf.constant_pool().clone());
        let fields = Field::new_fields(res.clone(), cf.fields());
        let methods = Method::new_methods(res.clone(), cf.methods());


        {
            let mut guard = res.write().unwrap();
            guard.constant_pool = Some(constant_pool);
            guard.fields = fields;
            guard.methods = methods;
        }

        // let super_class= cf.super_class_name();
        // let interfaces= vec![];
        // let instance_slot_count= 0;
        // let static_slot_count= 0;
        // let static_vars = 0;

        res
    }

    pub fn new_object(class: Arc<RwLock<Class>>) -> Arc<RwLock<Object>> {
        Object::new(class)
    }

    pub fn class_loader(&self) -> Arc<RwLock<ClassLoader>> {
        self.loader.clone()
    }

    pub fn is_public(&self) -> bool {
        AccessFlags::is_public(self.access_flags)
    }

    pub fn class_name(&self) -> Arc<String> {
        self.name.clone()
    }

    pub fn is_interface(&self) -> bool {
        AccessFlags::is_interface(self.access_flags)
    }
    pub fn is_abstract(&self) -> bool {
        AccessFlags::is_abstract(self.access_flags)
    }


    pub fn is_accessible_to(&self, other: &Class) -> bool {
        self.is_public() || self.package_name().eq(other.package_name())
    }
    pub fn package_name(&self) -> &str {
        match self.name.rfind("/") {
            Some(i) => {
                self.name.get(..i).unwrap()
            }
            None => "",
        }
    }

    pub fn get_main_method(&self) -> Option<Arc<Method>> {
        self.get_static_method("main", "([Ljava/lang/String;)V")
    }

    pub fn get_static_method(&self, name: &str, descriptor: &str) -> Option<Arc<Method>> {
        for method in &self.methods {
            if method.is_static() && method.name.as_str() == name && method.descriptor.as_str() == descriptor {
                return Some(method.clone());
            }
        }
        None
    }

    /// 判断另一个类是不是自己的父类
    pub fn is_sub_class_of(&self, other_class: Arc<RwLock<Class>>) -> bool {
        match &self.super_class {
            None => false,//没有父类，直接返回false
            Some(parent_class) => {
                parent_class.read().unwrap().name.as_str() == other_class.read().unwrap().name.as_str()
                    || parent_class.read().unwrap().is_sub_class_of(other_class.clone())
            }
        }
    }

    /// 判断该类是否实现了某个接口
    pub fn is_implements(&self, iface:  Arc<RwLock<Class>>) -> bool {
        for interface in &self.interfaces {
            let inf = interface.read().unwrap();
            let interface = inf.deref();
            if interface.name.as_str().eq(iface.clone().read().unwrap().name.as_str())
                || interface.is_sub_insterface_of(iface.clone()) {
                return true;
            }
        }
        match &self.super_class {
            None => false,
            Some(c) => c.read().unwrap().is_implements(iface)
        }
    }
    pub fn is_sub_insterface_of(&self, iface: Arc<RwLock<Class>>) -> bool {
        for super_interface in &self.interfaces {
            let si = super_interface.read().unwrap();
            let super_interface = si.deref();
            if super_interface.name.as_str().eq(iface.clone().read().unwrap().name.as_str())
                || super_interface.is_sub_insterface_of(iface.clone()) {
                return true;
            }
        }
        return false;
    }
}