#[allow(non_camel_case_types)]
pub struct GET_FIELD {
    index: u16
}

impl GET_FIELD {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for GET_FIELD {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16()
    }
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let thread_guard = thread.read().unwrap();
        let current_frame = thread_guard.current_frame();
        let mut frame = current_frame.borrow_mut();
        let cp = frame.method.constant_pool();
        //得到字段符号引用
        let mut cp_guard = cp.write().unwrap();
        let field_ref = cp_guard.get_constant_mut(self.index as usize).get_field_ref_mut();
        let field = field_ref.resolve_field();
        let field_guard = field.read().unwrap();
        if field_guard.is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        let descriptor = field_guard.descriptor.clone();
        let slot_id = field_guard.slot_id;
        let stack = frame.operand_stack();
        let slot = stack.pop_slot();
        if let Slot::Ref(obj_ref) = slot {
            match descriptor.chars().next() {
                Some(c) => match c {
                    'Z' | 'B' | 'C' | 'S' | 'I' => {
                        stack.push_int(obj_ref.read().unwrap().fields.get_int(slot_id));
                    }
                    'F' => {
                        stack.push_float(obj_ref.read().unwrap().fields.get_float(slot_id));
                    }
                    'J' => {
                        stack.push_long(obj_ref.read().unwrap().fields.get_long(slot_id));
                    }
                    'D' => {
                        stack.push_double(obj_ref.read().unwrap().fields.get_double(slot_id));
                    }
                    'L' | '[' => {
                        let obj_ref_guard = obj_ref.read().unwrap();
                        let fields = &obj_ref_guard.fields;
                        stack.push_slot(fields.get_slot(slot_id));
                    }
                    _ => panic!("impossible")
                },
                None => panic!("impossible")
            }
        } else {
            panic!("java.lang.NullPointerException")
        }
    }
}

impl Debug for GET_FIELD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(getfield=)")
    }
}