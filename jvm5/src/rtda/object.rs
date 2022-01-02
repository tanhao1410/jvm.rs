///java 虚拟机 可以操作两种类型的数据，基本类型与引用类型
/// 该结构体表示对象
pub struct Object{
    pub(crate) val : i32
}


impl PartialEq for Object {
    // #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self as *const Self == other as *const Self
    }
}