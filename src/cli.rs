use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version = env!("APP_VERSION"))]
pub(crate) struct Cli {
    /// 文件前缀
    #[clap(short = 'p', long = "prefix", default_value = "")]
    pub prefix: String,

    /// 只重命名文件
    #[clap(long = "rename-only")]
    pub rename_only: bool,

    /// 处理视频文件
    #[clap(long = "videos")]
    pub videos: bool,

    /// 目标目录
    #[clap()]
    pub dirs: Vec<String>,
}
