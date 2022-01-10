#[allow(non_camel_case_types)]
pub struct FADD {}

impl Instruction for FADD {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let v1 = stack.pop_float();
        let v2 = stack.pop_float();
        stack.push_float(v1 + v2);
    }
}

impl Debug for FADD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}