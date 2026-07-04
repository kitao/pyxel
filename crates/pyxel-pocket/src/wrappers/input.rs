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

pub unsafe fn add_functions(module: ffi::py_GlobalRef) {
    ffi::py_bind(module, c"btn(key)".as_ptr(), Some(btn));
    ffi::py_bind(
        module,
        c"btnp(key, hold=None, repeat=None)".as_ptr(),
        Some(btnp),
    );
    ffi::py_bind(module, c"btnr(key)".as_ptr(), Some(btnr));
}
