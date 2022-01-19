#[macro_use]
extern crate lazy_static;
use crate::jvm::jvm::JVM;
use crate::cmd::CMD;
mod cmd;
mod classpath;
mod utils;
mod classfile;
mod rtda;
mod instructions;
mod interpreter;
mod native;
mod constants;
mod jvm;

fn main() {
    let cmd = CMD::parse();
    if cmd.version_flag {
        println!("version:{}", env!("CARGO_PKG_VERSION"));
    } else if cmd.help_flag {
        println!("{}", "用法：java [options] <主类> [args...]");
    } else {
        let jvm = JVM::new(cmd);
        jvm.start();
    }
}






