mod cmd;
mod classpath;
mod utils;

use cmd::CMD;

use std::path::Path;
use classpath::classpath::ClassPath;

fn main() {
    let mut cmd = CMD::parse();
    if cmd.version_flag{
        println!("0.0.1");
    }else if cmd.help_flag{
        cmd.jre_option = "C:/jre1.8".to_string();
        cmd.cp_option = "C:/Users/tanhao/CLionProjects/jvm/jvm2/target/debug/tools".to_string();
        cmd.class = "Main".to_string();
        println!("{}","用法：java [options] <主类> [args...]");
        start_jvm(cmd);
    }else{
        start_jvm(cmd);
    }
}

fn start_jvm(cmd:CMD){
    println!("classpath:{},class:{},args:{:?}"
             ,cmd.cp_option
             ,cmd.class
             ,cmd.args);

    //启动类名
    let class_name = cmd.class.replace(".","/");

    //cp.read_class()
    let cp = ClassPath::parse(cmd.jre_option.clone(), cmd.cp_option.clone());
    if let Some(content) =  cp.read_class(class_name){
        println!("{:?}",content);
    }
}

