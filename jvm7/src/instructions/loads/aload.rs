#[allow(non_camel_case_types)]
pub struct ALOAD {
    index: usize
}

impl ALOAD {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
    pub fn new0() -> Self {
        Self { index: 0 }
    }
    fn _aload(thread: Arc<RwLock<Thread>>, index: usize) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let val = frame.local_vars().get_slot(index).clone();
        frame.operand_stack().push_slot(val);
    }
}

impl Instruction for ALOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {

        Self::_aload(thread, self.index);
    }
}

impl Debug for ALOAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(index={})", self.index)
    }
}