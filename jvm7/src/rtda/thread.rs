use crate::rtda::stack::Stack;
use crate::rtda::frame::Frame;
use std::cell::RefCell;
use std::rc::Rc;

/// 线程私有的运行时数据区，
pub struct Thread{
    pub(crate) pc : i32,
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

    pub fn push_frame(&mut self,frame : Rc<RefCell<Frame>>){
        self.stack.push(frame)
    }

    pub fn pop_frame(&mut self)->Option<Rc<RefCell<Frame>>>{
        self.stack.pop()
    }

    pub fn current_frame(&self)->Rc<RefCell<Frame>>{
        self.stack.top().as_ref().unwrap().clone()
    }

    pub fn is_stack_empty(&self)->bool{
        self.stack.top().is_none()
    }

}