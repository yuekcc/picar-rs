use anyhow::Result;
use structopt::StructOpt;
use std::env;

mod rename;
mod opt;

use rename::parse_dir;
use opt::Opt;

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    println!("-> 文件前缀：{}", opt.prefix);
    println!("-> 只重命名文件：{}", opt.rename_only);
    println!("-> 处理视频文件: {}", opt.videos);
    println!("-> 工作目录：{:?}", opt.dirs);
    println!("----");

    let work_dir = if opt.dirs.is_empty() {
        env::current_dir()?
    } else {
        opt.dirs[0].clone()
    };

    parse_dir(work_dir.as_path(), &opt).await;

    Ok(())
}
