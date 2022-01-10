use std::cell::RefCell;
use std::ops::{Deref, Add};
use std::rc::Rc;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

use crate::classfile::class_file::ClassFile;
use crate::rtda::frame::Frame;
use crate::rtda::heap::access_flags::AccessFlags;
use crate::rtda::heap::class_loader::ClassLoader;
use crate::rtda::heap::constant_pool::constant_pool::ConstantPool;
use crate::rtda::heap::field::Field;
use crate::rtda::heap::method::Method;
use crate::rtda::heap::object::Object;
use crate::rtda::slot::Slots;
use crate::rtda::thread::Thread;

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

    //判断该类是否已经初始化
    pub init_started: bool,

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
            init_started: false,
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
        res
    }

    pub fn new_object(class: Arc<RwLock<Class>>) -> Arc<RwLock<Object>> {
        Object::new(class)
    }

    pub fn is_array_class(&self) -> bool {
        self.name.as_bytes()[0] == b'['
    }

    /// 得到以该类为元素的数组类
    pub fn array_class(&self) -> Arc<RwLock<Class>> {
        ClassLoader::load_class(self.loader.clone(), self.array_class_name())
    }

    fn array_class_name(&self) -> Arc<String> {
        //如果本类是数组类，则它的数组类 是在原来的基础上再加一个[
        if self.name.starts_with('[') {
            return Arc::new("[".to_string().add(self.name.as_str()));
        }
        //基本类型，或引用类型
        let name = match self.name.as_str() {
            "void" => "V".to_string(),
            "boolean" => "Z".to_string(),
            "byte" => "B".to_string(),
            "shot" => "S".to_string(),
            "int" => "I".to_string(),
            "long" => "J".to_string(),
            "char" => "C".to_string(),
            "float" => "F".to_string(),
            "double" => "D".to_string(),
            name => {
                "L".to_string().add(name).add(";")
            }
        };
        Arc::new("[".to_string().add(name.as_str()))
    }

    /// 数组类创建数组对象
    pub fn new_array(class: Arc<RwLock<Class>>, len: usize) -> Arc<RwLock<Object>> {
        Object::new_array(class, len)
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

    pub fn get_field(&self, name: Arc<String>, desc: Arc<String>) -> Arc<RwLock<Field>> {
        for field in &self.fields {
            let guard = field.read().unwrap();
            if guard.name == name
                && guard.descriptor == desc {
                return field.clone();
            }
        }
        panic!("no field:{}", name)
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
    pub fn is_implements(&self, iface: Arc<RwLock<Class>>) -> bool {
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

    pub fn lookup_method(&self, name: &str, descriptor: &str) -> Option<Arc<Method>> {
        //先在自己及父类中查找，然后再去各接口中去找
        for method in &self.methods {
            if method.name.as_str() == name && method.descriptor.as_str() == descriptor {
                return Some(method.clone());
            }
        }

        if self.super_class.is_some() {
            let res = Self::lookup_method_in_class(self.super_class.as_ref().unwrap().clone(), name, descriptor);
            if res.is_some() {
                return res;
            }
        }

        //在接口中寻找
        Self::lookup_method_in_interface(&self.interfaces, name, descriptor)
    }

    fn lookup_method_in_class(class: Arc<RwLock<Class>>, name: &str, descriptor: &str) -> Option<Arc<Method>> {
        for method in &class.read().unwrap().methods {
            if method.name.as_str() == name && method.descriptor.as_str() == descriptor {
                return Some(method.clone());
            }
        }
        match &class.read().unwrap().super_class {
            Some(c) => Self::lookup_method_in_class(c.clone(), name, descriptor),
            None => None,
        }
    }

    fn lookup_method_in_interface(ifaces: &Vec<Arc<RwLock<Class>>>, name: &str, descriptor: &str) -> Option<Arc<Method>> {
        for interface in ifaces.iter() {
            for method in interface.read().unwrap().methods.iter() {
                if method.name.as_str().eq(name)
                    && method.descriptor.as_str().eq(descriptor) {
                    return Some(method.clone());
                }
            }
            let res = Self::lookup_method_in_interface(&interface.read().unwrap().interfaces, name, descriptor);
            if res.is_some() {
                return res;
            }
        }
        None
    }

    pub fn init_class(&mut self, mut thread: Arc<RwLock<Thread>>, mut guard: RwLockWriteGuard<Thread>) {
        //先把自己的初始化状态置位true
        self.init_started = true;
        //准备执行，把自己的初始化方法推入到虚拟机栈中
        let clinit = self.get_static_method("<clinit>", "()V");
        if let Some(clinit) = clinit {
            //创建一个帧,推入栈顶
            let new_frame = Frame::new(clinit.max_locals, clinit.max_stack, thread.clone(), clinit);
            guard.push_frame(Rc::new(RefCell::new(new_frame)));
        }
        //如果本类是类，执行父类的
        if !self.is_interface() {
            if let Some(super_class) = self.super_class.as_ref() {
                if !super_class.read().unwrap().init_started {
                    super_class.write().unwrap().init_class(thread, guard);
                }
            }
        }
    }
}