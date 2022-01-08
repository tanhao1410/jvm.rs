/// 返回指令，不需要操作数，需要做的有：
/// 把当前帧从虚拟机栈中弹出
/// 2.如果有返回值，则把返回值推向原来的栈帧中的操作数栈顶部
pub struct DRETURN{

}

impl Instruction for DRETURN {

    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let thread = frame.thread.clone();

        //把当前的栈帧弹出
        let current_frame = thread.write().unwrap().pop_frame().unwrap();
        let frame = thread.write().unwrap().pop_frame();
        //
        if let Some(frame) = frame{
            //返回值在当前帧的栈顶部
            let res = current_frame.borrow_mut().operand_stack().pop_double();
            frame.borrow_mut().operand_stack().push_double(res);
            thread.write().unwrap().push_frame(frame);
        }
    }
}

impl Debug for DRETURN {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(DRETURN)")
    }
}