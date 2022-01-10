#[allow(non_camel_case_types)]
pub struct ANEW_ARRAY {
    index: u16
}


impl ANEW_ARRAY {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for ANEW_ARRAY {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16()
    }
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let arc = thread.clone();
        let guard = arc.write().unwrap();
        let rc = guard.current_frame();
        let mut frame = rc.borrow_mut();
        let cp = frame.method.constant_pool();
        //得到类符号引用，为该数组元素类
        let mut cp_guard = cp.write().unwrap();
        let mut class_ref = cp_guard.get_constant_mut(self.index as usize).get_class_ref_mut();

        let class = class_ref.resolve_class();

        let array_class = class.read().unwrap().array_class();

        let operand_stack = frame.operand_stack();
        let count = operand_stack.pop_int();
        if count < 0 {
            panic!("java.lang.NegativeArraySizeException");
        }

        let arr_obj = Class::new_array(array_class, count as usize);
        frame.operand_stack().push_ref(arr_obj);
    }
}

impl Debug for ANEW_ARRAY {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(offset={})", self.index)
    }
}