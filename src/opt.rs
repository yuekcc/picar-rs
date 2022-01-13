#[derive(PartialEq, Debug)]
pub struct ParserOptions {
    /// 文件前缀
    pub prefix: String,

    /// 只重命名文件
    pub rename_only: bool,

    /// 处理视频文件
    pub videos: bool,

    /// 目标目录
    pub dirs: Vec<String>,
}
