struct BranchInstruction {
    offset: i32
}

impl BranchInstruction {
    fn new() -> Self {
        Self { offset: 0 }
    }
}

impl BranchInstruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        //不能直接转成i32，因为可能有负数
        self.offset = reader.read_u16() as i16 as i32;
    }

    fn branch(&self, frame: &mut Frame) {
        Self::_branch(frame, self.offset);
    }

    fn _branch(frame: &mut Frame, offset: i32) {
        frame.set_next_pc(frame.thread_pc() + offset);
    }
}
