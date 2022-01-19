use crate::rtda::thread::Thread;
use std::sync::{Arc, RwLock};
use crate::instructions::{BytecodeReader, new_instruction};

/// 解释器,执行一个方法
pub fn interpret(thread: Arc<RwLock<Thread>>, inst_log_flag: bool) {
    let mut code_reader = BytecodeReader::new(Arc::new(vec![]));
    //开始执行代码
    while !thread.read().unwrap().is_stack_empty() {
        //当前帧
        let frame = thread.read().unwrap().current_frame();
        let byte_code = frame.borrow().method.code.clone();
        //计算pc
        let pc = frame.borrow().next_pc();
        thread.write().unwrap().pc = pc;
        //解码指令
        code_reader.reset(byte_code, pc as usize);
        let op_code = code_reader.read_u8();
        let mut inst = new_instruction(op_code);
        inst.fetch_operands(&mut code_reader);
        frame.borrow_mut().set_next_pc(code_reader.pc() as i32);
        if inst_log_flag {
            println!("当前pc:{},当前指令id:{:?}", pc, op_code);
            //打印局部变量表与操作数栈信息
            frame.borrow().print_vars_and_stacks();
            let frame = frame.borrow();
            println!("正在执行的类：{}---方法：{}", frame.method.class.read().unwrap().name.as_str(), frame.method.name.as_str());
        }
        // 执行指令
        inst.execute(thread.clone());
    }
}
