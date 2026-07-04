use crate::{ffi, value};

struct PocketPyCallback;

impl pyxel::PyxelCallback for PocketPyCallback {
    fn update(&mut self, _pyxel: &mut pyxel::Pyxel) {
        unsafe {
            let module = ffi::py_getmodule(c"pyxel".as_ptr());
            crate::module::sync_variables();
            value::call_module_function(module, c"_update");
        }
    }

    fn draw(&mut self, _pyxel: &mut pyxel::Pyxel) {
        unsafe {
            let module = ffi::py_getmodule(c"pyxel".as_ptr());
            crate::module::sync_variables();
            value::call_module_function(module, c"_draw");
        }
    }
}

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

unsafe extern "C" fn pyxel_run(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let module = ffi::py_getmodule(c"pyxel".as_ptr());
    value::set_module_value(module, c"_update", value::arg(argv, 0));
    value::set_module_value(module, c"_draw", value::arg(argv, 1));

    pyxel::pyxel().run(PocketPyCallback);
    value::return_none();
    true
}

unsafe extern "C" fn pyxel_show(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().show_screen();
    value::return_none();
    true
}

unsafe extern "C" fn pyxel_flip(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().flip_screen();
    crate::module::sync_variables();
    value::return_none();
    true
}

unsafe extern "C" fn pyxel_quit(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().quit();
    value::return_none();
    true
}

unsafe extern "C" fn pyxel_reset(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().restart();
    value::return_none();
    true
}

unsafe extern "C" fn pyxel_title(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_title(&value::str_arg(argv, 0));
    value::return_none();
    true
}

unsafe extern "C" fn pyxel_icon(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let data = value::str_list_arg(argv, 0);
    let data_refs = data.iter().map(String::as_str).collect::<Vec<_>>();
    pyxel::pyxel().set_icon(
        &data_refs,
        value::int_arg(argv, 1) as u32,
        value::opt_int_arg(argv, 2).map(|value| value as pyxel::Color),
    );
    value::return_none();
    true
}

unsafe extern "C" fn pyxel_perf_monitor(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_perf_monitor(value::bool_arg(argv, 0));
    value::return_none();
    true
}

unsafe extern "C" fn pyxel_integer_scale(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_integer_scale(value::bool_arg(argv, 0));
    value::return_none();
    true
}

unsafe extern "C" fn pyxel_screen_mode(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_screen_mode(value::int_arg(argv, 0) as u32);
    value::return_none();
    true
}

unsafe extern "C" fn pyxel_fullscreen(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_fullscreen(value::bool_arg(argv, 0));
    value::return_none();
    true
}

unsafe extern "C" fn pyxel_resize(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let width = value::int_arg(argv, 0);
    let height = value::int_arg(argv, 1);
    if width <= 0 || height <= 0 {
        return value::raise_value_error("width and height must be greater than 0");
    }
    pyxel::pyxel().set_screen_size(width as u32, height as u32);
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
    ffi::py_bind(module, c"run(update, draw)".as_ptr(), Some(pyxel_run));
    ffi::py_bind(module, c"show()".as_ptr(), Some(pyxel_show));
    ffi::py_bind(module, c"flip()".as_ptr(), Some(pyxel_flip));
    ffi::py_bind(module, c"quit()".as_ptr(), Some(pyxel_quit));
    ffi::py_bind(module, c"reset()".as_ptr(), Some(pyxel_reset));
    ffi::py_bind(module, c"title(title)".as_ptr(), Some(pyxel_title));
    ffi::py_bind(
        module,
        c"icon(data, scale, colkey=None)".as_ptr(),
        Some(pyxel_icon),
    );
    ffi::py_bind(
        module,
        c"perf_monitor(enabled)".as_ptr(),
        Some(pyxel_perf_monitor),
    );
    ffi::py_bind(
        module,
        c"integer_scale(enabled)".as_ptr(),
        Some(pyxel_integer_scale),
    );
    ffi::py_bind(
        module,
        c"screen_mode(scr)".as_ptr(),
        Some(pyxel_screen_mode),
    );
    ffi::py_bind(
        module,
        c"fullscreen(enabled)".as_ptr(),
        Some(pyxel_fullscreen),
    );
    ffi::py_bind(
        module,
        c"resize(width, height)".as_ptr(),
        Some(pyxel_resize),
    );
}
