#[allow(non_camel_case_types)]
pub struct PUT_STATIC {
    index: u16
}

impl PUT_STATIC {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for PUT_STATIC {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16()
    }
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let arc = thread.clone();
        let guard = arc.write().unwrap();
        let rc = guard.current_frame();
        let mut frame = rc.borrow_mut();
        let cp = frame.method.constant_pool();
        let current_class = cp.read().unwrap().get_class();
        //得到字段符号引用
        let mut cp_guard = cp.write().unwrap();
        let mut field_ref = cp_guard.get_constant_mut(self.index as usize).get_field_ref_mut();

        let field = field_ref.resolve_field();
        let ref_class = field.read().unwrap().class.clone();

        //在解析完class后，判断类是否已经初始化，如果还未初始化，初始化类
        if !ref_class.read().unwrap().init_started{
            // frame.revert_next_pc();
            frame.set_next_pc(guard.pc);
            //执行初始化类
            ref_class.write().unwrap().init_class(thread,guard);
            return;
        }

        if !field.read().unwrap().is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        if field.read().unwrap().is_final() {
            //如果方法时final的，则两个类要是同一个类，或者是类初始化方法
            if current_class.read().unwrap().name.as_str() != ref_class.read().unwrap().name.as_str()
                || frame.method.name.as_str() != "<clinit>" {
                panic!("java.lang.IllegalAccessError");
            }
        }

        let descriptor = field.read().unwrap().descriptor.clone();
        let slot_id = field.read().unwrap().slot_id; // 根据槽ID可以获取在vars 中的位置

        let mut class_ref = ref_class.write().unwrap();
        let mut slots = &mut class_ref.static_vars;

        let stack = frame.operand_stack();
        match descriptor.chars().next() {
            Some(c) => match c {
                // todo 如果栈顶的槽放的是NIl ，需要处理
                'Z' | 'B' | 'C' | 'S' | 'I' => slots.set_int(slot_id, stack.pop_int()),
                'F' => slots.set_float(slot_id, stack.pop_float()),
                'J' => slots.set_long(slot_id, stack.pop_long()),
                'D' => slots.set_double(slot_id, stack.pop_double()),
                'L' | '[' => slots.set_slot(slot_id, stack.pop_slot()),
                _ => panic!("impossible")
            },
            None => panic!("impossible")
        }
    }
}


impl Debug for PUT_STATIC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(putstatic")
    }
}