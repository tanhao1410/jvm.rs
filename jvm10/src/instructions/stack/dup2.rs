#[allow(non_camel_case_types)]
pub struct DUP2 {}

impl Instruction for DUP2 {
    /*
    bottom -> top
    [...][c][b][a]____
              \____   |
                   |  |
                   V  V
    [...][c][b][a][b][a]
    */
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let slot1 = stack.pop_slot();
        let slot2 = stack.pop_slot();

        stack.push_slot(slot2.clone());
        stack.push_slot(slot1.clone());
        stack.push_slot(slot2);
        stack.push_slot(slot1);
    }
}

impl Debug for DUP2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}