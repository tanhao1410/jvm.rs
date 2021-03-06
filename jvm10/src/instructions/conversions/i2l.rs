#[allow(non_camel_case_types)]
pub struct I2L {}

impl Instruction for I2L {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let i = stack.pop_int();
        let l = i as i64;
        stack.push_long(l);
    }
}

impl Debug for I2L {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}