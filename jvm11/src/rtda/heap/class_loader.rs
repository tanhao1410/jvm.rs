use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};

use crate::classfile::class_file::ClassFile;
use crate::classpath::classpath::ClassPath;
use crate::constants::java_primitive_classes::get_java_primitive_class_names;
use crate::rtda::heap::access_flags::AccessFlags::ACC_PUBLIC;
use crate::rtda::heap::class::Class;
use crate::rtda::heap::constant_pool::constant::Constant;
use crate::rtda::heap::field::Field;
use crate::rtda::heap::object::Object;
use crate::rtda::heap::string_pool::get_java_str_obj_by_pool;
use crate::rtda::slot::Slots;
use std::borrow::BorrowMut;
use crate::constants::java_class_name::{JAVA_CLASS_NAME_CLASS, JAVA_CLASS_NAME_OBJECT, JAVA_CLASS_NAME_CLONEABLE, JAVA_CLASS_NAME_SERIALIZABLE};


lazy_static! {
    static ref CLASS_LOADER_MAP: Mutex<HashMap<&'static str, Arc<RwLock<ClassLoader>> >> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

/// 类加载器
pub struct ClassLoader {
    cp: ClassPath,
    class_map: HashMap<String, Arc<RwLock<Class>>>,
}

impl ClassLoader {

    pub fn get_system_class_loader()->Arc<RwLock<Self>>{
        let  mut loader_map = CLASS_LOADER_MAP.lock().unwrap();
        let loader_map = loader_map.borrow_mut();
        loader_map.get("bootstrap").unwrap().clone()
    }

    pub fn new(cp: ClassPath) -> Arc<RwLock<Self>> {
        let loader = Arc::new(RwLock::new(Self {
            cp,
            class_map: HashMap::new(),
        }));

        {
            let mut loader_map = CLASS_LOADER_MAP.lock().unwrap();
            let loader_map = loader_map.borrow_mut();
            loader_map.insert("bootstrap", loader.clone());
        }

        ClassLoader::load_java_lang_class(&loader);
        ClassLoader::load_java_primitive_classes(&loader);
        loader
    }

    /// 加载基本类型
    fn load_java_primitive_classes(loader: &Arc<RwLock<ClassLoader>>) {

        //先找到java.lang.Class
        let jl_class_class = ClassLoader::load_class(loader.clone()
                                                     , JAVA_CLASS_NAME_CLASS);

        for class_name in get_java_primitive_class_names() {
            let jl_class_obj = Object::new(jl_class_class.clone());

            let class = Arc::new(RwLock::new(Class {
                access_flags: ACC_PUBLIC as u16,
                name: Arc::new(class_name.to_string()),
                super_class_name: Arc::new("".to_string()),
                interface_names: vec![],
                constant_pool: None,
                fields: vec![],
                methods: vec![],
                loader: loader.clone(),
                super_class: None,
                interfaces: vec![],
                instance_slot_count: 0,
                static_slot_count: 0,
                static_vars: Default::default(),
                init_started: true,
                j_class: None,
                source_file: Arc::new("".to_string())
            }));

            let jl_class_obj_cp = jl_class_obj.clone();
            let mut jl_class_obj_guard = jl_class_obj_cp.write().unwrap();
            jl_class_obj_guard.extra = Some(class.clone());

            let class2 = class.clone();
            let mut class_guard = class2.write().unwrap();
            class_guard.j_class = Some(jl_class_obj);

            //将该基本类加入到类map中
            let mut class_cloader_guard = loader.write().unwrap();
            let map = &mut class_cloader_guard.class_map;
            map.insert(class_name.to_string(), class);
        }

    }

    fn load_java_lang_class(loader: &Arc<RwLock<ClassLoader>>) {
        //需要先去加载基本的类，主要是java.lang.Class类，因为每一个加载的类，都要生成一个这样的对象
        let jl_class_class = Self::load_class(loader.clone(), JAVA_CLASS_NAME_CLASS);
        //在加载java.lang.Class之后，它还顺带加载了Object等，因此，先将这些之前加载的进行赋值
        let load_cp = loader.clone();
        let mut load_guard = load_cp.write().unwrap();
        let map = &mut load_guard.class_map;
        for (_, class) in map.iter_mut() {
            let mut class_guard = class.write().unwrap();
            if class_guard.j_class.is_none() {
                //创建一个java Class 对象
                let jl_class_obj = Object::new(jl_class_class.clone());
                //该对象的extra指向 对应的 类
                jl_class_obj.write().unwrap().extra = Some(class.clone());
                //给类对应上具体的
                class_guard.j_class = Some(jl_class_obj);
            }
        }
    }


    pub fn load_class<T:AsRef<str>>(loader: Arc<RwLock<Self>>, name: T) -> Arc<RwLock<Class>> {
        let loader_cp = loader.clone();
        if let Some(class) = loader_cp.read().unwrap().class_map.get(name.as_ref()) {
            return class.clone();
        }

        if name.as_ref().as_bytes()[0] == b'[' {
            let class = Self::load_array_class(loader.clone(), name);

            //获取到java.lang.Class 该类结构，然后给每一个新加载的类创建一个类对象
            let loader_cp = loader.clone();
            let loader_reader = loader_cp.read().unwrap();
            let map = &loader_reader.class_map;

            if let Some(jl_class_class) = map.get("java/lang/Class") {
                let jl_class_obj = Object::new(jl_class_class.clone());
                jl_class_obj.write().unwrap().extra = Some(class.clone());
                class.write().unwrap().j_class = Some(jl_class_obj)
            }
            return class;
        }

        let array_class = Self::load_non_array_class(loader.clone(), name);

        //获取到java.lang.Class 该类结构，然后给每一个新加载的类创建一个类对象
        let loader_cp = loader.clone();
        let loader_reader = loader_cp.read().unwrap();
        let map = &loader_reader.class_map;

        if let Some(jl_class_class) = map.get("java/lang/Class") {
            //每一个新加载的类，都要为其创建一个java.lang.Class类对象， 问题？ java.lang.Class自身加载的时候
            let jl_class_obj = Object::new(jl_class_class.clone());

            jl_class_obj.write().unwrap().extra = Some(array_class.clone());

            array_class.write().unwrap().j_class = Some(jl_class_obj)
        }
        array_class
    }

    /// 加载数组类，数组类不需要初始化
    fn load_array_class<T:AsRef<str>>(loader: Arc<RwLock<Self>>, name: T) -> Arc<RwLock<Class>> {
        let cloneable_name = Arc::new(JAVA_CLASS_NAME_CLONEABLE.to_string());
        let serializable_name = Arc::new(JAVA_CLASS_NAME_SERIALIZABLE.to_string());
        let object_name = Arc::new(JAVA_CLASS_NAME_OBJECT.to_string());
        let array_class = Arc::new(RwLock::new(Class {
            access_flags: ACC_PUBLIC as u16,
            name: Arc::new(name.as_ref().to_string()),
            super_class_name: object_name,
            interface_names: vec![cloneable_name, serializable_name],
            constant_pool: None,
            fields: vec![],
            methods: vec![],
            loader: loader.clone(),
            super_class: Some(ClassLoader::load_class(loader.clone(), JAVA_CLASS_NAME_OBJECT)),
            interfaces: vec![ClassLoader::load_class(loader.clone(), JAVA_CLASS_NAME_SERIALIZABLE),
                             ClassLoader::load_class(loader.clone(), JAVA_CLASS_NAME_CLONEABLE)],
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: Default::default(),
            init_started: true,
            j_class: None,
            source_file: Arc::new("".to_string())
        }));
        //加入map中
        loader.write().unwrap().class_map.insert(name.as_ref().to_string(), array_class.clone());
        array_class
    }

    fn load_non_array_class<T:AsRef<str>>(loader: Arc<RwLock<Self>>, name: T) -> Arc<RwLock<Class>> {
        let data = Self::read_class(loader.clone(), name);
        let class = Self::define_class(loader, data);
        Self::link(class.clone());
        class
    }

    fn read_class<T:AsRef<str>>(loader: Arc<RwLock<Self>>, name: T) -> Vec<u8> {
        match loader.read().unwrap().cp.read_class(name.as_ref().to_string()) {
            Some(res) => res,
            None => panic!("java.lang.ClassNotFoundException: {}", name.as_ref())
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
            class.write().unwrap().super_class = Some(Self::load_class(loader, super_class_name.as_str()))
        }
    }

    fn resolve_interfaces(loader: Arc<RwLock<Self>>, class: Arc<RwLock<Class>>) {
        let mut class_guard = class.write().unwrap();
        let interface_names = class_guard.interface_names.clone();
        let interfaces = interface_names.into_iter()
            .map(|name| {
                Self::load_class(loader.clone(), name.as_str())
            })
            .collect::<Vec<_>>();
        class_guard.interfaces = interfaces;
    }

    fn link(class: Arc<RwLock<Class>>) {
        Self::verify(class.clone());
        Self::prepare(class);
    }

    fn verify(_class: Arc<RwLock<Class>>) {
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
        let mut class_guard = class.write().unwrap();

        //要从他的父类开编起
        if let Some(super_class) = &class_guard.super_class{
            instance_count += super_class.read().unwrap().instance_slot_count;
        }

        for field in class_guard.fields.iter_mut() {
            let mut filed_guard = field.write().unwrap();
            if filed_guard.is_static() {
                filed_guard.slot_id = static_count;
                static_count += 1;
                if filed_guard.is_long_or_double() {
                    static_count += 1;
                }
            } else {
                filed_guard.slot_id = instance_count as usize;
                instance_count += 1;
                if filed_guard.is_long_or_double() {
                    instance_count += 1;
                }
            }
        }
        class_guard.static_slot_count = static_count as u32;
        class_guard.instance_slot_count = instance_count  as u32;
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
                            let java_str = get_java_str_obj_by_pool(Arc::new(val.clone()));
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