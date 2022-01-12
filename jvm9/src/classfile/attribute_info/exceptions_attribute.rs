use crate::classfile::class_reader::ClassReader;

pub struct ExceptionsAttribute{
    exception_index_table:Vec<u16>
}

impl ExceptionsAttribute{
    pub fn new(reader:&mut ClassReader)->Self{
        //读取个数
        // let count = reader.read_u16();
        // let mut exception_index_table = vec![];
        // for i in 0..count{
        //     exception_index_table.push(reader.read_u16());
        // }
        let exception_index_table = reader.read_u16s();
        Self{exception_index_table}
    }
}