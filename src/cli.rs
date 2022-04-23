use std::fmt::Display;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version = env!("APP_VERSION"))]
pub(crate) struct Cli {
    /// 文件前缀
    #[clap(long = "prefix")]
    pub prefix: Option<String>,

    /// 只重命名文件
    #[clap(long = "rename-only")]
    pub rename_only: bool,

    /// 处理视频文件
    #[clap(long = "videos")]
    pub videos: bool,

    /// 工作目录，未设置时将使用当前目录
    #[clap()]
    pub dirs: Vec<String>,
}

impl Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "文件前缀：{:?}，", self.prefix)?;
        write!(f, "只重命名文件：{}，", self.rename_only)?;
        write!(f, "处理视频文件：{}，", self.videos)?;
        write!(f, "工作目录：{:?}", self.dirs)?;

        Ok(())
    }
}
