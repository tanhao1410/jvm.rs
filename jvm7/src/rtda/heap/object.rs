use std::sync::{RwLock, Arc};
use crate::rtda::heap::class::Class;
use crate::rtda::slot::Slots;

///java 虚拟机 可以操作两种类型的数据，基本类型与引用类型
/// 该结构体表示对象
pub struct Object{
    pub class : Arc<RwLock<Class>>,
    pub fields: Slots
}

impl Object{

    /// 生成一个对象
    pub fn new(class: Arc<RwLock<Class>>) -> Arc<RwLock<Object>> {
        let fields = Slots::new(class.read().unwrap().instance_slot_count as usize);
        Arc::new(RwLock::new(Self { class: class, fields }))
    }

    /// 判断一个对象是不是某个类或接口的实例
    pub fn is_instanceof(&self,class: Arc<RwLock<Class>>)->bool{
        //看该对象的类是否是指定的class，如果不是，则看它的类的父类，它的类的接口
        if self.class.clone().read().unwrap().name.as_str() == class.read().unwrap().name.as_str(){
            return true;
        }
        //判断传入的类是否是该类的父类
        if !class.read().unwrap().is_interface(){
            return self.class.read().unwrap().is_sub_class_of(class);
        }else{
            //传入的是接口，判断对象的类是否实现了该接口
            return self.class.read().unwrap().is_implements(class);
        }

    }

}


impl PartialEq for Object {
    // #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self as *const Self == other as *const Self
    }
}