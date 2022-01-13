build:
    cargo build

release:
    cargo build --release
    ls -ahl target/release

push:
    git push
