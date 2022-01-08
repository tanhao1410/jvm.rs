use crate::rtda::frame::Frame;
use std::sync::{RwLock, Arc, RwLockWriteGuard};
use crate::rtda::heap::method::Method;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use crate::rtda::thread::Thread;

///调用一个方法
pub fn invoke_method(mut guard: RwLockWriteGuard<Thread>, method: Arc<Method>, thread: Arc<RwLock<Thread>>, mut frame:RefMut<Frame>) {


    //let rc = guard.current_frame();
    //let mut frame = rc.borrow_mut();
    let arg_slot_count = method.arg_slot_count;

    //创建一个新的栈帧
    let mut new_frame = Rc::new(RefCell::new(
        Frame::new(method.max_locals, method.max_stack, thread.clone(), method.clone())));

    //thread.write().unwrap().push_frame(new_frame.clone());
    guard.push_frame(new_frame.clone());

    //设置参数，从原来的操作数栈中弹出参数，放入到新栈帧中的局部变量表中
    if arg_slot_count > 0 {
        for index in (0..arg_slot_count).rev() {
            //从原来栈中弹出
            let slot = frame.operand_stack().pop_slot();
            new_frame.borrow_mut().local_vars().set_slot(index, slot);
        }
    }

    //临时处理Object中的本地方法
    if method.is_native(){
        if method.name.as_str() == "registerNatives"{
            guard.pop_frame();
        }else{
            panic!("native method:{}",method.name.as_str());
        }
    }
}
