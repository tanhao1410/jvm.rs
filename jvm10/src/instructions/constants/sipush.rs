#[allow(non_camel_case_types)]
pub struct SIPUSH {
    val: i16
}

impl SIPUSH {
    pub fn new() -> Self {
        Self { val: 0 }
    }
}

impl Instruction for SIPUSH {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.val = reader.read_u16() as i16;
    }

    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        frame.operand_stack().push_int(self.val as i32);
    }
}

impl Debug for SIPUSH {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(val={})", self.val)
    }
}