
use crate::rtda::thread::Thread;
use crate::rtda::frame::Frame;
use std::sync::{Arc, RwLock};
use crate::instructions::{BytecodeReader, new_instruction};
use crate::rtda::heap::method::Method;
use std::cell::RefCell;
use std::rc::Rc;
use crate::rtda::heap::object::Object;
use crate::rtda::slot::Slot;

/// 解释器,执行一个方法
pub fn interpret(method:Arc<Method>,inst_log_flag :bool,args_array:Arc<RwLock<Object>>){

    //创建一个线程
    let thread = Arc::new(RwLock::new(Thread::new()));
    //创建一个方法帧
    let mut frame = Frame::new(thread.clone(), method);

    let local_vars = frame.local_vars_mut();
    local_vars.set_slot(0,Slot::Ref(args_array));

    thread.write().unwrap().push_frame(Rc::new(RefCell::new(frame)));

    exe_loop(thread,inst_log_flag);
}

pub fn exe_loop(thread:Arc<RwLock<Thread>>,inst_log_flag :bool){
    let mut code_reader = BytecodeReader::new(Arc::new(vec![]));
    let mut num = 0;
    //开始执行代码
    loop{
        //弹出当前帧
        let frame = thread.read().unwrap().current_frame();

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

        if inst_log_flag{
            println!("pc:{},inst_id:{:?}",pc,op_code);
            //打印局部变量表与操作数栈信息
            frame.borrow().print_vars_and_stacks();
            num += 1;

            let frame = frame.borrow();
            println!("{}-{}-{}", frame.method.class.read().unwrap().name.as_str(),frame.method.name.as_str(), num);
        }

        // 执行指令
        inst.execute(thread.clone());
        if thread.read().unwrap().is_stack_empty(){
            break;
        }
    }
}