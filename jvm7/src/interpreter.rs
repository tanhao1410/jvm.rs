
use crate::classfile::member_info::MemberInfo;
use crate::rtda::thread::Thread;
use crate::rtda::frame::Frame;
use std::sync::{Arc, RwLock};
use crate::instructions::{BytecodeReader, new_instruction};
use crate::instructions::Instruction;
use crate::rtda::heap::method::Method;
use std::cell::RefCell;
use std::rc::Rc;

use std::ops::DerefMut;


/// 解释器,执行一个方法
pub fn interpret(method:Arc<Method>,inst_log_flag :bool){
    //方法里的code属性
    let max_locals = method.max_locals;
    let max_stack = method.max_stack;
    //let byte_code = method.code.clone();
    //创建一个线程
    let mut thread = Arc::new(RwLock::new(Thread::new()));
    //创建一个方法帧
    let frame = Frame::new(max_locals,max_stack,thread.clone(),method);
    thread.write().unwrap().push_frame(Rc::new(RefCell::new(frame)));

    let mut code_reader = BytecodeReader::new(Arc::new(vec![]));

    exe_loop(thread,inst_log_flag,code_reader);
}

pub fn exe_loop(thread:Arc<RwLock<Thread>>,inst_log_flag :bool,mut code_reader:BytecodeReader){
    //开始执行代码
    loop{

        //弹出当前帧
        let mut frame = thread.read().unwrap().current_frame();

        //frame.borrow_mut().
        let byte_code = frame.borrow().method.code.clone();

        //计算pc
        let pc = frame.borrow().next_pc();

        thread.write().unwrap().pc = pc;

        //解码指令
        code_reader.reset(byte_code,pc as usize);

        let op_code = code_reader.read_u8();
        let mut inst =  new_instruction(op_code);
        inst.fetch_operands(&mut code_reader);
        frame.borrow_mut().set_next_pc(code_reader.pc() as i32);

        println!("pc:{},inst_id:{:?}",pc,op_code);

        // 执行指令
        inst.execute(thread.clone());


        frame.borrow().print_vars_and_stacks();
        println!("----------------------------------");

        if thread.read().unwrap().is_stack_empty(){
            break;
        }
    }
}