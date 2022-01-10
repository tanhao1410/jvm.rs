#[allow(non_camel_case_types)]
pub struct INVOKE_SPECIAL {
    index: u16
}

impl INVOKE_SPECIAL {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for INVOKE_SPECIAL {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let arc = thread.clone();
        let guard = arc.write().unwrap();
        let rc = guard.current_frame();
        let mut frame = rc.borrow_mut();
        let current_class = frame.method.class.clone();
        let cp = frame.method.constant_pool();
        let mut cp_guard = cp.write().unwrap();
        let method_ref = cp_guard.get_constant_mut(self.index as usize).get_method_ref_mut();

        let method = method_ref.resolve_method();
        let class = method_ref.class.as_ref().clone().unwrap();

        //如果方法的名称是init 即构造函数，但方法引用的类与方法所属的类不相同，报错
        if method.name.as_ref().eq("<init>") && method.class.read().unwrap().name.as_str() != class.read().unwrap().name.as_str() {
            panic!("java.lang.NoSuchMethodError");
        }

        //是静态方法
        if method.is_static() {
            panic!("java.lang.IncopatibleClassChangeError");
        }

        //查看操作数栈中的this引用，如果该引用是null,抛出异常
        match frame.operand_stack().get_ref_from_top(method.arg_slot_count) {
            Slot::Nil() => panic!("java.lang.NullPointException"),
            _ => {}
        }


        //确保protected方法只能被声明该方法的类或子类调用
        //TODO

        //如果调用的是超类中的函数，但不是构造函数，且当前类的ACC_SUPER标准被设置，需要一个额外的过程查找最终要调用的方法；
        //否则，前面从方法符号引用中解析出来的方法就是要调用的方法

        //如果查找过程失败，或找到的方法时抽象的，抛出异常。

        invoke_method(guard, method,thread,frame);
    }
}

impl Debug for INVOKE_SPECIAL {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}