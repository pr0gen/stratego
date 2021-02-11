use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn write_into_file(file_name: &str, to_write: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", to_write) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
