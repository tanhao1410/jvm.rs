#[allow(non_camel_case_types)]
pub struct F2I {}

impl Instruction for F2I {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let f = stack.pop_float();
        let i = f as i32;
        stack.push_int(i);
    }
}

impl Debug for F2I {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}