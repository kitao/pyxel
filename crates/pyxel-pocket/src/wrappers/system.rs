use crate::{ffi, value};

unsafe extern "C" fn pyxel_init(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let width = value::int_arg(argv, 0) as u32;
    let height = value::int_arg(argv, 1) as u32;
    let title = value::opt_str_arg(argv, 2);
    let fps = value::opt_int_arg(argv, 3).map(|v| v as u32);
    let quit_key = value::opt_int_arg(argv, 4).map(|v| v as pyxel::Key);
    let display_scale = value::opt_int_arg(argv, 5).map(|v| v as u32);
    let capture_scale = value::opt_int_arg(argv, 6).map(|v| v as u32);
    let capture_sec = value::opt_int_arg(argv, 7).map(|v| v as u32);
    let headless = value::opt_bool_arg(argv, 8);

    pyxel::init(
        width,
        height,
        title.as_deref(),
        fps,
        quit_key,
        display_scale,
        capture_scale,
        capture_sec,
        headless,
    );
    crate::module::sync_variables();
    value::return_none();
    true
}

pub unsafe fn add_functions(module: ffi::py_GlobalRef) {
    ffi::py_bind(
        module,
        c"init(width, height, title=None, fps=None, quit_key=None, display_scale=None, capture_scale=None, capture_sec=None, headless=None)".as_ptr(),
        Some(pyxel_init),
    );
}
