mod rename;

use std::{env, path::PathBuf};

use rename::parse_dir;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// 文件前缀
    #[structopt(long)]
    prefix: String,

    /// 只重命名文件
    #[structopt(long)]
    rename_only: bool,

    /// 处理视频文件
    #[structopt(long)]
    videos: bool,

    /// 目标目录
    #[structopt(name = "dir", parse(from_os_str))]
    dirs: Vec<PathBuf>,
}

#[async_std::main]
async fn main() {
    let opt = Opt::from_args();

    println!("-> prefix: {}", opt.prefix);
    println!("-> rename only: {}", opt.rename_only);
    println!("-> parse video files: {}", opt.videos);
    println!("-> dir name: {:?}", opt.dirs);

    let target_dir = if opt.dirs.is_empty() {
        env::current_dir().unwrap()
    } else {
        opt.dirs[0].clone()
    };

    parse_dir(target_dir.as_path()).await;
}
