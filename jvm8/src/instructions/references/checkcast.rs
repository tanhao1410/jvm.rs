#[allow(non_camel_case_types)]
/// 与instanceof 很像，但它不用更改操作数栈，如果类型不匹配，直接报错
/// null 可以是任何对象的实例
pub struct CHECK_CAST {
    index: u16
}

impl CHECK_CAST {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for CHECK_CAST {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16()
    }
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
        let mut frame = rc.borrow_mut();
        let cp = frame.method.constant_pool();
        //得到类引用
        let mut cp_guard = cp.write().unwrap();
        let mut class_ref = cp_guard.get_constant_mut(self.index as usize).get_class_ref_mut();

        let class = class_ref.resolve_class();
        //从栈顶弹出一个引用，查看该引用是否是该类或接口的一个实例
        let slot = frame.operand_stack().pop_slot();
        //checkcast不能改变操作数栈
        match slot {
            Slot::Ref(obj_ref) => {
                if !obj_ref.read().unwrap().is_instanceof(class) {
                    panic!("java.lang.ClassCastException")
                }
                frame.operand_stack().push_slot(Slot::Ref(obj_ref));
            }
            //nil的情况下，结果为0
            Slot::Nil() => frame.operand_stack().push_slot(slot),
            Slot::Num(num) => frame.operand_stack().push_slot(slot)
        }
    }
}

impl Debug for CHECK_CAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(CHECK_CAST=)")
    }
}