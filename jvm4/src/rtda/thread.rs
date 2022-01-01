use crate::rtda::stack::Stack;
use crate::rtda::frame::Frame;
use std::sync::Arc;

/// 线程私有的运行时数据区，
struct Thread{
    pc : i32,
    stack:Stack
}

impl Thread {
    pub fn new()->Self{
        Thread{pc:0,stack:Stack::new(1024)}
    }

    pub fn pc(&self)->i32{
        self.pc
    }

    pub fn set_pc(&mut self,pc : i32){
        self.pc = pc
    }

    pub fn push_frame(&mut self,frame : Arc<Frame>){
        self.stack.push(frame)
    }

    pub fn pop_frame(&mut self)->Option<Arc<Frame>>{
        self.stack.pop()
    }

    pub fn current_frame(&self)->Option<Arc<Frame>>{
        self.stack.top()
    }

}