use std::env;
use std::ffi::OsStr;
use std::path::Path;

pub fn command_args() -> Vec<String> {
    env::args().collect()
}

pub fn file_extension(filename: &str) -> String {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap()
        .to_lowercase()
}
