use super::{
    color_arg, drop_image, farg, ffi, font_from_ref, image_arg, make_image, new_userdata,
    noop_init, rc_mut, rc_ref, str_list_from_arg, tilemap_arg, userdata, value, TP_IMAGE,
};

// Image

unsafe extern "C" fn image_new(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let cls = ffi::py_totype(value::arg(argv, 0));
    new_userdata(
        ffi::py_retval(),
        cls,
        pyxel::Image::new(
            value::int_arg(argv, 1) as u32,
            value::int_arg(argv, 2) as u32,
        ),
    );
    true
}

unsafe extern "C" fn image_from_image(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match pyxel::Image::from_image(&value::str_arg(argv, 0), value::opt_bool_arg(argv, 1)) {
        Ok(image) => {
            make_image(ffi::py_retval(), image);
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn image_width(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    value::return_int(rc_ref(image).width() as i64);
    true
}

unsafe extern "C" fn image_height(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    value::return_int(rc_ref(image).height() as i64);
    true
}

unsafe extern "C" fn image_data_ptr(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    value::return_int(rc_mut(image).data_ptr() as usize as i64);
    true
}

unsafe extern "C" fn image_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    let data = str_list_from_arg(argv, 3);
    let data_refs = data.iter().map(String::as_str).collect::<Vec<_>>();
    rc_mut(image).set(
        value::int_arg(argv, 1) as i32,
        value::int_arg(argv, 2) as i32,
        &data_refs,
    );
    value::return_none();
    true
}

unsafe extern "C" fn image_load(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    match rc_mut(image).load(
        value::int_arg(argv, 1) as i32,
        value::int_arg(argv, 2) as i32,
        &value::str_arg(argv, 3),
        value::opt_bool_arg(argv, 4),
    ) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn image_save(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    match rc_ref(image).save(&value::str_arg(argv, 1), value::int_arg(argv, 2) as u32) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn image_clip(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    match (
        value::opt_float_arg(argv, 1),
        value::opt_float_arg(argv, 2),
        value::opt_float_arg(argv, 3),
        value::opt_float_arg(argv, 4),
    ) {
        (Some(x), Some(y), Some(width), Some(height)) => {
            rc_mut(image).set_clip_rect(x, y, width, height);
        }
        (None, None, None, None) => rc_mut(image).reset_clip_rect(),
        _ => return value::raise_exception("clip() takes 0 or 4 arguments"),
    }
    value::return_none();
    true
}

unsafe extern "C" fn image_camera(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    match (value::opt_float_arg(argv, 1), value::opt_float_arg(argv, 2)) {
        (Some(x), Some(y)) => rc_mut(image).set_camera(x, y),
        (None, None) => rc_mut(image).reset_camera(),
        _ => return value::raise_exception("camera() takes 0 or 2 arguments"),
    }
    value::return_none();
    true
}

unsafe extern "C" fn image_pal(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    match (value::opt_int_arg(argv, 1), value::opt_int_arg(argv, 2)) {
        (Some(src), Some(dst)) => rc_mut(image).map_color(src as pyxel::Color, dst as pyxel::Color),
        (None, None) => rc_mut(image).reset_color_map(),
        _ => return value::raise_exception("pal() takes 0 or 2 arguments"),
    }
    value::return_none();
    true
}

unsafe extern "C" fn image_dither(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(alpha) = value::float_arg(argv, 1) else {
        return false;
    };
    rc_mut(userdata::<pyxel::RcImage>(value::arg(argv, 0))).set_dithering(alpha);
    value::return_none();
    true
}

macro_rules! image_draw {
    ($name:ident, $method:ident, $($arg:expr),+ $(,)?) => {
        unsafe extern "C" fn $name(_argc: i32, argv: ffi::py_StackRef) -> bool {
            let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
            rc_mut(image).$method($($arg(argv)),+);
            value::return_none();
            true
        }
    };
}

image_draw!(image_cls, clear, |argv| color_arg(argv, 1));
image_draw!(
    image_pset,
    set_pixel,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| color_arg(argv, 3)
);
image_draw!(
    image_line,
    draw_line,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| color_arg(argv, 5)
);
image_draw!(
    image_rect,
    draw_rect,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| color_arg(argv, 5)
);
image_draw!(
    image_rectb,
    draw_rect_border,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| color_arg(argv, 5)
);
image_draw!(
    image_circ,
    draw_circle,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| color_arg(argv, 4)
);
image_draw!(
    image_circb,
    draw_circle_border,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| color_arg(argv, 4)
);
image_draw!(
    image_elli,
    draw_ellipse,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| color_arg(argv, 5)
);
image_draw!(
    image_ellib,
    draw_ellipse_border,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| color_arg(argv, 5)
);
image_draw!(
    image_tri,
    draw_triangle,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| farg(argv, 5),
    |argv| farg(argv, 6),
    |argv| color_arg(argv, 7)
);
image_draw!(
    image_trib,
    draw_triangle_border,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| farg(argv, 5),
    |argv| farg(argv, 6),
    |argv| color_arg(argv, 7)
);
image_draw!(
    image_fill,
    flood_fill,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| color_arg(argv, 3)
);

unsafe extern "C" fn image_pget(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    value::return_int(rc_ref(image).pixel(farg(argv, 1), farg(argv, 2)) as i64);
    true
}

unsafe extern "C" fn image_blt(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(source) = image_arg(value::arg(argv, 3)) else {
        return value::raise_exception("Invalid image");
    };
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    rc_mut(image).draw_image(
        farg(argv, 1),
        farg(argv, 2),
        &source,
        farg(argv, 4),
        farg(argv, 5),
        farg(argv, 6),
        farg(argv, 7),
        value::opt_int_arg(argv, 8).map(|color| color as pyxel::Color),
        value::opt_float_arg(argv, 9),
        value::opt_float_arg(argv, 10),
    );
    value::return_none();
    true
}

unsafe extern "C" fn image_bltm(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(source) = tilemap_arg(value::arg(argv, 3)) else {
        return value::raise_exception("Invalid tilemap");
    };
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    rc_mut(image).draw_tilemap(
        farg(argv, 1),
        farg(argv, 2),
        &source,
        farg(argv, 4),
        farg(argv, 5),
        farg(argv, 6),
        farg(argv, 7),
        value::opt_int_arg(argv, 8).map(|color| color as pyxel::Color),
        value::opt_float_arg(argv, 9),
        value::opt_float_arg(argv, 10),
    );
    value::return_none();
    true
}

unsafe extern "C" fn image_blt3d(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(source) = image_arg(value::arg(argv, 5)) else {
        return value::raise_exception("Invalid image");
    };
    let Some(pos) = value::tuple3_float_arg(argv, 6) else {
        return false;
    };
    let Some(rot) = value::tuple3_float_arg(argv, 7) else {
        return false;
    };
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    rc_mut(image).draw_image_3d(
        farg(argv, 1),
        farg(argv, 2),
        farg(argv, 3),
        farg(argv, 4),
        &source,
        pos,
        rot,
        value::opt_float_arg(argv, 8),
        value::opt_int_arg(argv, 9).map(|color| color as pyxel::Color),
    );
    value::return_none();
    true
}

unsafe extern "C" fn image_bltm3d(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(source) = tilemap_arg(value::arg(argv, 5)) else {
        return value::raise_exception("Invalid tilemap");
    };
    let Some(pos) = value::tuple3_float_arg(argv, 6) else {
        return false;
    };
    let Some(rot) = value::tuple3_float_arg(argv, 7) else {
        return false;
    };
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    rc_mut(image).draw_tilemap_3d(
        farg(argv, 1),
        farg(argv, 2),
        farg(argv, 3),
        farg(argv, 4),
        &source,
        pos,
        rot,
        value::opt_float_arg(argv, 8),
        value::opt_int_arg(argv, 9).map(|color| color as pyxel::Color),
    );
    value::return_none();
    true
}

unsafe extern "C" fn image_text(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let font = if value::is_none(value::arg(argv, 5)) {
        None
    } else {
        Some(font_from_ref(value::arg(argv, 5)))
    };
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    rc_mut(image).draw_text(
        farg(argv, 1),
        farg(argv, 2),
        &value::str_arg(argv, 3),
        color_arg(argv, 4),
        font.as_ref(),
    );
    value::return_none();
    true
}

pub(super) unsafe fn register(module: ffi::py_GlobalRef) {
    TP_IMAGE = value::new_type(module, c"Image", Some(drop_image));
    let image_type = ffi::py_tpobject(TP_IMAGE);
    ffi::py_bind(
        image_type,
        c"__new__(cls, width, height)".as_ptr(),
        Some(image_new),
    );
    ffi::py_bind(
        image_type,
        c"__init__(self, width, height)".as_ptr(),
        Some(noop_init),
    );
    ffi::py_bind(
        image_type,
        c"from_image(filename, include_colors=None)".as_ptr(),
        Some(image_from_image),
    );
    ffi::py_bindproperty(TP_IMAGE, c"width".as_ptr(), Some(image_width), None);
    ffi::py_bindproperty(TP_IMAGE, c"height".as_ptr(), Some(image_height), None);
    ffi::py_bindmethod(TP_IMAGE, c"data_ptr".as_ptr(), Some(image_data_ptr));
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
    ffi::py_bind(
        image_type,
        c"load(self, x, y, filename, include_colors=None)".as_ptr(),
        Some(image_load),
    );
    ffi::py_bind(
        image_type,
        c"clip(self, x=None, y=None, w=None, h=None)".as_ptr(),
        Some(image_clip),
    );
    ffi::py_bind(
        image_type,
        c"camera(self, x=None, y=None)".as_ptr(),
        Some(image_camera),
    );
    ffi::py_bind(
        image_type,
        c"pal(self, col1=None, col2=None)".as_ptr(),
        Some(image_pal),
    );
    ffi::py_bind(
        image_type,
        c"dither(self, alpha)".as_ptr(),
        Some(image_dither),
    );
    ffi::py_bind(
        image_type,
        c"blt(self, x, y, img, u, v, w, h, colkey=None, rotate=None, scale=None)".as_ptr(),
        Some(image_blt),
    );
    ffi::py_bind(
        image_type,
        c"bltm(self, x, y, tm, u, v, w, h, colkey=None, rotate=None, scale=None)".as_ptr(),
        Some(image_bltm),
    );
    ffi::py_bind(
        image_type,
        c"blt3d(self, x, y, w, h, img, pos, rot, fov=None, colkey=None)".as_ptr(),
        Some(image_blt3d),
    );
    ffi::py_bind(
        image_type,
        c"bltm3d(self, x, y, w, h, tm, pos, rot, fov=None, colkey=None)".as_ptr(),
        Some(image_bltm3d),
    );
    ffi::py_bind(
        image_type,
        c"text(self, x, y, s, col, font=None)".as_ptr(),
        Some(image_text),
    );
}
