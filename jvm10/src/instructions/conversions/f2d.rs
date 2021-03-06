#[allow(non_camel_case_types)]
pub struct F2D {}

impl Instruction for F2D {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let f = stack.pop_float();
        let d = f as f64;
        stack.push_double(d);
    }
}

impl Debug for F2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}