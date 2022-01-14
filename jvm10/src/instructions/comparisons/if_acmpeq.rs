#[allow(non_camel_case_types)]
pub struct IF_ACMPEQ {
    base: BranchInstruction
}

impl IF_ACMPEQ {
    pub fn new() -> Self {
        Self { base: BranchInstruction::new() }
    }
}

impl Instruction for IF_ACMPEQ {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.base.fetch_operands(reader);
    }

    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let arc = thread.clone();
        let guard = arc.read().unwrap();
        let rc = guard.current_frame();
        let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let val2 = stack.pop_slot();
        let val1 = stack.pop_slot();
        match (val1, val2) {
            (Slot::Nil(), Slot::Nil()) => self.base.branch(frame),
            (Slot::Ref(v1), Slot::Ref(v2)) => {
                if v1.read().unwrap().eq(&v2.read().unwrap()) {
                    self.base.branch(frame);
                }
            }
            _ => {}
        }

        // if val1.read().unwrap().deref() == val2.read().unwrap().deref() {
        //     self.base.branch(frame);
        // }
    }
}

impl Debug for IF_ACMPEQ {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(offset={})", self.base.offset)
    }
}