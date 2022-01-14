use crate::rtda::slot::Slot;
use std::sync::{Arc, RwLock};
use crate::rtda::heap::object::Object;
use crate::rtda::heap::class_loader::ClassLoader;
use crate::rtda::heap::array_datas::ArrayDatas;

/// 从java string 对象的slot中生成 rust 中的string
pub fn get_string_from_slot(slot: &Slot) -> String {
    match slot {
        Slot::Ref(j_str_obj) => {
            let loader = ClassLoader::get_system_class_loader();
            //这是java的string 对象
            let j_str_obj = j_str_obj.read().unwrap();

            //从j_str_obj中找到属性为value的字符 数组

            //需要用到classloader
            let java_string_class = ClassLoader::load_class(loader, Arc::new("java/lang/String".to_string()));
            let java_str_class_guard = java_string_class.read().unwrap();
            let field = java_str_class_guard.get_field("value", "[C");
            //这是value 属性所在的槽号
            let field_id = field.read().unwrap().slot_id;

            // java string对象所有的槽
            let datas = &j_str_obj.fields.slots;

            // java string 对象中的 value 字段的 值 ，类型为char 数组
            if let Slot::Ref(char_array) = &datas[field_id] {
                //从char_array中获取到字符数组
                let char_array = char_array.read().unwrap();
                let chars = char_array.chars().clone();
                let res = String::from_utf16_lossy(&chars);
                return res;
            }
            unreachable!()
        }
        Slot::Nil() => "".to_string(),
        _ => unreachable!()
    }
}

/// 从rust string 得到java 的String 对象
pub fn get_java_string<T: AsRef<str>>(r_str: T) -> Arc<RwLock<Object>> {
    let loader = ClassLoader::get_system_class_loader();
    let r_str = r_str.as_ref();
    let chars = r_str.encode_utf16().collect::<Vec<_>>();
    let char_arr_class = ClassLoader::load_class(loader.clone(), Arc::new("[C".to_string()));
    let char_arr = Object::new_array(char_arr_class, chars.len());
    let mut char_arr_guard = char_arr.write().unwrap();
    char_arr_guard.datas = ArrayDatas::Chars(chars);

    let java_string_class = ClassLoader::load_class(loader, Arc::new("java/lang/String".to_string()));

    let java_str = Object::new(java_string_class.clone());

    let java_str_class_guard = java_string_class.read().unwrap();
    let field = java_str_class_guard.get_field("value", "[C");
    {
        let mut guard = java_str.write().unwrap();
        //let vec = &mut guard.refs_mut().slots;
        let datas = &mut guard.fields.slots;

        //从它的字段中找到 value,
        datas[field.read().unwrap().slot_id] = Slot::Ref(char_arr.clone());
    }
    java_str
}