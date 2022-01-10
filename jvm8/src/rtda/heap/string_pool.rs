use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use crate::rtda::heap::object::Object;
use crate::rtda::heap::class_loader::ClassLoader;
use crate::rtda::slot::Slot;
use crate::rtda::heap::array_datas::ArrayDatas;
use std::borrow::BorrowMut;


lazy_static! {
    static ref STRING_POOL: Mutex<HashMap<Arc<String>, Arc<RwLock<Object>>>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}
///公共的常量池
//pub const STRING_POOL:Arc<RwLock<StringPool>>  = Arc::new(RwLock::new(StringPool(HashMap::new())));


pub fn get_java_string(string: Arc<String>, loader: Arc<RwLock<ClassLoader>>) -> Arc<RwLock<Object>> {
    let mut guard_pool = STRING_POOL.lock().unwrap();
    let string_pool = guard_pool.borrow_mut();

    if let Some(java_string) = string_pool.get(&string) {
        java_string.clone()
    } else {
        //常量池中没有的话

        //把传递过来的字符串转化成字符数组，再创建一个java字符串实例，把java字符串实例的value设置为刚刚的数组


        // 得到 字节数组对象
        let chars = string.encode_utf16().collect::<Vec<_>>();
        let char_arr_class = ClassLoader::load_class(loader.clone(), Arc::new("[C".to_string()));
        let char_arr = Object::new_array(char_arr_class, chars.len());
        let mut char_arr_guard = char_arr.write().unwrap();
        char_arr_guard.datas = ArrayDatas::Chars(chars);


        let java_string_class = ClassLoader::load_class(loader, Arc::new("java/lang/String".to_string()));

        let java_str = Object::new(java_string_class.clone());

        let java_str_class_guard = java_string_class.read().unwrap();
        let field = java_str_class_guard.get_field(Arc::new("value".to_string()),
                                                   Arc::new("[C".to_string()));

        {
            let mut guard = java_str.write().unwrap();
            //let vec = &mut guard.refs_mut().slots;
            let datas = &mut guard.fields.slots;

            //从它的字段中找到 value,
            datas[field.read().unwrap().slot_id] = Slot::Ref(char_arr.clone());
        }

        //放入到容器中
        string_pool.insert(string, java_str.clone());

        java_str
    }
}



