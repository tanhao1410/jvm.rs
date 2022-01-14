#[allow(non_camel_case_types)]
pub struct IADD {}

impl Instruction for IADD {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let v1 = stack.pop_i32();
        let v2 = stack.pop_int();
        stack.push_int(v1 + v2);
    }
}

impl Debug for IADD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}