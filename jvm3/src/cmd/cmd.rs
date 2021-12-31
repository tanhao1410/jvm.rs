use std::env;

#[derive(Debug)]
pub struct CMD {
    /// 帮助信息
    pub help_flag: bool,
    /// 版本信息
    pub version_flag: bool,
    // classpath
    pub cp_option: String,
    ///jre 环境
    pub jre_option:String,
    /// 类文件
    pub class: String,
    ///方法参数
    pub args: Vec<String>,
}

impl CMD {
    pub fn parse()->Self{
        let mut cmd = CMD{
            help_flag:false,
            version_flag:false,
            cp_option:"./".to_string(),//默认当前目录
            jre_option: "".to_string(),
            class: "".to_string(),
            args: vec![]
        };
        let envs = env::args();
        let args = envs.collect::<Vec<String>>();
        //没有参数时，显示帮助
        if args.len() == 1{
            cmd.help_flag = true;
            return cmd;
        }
        //解析-v
        if args.iter().any(|arg|arg.eq_ignore_ascii_case("-V")){
            cmd.version_flag = true;
            return cmd;
        }
        //解析-cp,-Xjre,class,args


        let mut i = 1;
        while i < args.len(){
            if args[i].eq("-cp"){
                cmd.cp_option = args[i+1].clone();
            }else if args[i].eq("-Xjre"){
                cmd.jre_option = args[i + 1].clone();
            }else{
                cmd.class = args[i].clone();
                break;
            }
            i += 2;
        }

        //args 参数
        cmd.args = args.into_iter().skip(i).collect();
        cmd
    }
}