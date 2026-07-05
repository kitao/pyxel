use super::{drop_font, ffi, new_userdata, noop_init, rc_mut, userdata, value, TP_FONT};

// Font

unsafe extern "C" fn font_new(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match pyxel::Font::new(&value::str_arg(argv, 1), value::opt_float_arg(argv, 2)) {
        Ok(font) => {
            let cls = ffi::py_totype(value::arg(argv, 0));
            new_userdata(ffi::py_retval(), cls, font);
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn font_text_width(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let font = userdata::<pyxel::RcFont>(value::arg(argv, 0));
    value::return_int(rc_mut(font).text_width(&value::str_arg(argv, 1)) as i64);
    true
}

pub(super) unsafe fn register(module: ffi::py_GlobalRef) {
    TP_FONT = value::new_type(module, c"Font", Some(drop_font));
    let font_type = ffi::py_tpobject(TP_FONT);
    ffi::py_bind(
        font_type,
        c"__new__(cls, filename, font_size=None)".as_ptr(),
        Some(font_new),
    );
    ffi::py_bind(
        font_type,
        c"__init__(self, filename, font_size=None)".as_ptr(),
        Some(noop_init),
    );
    ffi::py_bindmethod(TP_FONT, c"text_width".as_ptr(), Some(font_text_width));
}
