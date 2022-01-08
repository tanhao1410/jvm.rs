#[allow(non_camel_case_types)]
pub struct DSTORE {
    index: usize
}

impl DSTORE {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
    pub fn new0() -> Self {
        Self { index: 0 }
    }
    fn _dstore(thread: Arc<RwLock<Thread>>, index: usize) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let val = frame.operand_stack().pop_f64();
        frame.local_vars().set_f64(index as usize, val);
    }
}

impl Instruction for DSTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        Self::_dstore(thread, self.index);
    }
}

impl Debug for DSTORE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(index={})", self.index)
    }
}