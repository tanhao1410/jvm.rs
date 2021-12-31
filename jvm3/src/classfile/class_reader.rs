pub struct ClassReader{
    data : Vec<u8>,
    pc:usize,
}

impl ClassReader{
    pub fn new(data : Vec<u8>)->Self{
        ClassReader{data,pc:0}
    }

    pub fn read_u8(&mut self)->u8{
        let res = self.data[self.pc];
        self.pc += 1;
        res
    }

    pub fn read_u16(&mut self)->u16{
        //读取两个字节
        let mut bytes = [0u8;2];
        bytes[0] = self.data[self.pc];
        bytes[1] = self.data[self.pc + 1];
        self.pc += 2;
        u16::from_be_bytes(bytes)
    }

    pub fn read_u32(&mut self) -> u32 {
        const S: usize = std::mem::size_of::<u32>();
        let mut bytes: [u8; S] = [0; S];
        for i in self.pc..self.pc + S {
            bytes[i - self.pc] = self.data[i];
        }
        self.pc += S;
        u32::from_be_bytes(bytes)
    }

    pub fn read_u64(&mut self) -> u64 {
        const S: usize = 8;
        let mut bytes: [u8; S] = [0; S];
        for i in self.pc..self.pc + S {
            bytes[i - self.pc] = self.data[i];
        }
        self.pc += S;
        u64::from_be_bytes(bytes)
    }

    ///读取一个u16表，表的大小由开头的u16来表示
    pub fn read_u16s(&mut self) ->Vec<u16>{
        let size = self.read_u16();
        let mut res = vec![0; size as usize];
        for i in 0..size{
            res[i as usize] = self.read_u16();
        }
        res
    }

    pub fn read_bytes(&mut self, length: u32) -> Vec<u8> {
        let mut n = length;
        let mut v = Vec::new();
        while n > 0 {
            v.push(self.read_u8());
            n -= 1;
        }
        v
    }

}