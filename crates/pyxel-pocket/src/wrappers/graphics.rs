use crate::{ffi, value};

unsafe extern "C" fn cls(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().clear(value::int_arg(argv, 0) as pyxel::Color);
    value::return_none();
    true
}

unsafe extern "C" fn pset(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(x) = value::float_arg(argv, 0) else {
        return false;
    };
    let Some(y) = value::float_arg(argv, 1) else {
        return false;
    };
    pyxel::pyxel().set_pixel(x, y, value::int_arg(argv, 2) as pyxel::Color);
    value::return_none();
    true
}

unsafe extern "C" fn line(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(x1) = value::float_arg(argv, 0) else {
        return false;
    };
    let Some(y1) = value::float_arg(argv, 1) else {
        return false;
    };
    let Some(x2) = value::float_arg(argv, 2) else {
        return false;
    };
    let Some(y2) = value::float_arg(argv, 3) else {
        return false;
    };
    pyxel::pyxel().draw_line(x1, y1, x2, y2, value::int_arg(argv, 4) as pyxel::Color);
    value::return_none();
    true
}

unsafe extern "C" fn rect(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(x) = value::float_arg(argv, 0) else {
        return false;
    };
    let Some(y) = value::float_arg(argv, 1) else {
        return false;
    };
    let Some(width) = value::float_arg(argv, 2) else {
        return false;
    };
    let Some(height) = value::float_arg(argv, 3) else {
        return false;
    };
    pyxel::pyxel().draw_rect(x, y, width, height, value::int_arg(argv, 4) as pyxel::Color);
    value::return_none();
    true
}

unsafe extern "C" fn rectb(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(x) = value::float_arg(argv, 0) else {
        return false;
    };
    let Some(y) = value::float_arg(argv, 1) else {
        return false;
    };
    let Some(width) = value::float_arg(argv, 2) else {
        return false;
    };
    let Some(height) = value::float_arg(argv, 3) else {
        return false;
    };
    pyxel::pyxel().draw_rect_border(x, y, width, height, value::int_arg(argv, 4) as pyxel::Color);
    value::return_none();
    true
}

unsafe extern "C" fn text(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(x) = value::float_arg(argv, 0) else {
        return false;
    };
    let Some(y) = value::float_arg(argv, 1) else {
        return false;
    };
    let text = value::opt_str_arg(argv, 2).unwrap_or_default();
    pyxel::pyxel().draw_text(x, y, &text, value::int_arg(argv, 3) as pyxel::Color, None);
    value::return_none();
    true
}

pub unsafe fn add_functions(module: ffi::py_GlobalRef) {
    ffi::py_bind(module, c"cls(col)".as_ptr(), Some(cls));
    ffi::py_bind(module, c"pset(x, y, col)".as_ptr(), Some(pset));
    ffi::py_bind(module, c"line(x1, y1, x2, y2, col)".as_ptr(), Some(line));
    ffi::py_bind(module, c"rect(x, y, w, h, col)".as_ptr(), Some(rect));
    ffi::py_bind(module, c"rectb(x, y, w, h, col)".as_ptr(), Some(rectb));
    ffi::py_bind(module, c"text(x, y, s, col)".as_ptr(), Some(text));
}
