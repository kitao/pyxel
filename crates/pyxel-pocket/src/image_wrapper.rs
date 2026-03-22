use crate::ffi;
use crate::helpers::*;

pub static mut TP_IMAGE: ffi::py_Type = 0;
pub static mut TP_IMAGES: ffi::py_Type = 0;

pub unsafe fn image_ptr(r: ffi::py_Ref) -> *mut pyxel::Image {
    *(ffi::py_touserdata(r) as *mut *mut pyxel::Image)
}

pub unsafe fn new_image_obj(out: ffi::py_OutRef, ptr: *mut pyxel::Image) {
    let ud = ffi::py_newobject(out, TP_IMAGE, 0, size_of::<*mut pyxel::Image>() as i32);
    *(ud as *mut *mut pyxel::Image) = ptr;
}

// Helper to get &mut Image from argv[0] (self)
macro_rules! img {
    ($argv:expr) => {
        &mut *image_ptr(arg($argv, 0))
    };
}

unsafe extern "C" fn image_width(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_int(img!(argv).width() as i64);
    true
}

unsafe extern "C" fn image_height(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_int(img!(argv).height() as i64);
    true
}

unsafe extern "C" fn image_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let data = arg_str_list(argv, 3);
    let data_refs: Vec<&str> = data.iter().map(String::as_str).collect();
    img!(argv).set(arg_float(argv, 1) as i32, arg_float(argv, 2) as i32, &data_refs);
    ret_none();
    true
}

unsafe extern "C" fn image_load(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if let Err(e) = img!(argv).load(
        arg_float(argv, 1) as i32,
        arg_float(argv, 2) as i32,
        arg_str(argv, 3),
        arg_opt_bool(argv, 4),
    ) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

unsafe extern "C" fn image_save(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if let Err(e) = img!(argv).save(arg_str(argv, 1), arg_float(argv, 2) as u32) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

unsafe extern "C" fn image_clip(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if arg_is_none(argv, 1) {
        img!(argv).reset_clip_rect();
    } else {
        img!(argv).set_clip_rect(
            arg_float(argv, 1) as f32, arg_float(argv, 2) as f32,
            arg_float(argv, 3) as f32, arg_float(argv, 4) as f32,
        );
    }
    ret_none();
    true
}

unsafe extern "C" fn image_camera(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if arg_is_none(argv, 1) {
        img!(argv).reset_draw_offset();
    } else {
        img!(argv).set_draw_offset(arg_float(argv, 1) as f32, arg_float(argv, 2) as f32);
    }
    ret_none();
    true
}

unsafe extern "C" fn image_pal(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if arg_is_none(argv, 1) {
        img!(argv).reset_color_map();
    } else {
        img!(argv).map_color(arg_int(argv, 1) as u8, arg_int(argv, 2) as u8);
    }
    ret_none();
    true
}

unsafe extern "C" fn image_dither(_argc: i32, argv: ffi::py_StackRef) -> bool {
    img!(argv).set_dithering(arg_float(argv, 1) as f32);
    ret_none();
    true
}

unsafe extern "C" fn image_cls(_argc: i32, argv: ffi::py_StackRef) -> bool {
    img!(argv).clear(arg_int(argv, 1) as u8);
    ret_none();
    true
}

unsafe extern "C" fn image_pget(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_int(img!(argv).get_pixel(arg_float(argv, 1) as f32, arg_float(argv, 2) as f32) as i64);
    true
}

unsafe extern "C" fn image_pset(_argc: i32, argv: ffi::py_StackRef) -> bool {
    img!(argv).set_pixel(
        arg_float(argv, 1) as f32, arg_float(argv, 2) as f32, arg_int(argv, 3) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn image_line(_argc: i32, argv: ffi::py_StackRef) -> bool {
    img!(argv).draw_line(
        arg_float(argv, 1) as f32, arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32, arg_float(argv, 4) as f32,
        arg_int(argv, 5) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn image_rect(_argc: i32, argv: ffi::py_StackRef) -> bool {
    img!(argv).draw_rect(
        arg_float(argv, 1) as f32, arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32, arg_float(argv, 4) as f32,
        arg_int(argv, 5) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn image_rectb(_argc: i32, argv: ffi::py_StackRef) -> bool {
    img!(argv).draw_rect_border(
        arg_float(argv, 1) as f32, arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32, arg_float(argv, 4) as f32,
        arg_int(argv, 5) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn image_circ(_argc: i32, argv: ffi::py_StackRef) -> bool {
    img!(argv).draw_circle(
        arg_float(argv, 1) as f32, arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32, arg_int(argv, 4) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn image_circb(_argc: i32, argv: ffi::py_StackRef) -> bool {
    img!(argv).draw_circle_border(
        arg_float(argv, 1) as f32, arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32, arg_int(argv, 4) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn image_elli(_argc: i32, argv: ffi::py_StackRef) -> bool {
    img!(argv).draw_ellipse(
        arg_float(argv, 1) as f32, arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32, arg_float(argv, 4) as f32,
        arg_int(argv, 5) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn image_ellib(_argc: i32, argv: ffi::py_StackRef) -> bool {
    img!(argv).draw_ellipse_border(
        arg_float(argv, 1) as f32, arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32, arg_float(argv, 4) as f32,
        arg_int(argv, 5) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn image_tri(_argc: i32, argv: ffi::py_StackRef) -> bool {
    img!(argv).draw_triangle(
        arg_float(argv, 1) as f32, arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32, arg_float(argv, 4) as f32,
        arg_float(argv, 5) as f32, arg_float(argv, 6) as f32,
        arg_int(argv, 7) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn image_trib(_argc: i32, argv: ffi::py_StackRef) -> bool {
    img!(argv).draw_triangle_border(
        arg_float(argv, 1) as f32, arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32, arg_float(argv, 4) as f32,
        arg_float(argv, 5) as f32, arg_float(argv, 6) as f32,
        arg_int(argv, 7) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn image_fill(_argc: i32, argv: ffi::py_StackRef) -> bool {
    img!(argv).flood_fill(
        arg_float(argv, 1) as f32, arg_float(argv, 2) as f32, arg_int(argv, 3) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn image_blt(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let img_ref = arg(argv, 3);
    let src_img = if ffi::py_isinstance(img_ref, TP_IMAGE) {
        image_ptr(img_ref)
    } else {
        pyxel::images()[ffi::py_toint(img_ref) as usize]
    };
    img!(argv).draw_image(
        arg_float(argv, 1) as f32, arg_float(argv, 2) as f32,
        src_img,
        arg_float(argv, 4) as f32, arg_float(argv, 5) as f32,
        arg_float(argv, 6) as f32, arg_float(argv, 7) as f32,
        arg_opt_int(argv, 8).map(|v| v as u8),
        arg_opt_float(argv, 9).map(|v| v as f32),
        arg_opt_float(argv, 10).map(|v| v as f32),
    );
    ret_none();
    true
}

unsafe extern "C" fn image_text(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let font = if arg_is_none(argv, 5) {
        None
    } else {
        Some(*(ffi::py_touserdata(arg(argv, 5)) as *mut *mut pyxel::Font))
    };
    img!(argv).draw_text(
        arg_float(argv, 1) as f32, arg_float(argv, 2) as f32,
        arg_str(argv, 3), arg_int(argv, 4) as u8, font,
    );
    ret_none();
    true
}

unsafe extern "C" fn image_bltm(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tm_ref = arg(argv, 3);
    let tm_ptr = if is_int(tm_ref) {
        pyxel::tilemaps()[ffi::py_toint(tm_ref) as usize]
    } else {
        *(ffi::py_touserdata(tm_ref) as *mut *mut pyxel::Tilemap)
    };
    img!(argv).draw_tilemap(
        arg_float(argv, 1) as f32, arg_float(argv, 2) as f32,
        tm_ptr,
        arg_float(argv, 4) as f32, arg_float(argv, 5) as f32,
        arg_float(argv, 6) as f32, arg_float(argv, 7) as f32,
        arg_opt_int(argv, 8).map(|v| v as u8),
        arg_opt_float(argv, 9).map(|v| v as f32),
        arg_opt_float(argv, 10).map(|v| v as f32),
    );
    ret_none();
    true
}

unsafe extern "C" fn image_new(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let cls = ffi::py_totype(arg(argv, 0));
    let w = arg_int(argv, 1) as u32;
    let h = arg_int(argv, 2) as u32;
    let ptr = pyxel::Image::new(w, h);
    let ud = ffi::py_newobject(ffi::py_retval(), cls, 0, size_of::<*mut pyxel::Image>() as i32);
    *(ud as *mut *mut pyxel::Image) = ptr;
    true
}

unsafe extern "C" fn image_from_image(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let filename = arg_str(argv, 0);
    let include_colors = arg_opt_bool(argv, 1);
    match pyxel::Image::from_image(filename, include_colors) {
        Ok(ptr) => {
            new_image_obj(ffi::py_retval(), ptr);
            true
        }
        Err(e) => raise_exc(&e),
    }
}

pub unsafe fn add_image_class(m: ffi::py_GlobalRef) {
    TP_IMAGE = new_type(c"Image", m);

    ffi::py_bindproperty(TP_IMAGE, c"width".as_ptr(), Some(image_width), None);
    ffi::py_bindproperty(TP_IMAGE, c"height".as_ptr(), Some(image_height), None);

    ffi::py_bindmethod(TP_IMAGE, c"set".as_ptr(), Some(image_set));
    ffi::py_bindmethod(TP_IMAGE, c"save".as_ptr(), Some(image_save));
    ffi::py_bindmethod(TP_IMAGE, c"cls".as_ptr(), Some(image_cls));
    ffi::py_bindmethod(TP_IMAGE, c"pget".as_ptr(), Some(image_pget));
    ffi::py_bindmethod(TP_IMAGE, c"pset".as_ptr(), Some(image_pset));
    ffi::py_bindmethod(TP_IMAGE, c"line".as_ptr(), Some(image_line));
    ffi::py_bindmethod(TP_IMAGE, c"rect".as_ptr(), Some(image_rect));
    ffi::py_bindmethod(TP_IMAGE, c"rectb".as_ptr(), Some(image_rectb));
    ffi::py_bindmethod(TP_IMAGE, c"circ".as_ptr(), Some(image_circ));
    ffi::py_bindmethod(TP_IMAGE, c"circb".as_ptr(), Some(image_circb));
    ffi::py_bindmethod(TP_IMAGE, c"elli".as_ptr(), Some(image_elli));
    ffi::py_bindmethod(TP_IMAGE, c"ellib".as_ptr(), Some(image_ellib));
    ffi::py_bindmethod(TP_IMAGE, c"tri".as_ptr(), Some(image_tri));
    ffi::py_bindmethod(TP_IMAGE, c"trib".as_ptr(), Some(image_trib));
    ffi::py_bindmethod(TP_IMAGE, c"fill".as_ptr(), Some(image_fill));
    // text uses py_bind for optional font parameter

    let tp_obj = ffi::py_tpobject(TP_IMAGE);
    bind(tp_obj, c"load(self, x, y, filename, include_colors=None)", Some(image_load));
    bind(tp_obj, c"clip(self, x=None, y=None, w=None, h=None)", Some(image_clip));
    bind(tp_obj, c"camera(self, x=None, y=None)", Some(image_camera));
    bind(tp_obj, c"pal(self, col1=None, col2=None)", Some(image_pal));
    bind(tp_obj, c"dither(self, alpha)", Some(image_dither));
    bind(tp_obj, c"blt(self, x, y, img, u, v, w, h, colkey=None, rotate=None, scale=None)", Some(image_blt));
    bind(tp_obj, c"bltm(self, x, y, tm, u, v, w, h, colkey=None, rotate=None, scale=None)", Some(image_bltm));
    bind(tp_obj, c"text(self, x, y, s, col, font=None)", Some(image_text));
    bind(tp_obj, c"__new__(cls, width, height)", Some(image_new));
    bind(tp_obj, c"from_image(filename, include_colors=None)", Some(image_from_image));

    // Images collection
    impl_object_collection!(pyxel::images, new_image_obj, images_getitem, images_setitem, images_len, images_iter);
    register_collection!(TP_IMAGES, c"Images", m, images_getitem, images_setitem, images_len, images_iter);
}
