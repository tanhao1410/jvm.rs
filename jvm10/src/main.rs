#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, RwLock};
use classpath::classpath::ClassPath;
use cmd::CMD;
use rtda::heap::object::Object;
use crate::rtda::heap::class_loader::ClassLoader;
use crate::rtda::heap::string_pool::get_java_str_obj_by_pool;
use crate::rtda::slot::Slot;

mod cmd;
mod classpath;
mod utils;
mod classfile;
mod rtda;
mod instructions;
mod interpreter;
mod native;
mod constants;

fn main() {
    let mut cmd = CMD::parse();
    if cmd.version_flag {
        println!("version:{}", env!("CARGO_PKG_VERSION"));
    } else if cmd.help_flag {
        cmd.jre_option = "C:/jre1.8".to_string();
        cmd.cp_option = "C:/Users/tanhao/CLionProjects/jvm/jvm10/target/debug".to_string();
        //cmd.class = "jvmgo.book.ch08.PrintArgs".to_string();
        cmd.class = "jvmgo.book.ch10.ExceptionTest".to_string();
        //cmd.class = "jvmgo.book.ch09.InvokeVirtualTest".to_string();
        cmd.args = vec!["1".to_string()];
        cmd.inst_log_flag = false;
        println!("{}", "用法：java [options] <主类> [args...]");
        start_jvm(cmd);
    } else {
        start_jvm(cmd);
    }
}

fn start_jvm(cmd: CMD) {
    //启动类名
    let class_name = Arc::new(cmd.class.replace(".", "/"));

    let cp = ClassPath::parse(cmd.jre_option.clone(), cmd.cp_option.clone());

    let class_loader = ClassLoader::new(cp);

    let main_class = ClassLoader::load_class(class_loader.clone(), class_name.clone());

    let main_method = main_class.read().unwrap().get_main_method();

    let args_array = create_args_array(class_loader, &cmd.args);

    //注册native方法
    native::init();

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
            let java_string = get_java_str_obj_by_pool(Arc::new(args[i].to_string()));
            slots[i] = Slot::Ref(java_string);
        }
    }
    args_arr
}




