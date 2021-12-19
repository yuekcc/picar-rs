use anyhow::Result;
use futures::future;
use std::path::{Path, PathBuf};

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

fn get_datetime_from_file(file_name: &Path) -> Result<Vec<String>> {
    let exif_data = read_exif(file_name)?;

    let date_time_fields = exif_data
        .fields()
        .filter(|it| it.tag.to_string().to_lowercase().contains("datetime"))
        .map(|it| it.display_value().to_string())
        .collect();

    Ok(date_time_fields)
}

async fn parse_file(file_path: PathBuf) -> Result<()> {
    let datetime_list = get_datetime_from_file(file_path.as_path());

    match datetime_list {
        Ok(list) => {
            println!(
                "\t处理文件：{}；获取拍照时间：{}",
                file_path.display(),
                list[0]
            );
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

    use super::{get_datetime_from_file, read_dir, read_exif};

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
        let date_time_fields = get_datetime_from_file(file_path);

        assert!(date_time_fields.is_ok());

        let _date_time_fields = date_time_fields.unwrap();
        assert!(!_date_time_fields.is_empty());

        // println!("date_time_fields is {:?}", _date_time_fields);
    }

    #[test]
    fn 获取目录的文件列表() {
        let dir = Path::new("testdata");
        let files = read_dir(dir);
        assert!(!files.is_empty())
    }
}
