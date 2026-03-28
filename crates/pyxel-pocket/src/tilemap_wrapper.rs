use crate::ffi;
use crate::helpers::*;
use crate::image_wrapper::{image_ptr, new_image_obj};

// Tile = (u16, u16) — stored as a single int (u16 << 16 | u16) in Python
unsafe fn arg_tile(argv: ffi::py_StackRef, i: usize) -> pyxel::Tile {
    // In Python, tiles are passed as tuples (u, v) or integers
    let r = arg(argv, i);
    if ffi::py_istype(r, ffi::py_PredefinedType_tp_tuple as ffi::py_Type) {
        let u = ffi::py_toint(ffi::py_tuple_getitem(r, 0)) as u16;
        let v = ffi::py_toint(ffi::py_tuple_getitem(r, 1)) as u16;
        (u, v)
    } else {
        let val = ffi::py_toint(r);
        ((val >> 16) as u16, (val & 0xFFFF) as u16)
    }
}

unsafe fn ret_tile(t: pyxel::Tile) {
    let out = ffi::py_newtuple(ffi::py_retval(), 2);
    ffi::py_newint(out.add(0), t.0 as i64);
    ffi::py_newint(out.add(1), t.1 as i64);
}

pub static mut TP_TILEMAP: ffi::py_Type = 0;
pub static mut TP_TILEMAPS: ffi::py_Type = 0;

unsafe fn tm(argv: ffi::py_StackRef) -> &'static mut pyxel::Tilemap {
    &mut *(*(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Tilemap))
}

pub unsafe fn new_tilemap_obj(out: ffi::py_OutRef, ptr: *mut pyxel::Tilemap) {
    let ud = ffi::py_newobject(out, TP_TILEMAP, 0, size_of::<*mut pyxel::Tilemap>() as i32);
    *(ud as *mut *mut pyxel::Tilemap) = ptr;
}

unsafe extern "C" fn tilemap_width(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_int(tm(argv).width() as i64);
    true
}

unsafe extern "C" fn tilemap_height(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_int(tm(argv).height() as i64);
    true
}

unsafe extern "C" fn tilemap_imgsrc_getter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match &tm(argv).imgsrc {
        pyxel::ImageSource::Index(index) => ret_int(*index as i64),
        pyxel::ImageSource::Image(image) => new_image_obj(ffi::py_retval(), *image),
    }
    true
}

unsafe extern "C" fn tilemap_imgsrc_setter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let val = arg(argv, 1);
    if is_int(val) {
        tm(argv).imgsrc = pyxel::ImageSource::Index(ffi::py_toint(val) as u32);
    } else {
        tm(argv).imgsrc = pyxel::ImageSource::Image(image_ptr(val));
    }
    ret_none();
    true
}

unsafe extern "C" fn tilemap_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let data = arg_str_list(argv, 3);
    let data_refs: Vec<&str> = data.iter().map(String::as_str).collect();
    tm(argv).set(
        arg_float(argv, 1) as i32,
        arg_float(argv, 2) as i32,
        &data_refs,
    );
    ret_none();
    true
}

unsafe extern "C" fn tilemap_load(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if let Err(e) = tm(argv).load(
        arg_float(argv, 1) as i32,
        arg_float(argv, 2) as i32,
        arg_str(argv, 3),
        arg_int(argv, 4) as u32,
    ) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

unsafe extern "C" fn tilemap_clip(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if arg_is_none(argv, 1) {
        tm(argv).reset_clip_rect();
    } else {
        tm(argv).set_clip_rect(
            arg_float(argv, 1) as f32,
            arg_float(argv, 2) as f32,
            arg_float(argv, 3) as f32,
            arg_float(argv, 4) as f32,
        );
    }
    ret_none();
    true
}

unsafe extern "C" fn tilemap_camera(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if arg_is_none(argv, 1) {
        tm(argv).reset_draw_offset();
    } else {
        tm(argv).set_draw_offset(arg_float(argv, 1) as f32, arg_float(argv, 2) as f32);
    }
    ret_none();
    true
}

unsafe extern "C" fn tilemap_cls(_argc: i32, argv: ffi::py_StackRef) -> bool {
    tm(argv).clear(arg_tile(argv, 1));
    ret_none();
    true
}

unsafe extern "C" fn tilemap_pget(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_tile(tm(argv).get_tile(arg_float(argv, 1) as f32, arg_float(argv, 2) as f32));
    true
}

unsafe extern "C" fn tilemap_pset(_argc: i32, argv: ffi::py_StackRef) -> bool {
    tm(argv).set_tile(
        arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32,
        arg_tile(argv, 3),
    );
    ret_none();
    true
}

unsafe extern "C" fn tilemap_line(_argc: i32, argv: ffi::py_StackRef) -> bool {
    tm(argv).draw_line(
        arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32,
        arg_float(argv, 4) as f32,
        arg_tile(argv, 5),
    );
    ret_none();
    true
}

unsafe extern "C" fn tilemap_rect(_argc: i32, argv: ffi::py_StackRef) -> bool {
    tm(argv).draw_rect(
        arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32,
        arg_float(argv, 4) as f32,
        arg_tile(argv, 5),
    );
    ret_none();
    true
}

unsafe extern "C" fn tilemap_rectb(_argc: i32, argv: ffi::py_StackRef) -> bool {
    tm(argv).draw_rect_border(
        arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32,
        arg_float(argv, 4) as f32,
        arg_tile(argv, 5),
    );
    ret_none();
    true
}

unsafe extern "C" fn tilemap_circ(_argc: i32, argv: ffi::py_StackRef) -> bool {
    tm(argv).draw_circle(
        arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32,
        arg_tile(argv, 4),
    );
    ret_none();
    true
}

unsafe extern "C" fn tilemap_circb(_argc: i32, argv: ffi::py_StackRef) -> bool {
    tm(argv).draw_circle_border(
        arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32,
        arg_tile(argv, 4),
    );
    ret_none();
    true
}

unsafe extern "C" fn tilemap_elli(_argc: i32, argv: ffi::py_StackRef) -> bool {
    tm(argv).draw_ellipse(
        arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32,
        arg_float(argv, 4) as f32,
        arg_tile(argv, 5),
    );
    ret_none();
    true
}

unsafe extern "C" fn tilemap_ellib(_argc: i32, argv: ffi::py_StackRef) -> bool {
    tm(argv).draw_ellipse_border(
        arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32,
        arg_float(argv, 4) as f32,
        arg_tile(argv, 5),
    );
    ret_none();
    true
}

unsafe extern "C" fn tilemap_tri(_argc: i32, argv: ffi::py_StackRef) -> bool {
    tm(argv).draw_triangle(
        arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32,
        arg_float(argv, 4) as f32,
        arg_float(argv, 5) as f32,
        arg_float(argv, 6) as f32,
        arg_tile(argv, 7),
    );
    ret_none();
    true
}

unsafe extern "C" fn tilemap_trib(_argc: i32, argv: ffi::py_StackRef) -> bool {
    tm(argv).draw_triangle_border(
        arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32,
        arg_float(argv, 4) as f32,
        arg_float(argv, 5) as f32,
        arg_float(argv, 6) as f32,
        arg_tile(argv, 7),
    );
    ret_none();
    true
}

unsafe extern "C" fn tilemap_fill(_argc: i32, argv: ffi::py_StackRef) -> bool {
    tm(argv).flood_fill(
        arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32,
        arg_tile(argv, 3),
    );
    ret_none();
    true
}

unsafe extern "C" fn tilemap_collide(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let list_ref = arg(argv, 7);
    let len = ffi::py_list_len(list_ref);
    let walls: Vec<pyxel::Tile> = (0..len)
        .map(|i| {
            let item = ffi::py_list_getitem(list_ref, i);
            let u = ffi::py_toint(ffi::py_tuple_getitem(item, 0)) as u16;
            let v = ffi::py_toint(ffi::py_tuple_getitem(item, 1)) as u16;
            (u, v)
        })
        .collect();
    let (dx, dy) = tm(argv).collide(
        arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32,
        arg_float(argv, 3) as f32,
        arg_float(argv, 4) as f32,
        arg_float(argv, 5) as f32,
        arg_float(argv, 6) as f32,
        &walls,
    );
    let out = ffi::py_newtuple(ffi::py_retval(), 2);
    ffi::py_newfloat(out.add(0), dx as f64);
    ffi::py_newfloat(out.add(1), dy as f64);
    true
}

unsafe extern "C" fn tilemap_from_tmx(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let filename = arg_str(argv, 0);
    let layer = arg_int(argv, 1) as u32;
    match pyxel::Tilemap::from_tmx(filename, layer) {
        Ok(ptr) => {
            new_tilemap_obj(ffi::py_retval(), ptr);
            true
        }
        Err(e) => raise_exc(&e),
    }
}

pub unsafe fn add_tilemap_class(m: ffi::py_GlobalRef) {
    TP_TILEMAP = new_type(c"Tilemap", m);

    ffi::py_bindproperty(TP_TILEMAP, c"width".as_ptr(), Some(tilemap_width), None);
    ffi::py_bindproperty(TP_TILEMAP, c"height".as_ptr(), Some(tilemap_height), None);
    ffi::py_bindproperty(
        TP_TILEMAP,
        c"imgsrc".as_ptr(),
        Some(tilemap_imgsrc_getter),
        Some(tilemap_imgsrc_setter),
    );

    // Deprecated aliases
    ffi::py_bindproperty(
        TP_TILEMAP,
        c"image".as_ptr(),
        Some(tilemap_imgsrc_getter),
        Some(tilemap_imgsrc_setter),
    );
    ffi::py_bindproperty(
        TP_TILEMAP,
        c"refimg".as_ptr(),
        Some(tilemap_imgsrc_getter),
        Some(tilemap_imgsrc_setter),
    );

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

    let tp_obj = ffi::py_tpobject(TP_TILEMAP);
    bind(
        tp_obj,
        c"clip(self, x=None, y=None, w=None, h=None)",
        Some(tilemap_clip),
    );
    bind(
        tp_obj,
        c"camera(self, x=None, y=None)",
        Some(tilemap_camera),
    );
    bind(tp_obj, c"from_tmx(filename, layer)", Some(tilemap_from_tmx));

    // Tilemaps collection
    impl_object_collection!(
        pyxel::tilemaps,
        new_tilemap_obj,
        tilemaps_getitem,
        tilemaps_setitem,
        tilemaps_len,
        tilemaps_iter
    );
    register_collection!(
        TP_TILEMAPS,
        c"Tilemaps",
        m,
        tilemaps_getitem,
        tilemaps_setitem,
        tilemaps_len,
        tilemaps_iter
    );
}
