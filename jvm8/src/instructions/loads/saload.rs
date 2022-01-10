#[allow(non_camel_case_types)]
pub struct SALOAD {}


impl Instruction for SALOAD {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let frame = guard.current_frame();
        let mut ref_mut = frame.borrow_mut();
        let stack = ref_mut.operand_stack();

        //第一个操作数，数组索引，第二个操作数，数组引用
        let index = stack.pop_int();
        let arr_ref = stack.pop_slot();
        if let Slot::Ref(arr_ref) = arr_ref {
            let arr_obj = arr_ref.read().unwrap();
            let arr_len = arr_obj.array_length();
            if index < 0 || index >= arr_len as i32 {
                panic!("java.lang.ArrayIndexOutOfBoundsException")
            }

            //看数组里存放的是什么
            //看数组里存放的是什么
            let datas = arr_obj.shorts();
            stack.push_int(datas[index as usize] as i32);
        } else {
            panic!("java.lang.NullPointerException")
        }
    }
}

impl Debug for SALOAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}