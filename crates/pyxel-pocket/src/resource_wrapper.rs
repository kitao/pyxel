use crate::ffi;
use crate::helpers::*;

unsafe extern "C" fn pyxel_load(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let filename = arg_str(argv, 0);
    let exclude_images = arg_opt_bool(argv, 1);
    let exclude_tilemaps = arg_opt_bool(argv, 2);
    let exclude_sounds = arg_opt_bool(argv, 3);
    let exclude_musics = arg_opt_bool(argv, 4);
    if let Err(e) = pyxel::pyxel().load_resource(
        filename,
        exclude_images,
        exclude_tilemaps,
        exclude_sounds,
        exclude_musics,
    ) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

unsafe extern "C" fn pyxel_save(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let filename = arg_str(argv, 0);
    let exclude_images = arg_opt_bool(argv, 1);
    let exclude_tilemaps = arg_opt_bool(argv, 2);
    let exclude_sounds = arg_opt_bool(argv, 3);
    let exclude_musics = arg_opt_bool(argv, 4);
    if let Err(e) = pyxel::pyxel().save_resource(
        filename,
        exclude_images,
        exclude_tilemaps,
        exclude_sounds,
        exclude_musics,
    ) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

unsafe extern "C" fn pyxel_load_pal(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if let Err(e) = pyxel::pyxel().load_palette(arg_str(argv, 0)) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

unsafe extern "C" fn pyxel_save_pal(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if let Err(e) = pyxel::pyxel().save_palette(arg_str(argv, 0)) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

unsafe extern "C" fn pyxel_screenshot(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let filename = arg_opt_str(argv, 0);
    let scale = arg_opt_int(argv, 1).map(|v| v as u32);
    if let Err(e) = pyxel::pyxel().take_screenshot(filename, scale) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

unsafe extern "C" fn pyxel_screencast(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let filename = arg_opt_str(argv, 0);
    let scale = arg_opt_int(argv, 1).map(|v| v as u32);
    if let Err(e) = pyxel::pyxel().save_screencast(filename, scale) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

unsafe extern "C" fn pyxel_reset_screencast(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().reset_screencast();
    ret_none();
    true
}

unsafe extern "C" fn pyxel_user_data_dir(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match pyxel::pyxel().user_data_dir(arg_str(argv, 0), arg_str(argv, 1)) {
        Ok(dir) => {
            ret_str(&dir);
            true
        }
        Err(e) => raise_exc(&e),
    }
}

// Internal helper for saving screen to a specific path
unsafe extern "C" fn pyxel_save_screen(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let filename = arg_str(argv, 0);
    let scale = arg_opt_int(argv, 1).map(|v| v as u32).unwrap_or(1);
    if let Err(e) = pyxel::screen().save(filename, scale) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

pub unsafe fn add_resource_functions(m: ffi::py_GlobalRef) {
    bind(m, c"load(filename, exclude_images=None, exclude_tilemaps=None, exclude_sounds=None, exclude_musics=None)", Some(pyxel_load));
    bind(m, c"save(filename, exclude_images=None, exclude_tilemaps=None, exclude_sounds=None, exclude_musics=None)", Some(pyxel_save));
    bind(m, c"load_pal(filename)", Some(pyxel_load_pal));
    bind(m, c"save_pal(filename)", Some(pyxel_save_pal));
    bind(
        m,
        c"screenshot(filename=None, scale=None)",
        Some(pyxel_screenshot),
    );
    bind(
        m,
        c"screencast(filename=None, scale=None)",
        Some(pyxel_screencast),
    );
    bindfunc(m, c"reset_screencast", Some(pyxel_reset_screencast));
    bind(
        m,
        c"user_data_dir(vendor_name, app_name)",
        Some(pyxel_user_data_dir),
    );
    bind(
        m,
        c"_save_screen(filename, scale=None)",
        Some(pyxel_save_screen),
    );
}
