#[allow(non_camel_case_types)]
pub struct IMUL {}

impl Instruction for IMUL {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
        let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let x = v1.overflowing_mul(v2);
        stack.push_int(x.0);
    }
}

impl Debug for IMUL {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}