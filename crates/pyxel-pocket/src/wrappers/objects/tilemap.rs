use super::{
    drop_tilemap, farg, ffi, image_from_ref, make_image, make_tilemap, new_userdata, noop_init,
    rc_mut, rc_ref, str_list_from_arg, tile_arg, tile_from_ref, tilemap_arg, userdata, value,
    TP_IMAGE, TP_TILEMAP,
};

// Tilemap

unsafe fn image_source_arg(object: ffi::py_Ref) -> Option<pyxel::ImageSource> {
    if value::is_int(object) {
        Some(pyxel::ImageSource::Index(ffi::py_toint(object) as u32))
    } else if ffi::py_isinstance(object, TP_IMAGE) {
        Some(pyxel::ImageSource::Image(image_from_ref(object)))
    } else {
        None
    }
}

unsafe extern "C" fn tilemap_new(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(imgsrc) = image_source_arg(value::arg(argv, 3)) else {
        return value::raise_exception("Invalid image source");
    };
    let cls = ffi::py_totype(value::arg(argv, 0));
    new_userdata(
        ffi::py_retval(),
        cls,
        pyxel::Tilemap::new(
            value::int_arg(argv, 1) as u32,
            value::int_arg(argv, 2) as u32,
            imgsrc,
        ),
    );
    true
}

unsafe extern "C" fn tilemap_from_tmx(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match pyxel::Tilemap::from_tmx(&value::str_arg(argv, 0), value::int_arg(argv, 1) as u32) {
        Ok(tilemap) => {
            make_tilemap(ffi::py_retval(), tilemap);
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn tilemap_width(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    value::return_int(rc_ref(tilemap).width() as i64);
    true
}

unsafe extern "C" fn tilemap_height(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    value::return_int(rc_ref(tilemap).height() as i64);
    true
}

unsafe extern "C" fn tilemap_imgsrc_get(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    match &rc_ref(tilemap).imgsrc {
        pyxel::ImageSource::Index(index) => value::return_int(*index as i64),
        pyxel::ImageSource::Image(image) => make_image(ffi::py_retval(), image.clone()),
    }
    true
}

unsafe extern "C" fn tilemap_imgsrc_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(imgsrc) = image_source_arg(value::arg(argv, 1)) else {
        return value::raise_exception("Invalid image source");
    };
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    rc_mut(tilemap).imgsrc = imgsrc;
    value::return_none();
    true
}

unsafe extern "C" fn tilemap_image_get(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    let image = match &rc_ref(tilemap).imgsrc {
        pyxel::ImageSource::Index(index) => {
            let Some(image) = pyxel::images().get(*index as usize).cloned() else {
                return value::raise_exception("Invalid image index");
            };
            image
        }
        pyxel::ImageSource::Image(image) => image.clone(),
    };
    make_image(ffi::py_retval(), image);
    true
}

unsafe extern "C" fn tilemap_image_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if !ffi::py_isinstance(value::arg(argv, 1), TP_IMAGE) {
        return value::raise_exception("Invalid image");
    }
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    rc_mut(tilemap).imgsrc = pyxel::ImageSource::Image(image_from_ref(value::arg(argv, 1)));
    value::return_none();
    true
}

unsafe extern "C" fn tilemap_refimg_get(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    match &rc_ref(tilemap).imgsrc {
        pyxel::ImageSource::Index(index) => value::return_int(*index as i64),
        pyxel::ImageSource::Image(_) => value::return_none(),
    }
    true
}

unsafe extern "C" fn tilemap_refimg_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    rc_mut(tilemap).imgsrc = pyxel::ImageSource::Index(value::int_arg(argv, 1) as u32);
    value::return_none();
    true
}

unsafe extern "C" fn tilemap_data_ptr(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    value::return_int(rc_mut(tilemap).data_ptr() as usize as i64);
    true
}

unsafe extern "C" fn tilemap_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    let data = str_list_from_arg(argv, 3);
    let data_refs = data.iter().map(String::as_str).collect::<Vec<_>>();
    rc_mut(tilemap).set(
        value::int_arg(argv, 1) as i32,
        value::int_arg(argv, 2) as i32,
        &data_refs,
    );
    value::return_none();
    true
}

unsafe extern "C" fn tilemap_load(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    match rc_mut(tilemap).load(
        value::int_arg(argv, 1) as i32,
        value::int_arg(argv, 2) as i32,
        &value::str_arg(argv, 3),
        value::int_arg(argv, 4) as u32,
    ) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn tilemap_clip(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    match (
        value::opt_float_arg(argv, 1),
        value::opt_float_arg(argv, 2),
        value::opt_float_arg(argv, 3),
        value::opt_float_arg(argv, 4),
    ) {
        (Some(x), Some(y), Some(width), Some(height)) => {
            rc_mut(tilemap).set_clip_rect(x, y, width, height);
        }
        (None, None, None, None) => rc_mut(tilemap).reset_clip_rect(),
        _ => return value::raise_exception("clip() takes 0 or 4 arguments"),
    }
    value::return_none();
    true
}

unsafe extern "C" fn tilemap_camera(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    match (value::opt_float_arg(argv, 1), value::opt_float_arg(argv, 2)) {
        (Some(x), Some(y)) => rc_mut(tilemap).set_camera(x, y),
        (None, None) => rc_mut(tilemap).reset_camera(),
        _ => return value::raise_exception("camera() takes 0 or 2 arguments"),
    }
    value::return_none();
    true
}

macro_rules! tilemap_draw {
    ($name:ident, $method:ident, $($arg:expr),+ $(,)?) => {
        unsafe extern "C" fn $name(_argc: i32, argv: ffi::py_StackRef) -> bool {
            let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
            rc_mut(tilemap).$method($($arg(argv)),+);
            value::return_none();
            true
        }
    };
}

tilemap_draw!(tilemap_cls, clear, |argv| tile_arg(argv, 1));
tilemap_draw!(
    tilemap_pset,
    set_tile,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| tile_arg(argv, 3)
);
tilemap_draw!(
    tilemap_line,
    draw_line,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| tile_arg(argv, 5)
);
tilemap_draw!(
    tilemap_rect,
    draw_rect,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| tile_arg(argv, 5)
);
tilemap_draw!(
    tilemap_rectb,
    draw_rect_border,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| tile_arg(argv, 5)
);
tilemap_draw!(
    tilemap_circ,
    draw_circle,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| tile_arg(argv, 4)
);
tilemap_draw!(
    tilemap_circb,
    draw_circle_border,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| tile_arg(argv, 4)
);
tilemap_draw!(
    tilemap_elli,
    draw_ellipse,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| tile_arg(argv, 5)
);
tilemap_draw!(
    tilemap_ellib,
    draw_ellipse_border,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| tile_arg(argv, 5)
);
tilemap_draw!(
    tilemap_tri,
    draw_triangle,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| farg(argv, 5),
    |argv| farg(argv, 6),
    |argv| tile_arg(argv, 7)
);
tilemap_draw!(
    tilemap_trib,
    draw_triangle_border,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| farg(argv, 5),
    |argv| farg(argv, 6),
    |argv| tile_arg(argv, 7)
);
tilemap_draw!(
    tilemap_fill,
    flood_fill,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| tile_arg(argv, 3)
);

unsafe extern "C" fn tilemap_pget(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    value::return_tile(rc_ref(tilemap).tile(farg(argv, 1), farg(argv, 2)));
    true
}

unsafe extern "C" fn tilemap_collide(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let walls_ref = value::arg(argv, 7);
    let walls = (0..ffi::py_list_len(walls_ref))
        .map(|i| tile_from_ref(ffi::py_list_getitem(walls_ref, i)))
        .collect::<Vec<_>>();
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    value::return_float_pair(rc_ref(tilemap).collide(
        farg(argv, 1),
        farg(argv, 2),
        farg(argv, 3),
        farg(argv, 4),
        farg(argv, 5),
        farg(argv, 6),
        &walls,
    ));
    true
}

unsafe extern "C" fn tilemap_blt(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(source) = tilemap_arg(value::arg(argv, 3)) else {
        return value::raise_exception("Invalid tilemap");
    };
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    rc_mut(tilemap).draw_tilemap(
        farg(argv, 1),
        farg(argv, 2),
        &source,
        farg(argv, 4),
        farg(argv, 5),
        farg(argv, 6),
        farg(argv, 7),
        if value::is_none(value::arg(argv, 8)) {
            None
        } else {
            Some(tile_arg(argv, 8))
        },
        value::opt_float_arg(argv, 9),
        value::opt_float_arg(argv, 10),
    );
    value::return_none();
    true
}

pub(super) unsafe fn register(module: ffi::py_GlobalRef) {
    TP_TILEMAP = value::new_type(module, c"Tilemap", Some(drop_tilemap));
    let tilemap_type = ffi::py_tpobject(TP_TILEMAP);
    ffi::py_bind(
        tilemap_type,
        c"__new__(cls, width, height, imgsrc)".as_ptr(),
        Some(tilemap_new),
    );
    ffi::py_bind(
        tilemap_type,
        c"__init__(self, width, height, imgsrc)".as_ptr(),
        Some(noop_init),
    );
    ffi::py_bind(
        tilemap_type,
        c"from_tmx(filename, layer)".as_ptr(),
        Some(tilemap_from_tmx),
    );
    ffi::py_bindproperty(TP_TILEMAP, c"width".as_ptr(), Some(tilemap_width), None);
    ffi::py_bindproperty(TP_TILEMAP, c"height".as_ptr(), Some(tilemap_height), None);
    ffi::py_bindproperty(
        TP_TILEMAP,
        c"imgsrc".as_ptr(),
        Some(tilemap_imgsrc_get),
        Some(tilemap_imgsrc_set),
    );
    ffi::py_bindproperty(
        TP_TILEMAP,
        c"image".as_ptr(),
        Some(tilemap_image_get),
        Some(tilemap_image_set),
    );
    ffi::py_bindproperty(
        TP_TILEMAP,
        c"refimg".as_ptr(),
        Some(tilemap_refimg_get),
        Some(tilemap_refimg_set),
    );
    ffi::py_bindmethod(TP_TILEMAP, c"data_ptr".as_ptr(), Some(tilemap_data_ptr));
    ffi::py_bindmethod(TP_TILEMAP, c"set".as_ptr(), Some(tilemap_set));
    ffi::py_bindmethod(TP_TILEMAP, c"load".as_ptr(), Some(tilemap_load));
    ffi::py_bindmethod(TP_TILEMAP, c"cls".as_ptr(), Some(tilemap_cls));
    ffi::py_bindmethod(TP_TILEMAP, c"pget".as_ptr(), Some(tilemap_pget));
    ffi::py_bindmethod(TP_TILEMAP, c"pset".as_ptr(), Some(tilemap_pset));
    ffi::py_bindmethod(TP_TILEMAP, c"line".as_ptr(), Some(tilemap_line));
    ffi::py_bindmethod(TP_TILEMAP, c"rect".as_ptr(), Some(tilemap_rect));
    ffi::py_bindmethod(TP_TILEMAP, c"rectb".as_ptr(), Some(tilemap_rectb));
    ffi::py_bindmethod(TP_TILEMAP, c"circ".as_ptr(), Some(tilemap_circ));
    ffi::py_bindmethod(TP_TILEMAP, c"circb".as_ptr(), Some(tilemap_circb));
    ffi::py_bindmethod(TP_TILEMAP, c"elli".as_ptr(), Some(tilemap_elli));
    ffi::py_bindmethod(TP_TILEMAP, c"ellib".as_ptr(), Some(tilemap_ellib));
    ffi::py_bindmethod(TP_TILEMAP, c"tri".as_ptr(), Some(tilemap_tri));
    ffi::py_bindmethod(TP_TILEMAP, c"trib".as_ptr(), Some(tilemap_trib));
    ffi::py_bindmethod(TP_TILEMAP, c"fill".as_ptr(), Some(tilemap_fill));
    ffi::py_bindmethod(TP_TILEMAP, c"collide".as_ptr(), Some(tilemap_collide));
    ffi::py_bind(
        tilemap_type,
        c"clip(self, x=None, y=None, w=None, h=None)".as_ptr(),
        Some(tilemap_clip),
    );
    ffi::py_bind(
        tilemap_type,
        c"camera(self, x=None, y=None)".as_ptr(),
        Some(tilemap_camera),
    );
    ffi::py_bind(
        tilemap_type,
        c"blt(self, x, y, tm, u, v, w, h, tilekey=None, rotate=None, scale=None)".as_ptr(),
        Some(tilemap_blt),
    );
}
