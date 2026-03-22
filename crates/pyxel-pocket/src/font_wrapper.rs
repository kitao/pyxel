use crate::ffi;
use crate::helpers::*;

pub static mut TP_FONT: ffi::py_Type = 0;

unsafe fn fnt(argv: ffi::py_StackRef) -> &'static mut pyxel::Font {
    &mut *(*(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Font))
}

unsafe extern "C" fn font_text_width(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_int(fnt(argv).text_width(arg_str(argv, 1)) as i64);
    true
}

unsafe extern "C" fn font_new(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let cls = ffi::py_totype(arg(argv, 0));
    let filename = arg_str(argv, 1);
    let font_size = arg_opt_float(argv, 2).map(|v| v as f32);
    match pyxel::Font::new(filename, font_size) {
        Ok(ptr) => {
            let ud =
                ffi::py_newobject(ffi::py_retval(), cls, 0, size_of::<*mut pyxel::Font>() as i32);
            *(ud as *mut *mut pyxel::Font) = ptr;
            true
        }
        Err(e) => raise_exc(&e),
    }
}

pub unsafe fn add_font_class(m: ffi::py_GlobalRef) {
    TP_FONT = new_type(c"Font", m);
    ffi::py_bindmethod(TP_FONT, c"text_width".as_ptr(), Some(font_text_width));
    let tp_obj = ffi::py_tpobject(TP_FONT);
    bind(tp_obj, c"__new__(cls, filename, font_size=None)", Some(font_new));
}
