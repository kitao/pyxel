// Bindgen emits C-style PocketPy names and unused declarations that mirror the C
// header.
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
