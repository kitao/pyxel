#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::float_cmp,
    clippy::fn_params_excessive_bools,
    clippy::items_after_statements,
    clippy::match_same_arms,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::range_plus_one,
    clippy::redundant_closure,
    clippy::significant_drop_tightening,
    clippy::suboptimal_flops,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::unused_self,
    clippy::wildcard_imports,
    clippy::zero_ptr
)]

#[macro_use]
pub mod utils;
pub mod audio;
pub mod blipbuf;
pub mod canvas;
pub mod channel;
pub mod graphics;
pub mod image;
pub mod input;
pub mod math;
pub mod music;
pub mod oscillator;
pub mod prelude;
pub mod profiler;
pub mod pyxel;
pub mod rectarea;
pub mod resource;
pub mod screencast;
pub mod settings;
pub mod sound;
pub mod system;
pub mod tilemap;

pub use pyxel_platform::keys;
