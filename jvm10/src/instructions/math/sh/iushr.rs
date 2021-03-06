#[allow(non_camel_case_types)]
pub struct IUSHR {}

impl Instruction for IUSHR {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let s = (v2 as u32) & 0x1f;
        stack.push_int(((v1 as u32) >> s) as i32);
    }
}

impl Debug for IUSHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}