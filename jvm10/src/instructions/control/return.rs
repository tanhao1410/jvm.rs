/// 返回指令，不需要操作数，需要做的有：
/// 把当前帧从虚拟机栈中弹出
/// 2.如果有返回值，则把返回值推向原来的栈帧中的操作数栈顶部
pub struct RETURN{

}

impl Instruction for RETURN {

    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        //把当前的栈帧弹出
        thread.write().unwrap().pop_frame();
    }
}

impl Debug for RETURN {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(RETURN)")
    }
}