use crate::native::registry::register;
use crate::rtda::local_vars::LocalVars;
use crate::rtda::slot::{Slots, Slot};
use crate::rtda::heap::array_datas::ArrayDatas;
use crate::rtda::heap::string_pool::intern_string;

pub fn init() {
    register("java/lang/String",
             "intern",
             "()Ljava/lang/String;"
             , intern);
}

fn intern(local_vars: &LocalVars) -> Option<Slots> {
    let this = local_vars.get_slot(0);

    //如果字符串还没有入字符串常量池，则放入并返回该字符串，否则找到已放入池中的字符返回
    let arc = intern_string(this);
    Some(Slots::from_one_slot(Slot::Ref(arc)))
}