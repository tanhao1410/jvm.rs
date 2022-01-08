#[allow(non_camel_case_types)]
pub struct INEG {}

impl Instruction for INEG {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let val = stack.pop_int();
        stack.push_int(-val);
    }
}

impl Debug for INEG {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}