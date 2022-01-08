#[allow(non_camel_case_types)]
pub struct FLOAD {
    index: usize
}

impl FLOAD {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
    pub fn new0() -> Self {
        Self { index: 0 }
    }
    fn _fload(thread: Arc<RwLock<Thread>>, index: usize) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let val = frame.local_vars().get_float(index);
        frame.operand_stack().push_float(val);
    }
}

impl Instruction for FLOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        Self::_fload(thread, self.index);
    }
}

impl Debug for FLOAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(index={})", self.index)
    }
}