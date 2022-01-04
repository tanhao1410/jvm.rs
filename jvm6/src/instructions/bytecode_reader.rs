/// 专门读取字节码的
pub struct BytecodeReader {
    code: Arc<Vec<u8>>,
    pc: usize,
}

impl BytecodeReader {
    pub fn new(code:Arc<Vec<u8>>)->Self{
        BytecodeReader{code,pc:0}
    }

    pub fn pc(&self)->usize{
        self.pc
    }

    pub fn reset(&mut self,code:Arc<Vec<u8>>,pc:usize){
        self.pc = pc;
        self.code = code;
    }

    pub fn read_u8(&mut self)->u8{
        let u = self.code[self.pc as usize];
        self.pc += 1;
        u
    }

    fn read_i8(&mut self)->i8{
        self.read_u8() as i8
    }

    fn read_u16(&mut self)->u16{

        let high = self.read_u8() as u16;
        let low = self.read_u8() as u16;
        let res = (high << 8) | low;
        res
    }

    fn read_u32(&mut self) -> u32 {
        const S: usize = std::mem::size_of::<u32>();
        let mut bytes: [u8; S] = [0; S];
        for i in self.pc..self.pc + S {
            bytes[i - self.pc] = self.code[i];
        }
        self.pc += S;
        u32::from_be_bytes(bytes)
    }

    pub fn read_i32s(&mut self, n: i32) -> Vec<i32> {
        let mut ints = Vec::with_capacity(n as usize);
        for _ in 0..n {
            ints.push(self.read_u32() as i32);
        }
        ints
    }
    fn skip_padding(&mut self) {
        while self.pc % 4 != 0 {
            self.read_u8();
        }
    }
}