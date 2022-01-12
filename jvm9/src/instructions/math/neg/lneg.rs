#[allow(non_camel_case_types)]
pub struct LNEG {}

impl Instruction for LNEG {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let val = stack.pop_long();
        stack.push_long(-val);
    }
}

impl Debug for LNEG {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}