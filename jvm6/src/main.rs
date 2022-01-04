mod cmd;
mod classpath;
mod utils;
mod classfile;
mod rtda;
mod instructions;
mod interpreter;

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

fn main() {
    let mut cmd = CMD::parse();
    if cmd.version_flag {
        println!("0.0.1");
    } else if cmd.help_flag {
        cmd.jre_option = "C:/jre1.8".to_string();
        cmd.cp_option = "C:/Users/tanhao/CLionProjects/jvm/jvm6/target/debug".to_string();
        cmd.class = "jvmgo.book.ch06.MyObject".to_string();
        println!("{}", "用法：java [options] <主类> [args...]");
        start_jvm(cmd);
    } else {
        start_jvm(cmd);
    }
}

fn start_jvm(cmd: CMD) {
    println!("jre:{},classpath:{},class:{},args:{:?}", cmd.jre_option, cmd.cp_option, cmd.class, cmd.args);

    //启动类名
    let class_name = Arc::new(cmd.class.replace(".", "/"));


    let cp = ClassPath::parse(cmd.jre_option.clone(), cmd.cp_option.clone());

    let class_loader = Arc::new(RwLock::new(ClassLoader::new(cp)));
    let main_class = ClassLoader::load_class(class_loader, class_name.clone());
    let main_method = main_class.read().unwrap().get_main_method();
    if let Some(main_method) = main_method{
        //测试解释器
        interpreter::interpret(main_method);
    }else{
        panic!("Main method not found in class {}",class_name);
    }

}


