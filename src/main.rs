use anyhow::Result;
use std::{env, path::PathBuf, process};

mod opt;
mod rename;
use rename::parse_dir;

gflags::define! {
    /// 文件前缀
    --prefix: &str
}

gflags::define! {
    /// 只重命名文件（保持目录不变）
    --rename-only = false
}

gflags::define! {
    /// 处理视频文件
    --include-videos = false
}

gflags::define! {
    /// 显示帮助信息
    -h, --help = false
}

gflags::define! {
    /// 显示版本号
    -v, --version = false
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let dirs = gflags::parse();

    if HELP.flag {
        gflags::print_help_and_exit(0);
    }

    if VERSION.flag {
        println!("{}-{}", env!("CARGO_PKG_VERSION"), env!("GIT_HASH"));
        process::exit(0);
    }

    let opt = opt::ParserOptions {
        prefix: PREFIX.flag.to_string(),
        rename_only: RENAME_ONLY.flag,
        videos: INCLUDE_VIDEOS.flag,
        dirs: dirs.iter().map(|s| s.to_string()).collect(),
    };

    println!("-> 文件前缀：{}", opt.prefix);
    println!("-> 只重命名文件：{}", opt.rename_only);
    println!("-> 处理视频文件: {}", opt.videos);
    println!("-> 工作目录：{:?}", opt.dirs);
    println!("----");

    let work_dir = if opt.dirs.is_empty() {
        env::current_dir()?
    } else {
        PathBuf::from(&opt.dirs[0])
    };

    parse_dir(work_dir.as_path(), &opt).await;

    Ok(())
}
