mod rename;

use rename::parse_dir;

fn main() {
    parse_dir("testdata").unwrap();
}
