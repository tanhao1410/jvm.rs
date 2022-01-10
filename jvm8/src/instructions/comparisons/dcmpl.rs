#[allow(non_camel_case_types)]
pub struct DCMPL {}

impl Instruction for DCMPL {
    fn execute(&mut self, thread:Arc<RwLock<Thread>>) {
        let arc = thread.clone();
        let guard = arc.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let stack = frame.operand_stack();
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        stack.push_int(
            if v1 > v2 {
                1
            } else if v1 == v2 {
                0
            } else if v1 < v2 {
                -1
            } else {
                -1
            });
    }
}
impl Debug for DCMPL {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}