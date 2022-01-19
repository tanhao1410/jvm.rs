#[allow(non_camel_case_types)]
pub struct ICONST_5 {}

impl Instruction for ICONST_5 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        frame.operand_stack().push_int(5);
    }
}

impl Debug for ICONST_5 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}