#[allow(non_camel_case_types)]
pub struct ICONST_3 {}

impl Instruction for ICONST_3 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        frame.operand_stack().push_int(3);
    }
}

impl Debug for ICONST_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}