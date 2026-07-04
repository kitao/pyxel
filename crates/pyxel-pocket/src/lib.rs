#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

mod ffi;

pub fn version_marker() -> &'static str {
    include_str!("../vendor/pocketpy/VERSION").trim()
}
