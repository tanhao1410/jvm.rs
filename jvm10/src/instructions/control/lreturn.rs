/// 返回指令，不需要操作数，需要做的有：
/// 把当前帧从虚拟机栈中弹出
/// 2.如果有返回值，则把返回值推向原来的栈帧中的操作数栈顶部
pub struct LRETURN {}

impl Instruction for LRETURN {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        // let guard = thread.read().unwrap();
        // let rc = guard.current_frame();
        // let mut frame = rc.borrow_mut();


        //把当前的栈帧弹出

        //问题？此处需要thread的写指针，但此时调用深的话，已经有人持有了读，导致死锁的产生。

        let mut guard = thread.write().unwrap();
        let current_frame = guard.pop_frame().unwrap();
        let frame = guard.pop_frame();
        //
        if let Some(frame) = frame {
            //返回值在当前帧的栈顶部
            let res = current_frame.borrow_mut().operand_stack().pop_long();
            frame.borrow_mut().operand_stack().push_long(res);
            guard.push_frame(frame);
        }
        //todo
    }
}

impl Debug for LRETURN {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(LRETURN)")
    }
}