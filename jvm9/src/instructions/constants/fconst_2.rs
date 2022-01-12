#[allow(non_camel_case_types)]
pub struct FCONST_2 {}

impl Instruction for FCONST_2 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        frame.operand_stack().push_float(2f32);
    }
}

impl Debug for FCONST_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}