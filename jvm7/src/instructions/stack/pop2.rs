#[allow(non_camel_case_types)]
pub struct POP2 {}

impl Instruction for POP2 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        stack.pop_slot();
        stack.pop_slot();
    }
}

impl Debug for POP2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}