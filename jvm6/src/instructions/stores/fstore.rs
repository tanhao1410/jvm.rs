#[allow(non_camel_case_types)]
pub struct FSTORE {
    index: usize
}

impl FSTORE {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
    pub fn new0() -> Self {
        Self { index: 0 }
    }
    fn _fstore(frame: &mut Frame, index: usize) {
        let val = frame.operand_stack().pop_f32();
        frame.local_vars().set_f32(index as usize, val);
    }
}

impl Instruction for FSTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_fstore(frame, self.index);
    }
}

impl Debug for FSTORE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(index={})", self.index)
    }
}