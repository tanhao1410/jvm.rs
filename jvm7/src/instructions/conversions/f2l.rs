#[allow(non_camel_case_types)]
pub struct F2L {}

impl Instruction for F2L {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let f = stack.pop_float();
        let l = f as i64;
        stack.push_long(l);
    }
}

impl Debug for F2L {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}