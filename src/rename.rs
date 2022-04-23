use futures::future;
use once_cell::sync::OnceCell;
use std::path::{Path, PathBuf};
use time::{
    format_description::{self, FormatItem},
    PrimitiveDateTime,
};
use tokio::fs;
use tracing::{debug, info, warn};

use crate::cli::Cli;

#[derive(Debug)]
pub struct DateTimeStyle {
    /// EXIF 日期格式：`[year]-[month]-[day] [hour]:[minute]:[second]`
    pub exif: Vec<FormatItem<'static>>,

    /// 归档目录日期格式：`[year][month]`
    pub folder: Vec<FormatItem<'static>>,

    /// 文件名日期格式：`[year][month][day]_[hour][minute][second]`
    pub filename: Vec<FormatItem<'static>>,
}

static DT_STYLE: OnceCell<DateTimeStyle> = OnceCell::new();

impl DateTimeStyle {
    pub fn global() -> &'static DateTimeStyle {
        DT_STYLE.get().expect("未初始化")
    }

    pub fn init() -> anyhow::Result<()> {
        let exif = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")?;
        let folder = format_description::parse("[year][month]")?;
        let filename = format_description::parse("[year][month][day]_[hour][minute][second]")?;

        let instance = DateTimeStyle {
            exif,
            folder,
            filename,
        };

        DT_STYLE.set(instance).unwrap();
        Ok(())
    }
}

fn read_dir(dir: &Path) -> Vec<PathBuf> {
    let entries = dir.read_dir().expect("not a directory");
    entries
        .flatten()
        .filter(|it| it.path().is_file())
        .map(|entry| entry.path())
        .collect()
}

async fn read_exif(file_name: &Path) -> anyhow::Result<exif::Exif> {
    let file = fs::File::open(file_name).await?.into_std().await;
    let mut buf_reader = std::io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();

    let metadata = exif_reader.read_from_container(&mut buf_reader)?;
    Ok(metadata)
}

async fn read_datetime(file_name: &Path) -> anyhow::Result<Vec<String>> {
    let exif_data = read_exif(file_name).await?;

    let datetime_fields = exif_data
        .fields()
        .filter(|it| it.tag.to_string().to_lowercase().contains("datetime"))
        .map(|it| it.display_value().to_string())
        .collect();

    Ok(datetime_fields)
}

fn gen_new_path(file_path: &Path, dt: &PrimitiveDateTime, opt: &Cli) -> anyhow::Result<PathBuf> {
    let ext = file_path
        .extension()
        .expect("不支持空白拓展名")
        .to_str()
        .unwrap();

    let basedir = file_path.parent().unwrap();

    let mut result = PathBuf::new();
    result.push(basedir);

    if !opt.rename_only {
        let archive_dir = dt.format(&(DateTimeStyle::global().folder))?;
        result.push(archive_dir);
    }

    let _dt = dt.format(&(DateTimeStyle::global().filename))?;
    let new_name = format!("{}{}.{}", opt.prefix, _dt, ext);

    result.push(new_name);

    Ok(result)
}

async fn parse_file(file_path: PathBuf, opt: &Cli) -> anyhow::Result<()> {
    let datetime_list = read_datetime(file_path.as_path()).await;

    match datetime_list {
        Ok(list) => {
            info!("处理文件：{}；拍照时间：{}", file_path.display(), list[0]);

            let dt = PrimitiveDateTime::parse(&list[0], &(DateTimeStyle::global().exif))?;
            let new_path = gen_new_path(file_path.as_path(), &dt, opt)?;
            debug!("新文件路径 = {}", new_path.as_path().display());

            if !opt.rename_only && new_path.parent().is_some() {
                let parent = new_path.parent().unwrap();
                if !parent.is_dir() {
                    debug!("创建归档目录，路径：{}", parent.display());
                    fs::create_dir_all(parent).await?;
                }
            }

            fs::rename(file_path, new_path).await.expect("无法移动文件");
        }
        Err(err) => {
            warn!("处理文件：{}；出错！{}", file_path.display(), err);
        }
    };

    Ok(())
}

pub(crate) async fn parse_dir(dir: &Path, opt: &Cli) {
    DateTimeStyle::init().expect("无法初始化 DateTimeStyle");

    info!("整理目录：{} ", dir.display());

    let renames = read_dir(dir).into_iter().map(|it| parse_file(it, opt));
    future::join_all(renames).await;

    info!("完成");
}

#[cfg(test)]
mod test {
    use std::path::Path;
    use time::{format_description, PrimitiveDateTime};

    use super::*;
    use crate::cli::Cli;

    #[tokio::test]
    async fn 获取_exif_数据() {
        let file_path = Path::new("testdata/1.jpg");
        let exif_data = read_exif(file_path).await;

        assert!(exif_data.is_ok())
    }

    #[tokio::test]
    async fn 获取_exif_数据_非法文件场景() {
        let file_path = Path::new("testdata/error_image.jpg");
        let exif_data = read_exif(file_path).await;

        assert!(exif_data.is_err())
    }

    #[tokio::test]
    async fn 从文件中获取时间日期数据() {
        let file_path = Path::new("testdata/1.jpg");
        let date_time_fields = read_datetime(file_path).await;

        assert!(date_time_fields.is_ok());

        let _date_time_fields = date_time_fields.unwrap();
        assert!(!_date_time_fields.is_empty());
    }

    #[test]
    fn 从时间日期数据创建新的文件名() {
        DateTimeStyle::init().expect("无法初始化 DateTimeStyle");

        let file_path = Path::new("testdata/1.jpg");

        let datetime_format =
            format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();

        let dt = PrimitiveDateTime::parse("2015-11-16 20:07:54", &datetime_format);
        println!("dt: {:?}", dt);
        assert!(dt.is_ok());

        let opt = Cli {
            rename_only: false,
            dirs: Vec::new(),
            prefix: String::new(),
            videos: false,
        };
        let new_path = gen_new_path(file_path, &dt.unwrap(), &opt);

        assert!(new_path.is_ok());

        let _new_path = new_path.unwrap();
        let __new_path = _new_path.as_path();

        println!("new_path: {:?}", __new_path);
        assert_eq!(__new_path, Path::new("testdata/201511/20151116_200754.jpg"));
    }

    #[test]
    fn 获取目录的文件列表() {
        let dir = Path::new("testdata");
        let files = read_dir(dir);
        assert!(!files.is_empty())
    }
}
