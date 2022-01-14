use crate::native::registry::register;
use crate::rtda::slot::{Slots, Slot};
use crate::rtda::local_vars::LocalVars;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use crate::rtda::heap::array_datas::ArrayDatas;
use crate::rtda::heap::object::Object;
use crate::utils::string_utils::get_java_string;
use crate::rtda::heap::class::Class;
use crate::rtda::thread::Thread;


pub fn init() {
    register("java/lang/Throwable",
             "fillInStackTrace",
             "(I)Ljava/lang/Throwable;"
             , fill_in_stack_trace);
}

/// 用来记录java虚拟机栈信息
struct StackTraceElement {
    file_name: Arc<String>,
    class_name: Arc<String>,
    method_name: Arc<String>,
    line_number: i32,
}

impl StackTraceElement {
    pub fn to_java_string(&self) -> Arc<RwLock<Object>> {
        let mut msg = "\tat ".to_string();
        msg.push_str(self.class_name.as_str());
        msg.push('.');
        msg.push_str(self.method_name.as_str());
        msg.push('(');
        msg.push_str(self.file_name.as_str());
        msg.push(':');
        msg.push_str(self.line_number.to_string().as_str());
        get_java_string(msg.replace("/","."))
    }
}


fn fill_in_stack_trace(local_vars: &LocalVars,thread:RwLockWriteGuard<Thread>) -> Option<Slots> {
    let this = local_vars.get_slot(0);
    let res = match this {
        Slot::Ref(this) => {
            {
                let this = this.clone();
                let stack_infos = create_stack_trace_elements(this.clone(),thread);
                let mut this_guard = this.write().unwrap();
                this_guard.datas = stack_infos;
            }

            Slots::from_one_slot(Slot::Ref(this.clone()))
        }
        _ => unreachable!()
    };
    Some(res)
}

fn create_stack_trace_elements(obj: Arc<RwLock<Object>>,thread:RwLockWriteGuard<Thread>) -> ArrayDatas {

    let skip = distance_to_object(obj.read().unwrap().class.clone()) + 2;

    let frames = thread.stack.frames.clone();
    let size = frames.len() - skip;

    let frames = &frames[..size];

    //应该是遍历frame
    let slots = frames
        .iter()
        .map(|frame| {
            let frame = frame.borrow();
            let method = frame.method.clone();
            let class = method.class.clone();
            let class = class.read().unwrap();
            let info = StackTraceElement {
                file_name: class.source_file.clone(),
                class_name: class.name.clone(),
                method_name: method.name.clone(),
                line_number: method.get_line_number(frame.next_pc() - 1),
            };
            Slot::Ref(info.to_java_string())
        })
        .collect::<Vec<Slot>>();

    ArrayDatas::Refs(Slots { slots })
}

fn distance_to_object(class : Arc<RwLock<Class>>)-> usize{
    let mut distance = 0;
    let class = class.read().unwrap();
    if class.super_class.is_some(){
        distance += 1;
        distance += distance_to_object(class.super_class.as_ref().unwrap().clone());
    }
    distance
}