use std::process::Command;

fn main() {
    let status = Command::new("python3")
        .arg("check_lib.py")
        .status()
        .expect("Failed to run check_lib.py");

    if !status.success() {
        panic!("Library check failed. See error above.");
    }
}
