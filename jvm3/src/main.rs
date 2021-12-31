mod cmd;
mod classpath;
mod utils;
mod classfile;

use cmd::CMD;

use classpath::classpath::ClassPath;
use crate::classfile::class_file::ClassFile;

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
    print_class_info(&class_file);
}

fn load_class(class_name:String,cp :ClassPath)->ClassFile{
    if let Some(class_data) = cp.read_class(class_name.clone()){
        ClassFile::parse(class_data)
    }else{
        panic!("class {},Not found",class_name);
    }
}

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

