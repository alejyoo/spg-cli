use std::io::Write;
use std::process::{Command, Stdio};

pub fn clip_copy(text: &str) {
    match Command::new("clip.exe").stdin(Stdio::piped()).spawn() {
        Ok(mut process) => {
            if let Some(input) = process.stdin.as_mut() {
                let _ = input.write_all(text.as_bytes());
            }
        }
        Err(_) => {}
    }
}
