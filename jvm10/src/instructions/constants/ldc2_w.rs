#[allow(non_camel_case_types)]
/// 从常量池中取出常量，推入操作数栈
pub struct LDC2_W {
    val: u16
}

impl LDC2_W {
    pub fn new() -> Self {
        Self { val: 0 }
    }
}

impl Instruction for LDC2_W {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.val = reader.read_u16();
    }

    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let cp = frame.method.constant_pool();
        let guard = cp.read().unwrap();
        let constant = guard.get_constant(self.val as usize);
        match constant {
            Constant::Double(val) => frame.operand_stack().push_double(*val),
            Constant::Long(val) => frame.operand_stack().push_long(*val),
            //Constant::Class()=>,
            _ => panic!("ldc:error")
        }
    }
}

impl Debug for LDC2_W {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(val={})", self.val)
    }
}