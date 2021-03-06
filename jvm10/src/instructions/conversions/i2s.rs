#[allow(non_camel_case_types)]
pub struct I2S {}

impl Instruction for I2S {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let i = stack.pop_int();
        let s = (i as i16) as i32;
        stack.push_int(s);
    }
}

impl Debug for I2S {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}