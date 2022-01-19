#[allow(non_camel_case_types)]
pub struct L2D {}

impl Instruction for L2D {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let l = stack.pop_long();
        let d = l as f64;
        stack.push_double(d);
    }
}

impl Debug for L2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}