#[allow(non_camel_case_types)]
pub struct D2F {}

impl Instruction for D2F {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let d = stack.pop_double();
        let f = d as f32;
        stack.push_float(f);
    }
}

impl Debug for D2F {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}