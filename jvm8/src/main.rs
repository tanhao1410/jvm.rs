mod cmd;
mod classpath;
mod utils;
mod classfile;
mod rtda;
mod instructions;
mod interpreter;
#[macro_use]
extern crate lazy_static;

use cmd::CMD;
use classpath::classpath::ClassPath;
use crate::classfile::class_file::ClassFile;
use crate::rtda::local_vars::LocalVars;
use crate::rtda::frame::Frame;
use std::sync::{Arc, RwLock};
use rtda::heap::object::Object;
use crate::rtda::operand_stack::OperandStack;
use crate::rtda::thread::Thread;
use crate::rtda::heap::method::Method;
use crate::rtda::heap::class_loader::ClassLoader;
use std::cell::RefCell;
use std::rc::Rc;
use crate::rtda::heap::string_pool::get_java_string;
use crate::rtda::slot::Slot;

fn main() {
    let mut cmd = CMD::parse();
    if cmd.version_flag {
        println!("0.0.1");
    } else if cmd.help_flag {
        cmd.jre_option = "C:/jre1.8".to_string();
        cmd.cp_option = "C:/Users/tanhao/CLionProjects/jvm/jvm8/target/debug".to_string();
        //cmd.class = "jvmgo.book.ch07.FibonacciTest".to_string();
        //cmd.class = "jvmgo.book.ch07.LReturn".to_string();
        //cmd.class = "jvmgo.book.ch08.BubbleSortTest".to_string();
        //cmd.class = "jvmgo.book.ch08.MutliArrayTest".to_string();
        cmd.class = "jvmgo.book.ch08.PrintArgs".to_string();
        cmd.args = vec!["foo".to_string(),"bar".to_string(),"你好！".to_string()];
        cmd.inst_log_flag = false;
        println!("{}", "用法：java [options] <主类> [args...]");
        start_jvm(cmd);
    } else {
        start_jvm(cmd);
    }
}

fn start_jvm(cmd: CMD) {
    //println!("jre:{},classpath:{},class:{},args:{:?}", cmd.jre_option, cmd.cp_option, cmd.class, cmd.args);

    //启动类名
    let class_name = Arc::new(cmd.class.replace(".", "/"));

    let cp = ClassPath::parse(cmd.jre_option.clone(), cmd.cp_option.clone());

    let class_loader = Arc::new(RwLock::new(ClassLoader::new(cp)));

    let main_class = ClassLoader::load_class(class_loader.clone(), class_name.clone());

    let main_method = main_class.read().unwrap().get_main_method();

    let args_array = create_args_array(class_loader, &cmd.args);

    if let Some(main_method) = main_method{
        //测试解释器
        interpreter::interpret(main_method,cmd.inst_log_flag,args_array);
    }else{
        panic!("Main method not found in class {}",class_name);
    }

}

fn create_args_array(loader:Arc<RwLock<ClassLoader>>,args:&Vec<String>)->Arc<RwLock<Object>>{
    let string_class = ClassLoader::load_class(loader.clone(),Arc::new("java/lang/String".to_string()));
    let args_arr = Object::new_array(string_class, args.len());
    //需要把该数组中的元素替换成java中的字符串
    {
        let mut guard = args_arr.write().unwrap();
        let slots =  &mut guard.refs_mut().slots;
        for i in 0..args.len(){
            let java_string = get_java_string(Arc::new(args[i].to_string()), loader.clone());
            slots[i] = Slot::Ref(java_string);
        }
    }
    args_arr
}




