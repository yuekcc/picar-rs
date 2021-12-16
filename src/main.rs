use std::path::Path;

use anyhow::Result;

fn main() {
    parse_dir("testdata").unwrap();
}

fn parse_dir(dir: &str) -> Result<()> {
    let path = Path::new(dir);
    let entries = path.read_dir().expect("not a directory");

    for entry in entries.flatten() {
        let file_path = entry.path();

        if file_path.is_file() {
            println!("parsing file: {}", file_path.display());
            show_exif_from_file(file_path.as_path())
                .unwrap_or_else(|err| panic!("parse file: {} , {}", file_path.display(), err));
        } else {
            println!("{} is a dir, ignore", file_path.display());
        }
    }

    Ok(())
}

fn show_exif_from_file(name: &Path) -> Result<()> {
    println!("\nshow exif of file: {}\n----", name.display());

    let file = std::fs::File::open(name)?;
    let mut buf_reader = std::io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();

    let exif = exif_reader
        .read_from_container(&mut buf_reader)
        .unwrap_or_else(|err| {
            panic!("read exif data error, {}", err);
        });

    for f in exif.fields() {
        if f.tag.to_string().to_lowercase().contains("datetime") {
            println!(
                "\tTag: {}; IFD Number: {}; Value: {}",
                f.tag,
                f.ifd_num,
                f.display_value().with_unit(&exif)
            );
        }
    }

    Ok(())
}
