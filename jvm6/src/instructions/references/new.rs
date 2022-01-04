#[allow(non_camel_case_types)]
pub struct NEW {
    index: u16
}
impl NEW {
    pub fn new() -> Self {
        Self { index:0 }
    }
}

impl Instruction for NEW {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader){
        self.index = _reader.read_u16()
    }
    fn execute(&mut self, frame: &mut Frame) {
        let cp = frame.method.constant_pool();
        //得到类引用
        let mut guard = cp.write().unwrap();
        let mut class_ref = guard.get_constant_mut(self.index as usize).get_class_ref_mut();
        let class = class_ref.resolve_class();

        //问题？想解析类引用，就需要可变的借用常量池，而解析类引用过程中，又需要不可变借用常量池，产生了冲突，导致了死锁的发生
        //解决：在类引用里放置一个当前类的指针，不用通过常量池来获取

        if class.read().unwrap().is_abstract() || class.read().unwrap().is_interface(){
            panic!("java.lang.InstantiationError");
        }

        let obj = Class::new_object(class);
        frame.operand_stack().push_ref(obj);
    }
}

impl Debug for NEW {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(offset={})", self.index)
    }
}