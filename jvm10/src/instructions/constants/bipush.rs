#[allow(non_camel_case_types)]
pub struct BIPUSH {
    val: i8
}

impl BIPUSH {
    pub fn new() -> Self {
        Self { val: 0 }
    }
}

impl Instruction for BIPUSH {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.val = reader.read_i8();
    }

    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        frame.operand_stack().push_int(self.val as i32);
    }
}

impl Debug for BIPUSH {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(val={})", self.val)
    }
}