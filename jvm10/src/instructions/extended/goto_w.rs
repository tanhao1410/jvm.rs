#[allow(non_camel_case_types)]
pub struct GOTO_W {
    offset: i32
}

impl GOTO_W {
    pub fn new() -> Self {
        Self { offset: 0 }
    }
}

impl Instruction for GOTO_W {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.offset = _reader.read_u32() as i32;
    }

    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
        let frame = rc.borrow_mut();
        BranchInstruction::_branch(frame, self.offset);
    }
}

impl Debug for GOTO_W {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(offset={})", self.offset)
    }
}