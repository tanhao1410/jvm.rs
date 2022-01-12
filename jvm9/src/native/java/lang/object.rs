use crate::native::registry::register;
use std::sync::Arc;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use crate::rtda::frame::Frame;
use crate::rtda::slot::{Slot, Slots};
use crate::rtda::local_vars::LocalVars;
use std::ptr::hash;
use std::collections::hash_map::DefaultHasher;
use std::ptr;
use std::hash::Hasher;
use crate::rtda::heap::class_loader::ClassLoader;

pub fn init() {
    register("java/lang/Object",
             "getClass",
             "()Ljava/lang/Class;"
             , get_class);
    register("java/lang/Object",
             "hashCode",
             "()I",
             hash_code);

    register("java/lang/Object",
             "clone",
             "()Ljava/lang/Object;",
             clone);
}

fn clone(local_vars:&LocalVars) ->Option<Slots>{
    let this = local_vars.get_slot(0);
    if let Slot::Ref(obj) = this{

        //判断对象是否实现了java/lang/Cloneable接口
        let class_loader = ClassLoader::get_system_class_loader();
        let cloneable_class = ClassLoader::load_class(class_loader, Arc::new(
            "java/lang/Cloneable".to_string()));
        let obj = obj.read().unwrap();
        if !obj.is_instanceof(cloneable_class){
            panic!("java.lang.CloneNotSupportedException")
        }

        let cp_obj = obj.clone();
        return Some(Slots::from_one_slot(Slot::Ref(cp_obj)))
    }
    panic!("java.lang.NullPointerException")
}

fn hash_code(local_vars: &LocalVars) -> Option<Slots> {
    let this = local_vars.get_slot(0);
    match this {
        Slot::Ref(obj) => {
            let mut hasher = DefaultHasher::new();

            ptr::hash(obj, &mut hasher);

            let hash_code = hasher.finish();
            Some(Slots::from_one_slot(Slot::Num(hash_code as u32)))
        }
        _ => panic!("java.lang.NullPointerException")
    }
}

//对应public final native Class<?> getClass();
fn get_class(local_vars: &LocalVars) -> Option<Slots> {
    //从局部变量表中拿出this引用，放在了局部变量表的位置0
    let slot = local_vars.get_slot(0);
    match slot {
        Slot::Ref(this) => {
            let this_guard = this.read().unwrap();
            let class = &this_guard.class;

            //拿到类对象
            let class_guard = class.read().unwrap();
            let j_class = class_guard.j_class.as_ref();

            Some(Slots::from_one_slot(Slot::Ref(j_class.unwrap().clone())))
        }
        _ => { None }
    }
}