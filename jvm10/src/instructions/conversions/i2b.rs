#[allow(non_camel_case_types)]
pub struct I2B {}

impl Instruction for I2B {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let i = stack.pop_int();
        let b = (i as i8) as i32;
        stack.push_int(b);
    }
}

impl Debug for I2B {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}