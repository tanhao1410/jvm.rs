use std::sync::{Arc, RwLock};
use crate::rtda::heap::class::Class;
use crate::classfile::attribute_info::code_attribute::ExceptionTableEntry;
use crate::rtda::heap::constant_pool::constant_pool::ConstantPool;

#[derive(Default)]
pub struct ExceptionTable {
    handlers: Vec<ExceptionHandler>
}

impl ExceptionTable {
    pub fn new(file_exception_table: &Vec<ExceptionTableEntry>, pool: Arc<RwLock<ConstantPool>>) -> Self {
        Self {
            handlers: file_exception_table
                .iter()
                .map(|info| {
                    ExceptionHandler::new(info, pool.clone())
                })
                .collect()
        }
    }

    pub fn find_exception_handler(&self, ex_class: Arc<RwLock<Class>>, pc: i32) -> Option<&ExceptionHandler> {
        for handler in &self.handlers {
            //在处理代码之间
            if pc >= handler.start_pc as i32 && pc < handler.end_pc as i32 {
                match &handler.catch_type {
                    Some(catch_class) => {
                        if catch_class.read().unwrap().name.as_str() == ex_class.read().unwrap().name.as_str()
                            || ex_class.read().unwrap().is_sub_class_of(catch_class.clone()) {
                            return Some(handler);
                        }
                    }
                    None => return Some(handler)
                }
            }
        }
        None
    }
}


pub struct ExceptionHandler {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: Option<Arc<RwLock<Class>>>,
    //pub catch_type: ClassRef,
}

impl ExceptionHandler {
    pub fn new(entry: &ExceptionTableEntry, pool: Arc<RwLock<ConstantPool>>) -> Self {
        //从常量池中解析异常类
        let mut pool = pool.write().unwrap();
        if entry.catch_type == 0 {
            Self {
                start_pc: entry.start_pc,
                end_pc: entry.end_pc,
                handler_pc: 0,
                catch_type: None,
            }
        } else {
            let class_ref = pool.get_constant_mut(entry.catch_type as usize).get_class_ref_mut();
            let catch_type = Some(class_ref.resolve_class());
            Self {
                start_pc: entry.start_pc,
                end_pc: entry.end_pc,
                handler_pc: entry.handler_pc,
                catch_type,
            }
        }
    }
}

