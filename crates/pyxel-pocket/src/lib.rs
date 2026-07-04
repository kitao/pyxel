#![warn(clippy::pedantic)]
// Relax pedantic lints inherent to mirroring the PocketPy C ABI and Python API:
// numeric casts cross FFI/Python integer domains, raw-pointer borrows feed C
// out-parameters, and generated wrapper surfaces share Python-style signatures.
#![allow(
    clippy::borrow_as_ptr,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::similar_names,
    clippy::too_many_arguments,
    clippy::too_many_lines
)]

mod ffi;
mod module;
mod runner;
mod runtime;
mod value;
#[cfg(target_os = "emscripten")]
mod web;
mod wrappers;

pub use runner::run_path;
pub use runtime::Runtime;

#[must_use]
pub fn version_marker() -> &'static str {
    include_str!("../vendor/pocketpy/VERSION").trim()
}
