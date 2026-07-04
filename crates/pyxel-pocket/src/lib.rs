#![warn(clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc
)]

mod ffi;
mod module;
mod runner;
mod runtime;
mod value;
mod wrappers;

pub use runner::run_path;
pub use runtime::Runtime;

pub fn version_marker() -> &'static str {
    include_str!("../vendor/pocketpy/VERSION").trim()
}
