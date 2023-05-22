use std::process::Command;

pub fn call(binary_name: &str) -> bool {
    match Command::new("which").arg(binary_name).output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}
