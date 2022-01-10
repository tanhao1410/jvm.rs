#[allow(non_camel_case_types)]
pub struct LSTORE {
    index: usize
}

impl LSTORE {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
    pub fn new0() -> Self {
        Self { index: 0 }
    }
    fn _lstore(thread: Arc<RwLock<Thread>>, index: usize) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let val = frame.operand_stack().pop_i64();
        frame.local_vars().set_i64(index as usize, val);
    }
}

impl Instruction for LSTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        Self::_lstore(thread, self.index);
    }
}

impl Debug for LSTORE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(index={})", self.index)
    }
}