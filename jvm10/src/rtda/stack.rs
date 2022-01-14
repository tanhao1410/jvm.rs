use crate::rtda::frame::Frame;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
/// java 虚拟机栈
pub struct Stack{
    max_size:usize,
    size :usize,
    //top: Option<Rc<RefCell<Frame>>>
    pub frames:Vec<Rc<RefCell<Frame>>>
}

impl Stack{
    pub fn new(max_size:usize)->Self{
        Stack{max_size,size:0, frames: vec![] }
    }

    pub fn push(&mut self,frame:Rc<RefCell<Frame>>){
        self.frames.push(frame);
    }

    pub fn pop(&mut self)->Option<Rc<RefCell<Frame>>>{
        self.frames.pop()
    }

    pub fn top(&self)->Option<Rc<RefCell<Frame>>>{
        let size = self.frames.len();
        if size > 0{
            let option = self.frames.get(size - 1).unwrap();
            Some(option.clone())
        }else{
            None
        }
    }
}