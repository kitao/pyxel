use crate::ffi;
use crate::helpers::*;

unsafe extern "C" fn pyxel_btn(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_bool(pyxel::pyxel().is_button_down(arg_int(argv, 0) as u32));
    true
}

unsafe extern "C" fn pyxel_btnp(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let key = arg_int(argv, 0) as u32;
    let hold = arg_opt_int(argv, 1).map(|v| v as u32);
    let repeat = arg_opt_int(argv, 2).map(|v| v as u32);
    ret_bool(pyxel::pyxel().is_button_pressed(key, hold, repeat));
    true
}

unsafe extern "C" fn pyxel_btnr(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_bool(pyxel::pyxel().is_button_released(arg_int(argv, 0) as u32));
    true
}

unsafe extern "C" fn pyxel_btnv(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_int(pyxel::pyxel().button_value(arg_int(argv, 0) as u32) as i64);
    true
}

unsafe extern "C" fn pyxel_mouse(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_mouse_visible(ffi::py_tobool(arg(argv, 0)));
    ret_none();
    true
}

unsafe extern "C" fn pyxel_warp_mouse(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_mouse_position(arg_float(argv, 0) as f32, arg_float(argv, 1) as f32);
    ret_none();
    true
}

pub unsafe fn add_input_functions(m: ffi::py_GlobalRef) {
    bind(m, c"btn(key)", Some(pyxel_btn));
    bind(m, c"btnp(key, hold=None, repeat=None)", Some(pyxel_btnp));
    bind(m, c"btnr(key)", Some(pyxel_btnr));
    bind(m, c"btnv(key)", Some(pyxel_btnv));
    bind(m, c"mouse(visible)", Some(pyxel_mouse));
    bind(m, c"warp_mouse(x, y)", Some(pyxel_warp_mouse));
}
