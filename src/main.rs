use anyhow::Result;

fn main() {
    let paths = &["testdata/1.jpg", "testdata/2.jpg", "testdata/3.jpg"];

    for path in paths {
        show_exif_from_file(path).unwrap();
    }
}

fn show_exif_from_file(name: &str) -> Result<()> {
    println!("\nshow exif of file: {}\n----", name);

    let file = std::fs::File::open(name)?;
    let mut buf_reader = std::io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();

    let exif = exif_reader.read_from_container(&mut buf_reader)?;

    for f in exif.fields() {
        println!(
            "\tTag: {}; IFD Number: {}; Value: {}",
            f.tag,
            f.ifd_num,
            f.display_value().with_unit(&exif)
        );
    }

    Ok(())
}
