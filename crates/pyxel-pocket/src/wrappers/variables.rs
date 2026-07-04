use crate::{ffi, value};

pub unsafe fn add_constants(module: ffi::py_GlobalRef) {
    value::set_const_int(module, "KEY_NONE", pyxel::KEY_NONE as i64);
    value::set_const_int(module, "KEY_Q", pyxel::KEY_Q as i64);
    value::set_const_int(module, "COLOR_BLACK", pyxel::COLOR_BLACK as i64);
    value::set_const_int(module, "COLOR_WHITE", pyxel::COLOR_WHITE as i64);
    value::set_const_int(module, "COLOR_RED", pyxel::COLOR_RED as i64);
}

pub unsafe fn sync(module: ffi::py_GlobalRef) {
    value::set_module_int(module, c"width", *pyxel::width() as i64);
    value::set_module_int(module, c"height", *pyxel::height() as i64);
    value::set_module_int(module, c"frame_count", *pyxel::frame_count() as i64);
    value::set_module_int(module, c"mouse_x", *pyxel::mouse_x() as i64);
    value::set_module_int(module, c"mouse_y", *pyxel::mouse_y() as i64);
}
