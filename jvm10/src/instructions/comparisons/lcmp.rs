#[allow(non_camel_case_types)]
pub struct LCMP {}

impl Instruction for LCMP {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
        let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        stack.push_int(match v1 - v2 {
            a if a > 0 => 1,
            0 => 0,
            _ => -1
        });
    }
}

impl Debug for LCMP {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}