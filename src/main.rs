mod rename;

use rename::parse_dir;

#[async_std::main]
async fn main() {
    parse_dir("testdata").await;
}
