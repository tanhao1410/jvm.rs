use std::sync::{RwLock, Arc};
use crate::rtda::heap::class::Class;
use crate::rtda::slot::Slots;
use crate::rtda::heap::array_datas::ArrayDatas;

///java 虚拟机 可以操作两种类型的数据，基本类型与引用类型
/// 该结构体表示对象
pub struct Object {
    pub class: Arc<RwLock<Class>>,
    pub fields: Slots,

    //如果是数组对象的话，fields 里面就是一个个数组了。
    //slots本身便是数组，怎么存 各种类型呢 slot本身可以存一个数或一个引用，但是对于 double与float来说，其本身
    //需要两个槽来存，因此，在设计上，依旧采用槽的形式，只是，部分类型用两个槽来存，注意数组大小与fields中槽多少的区别

    //采用额外的一个字段来存吧！
    pub datas: ArrayDatas,

    // 对于java.lang.Class派生出的Obejct,里面需要有一个额外字段来表示，该object 是哪一个类的
    pub extra: Option<Arc<RwLock<Class>>>,
}

impl Object {
    /// 生成一个对象
    pub fn new(class: Arc<RwLock<Class>>) -> Arc<RwLock<Object>> {
        let fields = Slots::new(class.read().unwrap().instance_slot_count as usize);
        Arc::new(RwLock::new(Self { class: class, fields, datas: Default::default(), extra: None }))
    }

    /// 创建数组，数组的元素是class 类
    pub fn new_array(class: Arc<RwLock<Class>>, len: usize) -> Arc<RwLock<Object>> {
        //如果类是数组类
        let cloned = class.clone();
        let guard = cloned.read().unwrap();
        let class_name = guard.name.as_bytes();
        // [z
        let array = match class_name[1] {
            b'Z' | b'B' => Self { class: class, fields: Default::default(), datas: ArrayDatas::Bytes(vec![0; len]), extra: None },
            b'C' => Self { class: class, fields: Default::default(), datas: ArrayDatas::Chars(vec![0; len]), extra: None },
            b'S' => Self { class: class, fields: Default::default(), datas: ArrayDatas::Shorts(vec![0; len]), extra: None },
            b'I' => Self { class: class, fields: Default::default(), datas: ArrayDatas::Ints(vec![0; len]), extra: None },
            b'J' => Self { class: class, fields: Default::default(), datas: ArrayDatas::Longs(vec![0; len]), extra: None },
            b'F' => Self { class: class, fields: Default::default(), datas: ArrayDatas::Floats(vec![0f32; len]), extra: None },
            b'D' => Self { class: class, fields: Default::default(), datas: ArrayDatas::Doubles(vec![0f64; len]), extra: None },
            _ => Self { class: class, fields: Default::default(), datas: ArrayDatas::Refs(Slots::new(len)), extra: None },
        };
        Arc::new(RwLock::new(array))
    }

    pub fn clone(&self) -> Arc<RwLock<Object>> {
        //复制它所有的东西
        let datas = self.datas.clone();
        let fields = self.fields.clone();
        let extra = match &self.extra {
            Some(obj) => Some(obj.clone()),
            None => None
        };
        let class = self.class.clone();

        Arc::new(RwLock::new(Self {
            class,
            fields,
            datas,
            extra,
        }))
    }

    /// 判断一个对象是不是某个类或接口的实例
    pub fn is_instanceof(&self, class: Arc<RwLock<Class>>) -> bool {
        //看该对象的类是否是指定的class，如果不是，则看它的类的父类，它的类的接口
        if self.class.clone().read().unwrap().name.as_str() == class.read().unwrap().name.as_str() {
            return true;
        }
        //判断传入的类是否是该类的父类
        if !class.read().unwrap().is_interface() {
            return self.class.read().unwrap().is_sub_class_of(class);
        } else {
            //传入的是接口，判断对象的类是否实现了该接口
            return self.class.read().unwrap().is_implements(class);
        }
    }

    /// 添加数组特有的方法，返回内容的引用
    pub fn bytes(&self) -> &Vec<i8> {
        match &self.datas {
            ArrayDatas::Bytes(datas) => datas,
            _ => panic!("object is not a i8 array")
        }
    }
    pub fn bytes_mut(&mut self) -> &mut Vec<i8> {
        match &mut self.datas {
            ArrayDatas::Bytes(datas) => datas,
            _ => panic!("object is not a i8 array")
        }
    }
    pub fn shorts(&self) -> &Vec<i16> {
        match &self.datas {
            ArrayDatas::Shorts(datas) => datas,
            _ => panic!("object is not a i16 array")
        }
    }
    pub fn shorts_mut(&mut self) -> &mut Vec<i16> {
        match &mut self.datas {
            ArrayDatas::Shorts(datas) => datas,
            _ => panic!("object is not a i16 array")
        }
    }
    pub fn ints(&self) -> &Vec<i32> {
        match &self.datas {
            ArrayDatas::Ints(datas) => datas,
            _ => panic!("object is not a i32 array")
        }
    }
    pub fn ints_mut(&mut self) -> &mut Vec<i32> {
        match &mut self.datas {
            ArrayDatas::Ints(datas) => datas,
            _ => panic!("object is not a i32 array")
        }
    }
    pub fn longs(&self) -> &Vec<i64> {
        match &self.datas {
            ArrayDatas::Longs(datas) => datas,
            _ => panic!("object is not a i64 array")
        }
    }
    pub fn longs_mut(&mut self) -> &mut Vec<i64> {
        match &mut self.datas {
            ArrayDatas::Longs(datas) => datas,
            _ => panic!("object is not a i64 array")
        }
    }
    pub fn chars(&self) -> &Vec<u16> {
        match &self.datas {
            ArrayDatas::Chars(datas) => datas,
            _ => panic!("object is not a u16 array")
        }
    }
    pub fn chars_mut(&mut self) -> &mut Vec<u16> {
        match &mut self.datas {
            ArrayDatas::Chars(datas) => datas,
            _ => panic!("object is not a u16 array")
        }
    }
    pub fn floats(&self) -> &Vec<f32> {
        match &self.datas {
            ArrayDatas::Floats(datas) => datas,
            _ => panic!("object is not a f32 array")
        }
    }
    pub fn floats_mut(&mut self) -> &mut Vec<f32> {
        match &mut self.datas {
            ArrayDatas::Floats(datas) => datas,
            _ => panic!("object is not a f32 array")
        }
    }
    pub fn doubles(&self) -> &Vec<f64> {
        match &self.datas {
            ArrayDatas::Doubles(datas) => datas,
            _ => panic!("object is not a f64 array")
        }
    }
    pub fn doubles_mut(&mut self) -> &mut Vec<f64> {
        match &mut self.datas {
            ArrayDatas::Doubles(datas) => datas,
            _ => panic!("object is not a f64 array")
        }
    }
    pub fn refs(&self) -> &Slots {
        match &self.datas {
            ArrayDatas::Refs(datas) => datas,
            _ => panic!("object is not a refs array")
        }
    }

    pub fn refs_mut(&mut self) -> &mut Slots {
        match &mut self.datas {
            ArrayDatas::Refs(datas) => datas,
            _ => panic!("object is not a refs array")
        }
    }

    /// 获取数组对象的长度
    pub fn array_length(&self) -> usize {
        match &self.datas {
            ArrayDatas::Empty => {
                panic!("not a array!")
            }
            ArrayDatas::Bytes(datas) => datas.len(),
            ArrayDatas::Shorts(datas) => datas.len(),
            ArrayDatas::Ints(datas) => datas.len(),
            ArrayDatas::Longs(datas) => datas.len(),
            ArrayDatas::Chars(datas) => datas.len(),
            ArrayDatas::Floats(datas) => datas.len(),
            ArrayDatas::Doubles(datas) => datas.len(),
            ArrayDatas::Refs(datas) => datas.slots.len(),
        }
    }
}


impl PartialEq for Object {
    // #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self as *const Self == other as *const Self
    }
}