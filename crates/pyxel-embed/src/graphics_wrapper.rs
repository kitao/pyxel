use rustpython_vm::function::FuncArgs;
use rustpython_vm::{PyObjectRef, PyResult, VirtualMachine};

use crate::helpers::*;

// Get an optional f32 from kwargs first, then positional args
fn kwarg_or_pos_f(
    args: &FuncArgs,
    name: &str,
    a: &[PyObjectRef],
    idx: usize,
    vm: &VirtualMachine,
) -> PyResult<Option<f32>> {
    if let Some(obj) = args.kwargs.get(name) {
        if vm.is_none(obj) {
            return Ok(None);
        }
        return Ok(Some(f(obj, vm)?));
    }
    of(a, idx, vm)
}

// Get an optional u8 from kwargs first, then positional args
fn kwarg_or_pos_c(
    args: &FuncArgs,
    name: &str,
    a: &[PyObjectRef],
    idx: usize,
    vm: &VirtualMachine,
) -> PyResult<Option<u8>> {
    if let Some(obj) = args.kwargs.get(name) {
        if vm.is_none(obj) {
            return Ok(None);
        }
        return Ok(Some(c(obj, vm)?));
    }
    oc(a, idx, vm)
}

pub fn cls(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    pyxel::pyxel().clear(c(&args.args[0], vm)?);
    Ok(())
}

pub fn pget(args: FuncArgs, vm: &VirtualMachine) -> PyResult<u8> {
    let a = &args.args;
    Ok(pyxel::pyxel().get_pixel(f(&a[0], vm)?, f(&a[1], vm)?))
}

pub fn pset(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    pyxel::pyxel().set_pixel(f(&a[0], vm)?, f(&a[1], vm)?, c(&a[2], vm)?);
    Ok(())
}

pub fn line(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    pyxel::pyxel().draw_line(
        f(&a[0], vm)?,
        f(&a[1], vm)?,
        f(&a[2], vm)?,
        f(&a[3], vm)?,
        c(&a[4], vm)?,
    );
    Ok(())
}

pub fn rect(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    pyxel::pyxel().draw_rect(
        f(&a[0], vm)?,
        f(&a[1], vm)?,
        f(&a[2], vm)?,
        f(&a[3], vm)?,
        c(&a[4], vm)?,
    );
    Ok(())
}

pub fn rectb(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    pyxel::pyxel().draw_rect_border(
        f(&a[0], vm)?,
        f(&a[1], vm)?,
        f(&a[2], vm)?,
        f(&a[3], vm)?,
        c(&a[4], vm)?,
    );
    Ok(())
}

pub fn circ(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    pyxel::pyxel().draw_circle(f(&a[0], vm)?, f(&a[1], vm)?, f(&a[2], vm)?, c(&a[3], vm)?);
    Ok(())
}

pub fn circb(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    pyxel::pyxel().draw_circle_border(f(&a[0], vm)?, f(&a[1], vm)?, f(&a[2], vm)?, c(&a[3], vm)?);
    Ok(())
}

pub fn elli(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    pyxel::pyxel().draw_ellipse(
        f(&a[0], vm)?,
        f(&a[1], vm)?,
        f(&a[2], vm)?,
        f(&a[3], vm)?,
        c(&a[4], vm)?,
    );
    Ok(())
}

pub fn ellib(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    pyxel::pyxel().draw_ellipse_border(
        f(&a[0], vm)?,
        f(&a[1], vm)?,
        f(&a[2], vm)?,
        f(&a[3], vm)?,
        c(&a[4], vm)?,
    );
    Ok(())
}

pub fn tri(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    pyxel::pyxel().draw_triangle(
        f(&a[0], vm)?,
        f(&a[1], vm)?,
        f(&a[2], vm)?,
        f(&a[3], vm)?,
        f(&a[4], vm)?,
        f(&a[5], vm)?,
        c(&a[6], vm)?,
    );
    Ok(())
}

pub fn trib(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    pyxel::pyxel().draw_triangle_border(
        f(&a[0], vm)?,
        f(&a[1], vm)?,
        f(&a[2], vm)?,
        f(&a[3], vm)?,
        f(&a[4], vm)?,
        f(&a[5], vm)?,
        c(&a[6], vm)?,
    );
    Ok(())
}

pub fn fill(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    pyxel::pyxel().flood_fill(f(&a[0], vm)?, f(&a[1], vm)?, c(&a[2], vm)?);
    Ok(())
}

pub fn blt(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    let x = f(&a[0], vm)?;
    let y = f(&a[1], vm)?;
    let u_coord = f(&a[3], vm)?;
    let v_coord = f(&a[4], vm)?;
    let w = f(&a[5], vm)?;
    let h = f(&a[6], vm)?;
    let colkey = kwarg_or_pos_c(&args, "colkey", a, 7, vm)?;
    let rotate = kwarg_or_pos_f(&args, "rotate", a, 8, vm)?;
    let scale = kwarg_or_pos_f(&args, "scale", a, 9, vm)?;

    // img arg: int index or Image object
    let img_obj = &a[2];
    if let Ok(idx) = u(img_obj, vm) {
        pyxel::pyxel().draw_image(x, y, idx, u_coord, v_coord, w, h, colkey, rotate, scale);
    } else if let Some(img) = img_obj.payload::<crate::image_wrapper::PyImage>() {
        unsafe {
            pyxel::screen().draw_image(
                x, y, img.inner, u_coord, v_coord, w, h, colkey, rotate, scale,
            );
        }
    } else {
        return Err(vm.new_type_error("expected int or Image".into()));
    }
    Ok(())
}

pub fn bltm(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    let x = f(&a[0], vm)?;
    let y = f(&a[1], vm)?;
    let u_coord = f(&a[3], vm)?;
    let v_coord = f(&a[4], vm)?;
    let w = f(&a[5], vm)?;
    let h = f(&a[6], vm)?;
    let colkey = kwarg_or_pos_c(&args, "colkey", a, 7, vm)?;
    let rotate = kwarg_or_pos_f(&args, "rotate", a, 8, vm)?;
    let scale = kwarg_or_pos_f(&args, "scale", a, 9, vm)?;

    // tm arg: int index or Tilemap object
    let tm_obj = &a[2];
    if let Ok(idx) = u(tm_obj, vm) {
        pyxel::pyxel().draw_tilemap(x, y, idx, u_coord, v_coord, w, h, colkey, rotate, scale);
    } else if let Some(tm) = tm_obj.payload::<crate::tilemap_wrapper::PyTilemap>() {
        unsafe {
            pyxel::screen().draw_tilemap(
                x, y, tm.inner, u_coord, v_coord, w, h, colkey, rotate, scale,
            );
        }
    } else {
        return Err(vm.new_type_error("expected int or Tilemap".into()));
    }
    Ok(())
}

pub fn text(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    let text_str = s(&a[2]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
    let x = f(&a[0], vm)?;
    let y = f(&a[1], vm)?;
    let col = c(&a[3], vm)?;
    // Optional 5th arg: Font object
    let font = a
        .get(4)
        .and_then(|o| o.payload::<crate::font_wrapper::PyFont>())
        .map(|f| f.inner);
    pyxel::pyxel().draw_text(x, y, text_str, col, font);
    Ok(())
}

pub fn clip(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    if a.is_empty() {
        pyxel::pyxel().reset_clip_rect();
    } else {
        pyxel::pyxel().set_clip_rect(f(&a[0], vm)?, f(&a[1], vm)?, f(&a[2], vm)?, f(&a[3], vm)?);
    }
    Ok(())
}

pub fn camera(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    if a.is_empty() {
        pyxel::pyxel().reset_draw_offset();
    } else {
        pyxel::pyxel().set_draw_offset(f(&a[0], vm)?, f(&a[1], vm)?);
    }
    Ok(())
}

pub fn pal(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    if a.is_empty() {
        pyxel::pyxel().reset_color_map();
    } else {
        pyxel::pyxel().map_color(c(&a[0], vm)?, c(&a[1], vm)?);
    }
    Ok(())
}

pub fn dither(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    pyxel::pyxel().set_dithering(f(&args.args[0], vm)?);
    Ok(())
}

pub fn blt3d(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    let x = f(&a[0], vm)?;
    let y = f(&a[1], vm)?;
    let w = f(&a[2], vm)?;
    let h = f(&a[3], vm)?;
    let img_obj = &a[4];
    let pos = extract_f32_tuple3(&a[5], vm)?;
    let rot = extract_f32_tuple3(&a[6], vm)?;
    let fov = args
        .kwargs
        .get("fov")
        .map(|o| f(o, vm))
        .or_else(|| of(a, 7, vm).transpose())
        .transpose()?;
    let colkey = args
        .kwargs
        .get("colkey")
        .map(|o| c(o, vm))
        .or_else(|| oc(a, 8, vm).transpose())
        .transpose()?;

    if let Ok(idx) = u(img_obj, vm) {
        pyxel::pyxel().draw_image_3d(x, y, w, h, idx, pos, rot, fov, colkey);
    } else if let Some(img) = img_obj.payload::<crate::image_wrapper::PyImage>() {
        unsafe {
            pyxel::screen().draw_image_3d(x, y, w, h, img.inner, pos, rot, fov, colkey);
        }
    } else {
        return Err(vm.new_type_error("expected int or Image".into()));
    }
    Ok(())
}

pub fn bltm3d(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    let x = f(&a[0], vm)?;
    let y = f(&a[1], vm)?;
    let w = f(&a[2], vm)?;
    let h = f(&a[3], vm)?;
    // tm arg: int index or Tilemap object
    let tm_obj = &a[4];
    // pos: (f32, f32, f32) tuple
    let pos = extract_f32_tuple3(&a[5], vm)?;
    // rot: (f32, f32, f32) tuple
    let rot = extract_f32_tuple3(&a[6], vm)?;
    let fov = args
        .kwargs
        .get("fov")
        .map(|o| f(o, vm))
        .or_else(|| of(a, 7, vm).transpose())
        .transpose()?;
    let colkey = args
        .kwargs
        .get("colkey")
        .map(|o| c(o, vm))
        .or_else(|| oc(a, 8, vm).transpose())
        .transpose()?;

    if let Ok(idx) = u(tm_obj, vm) {
        pyxel::pyxel().draw_tilemap_3d(x, y, w, h, idx, pos, rot, fov, colkey);
    } else if let Some(tm) = tm_obj.payload::<crate::tilemap_wrapper::PyTilemap>() {
        unsafe {
            pyxel::screen().draw_tilemap_3d(x, y, w, h, tm.inner, pos, rot, fov, colkey);
        }
    } else {
        return Err(vm.new_type_error("expected int or Tilemap".into()));
    }
    Ok(())
}

fn extract_f32_tuple3(
    obj: &rustpython_vm::PyObjectRef,
    vm: &VirtualMachine,
) -> PyResult<(f32, f32, f32)> {
    use rustpython_vm::builtins::PyTuple;
    let tup = obj
        .payload::<PyTuple>()
        .ok_or_else(|| vm.new_type_error("expected tuple of 3 floats".into()))?;
    let items = tup.as_slice();
    if items.len() != 3 {
        return Err(vm.new_value_error("expected tuple of 3 elements".into()));
    }
    Ok((f(&items[0], vm)?, f(&items[1], vm)?, f(&items[2], vm)?))
}
