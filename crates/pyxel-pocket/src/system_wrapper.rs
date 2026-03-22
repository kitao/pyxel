use std::process;

use crate::ffi;
use crate::helpers::*;
use crate::channel_wrapper::TP_CHANNELS;
use crate::image_wrapper::TP_IMAGES;
use crate::music_wrapper::TP_MUSICS;
use crate::sound_wrapper::TP_SOUNDS;
use crate::tilemap_wrapper::TP_TILEMAPS;
use crate::tone_wrapper::TP_TONES;
use crate::variable_wrapper::{set_screen_objects, TP_COLORS};

static mut NAME_UPDATE: ffi::py_Name = std::ptr::null_mut();
static mut NAME_DRAW: ffi::py_Name = std::ptr::null_mut();

unsafe extern "C" fn pyxel_init(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let w = arg_int(argv, 0) as u32;
    let h = arg_int(argv, 1) as u32;
    let title = arg_opt_str(argv, 2);
    let fps = arg_opt_int(argv, 3).map(|v| v as u32);
    let quit_key = arg_opt_int(argv, 4).map(|v| v as u32);
    let display_scale = arg_opt_int(argv, 5).map(|v| v as u32);
    let capture_scale = arg_opt_int(argv, 6).map(|v| v as u32);
    let capture_sec = arg_opt_int(argv, 7).map(|v| v as u32);
    let headless = arg_opt_bool(argv, 8);
    pyxel::init(
        w, h, title, fps, quit_key, display_scale, capture_scale, capture_sec, headless,
    );

    // Set static module attributes and collection singletons
    let m = ffi::py_getmodule(c"pyxel".as_ptr());
    set_module_int(m, c"width", w as i64);
    set_module_int(m, c"height", h as i64);
    ffi::py_newobject(
        ffi::py_emplacedict(m, ffi::py_name(c"images".as_ptr())),
        TP_IMAGES, 0, 0,
    );
    ffi::py_newobject(
        ffi::py_emplacedict(m, ffi::py_name(c"sounds".as_ptr())),
        TP_SOUNDS, 0, 0,
    );
    ffi::py_newobject(
        ffi::py_emplacedict(m, ffi::py_name(c"tilemaps".as_ptr())),
        TP_TILEMAPS, 0, 0,
    );
    ffi::py_newobject(
        ffi::py_emplacedict(m, ffi::py_name(c"musics".as_ptr())),
        TP_MUSICS, 0, 0,
    );
    ffi::py_newobject(
        ffi::py_emplacedict(m, ffi::py_name(c"channels".as_ptr())),
        TP_CHANNELS, 0, 0,
    );
    ffi::py_newobject(
        ffi::py_emplacedict(m, ffi::py_name(c"tones".as_ptr())),
        TP_TONES, 0, 0,
    );
    ffi::py_newobject(
        ffi::py_emplacedict(m, ffi::py_name(c"colors".as_ptr())),
        TP_COLORS, 0, 0,
    );
    set_screen_objects(m);
    sync_module_vars();
    ret_none();
    true
}

unsafe extern "C" fn pyxel_run(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let module = ffi::py_getmodule(c"pyxel".as_ptr());
    NAME_UPDATE = ffi::py_name(c"_update".as_ptr());
    NAME_DRAW = ffi::py_name(c"_draw".as_ptr());
    ffi::py_setdict(module, NAME_UPDATE, arg(argv, 0));
    ffi::py_setdict(module, NAME_DRAW, arg(argv, 1));

    struct Callback;
    impl pyxel::PyxelCallback for Callback {
        fn update(&mut self, _pyxel: &mut pyxel::Pyxel) {
            unsafe {
                sync_module_vars();
                call_py_func(NAME_UPDATE);
            }
        }
        fn draw(&mut self, _pyxel: &mut pyxel::Pyxel) {
            unsafe {
                sync_module_vars();
                call_py_func(NAME_DRAW);
            }
        }
    }

    pyxel::pyxel().run(Callback);
    ret_none();
    true
}

unsafe extern "C" fn pyxel_show(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().show_screen();
    ret_none();
    true
}

unsafe extern "C" fn pyxel_flip(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().flip_screen();
    sync_module_vars();
    ret_none();
    true
}

unsafe extern "C" fn pyxel_quit(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().quit();
    process::exit(0);
}

unsafe extern "C" fn pyxel_reset(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().restart();
    ret_none();
    true
}

unsafe extern "C" fn pyxel_title(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_title(arg_str(argv, 0));
    ret_none();
    true
}

unsafe extern "C" fn pyxel_icon(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let data = arg_str_list(argv, 0);
    let data_refs: Vec<&str> = data.iter().map(String::as_str).collect();
    let scale = arg_int(argv, 1) as u32;
    let colkey = arg_opt_int(argv, 2).map(|v| v as u8);
    pyxel::pyxel().set_icon(&data_refs, scale, colkey);
    ret_none();
    true
}

unsafe extern "C" fn pyxel_perf_monitor(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_perf_monitor(ffi::py_tobool(arg(argv, 0)));
    ret_none();
    true
}

unsafe extern "C" fn pyxel_integer_scale(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_integer_scale(ffi::py_tobool(arg(argv, 0)));
    ret_none();
    true
}

unsafe extern "C" fn pyxel_screen_mode(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_screen_mode(arg_int(argv, 0) as u32);
    ret_none();
    true
}

unsafe extern "C" fn pyxel_fullscreen(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_fullscreen(ffi::py_tobool(arg(argv, 0)));
    ret_none();
    true
}

unsafe extern "C" fn pyxel_pid_exists(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let _pid = arg_int(argv, 0) as u32;
    // PocketPy build does not include sysinfo; always return false
    ret_bool(false);
    true
}

unsafe extern "C" fn pyxel_reset_statics(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    pyxel::reset_statics();
    ret_none();
    true
}

pub unsafe fn add_system_functions(m: ffi::py_GlobalRef) {
    bind(m, c"init(width, height, title=None, fps=None, quit_key=None, display_scale=None, capture_scale=None, capture_sec=None, headless=None)", Some(pyxel_init));
    bind(m, c"run(update, draw)", Some(pyxel_run));
    bindfunc(m, c"show", Some(pyxel_show));
    bindfunc(m, c"flip", Some(pyxel_flip));
    bindfunc(m, c"quit", Some(pyxel_quit));
    bindfunc(m, c"reset", Some(pyxel_reset));
    bind(m, c"title(title)", Some(pyxel_title));
    bind(m, c"icon(data, scale, colkey=None)", Some(pyxel_icon));
    bind(m, c"perf_monitor(enabled)", Some(pyxel_perf_monitor));
    bind(m, c"integer_scale(enabled)", Some(pyxel_integer_scale));
    bind(m, c"screen_mode(scr)", Some(pyxel_screen_mode));
    bind(m, c"fullscreen(enabled)", Some(pyxel_fullscreen));
    bindfunc(m, c"_reset_statics", Some(pyxel_reset_statics));
    bind(m, c"_pid_exists(pid)", Some(pyxel_pid_exists));
}
