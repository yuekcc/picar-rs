use std::process::Command;

fn main() {
    let git_output = Command::new("git")
        .args(&["describe", "--always"])
        .output()
        .unwrap();
    let version = String::from_utf8(git_output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", version);
}
