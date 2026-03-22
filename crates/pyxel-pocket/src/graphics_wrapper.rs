use crate::ffi;
use crate::helpers::*;
use crate::image_wrapper::{image_ptr, TP_IMAGE};
use crate::tilemap_wrapper::TP_TILEMAP;

unsafe extern "C" fn pyxel_clip(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if arg_is_none(argv, 0) {
        pyxel::pyxel().reset_clip_rect();
    } else {
        pyxel::pyxel().set_clip_rect(
            arg_float(argv, 0) as f32,
            arg_float(argv, 1) as f32,
            arg_float(argv, 2) as f32,
            arg_float(argv, 3) as f32,
        );
    }
    ret_none();
    true
}

unsafe extern "C" fn pyxel_camera(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if arg_is_none(argv, 0) {
        pyxel::pyxel().reset_draw_offset();
    } else {
        pyxel::pyxel().set_draw_offset(
            arg_float(argv, 0) as f32,
            arg_float(argv, 1) as f32,
        );
    }
    ret_none();
    true
}

unsafe extern "C" fn pyxel_pal(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if arg_is_none(argv, 0) {
        pyxel::pyxel().reset_color_map();
    } else {
        pyxel::pyxel().map_color(arg_int(argv, 0) as u8, arg_int(argv, 1) as u8);
    }
    ret_none();
    true
}

unsafe extern "C" fn pyxel_dither(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_dithering(arg_float(argv, 0) as f32);
    ret_none();
    true
}

unsafe extern "C" fn pyxel_cls(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().clear(arg_int(argv, 0) as u8);
    ret_none();
    true
}

unsafe extern "C" fn pyxel_pget(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_int(
        pyxel::pyxel().get_pixel(arg_float(argv, 0) as f32, arg_float(argv, 1) as f32) as i64,
    );
    true
}

unsafe extern "C" fn pyxel_pset(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().set_pixel(
        arg_float(argv, 0) as f32,
        arg_float(argv, 1) as f32,
        arg_int(argv, 2) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn pyxel_line(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().draw_line(
        arg_float(argv, 0) as f32, arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32, arg_float(argv, 3) as f32,
        arg_int(argv, 4) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn pyxel_rect(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().draw_rect(
        arg_float(argv, 0) as f32, arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32, arg_float(argv, 3) as f32,
        arg_int(argv, 4) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn pyxel_rectb(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().draw_rect_border(
        arg_float(argv, 0) as f32, arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32, arg_float(argv, 3) as f32,
        arg_int(argv, 4) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn pyxel_circ(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().draw_circle(
        arg_float(argv, 0) as f32, arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32, arg_int(argv, 3) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn pyxel_circb(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().draw_circle_border(
        arg_float(argv, 0) as f32, arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32, arg_int(argv, 3) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn pyxel_elli(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().draw_ellipse(
        arg_float(argv, 0) as f32, arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32, arg_float(argv, 3) as f32,
        arg_int(argv, 4) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn pyxel_ellib(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().draw_ellipse_border(
        arg_float(argv, 0) as f32, arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32, arg_float(argv, 3) as f32,
        arg_int(argv, 4) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn pyxel_tri(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().draw_triangle(
        arg_float(argv, 0) as f32, arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32, arg_float(argv, 3) as f32,
        arg_float(argv, 4) as f32, arg_float(argv, 5) as f32,
        arg_int(argv, 6) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn pyxel_trib(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().draw_triangle_border(
        arg_float(argv, 0) as f32, arg_float(argv, 1) as f32,
        arg_float(argv, 2) as f32, arg_float(argv, 3) as f32,
        arg_float(argv, 4) as f32, arg_float(argv, 5) as f32,
        arg_int(argv, 6) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn pyxel_fill(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().flood_fill(
        arg_float(argv, 0) as f32, arg_float(argv, 1) as f32,
        arg_int(argv, 2) as u8,
    );
    ret_none();
    true
}

unsafe extern "C" fn pyxel_blt(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let x = arg_float(argv, 0) as f32;
    let y = arg_float(argv, 1) as f32;
    let img_ref = arg(argv, 2);
    let u = arg_float(argv, 3) as f32;
    let v = arg_float(argv, 4) as f32;
    let w = arg_float(argv, 5) as f32;
    let h = arg_float(argv, 6) as f32;
    let colkey = arg_opt_int(argv, 7).map(|v| v as u8);
    let rotate = arg_opt_float(argv, 8).map(|v| v as f32);
    let scale = arg_opt_float(argv, 9).map(|v| v as f32);
    if ffi::py_isinstance(img_ref, TP_IMAGE) {
        let img_ptr = image_ptr(img_ref);
        pyxel::screen().draw_image(x, y, img_ptr, u, v, w, h, colkey, rotate, scale);
    } else {
        pyxel::pyxel().draw_image(x, y, arg_int(argv, 2) as u32, u, v, w, h, colkey, rotate, scale);
    }
    ret_none();
    true
}

unsafe extern "C" fn pyxel_bltm(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let x = arg_float(argv, 0) as f32;
    let y = arg_float(argv, 1) as f32;
    let tm_ref = arg(argv, 2);
    let u = arg_float(argv, 3) as f32;
    let v = arg_float(argv, 4) as f32;
    let w = arg_float(argv, 5) as f32;
    let h = arg_float(argv, 6) as f32;
    let colkey = arg_opt_int(argv, 7).map(|v| v as u8);
    let rotate = arg_opt_float(argv, 8).map(|v| v as f32);
    let scale = arg_opt_float(argv, 9).map(|v| v as f32);
    if ffi::py_isinstance(tm_ref, TP_TILEMAP) {
        let tm_ptr = *(ffi::py_touserdata(tm_ref) as *mut *mut pyxel::Tilemap);
        pyxel::screen().draw_tilemap(x, y, tm_ptr, u, v, w, h, colkey, rotate, scale);
    } else {
        pyxel::pyxel().draw_tilemap(x, y, arg_int(argv, 2) as u32, u, v, w, h, colkey, rotate, scale);
    }
    ret_none();
    true
}

// Extract (f32, f32, f32) tuple from a Python tuple argument
unsafe fn arg_tuple3(argv: ffi::py_StackRef, i: usize) -> (f32, f32, f32) {
    let r = arg(argv, i);
    (
        ffi::py_tofloat(ffi::py_tuple_getitem(r, 0)) as f32,
        ffi::py_tofloat(ffi::py_tuple_getitem(r, 1)) as f32,
        ffi::py_tofloat(ffi::py_tuple_getitem(r, 2)) as f32,
    )
}

unsafe extern "C" fn pyxel_blt3d(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let img_ref = arg(argv, 4);
    let pos = arg_tuple3(argv, 5);
    let rot = arg_tuple3(argv, 6);
    let fov = arg_opt_float(argv, 7).map(|v| v as f32);
    let colkey = arg_opt_int(argv, 8).map(|v| v as u8);
    if ffi::py_isinstance(img_ref, TP_IMAGE) {
        pyxel::screen().draw_image_3d(
            arg_float(argv, 0) as f32, arg_float(argv, 1) as f32,
            arg_float(argv, 2) as f32, arg_float(argv, 3) as f32,
            image_ptr(img_ref), pos, rot, fov, colkey,
        );
    } else {
        pyxel::pyxel().draw_image_3d(
            arg_float(argv, 0) as f32, arg_float(argv, 1) as f32,
            arg_float(argv, 2) as f32, arg_float(argv, 3) as f32,
            ffi::py_toint(img_ref) as u32, pos, rot, fov, colkey,
        );
    }
    ret_none();
    true
}

unsafe extern "C" fn pyxel_bltm3d(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tm_ref = arg(argv, 4);
    let pos = arg_tuple3(argv, 5);
    let rot = arg_tuple3(argv, 6);
    let fov = arg_opt_float(argv, 7).map(|v| v as f32);
    let colkey = arg_opt_int(argv, 8).map(|v| v as u8);
    if ffi::py_isinstance(tm_ref, TP_TILEMAP) {
        let tm_ptr = *(ffi::py_touserdata(tm_ref) as *mut *mut pyxel::Tilemap);
        pyxel::screen().draw_tilemap_3d(
            arg_float(argv, 0) as f32, arg_float(argv, 1) as f32,
            arg_float(argv, 2) as f32, arg_float(argv, 3) as f32,
            tm_ptr, pos, rot, fov, colkey,
        );
    } else {
        pyxel::pyxel().draw_tilemap_3d(
            arg_float(argv, 0) as f32, arg_float(argv, 1) as f32,
            arg_float(argv, 2) as f32, arg_float(argv, 3) as f32,
            ffi::py_toint(tm_ref) as u32, pos, rot, fov, colkey,
        );
    }
    ret_none();
    true
}

unsafe extern "C" fn pyxel_text(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let font = if arg_is_none(argv, 4) {
        None
    } else {
        Some(*(ffi::py_touserdata(arg(argv, 4)) as *mut *mut pyxel::Font))
    };
    pyxel::pyxel().draw_text(
        arg_float(argv, 0) as f32, arg_float(argv, 1) as f32,
        arg_str(argv, 2), arg_int(argv, 3) as u8, font,
    );
    ret_none();
    true
}

pub unsafe fn add_graphics_functions(m: ffi::py_GlobalRef) {
    bind(m, c"clip(x=None, y=None, w=None, h=None)", Some(pyxel_clip));
    bind(m, c"camera(x=None, y=None)", Some(pyxel_camera));
    bind(m, c"pal(col1=None, col2=None)", Some(pyxel_pal));
    bind(m, c"dither(alpha)", Some(pyxel_dither));
    bind(m, c"cls(col)", Some(pyxel_cls));
    bind(m, c"pget(x, y)", Some(pyxel_pget));
    bind(m, c"pset(x, y, col)", Some(pyxel_pset));
    bind(m, c"line(x1, y1, x2, y2, col)", Some(pyxel_line));
    bind(m, c"rect(x, y, w, h, col)", Some(pyxel_rect));
    bind(m, c"rectb(x, y, w, h, col)", Some(pyxel_rectb));
    bind(m, c"circ(x, y, r, col)", Some(pyxel_circ));
    bind(m, c"circb(x, y, r, col)", Some(pyxel_circb));
    bind(m, c"elli(x, y, w, h, col)", Some(pyxel_elli));
    bind(m, c"ellib(x, y, w, h, col)", Some(pyxel_ellib));
    bind(m, c"tri(x1, y1, x2, y2, x3, y3, col)", Some(pyxel_tri));
    bind(m, c"trib(x1, y1, x2, y2, x3, y3, col)", Some(pyxel_trib));
    bind(m, c"fill(x, y, col)", Some(pyxel_fill));
    bind(m, c"blt(x, y, img, u, v, w, h, colkey=None, rotate=None, scale=None)", Some(pyxel_blt));
    bind(m, c"bltm(x, y, tm, u, v, w, h, colkey=None, rotate=None, scale=None)", Some(pyxel_bltm));
    bind(m, c"blt3d(x, y, w, h, img, pos, rot, fov=None, colkey=None)", Some(pyxel_blt3d));
    bind(m, c"bltm3d(x, y, w, h, tm, pos, rot, fov=None, colkey=None)", Some(pyxel_bltm3d));
    bind(m, c"text(x, y, s, col, font=None)", Some(pyxel_text));

    // Deprecated functions
    bind(m, c"image(img)", Some(pyxel_image_deprecated));
    bind(m, c"tilemap(tm)", Some(pyxel_tilemap_deprecated));
}

unsafe extern "C" fn pyxel_image_deprecated(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let index = arg_int(argv, 0) as usize;
    let images = pyxel::images();
    if index >= images.len() {
        return raise_exc("Invalid image index");
    }
    crate::image_wrapper::new_image_obj(ffi::py_retval(), images[index]);
    true
}

unsafe extern "C" fn pyxel_tilemap_deprecated(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let index = arg_int(argv, 0) as usize;
    let tilemaps = pyxel::tilemaps();
    if index >= tilemaps.len() {
        return raise_exc("Invalid tilemap index");
    }
    crate::tilemap_wrapper::new_tilemap_obj(ffi::py_retval(), tilemaps[index]);
    true
}
