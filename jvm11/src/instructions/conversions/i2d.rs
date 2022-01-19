#[allow(non_camel_case_types)]
pub struct I2D {}

impl Instruction for I2D {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let i = stack.pop_int();
        let d = i as f64;
        stack.push_double(d);
    }
}

impl Debug for I2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}