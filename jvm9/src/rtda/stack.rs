use crate::rtda::frame::Frame;
use std::cell::RefCell;
use std::rc::Rc;

/// java 虚拟机栈
pub struct Stack{
    max_size:usize,
    size :usize,
    top: Option<Rc<RefCell<Frame>>>
}

impl Stack{
    pub fn new(max_size:usize)->Self{
        Stack{max_size,size:0, top:None}
    }

    pub fn push(&mut self,frame:Rc<RefCell<Frame>>){
        self.size += 1;
        //判断是否是第一个
        if self.top.is_none(){
            self.top = Some(frame);
        } else{
            frame.borrow_mut().lower = self.top.take();
            self.top = Some(frame);
        }
    }

    pub fn pop(&mut self)->Option<Rc<RefCell<Frame>>>{
        if self.top.is_none(){
            None
        }else{
            self.size -= 1;
            let top = self.top.take().unwrap();
            //let mut next = self.top.as_ref().unwrap().borrow().lower.clone();
            self.top = top.borrow_mut().lower().take();
            //std::mem::swap(&mut self.top,);
            //self.top = next;
            //std::mem::swap(&mut self.top,&mut next);
            Some(top)
        }
    }

    pub fn top(&self)->Option<Rc<RefCell<Frame>>>{
        if let Some(top) = &self.top{
            Some(top.clone())
        }else{
            None
        }
    }
}