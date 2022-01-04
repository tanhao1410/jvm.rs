#[allow(non_camel_case_types)]
/// 从常量池中取出常量，推入操作数栈
pub struct LDC_W {
    val: u16
}

impl LDC_W {
    pub fn new() -> Self {
        Self { val: 0 }
    }
}

impl Instruction for LDC_W {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.val = reader.read_u16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let cp = frame.method.constant_pool();
        let guard = cp.read().unwrap();
        let constant = guard.get_constant(self.val as usize);
        match constant {
            Constant::Integer(val) => frame.operand_stack().push_int(*val),
            Constant::Float(val) => frame.operand_stack().push_float(*val),
            //Constant::String(val)=>,
            //Constant::Class()=>,
            _ => panic!("LDC_W:error")
        }
    }
}

impl Debug for LDC_W {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(val={})", self.val)
    }
}