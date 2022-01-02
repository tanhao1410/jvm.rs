#[allow(non_camel_case_types)]
pub struct IFNULL {
    base: BranchInstruction
}
impl IFNULL {
    pub fn new() -> Self {
        Self { base: BranchInstruction::new() }
    }
}

impl Instruction for IFNULL {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader){
        self.base.fetch_operands(_reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        if let Slot::Nil() =  frame.operand_stack().pop_slot(){
            self.base.branch(frame);
        }
    }
}

impl Debug for IFNULL {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(offset={})", self.base.offset)
    }
}