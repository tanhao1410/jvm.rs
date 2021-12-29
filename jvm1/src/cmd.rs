use structopt::StructOpt;

pub const HELP_MESSAGE : &str = "用法：java [options] <主类> [args...]";


#[derive(Debug, StructOpt)]
pub struct CMD {
    /// 帮助信息
    #[structopt(short="h", long="help")]
    pub help_flag: bool,
    /// 版本信息
    #[structopt(short="v", long="version")]
    pub version_flag: bool,
    #[structopt(short,long)]
    pub cp_option: Option<String>,
    /// 类文件
    #[structopt()]
    pub class: Option<String>,
    ///方法参数
    #[structopt(required = false)]
    pub args: Vec<String>,
}