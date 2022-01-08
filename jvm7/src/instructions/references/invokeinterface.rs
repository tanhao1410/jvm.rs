#[allow(non_camel_case_types)]
pub struct INVOKE_INTERFACE {
    index: u16
}

impl Instruction for INVOKE_INTERFACE {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
        //注意的是，它的操作码后面是跟了四个字节，前两个字节是一个常量池索引，指向一个接口方法引用
        //第二个是方法需要传递的参数个数，前面已经计算过了，不需要了
        //第三个，固定的一个0
        _reader.read_u16();
    }

    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let arc = thread.clone();
        let guard = arc.write().unwrap();
        let rc = guard.current_frame();
    let mut frame = rc.borrow_mut();
        //找到当前类
        let current_class = frame.method.class.clone();
        //得到它要调用的方法引用
        let cp = frame.method.constant_pool();
        let mut cp_guard = cp.write().unwrap();
        let method_ref = cp_guard.get_constant_mut(self.index as usize).get_method_ref_mut();

        //解析方法
        let method = method_ref.resolve_method();
        if method.is_static() || method.is_private() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        //从操作数栈中弹出this引用
        let slot = frame.operand_stack().get_ref_from_top(method.arg_slot_count - 1);
        match slot {
            Slot::Ref(obj) => {

                //判断调用该对象所在的类，是否实现了该接口
                if !method_ref.class.as_ref().unwrap().clone().read().unwrap().is_implements(current_class) {
                    panic!("java.lang.IncompatibleClassChangeError");
                }


                //从对象的类中查找真正要调用的方法，如果找不到方法，或者找到的是抽象方法，则需要抛出AbstractMethodException
                let method_in_invoke_obj = method_ref.class.as_ref().unwrap().read().unwrap().lookup_method(method.name.as_str(), method.descriptor.as_str());
                if let Some(invoke_method) = method_in_invoke_obj {
                    if invoke_method.is_abstract() {
                        panic!("java.lang.AbstractMethodError")
                    }

                    //方法必须是公共的
                    if !invoke_method.is_public() {
                        panic!("java.lang.IllegalAccessError")
                    }

                    //开始调用方法
                    method_invoke_logic::invoke_method(guard, invoke_method,thread,frame);
                } else {
                    panic!("java.lang.AbstractMethodError")
                }
            }
            // _=>panic!("java.lang.NullPointerException")
            _ => {
                panic!("java.lang.NullPointerException")
            }
        }
    }
}

impl Debug for INVOKE_INTERFACE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(INVOKE_INTERFACE)")
    }
}