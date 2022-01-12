#[allow(non_camel_case_types)]
pub struct INVOKE_VIRTUAL {
    index: u16
}

impl INVOKE_VIRTUAL {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for INVOKE_VIRTUAL {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let arc = thread.clone();
        let guard = arc.write().unwrap();
        let rc = guard.current_frame();
        let mut frame = rc.borrow_mut();
        //找到当前类
        let current_class = frame.method.class.clone();

        let loader = current_class.read().unwrap().loader.clone();
        //得到它要调用的方法引用
        let cp = frame.method.constant_pool();
        let mut cp_guard = cp.write().unwrap();
        let method_ref = cp_guard.get_constant_mut(self.index as usize).get_method_ref_mut();

        //解析方法
        let method = method_ref.resolve_method();
        if method.is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        //得到调用方法的对象，即
        let stack = frame.operand_stack();
        let slot = stack.get_ref_from_top(method.arg_slot_count - 1);

        match slot {
            Slot::Ref(obj) => {
                //判断调用该方法是否有权限
                //todo
                //从对象的类中查找真正要调用的方法，如果找不到方法，或者找到的是抽象方法，则需要抛出AbstractMethodException

                let obj = obj.clone();
                let obj = obj.read().unwrap();
                let obj_class = obj.class.clone();

                let method_in_invoke_obj = obj_class.read().unwrap().lookup_method(method.name.as_str(), method.descriptor.as_str());
                if let Some(invoke_method) = method_in_invoke_obj {
                    if invoke_method.is_abstract() {
                        panic!("java.lang.AbstractMethodError")
                    }

                    //开始调用方法
                    method_invoke_logic::invoke_method(guard, invoke_method, thread, frame);
                } else {
                    panic!("java.lang.IncompatibleClassChangeError")
                }
            }
            _ => {
                if method_ref.name.as_str() == "println" {
                    let stack = stack;
                    match method_ref.descriptor.as_str() {
                        "(Z)V" => println!("{}", stack.pop_int() != 0),
                        "(C)V" => println!("{}", stack.pop_int()),
                        "(I)V" | "(B)V" | "(S)V" => println!("{}", stack.pop_int()),
                        "(F)V" => println!("{}", stack.pop_float()),
                        "(J)V" => println!("{}", stack.pop_long()),
                        "(D)V" => println!("{}", stack.pop_double()),
                        "(Ljava/lang/String;)V" => {

                            let j_str_slot = stack.pop_slot();

                            let res = get_string_from_slot(loader, &j_str_slot);

                            println!("{}", res);
                            //java的string转成rust的string，打印出来
                        }
                        desc => panic!("println: {}", desc)
                    }
                    stack.pop_slot();
                    return;
                }
                panic!("java.lang.NullPointerException")
            }
        }
    }
}

impl Debug for INVOKE_VIRTUAL {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}