use std::sync::{Arc, RwLockWriteGuard};

use crate::native::registry::register;
use crate::rtda::heap::class_loader::ClassLoader;
use crate::rtda::heap::string_pool::get_java_str_obj_by_pool;
use crate::rtda::slot::{Slot, Slots};
use crate::utils::string_utils::get_string_from_slot;
use crate::rtda::local_vars::LocalVars;
use crate::rtda::thread::Thread;

pub fn init() {
    register("java/lang/Class",
             "getPrimitiveClass",
             "(Ljava/lang/String;)Ljava/lang/Class;"
             , get_primitive_class);

    register("java/lang/Class",
             "getName0",
             "()Ljava/lang/String;"
             , get_name0);

    register("java/lang/Class",
             "desiredAssertionStatus0",
             "(Ljava/lang/Class;)Z"
             , desired_assertion_status0);
}

//static native Class<?> getPrimitiveClass(String name);
fn get_primitive_class(local_vars: &LocalVars,_:RwLockWriteGuard<Thread>) -> Option<Slots> {
    //该方法时个静态方法，
    let slot = local_vars.get_slot(0);

    //调用类加载器加载该类

    let loader = ClassLoader::get_system_class_loader();

    let class_name = get_string_from_slot( slot);

    let class = ClassLoader::load_class(loader.clone(), Arc::new(class_name));
    let class_guard = class.read().unwrap();
    let j_class_obj = class_guard.j_class.as_ref().unwrap();
    Some(Slots::from_one_slot(Slot::Ref(j_class_obj.clone())))
}

//private native String getName0();
fn get_name0(local_vars: &LocalVars,_:RwLockWriteGuard<Thread>) -> Option<Slots> {

    let slot = local_vars.get_slot(0);
    match slot {
        Slot::Ref(this) => {
            let read_guard = this.read().unwrap();
            if let Some(class) = &read_guard.extra {
                let read_guard = class.read().unwrap();
                let class_name = read_guard.name.clone();
                let class_name = Arc::new(class_name.to_string().replace("/", "."));
                //转变成java string对象，放到栈顶
                let j_str_obj = get_java_str_obj_by_pool(class_name);

                return Some(Slots::from_one_slot(Slot::Ref(j_str_obj)));
            }
            None
        }
        _ => unreachable!()
    }
}

//private static native boolean desiredAssertionStatus0(Class<?> clazz);
fn desired_assertion_status0(_: &LocalVars,_:RwLockWriteGuard<Thread>) -> Option<Slots> {
    Some(Slots::from_one_slot(Slot::Num(0)))
}

