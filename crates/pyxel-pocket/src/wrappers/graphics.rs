use crate::{ffi, value};

unsafe extern "C" fn clip(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match (
        value::opt_float_arg(argv, 0),
        value::opt_float_arg(argv, 1),
        value::opt_float_arg(argv, 2),
        value::opt_float_arg(argv, 3),
    ) {
        (Some(x), Some(y), Some(width), Some(height)) => {
            pyxel::pyxel().set_clip_rect(x, y, width, height);
        }
        (None, None, None, None) => {
            pyxel::pyxel().reset_clip_rect();
        }
        _ => return value::raise_exception("clip() takes 0 or 4 arguments"),
    }
    value::return_none();
    true
}

unsafe extern "C" fn camera(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match (value::opt_float_arg(argv, 0), value::opt_float_arg(argv, 1)) {
        (Some(x), Some(y)) => {
            pyxel::pyxel().set_camera(x, y);
        }
        (None, None) => {
            pyxel::pyxel().reset_camera();
        }
        _ => return value::raise_exception("camera() takes 0 or 2 arguments"),
    }
    value::return_none();
    true
}

unsafe extern "C" fn pal(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match (value::opt_int_arg(argv, 0), value::opt_int_arg(argv, 1)) {
        (Some(color1), Some(color2)) => {
            pyxel::pyxel().map_color(color1 as pyxel::Color, color2 as pyxel::Color);
        }
        (None, None) => {
            pyxel::pyxel().reset_color_map();
        }
        _ => return value::raise_exception("pal() takes 0 or 2 arguments"),
    }
    value::return_none();
    true
}

unsafe extern "C" fn dither(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(alpha) = value::float_arg(argv, 0) else {
        return false;
    };
    pyxel::pyxel().set_dithering(alpha);
    value::return_none();
    true
}

unsafe extern "C" fn cls(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::pyxel().clear(value::int_arg(argv, 0) as pyxel::Color);
    value::return_none();
    true
}

unsafe extern "C" fn pget(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(x) = value::float_arg(argv, 0) else {
        return false;
    };
    let Some(y) = value::float_arg(argv, 1) else {
        return false;
    };
    value::return_int(pyxel::pyxel().pixel(x, y) as i64);
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

unsafe extern "C" fn circ(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(x) = value::float_arg(argv, 0) else {
        return false;
    };
    let Some(y) = value::float_arg(argv, 1) else {
        return false;
    };
    let Some(radius) = value::float_arg(argv, 2) else {
        return false;
    };
    pyxel::pyxel().draw_circle(x, y, radius, value::int_arg(argv, 3) as pyxel::Color);
    value::return_none();
    true
}

unsafe extern "C" fn circb(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(x) = value::float_arg(argv, 0) else {
        return false;
    };
    let Some(y) = value::float_arg(argv, 1) else {
        return false;
    };
    let Some(radius) = value::float_arg(argv, 2) else {
        return false;
    };
    pyxel::pyxel().draw_circle_border(x, y, radius, value::int_arg(argv, 3) as pyxel::Color);
    value::return_none();
    true
}

unsafe extern "C" fn elli(_argc: i32, argv: ffi::py_StackRef) -> bool {
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
    pyxel::pyxel().draw_ellipse(x, y, width, height, value::int_arg(argv, 4) as pyxel::Color);
    value::return_none();
    true
}

unsafe extern "C" fn ellib(_argc: i32, argv: ffi::py_StackRef) -> bool {
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
    pyxel::pyxel().draw_ellipse_border(
        x,
        y,
        width,
        height,
        value::int_arg(argv, 4) as pyxel::Color,
    );
    value::return_none();
    true
}

unsafe extern "C" fn tri(_argc: i32, argv: ffi::py_StackRef) -> bool {
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
    let Some(x3) = value::float_arg(argv, 4) else {
        return false;
    };
    let Some(y3) = value::float_arg(argv, 5) else {
        return false;
    };
    pyxel::pyxel().draw_triangle(
        x1,
        y1,
        x2,
        y2,
        x3,
        y3,
        value::int_arg(argv, 6) as pyxel::Color,
    );
    value::return_none();
    true
}

unsafe extern "C" fn trib(_argc: i32, argv: ffi::py_StackRef) -> bool {
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
    let Some(x3) = value::float_arg(argv, 4) else {
        return false;
    };
    let Some(y3) = value::float_arg(argv, 5) else {
        return false;
    };
    pyxel::pyxel().draw_triangle_border(
        x1,
        y1,
        x2,
        y2,
        x3,
        y3,
        value::int_arg(argv, 6) as pyxel::Color,
    );
    value::return_none();
    true
}

unsafe extern "C" fn fill(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(x) = value::float_arg(argv, 0) else {
        return false;
    };
    let Some(y) = value::float_arg(argv, 1) else {
        return false;
    };
    pyxel::pyxel().flood_fill(x, y, value::int_arg(argv, 2) as pyxel::Color);
    value::return_none();
    true
}

unsafe extern "C" fn blt(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(x) = value::float_arg(argv, 0) else {
        return false;
    };
    let Some(y) = value::float_arg(argv, 1) else {
        return false;
    };
    let image = value::int_arg(argv, 2) as u32;
    let Some(u) = value::float_arg(argv, 3) else {
        return false;
    };
    let Some(v) = value::float_arg(argv, 4) else {
        return false;
    };
    let Some(width) = value::float_arg(argv, 5) else {
        return false;
    };
    let Some(height) = value::float_arg(argv, 6) else {
        return false;
    };
    pyxel::pyxel().draw_image(
        x,
        y,
        image,
        u,
        v,
        width,
        height,
        value::opt_int_arg(argv, 7).map(|color| color as pyxel::Color),
        value::opt_float_arg(argv, 8),
        value::opt_float_arg(argv, 9),
    );
    value::return_none();
    true
}

unsafe extern "C" fn bltm(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(x) = value::float_arg(argv, 0) else {
        return false;
    };
    let Some(y) = value::float_arg(argv, 1) else {
        return false;
    };
    let tilemap = value::int_arg(argv, 2) as u32;
    let Some(u) = value::float_arg(argv, 3) else {
        return false;
    };
    let Some(v) = value::float_arg(argv, 4) else {
        return false;
    };
    let Some(width) = value::float_arg(argv, 5) else {
        return false;
    };
    let Some(height) = value::float_arg(argv, 6) else {
        return false;
    };
    pyxel::pyxel().draw_tilemap(
        x,
        y,
        tilemap,
        u,
        v,
        width,
        height,
        value::opt_int_arg(argv, 7).map(|color| color as pyxel::Color),
        value::opt_float_arg(argv, 8),
        value::opt_float_arg(argv, 9),
    );
    value::return_none();
    true
}

unsafe extern "C" fn blt3d(_argc: i32, argv: ffi::py_StackRef) -> bool {
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
    let image = value::int_arg(argv, 4) as u32;
    let Some(pos) = value::tuple3_float_arg(argv, 5) else {
        return false;
    };
    let Some(rot) = value::tuple3_float_arg(argv, 6) else {
        return false;
    };
    pyxel::pyxel().draw_image_3d(
        x,
        y,
        width,
        height,
        image,
        pos,
        rot,
        value::opt_float_arg(argv, 7),
        value::opt_int_arg(argv, 8).map(|color| color as pyxel::Color),
    );
    value::return_none();
    true
}

unsafe extern "C" fn bltm3d(_argc: i32, argv: ffi::py_StackRef) -> bool {
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
    let tilemap = value::int_arg(argv, 4) as u32;
    let Some(pos) = value::tuple3_float_arg(argv, 5) else {
        return false;
    };
    let Some(rot) = value::tuple3_float_arg(argv, 6) else {
        return false;
    };
    pyxel::pyxel().draw_tilemap_3d(
        x,
        y,
        width,
        height,
        tilemap,
        pos,
        rot,
        value::opt_float_arg(argv, 7),
        value::opt_int_arg(argv, 8).map(|color| color as pyxel::Color),
    );
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
    ffi::py_bind(
        module,
        c"clip(x=None, y=None, w=None, h=None)".as_ptr(),
        Some(clip),
    );
    ffi::py_bind(module, c"camera(x=None, y=None)".as_ptr(), Some(camera));
    ffi::py_bind(module, c"pal(col1=None, col2=None)".as_ptr(), Some(pal));
    ffi::py_bind(module, c"dither(alpha)".as_ptr(), Some(dither));
    ffi::py_bind(module, c"cls(col)".as_ptr(), Some(cls));
    ffi::py_bind(module, c"pget(x, y)".as_ptr(), Some(pget));
    ffi::py_bind(module, c"pset(x, y, col)".as_ptr(), Some(pset));
    ffi::py_bind(module, c"line(x1, y1, x2, y2, col)".as_ptr(), Some(line));
    ffi::py_bind(module, c"rect(x, y, w, h, col)".as_ptr(), Some(rect));
    ffi::py_bind(module, c"rectb(x, y, w, h, col)".as_ptr(), Some(rectb));
    ffi::py_bind(module, c"circ(x, y, r, col)".as_ptr(), Some(circ));
    ffi::py_bind(module, c"circb(x, y, r, col)".as_ptr(), Some(circb));
    ffi::py_bind(module, c"elli(x, y, w, h, col)".as_ptr(), Some(elli));
    ffi::py_bind(module, c"ellib(x, y, w, h, col)".as_ptr(), Some(ellib));
    ffi::py_bind(
        module,
        c"tri(x1, y1, x2, y2, x3, y3, col)".as_ptr(),
        Some(tri),
    );
    ffi::py_bind(
        module,
        c"trib(x1, y1, x2, y2, x3, y3, col)".as_ptr(),
        Some(trib),
    );
    ffi::py_bind(module, c"fill(x, y, col)".as_ptr(), Some(fill));
    ffi::py_bind(
        module,
        c"blt(x, y, img, u, v, w, h, colkey=None, rotate=None, scale=None)".as_ptr(),
        Some(blt),
    );
    ffi::py_bind(
        module,
        c"bltm(x, y, tm, u, v, w, h, colkey=None, rotate=None, scale=None)".as_ptr(),
        Some(bltm),
    );
    ffi::py_bind(
        module,
        c"blt3d(x, y, w, h, img, pos, rot, fov=None, colkey=None)".as_ptr(),
        Some(blt3d),
    );
    ffi::py_bind(
        module,
        c"bltm3d(x, y, w, h, tm, pos, rot, fov=None, colkey=None)".as_ptr(),
        Some(bltm3d),
    );
    ffi::py_bind(
        module,
        c"text(x, y, s, col, font=None)".as_ptr(),
        Some(text),
    );
}
