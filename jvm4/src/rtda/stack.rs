use crate::rtda::frame::Frame;
use std::sync::Arc;

/// java 虚拟机栈
pub struct Stack{
    max_size:usize,
    size :usize,
    top: Option<Arc<Frame>>
}

impl Stack{
    pub fn new(max_size:usize)->Self{
        Stack{max_size,size:0, top:None}
    }

    pub fn push(&mut self,frame:Arc<Frame>){
        //判断是否是第一个
        if self.top.is_none(){
            self.top = Some(frame);
        }
    }

    pub fn pop(&mut self)->Option<Arc<Frame>>{
        if let Some(top) = &self.top{
            top.lower()
        }else{
            None
        }

    }

    pub fn top(&self)->Option<Arc<Frame>>{
        if let Some(top) = &self.top{
            Some(top.clone())
        }else{
            None
        }
    }
}