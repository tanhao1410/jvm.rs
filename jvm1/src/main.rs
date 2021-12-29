mod cmd;
use structopt::StructOpt;
use cmd::CMD;
fn main() {
    let cmd:CMD = CMD::from_args();
    if cmd.version_flag{
        println!("0.0.1");
    }else if cmd.class.is_some(){
        start_jvm(cmd);
    }else{
        println!("{}",cmd::HELP_MESSAGE);
    }
}

fn start_jvm(cmd:CMD){
    println!("classpath:{},class:{},args:{:?}"
             ,cmd.cp_option.unwrap_or("./".to_string())
             ,cmd.class.unwrap()
             ,cmd.args);
}
