use std::sync::{Arc, RwLock};
use crate::rtda::operand_stack::OperandStack;
use crate::rtda::local_vars::LocalVars;
use crate::rtda::thread::Thread;
use crate::rtda::heap::method::Method;
use std::rc::Rc;
use std::cell::RefCell;


/// 虚拟机栈中的一个栈帧
pub struct Frame {
    pub lower: Option<Rc<RefCell<Frame>>>,
    //采用链表的形式来组织栈帧
    local_vars: LocalVars,
    operand_stack: OperandStack,

    //实现跳转指令
    pub next_pc: i32,
    pub thread: Arc<RwLock<Thread>>,

    //给frame添加一个method字段，通过该字段，可以方便的从栈帧中获取到所有常量池相关的
    pub method: Arc<Method>,
}

impl Frame {
    /// 打印 局部变量表与操作数栈信息
    pub fn print_vars_and_stacks(&self) {
        self.local_vars.print_vars();
        self.operand_stack.print_stack();
    }

    pub fn new(thread: Arc<RwLock<Thread>>, method: Arc<Method>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Frame {
            next_pc: 0,
            thread,
            lower: None,
            local_vars: LocalVars::new(method.max_locals),
            operand_stack: OperandStack::new(method.max_stack),
            method,
        }))
    }

    pub fn local_vars_mut(&mut self) -> &mut LocalVars {
        &mut (self.local_vars)
    }

    pub fn local_vars(&self) -> &LocalVars {
        &self.local_vars
    }

    pub fn operand_stack(&mut self) -> &mut OperandStack {
        &mut (self.operand_stack)
    }

    pub fn next_pc(&self) -> i32 {
        self.next_pc
    }

    pub fn thread_pc(&self) -> i32 {
        self.thread.read().unwrap().pc
    }

    pub fn set_next_pc(&mut self, pc: i32) {
        self.next_pc = pc;
    }
}