#[allow(non_camel_case_types)]
pub struct IFNE {
    base: BranchInstruction,
}

impl IFNE {
    pub fn new() -> Self {
        Self { base: BranchInstruction::new() }
    }
}

impl Instruction for IFNE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.base.fetch_operands(reader);
    }

    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let arc = thread.clone();
        let guard = arc.read().unwrap();
        let rc = guard.current_frame();
        let mut frame = rc.borrow_mut();
        let val = frame.operand_stack().pop_int();
        if val != 0 {
            self.base.branch(frame);
        }
    }
}

impl Debug for IFNE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(offset={})", self.base.offset)
    }
}