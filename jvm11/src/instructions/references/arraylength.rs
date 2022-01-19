#[allow(non_camel_case_types)]
pub struct ARRAY_LENGTH {}

impl Instruction for ARRAY_LENGTH {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let arc = thread.clone();
        let guard = arc.write().unwrap();
        let rc = guard.current_frame();
        let mut frame = rc.borrow_mut();

        let stack = frame.operand_stack();
        let slot = stack.pop_slot();
        let length = match slot{
            Slot::Ref(obj)=>{
                obj.read().unwrap().array_length()
            }
            _=>panic!(NULL_POINTER_EXCEPTION)
        };
        stack.push_int(length as i32);
    }
}

impl Debug for ARRAY_LENGTH {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(ARRAY_LENGTH)")
    }
}