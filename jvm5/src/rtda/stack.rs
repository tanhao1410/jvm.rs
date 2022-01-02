use crate::rtda::frame::Frame;
use std::sync::{Arc, RwLock};

/// java 虚拟机栈
pub struct Stack{
    max_size:usize,
    size :usize,
    top: Option<Arc<RwLock<Frame>>>
}

impl Stack{
    pub fn new(max_size:usize)->Self{
        Stack{max_size,size:0, top:None}
    }

    pub fn push(&mut self,frame:Arc<RwLock<Frame>>){
        //判断是否是第一个
        if self.top.is_none(){
            self.top = Some(frame);
        }
    }

    pub fn pop(&mut self)->Option<Arc<RwLock<Frame>>>{
        if self.top.is_none(){
            None
        }else{
            let top = self.top.take().unwrap();
            self.top = top.write().unwrap().lower().take();
            Some(top)
        }
        // if let Some(top) = &self.top{
        //     let option = top.read().unwrap().lower().clone();
        //     self.top = option;
        //     Some(top.clone())
        // }else{
        //     None
        // }

    }

    pub fn top(&self)->Option<Arc<RwLock<Frame>>>{
        if let Some(top) = &self.top{
            Some(top.clone())
        }else{
            None
        }
    }
}