use anyhow::Result;
use chrono::NaiveDateTime;
use futures::future;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn read_dir(dir: &Path) -> Vec<PathBuf> {
    let entries = dir.read_dir().expect("not a directory");
    entries
        .flatten()
        .filter(|it| it.path().is_file())
        .map(|entry| entry.path())
        .collect()
}

fn read_exif(file_name: &Path) -> Result<exif::Exif> {
    let file = std::fs::File::open(file_name)?;
    let mut buf_reader = std::io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();

    let metadata = exif_reader.read_from_container(&mut buf_reader)?;
    Ok(metadata)
}

fn read_datetime(file_name: &Path) -> Result<Vec<String>> {
    let exif_data = read_exif(file_name)?;

    let datetime_fields = exif_data
        .fields()
        .filter(|it| it.tag.to_string().to_lowercase().contains("datetime"))
        .map(|it| it.display_value().to_string())
        .collect();

    Ok(datetime_fields)
}

fn gen_new_path(file_path: &Path, dt: &NaiveDateTime) -> Result<PathBuf> {
    let file_name = file_path
        .file_name()
        .expect("无法获取文件名")
        .to_str()
        .unwrap();
    let basedir = file_path.parent().unwrap().display().to_string();

    let archive_dir = dt.format("%Y%m").to_string();

    let mut new_path = PathBuf::new();
    new_path.push(basedir);
    new_path.push(archive_dir);
    new_path.push(file_name);

    Ok(new_path)
}

async fn parse_file(file_path: PathBuf) -> Result<()> {
    let datetime_list = read_datetime(file_path.as_path());

    match datetime_list {
        Ok(list) => {
            println!(
                "\t处理文件：{}；获取拍照时间：{}",
                file_path.display(),
                list[0]
            );

            let dt = NaiveDateTime::parse_from_str(&list[0], "%Y-%m-%d %H:%M:%S");
            let new_path = gen_new_path(file_path.as_path(), &dt.unwrap()).unwrap();

            new_path.parent().map(fs::create_dir_all);

            fs::rename(file_path, new_path).expect("无法移动文件");
        }
        Err(err) => {
            println!("\t处理文件：{}；出错！{}", file_path.display(), err);
        }
    };

    Ok(())
}

pub async fn parse_dir(dir_name: &str) {
    let dir = Path::new(dir_name);
    println!("整理目录：{} 里文件：", dir.display());

    let jobs = read_dir(dir).into_iter().map(parse_file);

    future::join_all(jobs).await;
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use chrono::NaiveDateTime;

    use super::{gen_new_path, read_datetime, read_dir, read_exif};

    #[test]
    fn 获取_exif_数据() {
        let file_path = Path::new("testdata/1.jpg");
        let exif_data = read_exif(file_path);

        assert!(exif_data.is_ok())
    }

    #[test]
    fn 获取_exif_数据_非法文件场景() {
        let file_path = Path::new("testdata/error_image.jpg");
        let exif_data = read_exif(file_path);

        assert!(exif_data.is_err())
    }

    #[test]
    fn 从文件中获取时间日期数据() {
        let file_path = Path::new("testdata/1.jpg");
        let date_time_fields = read_datetime(file_path);

        assert!(date_time_fields.is_ok());

        let _date_time_fields = date_time_fields.unwrap();
        assert!(!_date_time_fields.is_empty());

        // println!("date_time_fields is {:?}", _date_time_fields);
    }

    #[test]
    fn 从时间日期数据创建新的文件名() {
        let file_path = Path::new("testdata/1.jpg");
        // let date_time_fields = get_datetime_from_file(file_path);

        // let datetime_str = &date_time_fields.unwrap()[0];
        // println!("datetime_str: '{}'", datetime_str);

        let dt = NaiveDateTime::parse_from_str("2015-11-16 20:07:54", "%Y-%m-%d %H:%M:%S");
        println!("dt: {:?}", dt);
        assert!(dt.is_ok());

        let new_path = gen_new_path(file_path, &dt.unwrap());
        assert!(new_path.is_ok());

        let _new_path = new_path.unwrap();
        let __new_path = _new_path.as_path();

        println!("new_path: {:?}", __new_path);
        assert_eq!(__new_path, Path::new("testdata/201511/1.jpg"));
    }

    #[test]
    fn 获取目录的文件列表() {
        let dir = Path::new("testdata");
        let files = read_dir(dir);
        assert!(!files.is_empty())
    }
}
