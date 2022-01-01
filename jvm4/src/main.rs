mod cmd;
mod classpath;
mod utils;
mod classfile;
mod rtda;

use cmd::CMD;

use classpath::classpath::ClassPath;
use crate::classfile::class_file::ClassFile;
use crate::rtda::local_vars::LocalVars;
use crate::rtda::frame::Frame;
use std::sync::{Arc, RwLock};
use crate::rtda::object::Object;
use crate::rtda::operand_stack::OperandStack;

fn main() {
    let mut cmd = CMD::parse();
    if cmd.version_flag{
        println!("0.0.1");
    }else if cmd.help_flag{
        cmd.jre_option = "C:/jre1.8".to_string();
        cmd.cp_option = "C:/Users/tanhao/CLionProjects/jvm/jvm2/target/debug/tools".to_string();
        cmd.class = "java.lang.String".to_string();
        println!("{}","用法：java [options] <主类> [args...]");
        start_jvm(cmd);
    }else{
        start_jvm(cmd);
    }
}

fn start_jvm(cmd:CMD){

    println!("jre:{},classpath:{},class:{},args:{:?}",cmd.jre_option, cmd.cp_option,cmd.class,cmd.args);

    //启动类名
    let class_name = cmd.class.replace(".","/");

    let cp = ClassPath::parse(cmd.jre_option.clone(), cmd.cp_option.clone());

    let class_file = load_class(class_name,cp);
    //print_class_info(&class_file);
    let mut frame = Frame::new(100, 100);
    test_local_vars(frame.local_vars());
    test_operand_stack(frame.operand_stack())
}

fn load_class(class_name:String,cp :ClassPath)->ClassFile{
    if let Some(class_data) = cp.read_class(class_name.clone()){
        ClassFile::parse(class_data)
    }else{
        panic!("class {},Not found",class_name);
    }
}

///第四章运行时数据区测试-局部变量表
fn test_local_vars(vars : &mut LocalVars){
    vars.set_i32(0,100);
    vars.set_i32(1,-100);
    vars.set_i64(2,2997924580);
    vars.set_i64(4,-2997924580);
    vars.set_f32(6,3.1415926);
    vars.set_f64(7,2.718281987);
    vars.set_ref(9,Arc::new(RwLock::new(Object{val:888})));

    println!("{}",vars.get_i32(0));
    println!("{}",vars.get_i32(1));
    println!("{}",vars.get_i64(2));
    println!("{}",vars.get_i64(4));
    println!("{}",vars.get_f32(6));
    println!("{}",vars.get_f64(7));
    println!("{}",vars.get_ref(9).read().unwrap().val);

}

///第四章运行时数据区测试-局部变量表
fn test_operand_stack(stack : &mut OperandStack){
    stack.push_i32(100);
    stack.push_i32(-100);
    stack.push_i64(2997924580);
    stack.push_i64(-2997924580);
    stack.push_f32(3.1415926);
    stack.push_f64(2.718281987);
    stack.push_ref(Arc::new(RwLock::new(Object{val:888})));

    println!("{}",stack.pop_ref().read().unwrap().val);
    println!("{}",stack.pop_f64());
    println!("{}",stack.pop_f32());
    println!("{}",stack.pop_i64());
    println!("{}",stack.pop_i64());
    println!("{}",stack.pop_i32());
    println!("{}",stack.pop_i32());

}


/// 第三章加载类信息测试
fn print_class_info(class_file: &ClassFile) {
   println!("version: {}.{}", class_file.major_version(), class_file.minor_version());
   println!("constants count: {}", class_file.constant_pool().read().unwrap().constants_count());
   println!("access flags: 0x{:x}", class_file.access_flags());
   println!("this class: {}", class_file.class_name());
   println!("super class: {}", class_file.super_class_name());
   println!("intrefaces: {:?}", class_file.interface_names());
   let fields = class_file.fields();
   println!("fields count: {}", fields.len());
   for field in fields {
       println!("    {}", field.name());
   }
   let methods = class_file.methods();
   println!("methods count: {}", methods.len());
   for method in methods {
       println!("    {}{}", method.name(), method.descriptor());
   }
}

