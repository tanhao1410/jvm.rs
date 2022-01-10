use crate::classpath::classpath::ClassPath;
use std::collections::HashMap;
use crate::rtda::heap::class::Class;
use std::sync::{RwLock, Arc};
use crate::classfile::class_file::ClassFile;
use crate::rtda::heap::field::Field;
use crate::rtda::heap::constant_pool::constant::Constant;
use crate::rtda::slot::{Slots};
use crate::rtda::heap::access_flags::AccessFlags::ACC_PUBLIC;
use crate::rtda::heap::string_pool::get_java_string;


/// 类加载器
pub struct ClassLoader {
    cp: ClassPath,
    class_map: HashMap<String, Arc<RwLock<Class>>>,
}

impl ClassLoader {
    pub fn new(cp: ClassPath) -> Self {
        Self {
            cp,
            class_map: HashMap::new(),
        }
    }

    pub fn load_class(loader: Arc<RwLock<Self>>, name: Arc<String>) -> Arc<RwLock<Class>> {
        if let Some(class) = loader.read().unwrap().class_map.get(name.as_ref()) {
            return class.clone();
        }
        if name.as_bytes()[0] == b'[' {
            return Self::load_array_class(loader, name);
        }
        Self::load_non_array_class(loader, name)
    }

    /// 加载数组类，数组类不需要初始化
    fn load_array_class(loader: Arc<RwLock<Self>>, name: Arc<String>) -> Arc<RwLock<Class>> {
        let cloneable_name = Arc::new("java/lang/Cloneable".to_string());
        let serializable_name = Arc::new("java/io/Serializable".to_string());
        let object_name = Arc::new("java/lang/Object".to_string());
        let mut array_class = Arc::new(RwLock::new(Class {
            access_flags: ACC_PUBLIC as u16,
            name: name.clone(),
            super_class_name: object_name.clone(),
            interface_names: vec![cloneable_name.clone(), serializable_name.clone()],
            constant_pool: None,
            fields: vec![],
            methods: vec![],
            loader: loader.clone(),
            super_class: Some(ClassLoader::load_class(loader.clone(), object_name)),
            interfaces: vec![ClassLoader::load_class(loader.clone(), serializable_name),
                             ClassLoader::load_class(loader.clone(), cloneable_name)],
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: Default::default(),
            init_started: true,
        }));
        //加入map中
        loader.write().unwrap().class_map.insert(name.to_string(), array_class.clone());
        array_class
    }

    fn load_non_array_class(loader: Arc<RwLock<Self>>, name: Arc<String>) -> Arc<RwLock<Class>> {
        let data = Self::read_class(loader.clone(), name);
        let class = Self::define_class(loader, data);
        Self::link(class.clone());
        class
    }

    fn read_class(loader: Arc<RwLock<Self>>, name: Arc<String>) -> Vec<u8> {
        match loader.read().unwrap().cp.read_class(name.to_string()) {
            Some(res) => res,
            None => panic!("java.lang.ClassNotFoundException: {}", name)
        }
    }

    fn define_class(loader: Arc<RwLock<Self>>, data: Vec<u8>) -> Arc<RwLock<Class>> {
        let class = Self::parse_class(loader.clone(), data);

        //  解析类的父类与接口
        Self::resolve_super_class(loader.clone(), class.clone());
        Self::resolve_interfaces(loader.clone(), class.clone());

        let class_name = class.read().unwrap().class_name();
        loader.write().unwrap().class_map.insert(class_name.to_string(), class.clone());
        class
    }

    fn parse_class(loader: Arc<RwLock<Self>>, data: Vec<u8>) -> Arc<RwLock<Class>> {
        //根据字节码文件生成ClassFile结构体
        let cf = ClassFile::parse(data);
        Class::new(&cf, loader)
    }

    fn resolve_super_class(loader: Arc<RwLock<Self>>, class: Arc<RwLock<Class>>) {
        if class.read().unwrap().name.to_string().ne("java/lang/Object") {
            let super_class_name = class.read().unwrap().super_class_name.clone();
            //加载父类：
            class.write().unwrap().super_class = Some(Self::load_class(loader, super_class_name))
        }
    }

    fn resolve_interfaces(loader: Arc<RwLock<Self>>, class: Arc<RwLock<Class>>) {
        let interface_names = class.read().unwrap().interface_names.clone();
        let interfaces = interface_names.into_iter()
            .map(|name| {
                Self::load_class(loader.clone(), name)
            })
            .collect::<Vec<_>>();
        class.write().unwrap().interfaces = interfaces;
    }

    fn link(class: Arc<RwLock<Class>>) {
        Self::verify(class.clone());
        Self::prepare(class);
    }

    fn verify(class: Arc<RwLock<Class>>) {
        // todo
    }

    fn prepare(class: Arc<RwLock<Class>>) {
        // 计算实例字段与静态字段的个数，并为静态字段分配空间
        Self::calc_field_slots(class.clone());

        Self::alloc_and_init_static_vars(class);
    }

    fn calc_field_slots(class: Arc<RwLock<Class>>) {
        let mut static_count = 0;
        let mut instance_count = 0;

        //let vec = &class.read().unwrap().fields;
        for field in class.read().unwrap().fields.iter() {
            if field.read().unwrap().is_static() {
                static_count += 1;
                if field.read().unwrap().is_long_or_double() {
                    static_count += 1;
                }
            } else {
                instance_count += 1;
                if field.read().unwrap().is_long_or_double() {
                    instance_count += 1;
                }
            }
        }
        class.write().unwrap().static_slot_count = static_count;
        class.write().unwrap().instance_slot_count = instance_count;
    }

    fn alloc_and_init_static_vars(class: Arc<RwLock<Class>>) {
        let var_len = class.read().unwrap().static_slot_count as usize;

        let mut vars = Slots::new(var_len);


        //又读class,又写class 冲突了

        for field in &class.read().unwrap().fields {
            let guard = field.read().unwrap();
            if guard.is_final() && guard.is_static() {
                Self::init_static_final_var(&mut vars, class.clone(), field.clone());
            }
        }

        class.write().unwrap().static_vars = vars;
    }

    /// 给final静态变量（基本类型，String)特殊赋值 ，他们的值在编译期间已知，该值存储在class文件常量池中
    fn init_static_final_var(vars: &mut Slots, class: Arc<RwLock<Class>>, field: Arc<RwLock<Field>>) {
        let constant_pool = class.read().unwrap().constant_pool.clone().unwrap();
        let cp_index = field.read().unwrap().const_value_index;
        let slot_id = field.read().unwrap().slot_id;
        if cp_index > 0 {
            match field.read().unwrap().descriptor.clone().as_str() {
                "Z" | "B" | "C" | "S" | "I" => {
                    match constant_pool.read().unwrap().get_constant(cp_index) {
                        Constant::Integer(val) => vars.set_int(slot_id, *val),
                        _ => panic!("impossible.")
                    }
                }
                "J" => {
                    match constant_pool.read().unwrap().get_constant(cp_index) {
                        Constant::Long(val) => vars.set_long(slot_id, *val),
                        _ => panic!("impossible.")
                    }
                }
                "F" => {
                    match constant_pool.read().unwrap().get_constant(cp_index) {
                        Constant::Float(val) => vars.set_float(slot_id, *val),
                        _ => panic!("impossible.")
                    }
                }
                "D" => {
                    match constant_pool.read().unwrap().get_constant(cp_index) {
                        Constant::Double(val) => vars.set_double(slot_id, *val),
                        _ => panic!("impossible.")
                    }
                }
                //处理字符串类型静态常量的初始化逻辑
                "Ljava/lang/String;" => {
                    match constant_pool.read().unwrap().get_constant(cp_index) {
                        Constant::String(val) => {
                            //怎么拿到常量池呢
                            let java_str = get_java_string(
                                Arc::new(val.clone()),
                                class.read().unwrap().loader.clone());
                            vars.set_ref(slot_id, java_str);
                        }
                        _ => panic!("impossible.")
                    }
                }
                _ => {
                    panic!("impossible.")
                }
            }
        }
    }
}