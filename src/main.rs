use anyhow::Result;
use clap::Parser;
use std::{env, path::PathBuf};
use tracing::info;
use tracing_subscriber;

mod cli;
mod rename;
use crate::{cli::Cli, rename::parse_dir};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    info!("-> 文件前缀：{}", cli.prefix);
    info!("-> 只重命名文件：{}", cli.rename_only);
    info!("-> 处理视频文件: {}", cli.videos);
    info!("-> 工作目录：{:?}", cli.dirs);
    info!("----");

    let work_dir = if cli.dirs.is_empty() {
        env::current_dir()?
    } else {
        PathBuf::from(&cli.dirs[0])
    };

    parse_dir(&work_dir, &cli);
    Ok(())
}
