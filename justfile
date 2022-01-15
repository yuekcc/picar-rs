build:
    cargo build

release:
    cargo build --release
    test -e target/release/picar-rs.exe && strip -s target/release/picar-rs.exe
    ls -ahl target/release

push:
    git push
