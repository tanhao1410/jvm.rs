#[allow(non_camel_case_types)]
pub struct GET_STATIC {
    index: u16
}

impl GET_STATIC {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for GET_STATIC {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16()
    }
    fn execute(&mut self, frame: &mut Frame) {
        let cp = frame.method.constant_pool();
        let current_class = cp.read().unwrap().get_class();
        //得到字段符号引用
        let mut guard = cp.write().unwrap();
        let mut field_ref = guard.get_constant_mut(self.index as usize).get_field_ref_mut();


        let field = field_ref.resolve_field();
        let ref_class = field.read().unwrap().class.clone();

        if !field.read().unwrap().is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        let descriptor = field.read().unwrap().descriptor.clone();
        let slot_id = field.read().unwrap().slot_id; // 根据槽ID可以获取在vars 中的位置

        let mut class_ref = ref_class.write().unwrap();
        let mut slots = &mut class_ref.static_vars;

        let stack = frame.operand_stack();
        match descriptor.chars().next() {
            Some(c) => match c {

                // todo 如果栈顶的槽放的是NIl ，需要处理
                'Z' | 'B' | 'C' | 'S' | 'I' => stack.push_int(slots.get_int(slot_id)),
                'F' => stack.push_float(slots.get_float(slot_id)),
                'J' => stack.push_long(slots.get_long(slot_id)),
                'D' => stack.push_double(slots.get_double(slot_id)),
                'L' | '[' => stack.push_slot(slots.get_slot(slot_id)),
                _ => panic!("impossible")
            },
            None => panic!("impossible")
        }
    }
}

impl Debug for GET_STATIC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(getstatic)")
    }
}