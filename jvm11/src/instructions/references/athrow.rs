#[allow(non_camel_case_types)]
pub struct A_THROW {}

impl Instruction for A_THROW {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let mut thread = thread.write().unwrap();
        let cur_frame = thread.current_frame();

        let mut cur_frame = cur_frame.borrow_mut();
        let pc = cur_frame.next_pc - 1;
        let cur_method= cur_frame.method.clone();
        let stack = cur_frame.operand_stack();

        //弹出栈顶的异常对象
        match stack.pop_slot() {
            Slot::Ref(ex) => {

                //现在当前栈帧里寻找
                match cur_method.find_exception_handler(ex.clone(), pc) {
                    Some(handler_pc) => {
                        stack.clear();
                        stack.push_ref(ex.clone());
                        cur_frame.set_next_pc(handler_pc);
                        return;
                    }
                    _ => {}
                }

                thread.pop_frame();
                find_and_goto_exception_handler(thread, ex.clone());
            }
            _ => panic!(NULL_POINTER_EXCEPTION)
        }
    }
}

/// 异常未捕获到
fn handler_uncaught_exception(mut thread: RwLockWriteGuard<Thread>, ex: Arc<RwLock<Object>>) {
    //清空java虚拟机栈
    thread.clear();
    //获取异常类中的信息
    let ex = ex.read().unwrap();
    let ex_class = ex.class.clone();
    let ex_class = ex_class.read().unwrap();
    let field = ex_class.get_field(EXCEPTION_DETAIL_MESSGAE_NAME, STRING_TYPE_DESCRIPTOR);
    let field_id = field.read().unwrap().slot_id;
    let slot = ex.fields.get_slot(field_id);

    //打印报错信息
    let msg = get_string_from_slot( &slot);
    println!("{}: {}", ex_class.class_name(), msg);

    //打印虚拟机栈信息
    //let field = &ex.fields.slots[field];
    //ex.read().unwrap().fields.slots
    // 采用ex.datas 来存储栈信息，因为ex是对象，因此该数组，实际上什么都没存
    let stack_infos = &ex.datas;
    match stack_infos {
        ArrayDatas::Refs(stack_infos) => {
            for stack_info in stack_infos.slots.iter().rev() {
                let stack_info = get_string_from_slot( stack_info);
                println!("{}",stack_info);
            }
        }
        _ => unreachable!()
    }
}

/// 找到异常处理，并将线程跳转到处理的部分
fn find_and_goto_exception_handler(mut thread: RwLockWriteGuard<Thread>, ex: Arc<RwLock<Object>>) {
    loop {
        let frame = thread.current_frame();
        let mut frame = frame.borrow_mut();
        //从当前帧开始遍历，查找方法的异常处理表，如果找不到，则将当前帧弹出，继续遍历
        let pc = frame.next_pc() - 1;
        match frame.method.find_exception_handler(ex.clone(), pc) {
            Some(handler_pc) => {
                let stack = frame.operand_stack();
                stack.clear();
                stack.push_ref(ex.clone());
                frame.set_next_pc(handler_pc);
                return;
            }
            _ => {}
        }
        thread.pop_frame();
        if thread.is_stack_empty() {
            handler_uncaught_exception(thread, ex);
            return;
        }
    }
}

impl Debug for A_THROW {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(A_THROW)")
    }
}