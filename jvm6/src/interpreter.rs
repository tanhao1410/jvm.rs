
use crate::classfile::member_info::MemberInfo;
use crate::rtda::thread::Thread;
use crate::rtda::frame::Frame;
use std::sync::{Arc, RwLock};
use crate::instructions::{BytecodeReader, new_instruction};
use crate::instructions::Instruction;
use std::borrow::{BorrowMut, Borrow};
use crate::rtda::heap::method::Method;


/// 解释器,执行一个方法
pub fn interpret(method:Arc<Method>){
    //方法里的code属性

    let max_locals = method.max_locals;
    let max_stack = method.max_stack;
    let byte_code = method.code.clone();

    //创建一个线程
    let mut thread = Arc::new(RwLock::new(Thread::new()));
    //创建一个方法帧
    let frame = Frame::new(max_locals,max_stack,thread.clone(),method);
    thread.write().unwrap().push_frame(Arc::new(RwLock::new(frame)));

    let mut frame = thread.write().unwrap().pop_frame().unwrap();
    let mut code_reader = BytecodeReader::new(byte_code.clone());

    //开始执行代码
    loop{
        //计算pc
        let pc = frame.read().unwrap().next_pc();

        thread.write().unwrap().pc = pc;

        //解码指令
        code_reader.reset(byte_code.clone(),pc as usize);
        let op_code = code_reader.read_u8();

        let mut inst =  new_instruction(op_code);
        inst.fetch_operands(&mut code_reader);
        frame.write().unwrap().set_next_pc(code_reader.pc() as i32);

        // 执行指令
        inst.execute(frame.write().unwrap().borrow_mut());
        println!("pc:{},inst_id:{:?}",pc,op_code);
        frame.read().unwrap().print_vars_and_stacks();
        println!("----------------------------------");
    };
}