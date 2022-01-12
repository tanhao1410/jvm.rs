#[allow(non_camel_case_types)]
pub struct IASTORE {}

impl Instruction for IASTORE{
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let frame = guard.current_frame();
        let mut ref_mut = frame.borrow_mut();
        let stack = ref_mut.operand_stack();

        let val = stack.pop_int();
        let index = stack.pop_int();
        let arr_ref = stack.pop_slot();
        if let Slot::Ref(arr_ref) = arr_ref{
            let mut arr_obj = arr_ref.write().unwrap();
            let arr_len = arr_obj.array_length();
            if index < 0 || index >= arr_len as i32{
                panic!("java.lang.ArrayIndexOutOfBoundsException")
            }
            let vec = arr_obj.ints_mut();
            vec[index as usize] = val;
        }else{
            panic!("java.lang.NullPointerException")
        }
    }
}

impl Debug for IASTORE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}