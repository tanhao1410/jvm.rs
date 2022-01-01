use std::sync::Arc;
use crate::rtda::operand_stack::OperandStack;
use crate::rtda::local_vars::LocalVars;

/// 虚拟机栈中的一个栈帧
pub struct Frame {
    lower: Option<Arc<Frame>>,
    //采用链表的形式来组织栈帧
    local_vars: LocalVars,
    operand_stack: OperandStack,
}

impl Frame {
    pub fn new(max_locals :usize,max_stack : usize) -> Self {
        Frame{lower:None,local_vars:LocalVars::new(max_locals),operand_stack:OperandStack::new(max_stack)}
    }

    pub fn lower(&self) -> Option<Arc<Frame>> {
        if let Some(next) = &self.lower {
            Some(next.clone())
        } else {
            None
        }
    }

    pub fn local_vars(&mut self) -> &mut LocalVars {
        &mut (self.local_vars)
    }

    pub fn operand_stack(&mut self) -> &mut OperandStack {
        &mut (self.operand_stack)
    }
}