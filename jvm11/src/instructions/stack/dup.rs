#[allow(non_camel_case_types)]
pub struct DUP {}

impl Instruction for DUP {
    /*
    bottom -> top
    [...][c][b][a]
                 \_
                   |
                   V
    [...][c][b][a][a]
    */
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let slot = stack.pop_slot();
        stack.push_slot(slot.clone());
        stack.push_slot(slot);
    }
}

impl Debug for DUP {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}