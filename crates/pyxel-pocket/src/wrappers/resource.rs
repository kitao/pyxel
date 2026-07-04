use crate::{ffi, value};

unsafe extern "C" fn load(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match pyxel::pyxel().load_resource(
        &value::str_arg(argv, 0),
        value::opt_bool_arg(argv, 1),
        value::opt_bool_arg(argv, 2),
        value::opt_bool_arg(argv, 3),
        value::opt_bool_arg(argv, 4),
    ) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn save(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match pyxel::pyxel().save_resource(
        &value::str_arg(argv, 0),
        value::opt_bool_arg(argv, 1),
        value::opt_bool_arg(argv, 2),
        value::opt_bool_arg(argv, 3),
        value::opt_bool_arg(argv, 4),
    ) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn load_pal(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match pyxel::pyxel().load_palette(&value::str_arg(argv, 0)) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn save_pal(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match pyxel::pyxel().save_palette(&value::str_arg(argv, 0)) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn screenshot(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match pyxel::pyxel().save_screenshot(
        value::opt_str_arg(argv, 0).as_deref(),
        value::opt_int_arg(argv, 1).map(|scale| scale as u32),
    ) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn screencast(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match pyxel::pyxel().save_screencast(
        value::opt_str_arg(argv, 0).as_deref(),
        value::opt_int_arg(argv, 1).map(|scale| scale as u32),
    ) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn reset_screencast(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().reset_screencast();
    value::return_none();
    true
}

unsafe extern "C" fn user_data_dir(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match pyxel::pyxel().user_data_dir(&value::str_arg(argv, 0), &value::str_arg(argv, 1)) {
        Ok(path) => {
            value::return_str(&path);
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

pub unsafe fn add_functions(module: ffi::py_GlobalRef) {
    ffi::py_bind(
        module,
        c"load(filename, exclude_images=None, exclude_tilemaps=None, exclude_sounds=None, exclude_musics=None)".as_ptr(),
        Some(load),
    );
    ffi::py_bind(
        module,
        c"save(filename, exclude_images=None, exclude_tilemaps=None, exclude_sounds=None, exclude_musics=None)".as_ptr(),
        Some(save),
    );
    ffi::py_bind(module, c"load_pal(filename)".as_ptr(), Some(load_pal));
    ffi::py_bind(module, c"save_pal(filename)".as_ptr(), Some(save_pal));
    ffi::py_bind(
        module,
        c"screenshot(filename=None, scale=None)".as_ptr(),
        Some(screenshot),
    );
    ffi::py_bind(
        module,
        c"screencast(filename=None, scale=None)".as_ptr(),
        Some(screencast),
    );
    ffi::py_bind(
        module,
        c"reset_screencast()".as_ptr(),
        Some(reset_screencast),
    );
    ffi::py_bind(
        module,
        c"user_data_dir(vendor_name, app_name)".as_ptr(),
        Some(user_data_dir),
    );
}
