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

    fn execute(&mut self, frame: &mut Frame) {

        //得到它要调用的方法名称
        let cp = frame.method.constant_pool();
        let guard = cp.read().unwrap();
        let method_constent = guard.get_constant(self.index as usize);
        match method_constent{
            Constant::MethodRef(method_ref)=>{
                if method_ref.name.as_str() == "println" {
                    let stack = frame.operand_stack();
                    match method_ref.descriptor.as_str() {
                        "(Z)V" => println!("{}", stack.pop_int() != 0),
                        "(C)V" => println!("{}", stack.pop_int()),
                        "(I)V" | "(B)V" | "(S)V" => println!("{}", stack.pop_int()),
                        "(F)V" => println!("{}", stack.pop_float()),
                        "(J)V" => println!("{}", stack.pop_long()),
                        "(D)V" => println!("{}", stack.pop_double()),
                        desc => panic!("println: {}", desc)
                    }
                    stack.pop_slot();
                }
            }
            _=>panic!("INVOKE_VIRTUAL error")
        }

    }
}

impl Debug for INVOKE_VIRTUAL {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}