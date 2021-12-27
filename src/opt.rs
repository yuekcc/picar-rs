use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
    /// 文件前缀
    #[structopt(long)]
    pub prefix: String,

    /// 只重命名文件
    #[structopt(long)]
    pub rename_only: bool,

    /// 处理视频文件
    #[structopt(long)]
    pub videos: bool,

    /// 目标目录
    #[structopt(name = "dir", parse(from_os_str))]
    pub dirs: Vec<PathBuf>,
}
