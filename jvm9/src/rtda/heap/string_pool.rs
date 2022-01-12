use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use crate::rtda::heap::object::Object;
use crate::rtda::heap::class_loader::ClassLoader;
use crate::rtda::slot::Slot;
use crate::rtda::heap::array_datas::ArrayDatas;
use std::borrow::BorrowMut;
use crate::rtda::frame::Frame;
use crate::utils::string_utils::{get_java_string, get_string_from_slot};


lazy_static! {
    static ref STRING_POOL: Mutex<HashMap<Arc<String>, Arc<RwLock<Object>>>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}

pub fn get_java_str_obj_by_pool(string: Arc<String>, loader: Arc<RwLock<ClassLoader>>) -> Arc<RwLock<Object>> {

    let mut guard_pool = STRING_POOL.lock().unwrap();
    let string_pool = guard_pool.borrow_mut();

    if let Some(java_string) = string_pool.get(&string) {
        java_string.clone()
    } else {
        let java_str = get_java_string( loader,string.clone());
        //放入到容器中
        string_pool.insert(string, java_str.clone());
        java_str
    }
}

pub fn intern_string(string : &Slot)-> Arc<RwLock<Object>>{
    let mut guard_pool = STRING_POOL.lock().unwrap();
    let string_pool = guard_pool.borrow_mut();

    //把java 字符串变成rust的字符串
    let string1 = Arc::new(get_string_from_slot(ClassLoader::get_system_class_loader(), string));

    if let Some(java_string) = string_pool.get(&string1){

        java_string.clone()
    }else{
        let java_str = get_java_string( ClassLoader::get_system_class_loader(),string1.clone());
        //放入到容器中
        string_pool.insert(string1, java_str.clone());
        java_str
    }
}




