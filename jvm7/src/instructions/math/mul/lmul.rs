#[allow(non_camel_case_types)]
pub struct LMUL {}

impl Instruction for LMUL {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        stack.push_long(v1 * v2);
    }
}

impl Debug for LMUL {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}