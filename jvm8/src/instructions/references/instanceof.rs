#[allow(non_camel_case_types)]
pub struct INSTANCE_OF {
    index: u16
}
impl INSTANCE_OF {
    pub fn new() -> Self {
        Self { index:0 }
    }
}

impl Instruction for INSTANCE_OF {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader){
        self.index = _reader.read_u16()
    }
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let cp = frame.method.constant_pool();
        //得到类引用
        let mut guard = cp.write().unwrap();
        let mut class_ref = guard.get_constant_mut(self.index as usize);
        match class_ref{
            Constant::Class( class_ref)=>{
                let class = class_ref.resolve_class();

                //从栈顶弹出一个引用，查看该引用是否是该类或接口的一个实例
                let slot = frame.operand_stack().pop_slot();
                match slot{
                    Slot::Ref(obj_ref)=>{
                        if obj_ref.read().unwrap().is_instanceof(class){
                            frame.operand_stack().push_int(1);
                        }else{
                            frame.operand_stack().push_int(0);
                        }
                    }
                    //nil的情况下，结果为0
                    _=>frame.operand_stack().push_int(0)
                }
            },
            _=>panic!("INSTANCE_OF 指令 error")
        }
    }
}

impl Debug for INSTANCE_OF {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(INSTANCE_OF=)")
    }
}