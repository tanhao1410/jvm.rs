#[allow(non_camel_case_types)]
pub struct INVOKE_STATIC {
    index: u16
}

impl INVOKE_STATIC {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for INVOKE_STATIC {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let arc = thread.clone();

        let mut guard = arc.write().unwrap();
        let rc = guard.current_frame();

        let mut frame = rc.borrow_mut();
        let cp = frame.method.constant_pool();

        let mut cp_guard = cp.write().unwrap();
        let method_ref = cp_guard.get_constant_mut(self.index as usize).get_method_ref_mut();
        let method = method_ref.resolve_method();

        let ref_class= method.class.clone();
        //在解析完class后，判断类是否已经初始化，如果还未初始化，初始化类
        if !ref_class.read().unwrap().init_started{
            frame.set_next_pc(guard.pc);
            //执行初始化类
            ref_class.write().unwrap().init_class(thread,guard);
            return;
        }

        if !method.is_static() {
            panic!("java.lang.IncopatibleClassChnageError");
        }

        //调用方法
        invoke_method(guard, method.clone(),thread,frame);

        //创建一个新的栈帧
        // let arg_slot_count = method.arg_slot_count;
        // let mut new_frame = Rc::new(RefCell::new(
        //     Frame::new(method.max_locals, method.max_stack, thread.clone(), method.clone())));
        //
        // //thread.write().unwrap().push_frame(new_frame.clone());
        // guard.push_frame(new_frame.clone());
        //
        // //设置参数，从原来的操作数栈中弹出参数，放入到新栈帧中的局部变量表中
        // if arg_slot_count > 0 {
        //     for index in (0..arg_slot_count).rev() {
        //         //从原来栈中弹出
        //         let slot = frame.operand_stack().pop_slot();
        //         new_frame.borrow_mut().local_vars().set_slot(index, slot);
        //     }
        // }

    }
}

impl Debug for INVOKE_STATIC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(INVOKE_STATIC)")
    }
}