use clap::Parser;
use std::{env, path::PathBuf};
use tracing::info;
use tracing_subscriber;

mod cli;
mod rename;
use crate::{cli::Cli, rename::parse_dir};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    info!("启动参数：{}", cli);
    info!("----");

    let work_dirs = if cli.dirs.is_empty() {
        vec![env::current_dir()?]
    } else {
        cli.dirs.iter().map(PathBuf::from).collect()
    };

    work_dirs.iter().for_each(|dir| {
        parse_dir(&dir, &cli);
    });

    Ok(())
}
