#[allow(non_camel_case_types)]
pub struct FREM {}

impl Instruction for FREM {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        let result = v1.rem_euclid(v2);
        stack.push_float(result);
    }
}

impl Debug for FREM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}