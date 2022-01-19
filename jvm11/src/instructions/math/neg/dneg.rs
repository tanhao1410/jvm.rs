#[allow(non_camel_case_types)]
pub struct DNEG {}

impl Instruction for DNEG {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let val = stack.pop_double();
        stack.push_double(-val);
    }
}

impl Debug for DNEG {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}