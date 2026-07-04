use crate::{ffi, value};

unsafe extern "C" fn btn(_argc: i32, argv: ffi::py_StackRef) -> bool {
    value::return_bool(pyxel::pyxel().is_button_down(value::int_arg(argv, 0) as pyxel::Key));
    true
}

unsafe extern "C" fn btnp(_argc: i32, argv: ffi::py_StackRef) -> bool {
    value::return_bool(pyxel::pyxel().is_button_pressed(
        value::int_arg(argv, 0) as pyxel::Key,
        value::opt_int_arg(argv, 1).map(|v| v as u32),
        value::opt_int_arg(argv, 2).map(|v| v as u32),
    ));
    true
}

unsafe extern "C" fn btnr(_argc: i32, argv: ffi::py_StackRef) -> bool {
    value::return_bool(pyxel::pyxel().is_button_released(value::int_arg(argv, 0) as pyxel::Key));
    true
}

unsafe extern "C" fn btnv(_argc: i32, argv: ffi::py_StackRef) -> bool {
    value::return_int(pyxel::pyxel().button_value(value::int_arg(argv, 0) as pyxel::Key) as i64);
    true
}

unsafe extern "C" fn mouse(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_mouse_visible(value::bool_arg(argv, 0));
    value::return_none();
    true
}

unsafe extern "C" fn set_btn(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_button_state(
        value::int_arg(argv, 0) as pyxel::Key,
        value::bool_arg(argv, 1),
    );
    value::return_none();
    true
}

unsafe extern "C" fn set_btnv(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_button_value(
        value::int_arg(argv, 0) as pyxel::Key,
        value::int_arg(argv, 1) as pyxel::KeyValue,
    );
    value::return_none();
    true
}

unsafe extern "C" fn set_mouse_pos(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(x) = value::float_arg(argv, 0) else {
        return false;
    };
    let Some(y) = value::float_arg(argv, 1) else {
        return false;
    };
    pyxel::pyxel().set_mouse_position(x, y);
    crate::module::sync_variables();
    value::return_none();
    true
}

unsafe extern "C" fn set_input_text(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_input_text(&value::str_arg(argv, 0));
    crate::module::sync_variables();
    value::return_none();
    true
}

unsafe extern "C" fn set_dropped_files(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let files = value::str_list_arg(argv, 0);
    let file_refs = files.iter().map(String::as_str).collect::<Vec<_>>();
    pyxel::pyxel().set_dropped_files(&file_refs);
    crate::module::sync_variables();
    value::return_none();
    true
}

pub unsafe fn add_functions(module: ffi::py_GlobalRef) {
    ffi::py_bind(module, c"btn(key)".as_ptr(), Some(btn));
    ffi::py_bind(
        module,
        c"btnp(key, hold=None, repeat=None)".as_ptr(),
        Some(btnp),
    );
    ffi::py_bind(module, c"btnr(key)".as_ptr(), Some(btnr));
    ffi::py_bind(module, c"btnv(key)".as_ptr(), Some(btnv));
    ffi::py_bind(module, c"mouse(visible)".as_ptr(), Some(mouse));
    ffi::py_bind(module, c"set_btn(key, state)".as_ptr(), Some(set_btn));
    ffi::py_bind(module, c"set_btnv(key, val)".as_ptr(), Some(set_btnv));
    ffi::py_bind(module, c"set_mouse_pos(x, y)".as_ptr(), Some(set_mouse_pos));
    ffi::py_bind(
        module,
        c"set_input_text(text)".as_ptr(),
        Some(set_input_text),
    );
    ffi::py_bind(
        module,
        c"set_dropped_files(files)".as_ptr(),
        Some(set_dropped_files),
    );
}
