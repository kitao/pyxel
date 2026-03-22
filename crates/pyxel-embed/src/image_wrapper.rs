use rustpython_vm::builtins::PyTypeRef;
use rustpython_vm::function::FuncArgs;
use rustpython_vm::types::Constructor;
use rustpython_vm::{pyclass, PyObjectRef, PyPayload, PyResult, VirtualMachine};

use crate::helpers::*;

#[pyclass(module = "pyxel", name = "Image")]
#[derive(Debug, PyPayload)]
pub struct PyImage {
    pub inner: *mut pyxel::Image,
}

unsafe impl Send for PyImage {}
unsafe impl Sync for PyImage {}

impl PyImage {
    pub fn wrap(inner: *mut pyxel::Image) -> Self {
        Self { inner }
    }

    #[allow(clippy::mut_from_ref)]
    fn img_mut(&self) -> &mut pyxel::Image {
        unsafe { &mut *self.inner }
    }
}

#[pyclass(with(Constructor))]
impl PyImage {
    #[pyclassmethod]
    fn from_image(_cls: PyTypeRef, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
        let a = &args.args;
        let filename = s(&a[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
        let include_colors = args
            .kwargs
            .get("include_colors")
            .map(|o| {
                use rustpython_vm::builtins::PyInt;
                o.payload::<PyInt>()
                    .map(|v| {
                        let i: i64 = v.as_bigint().try_into().unwrap_or(0);
                        i != 0
                    })
                    .unwrap_or(false)
            })
            .or_else(|| ob(a, 1, vm));
        pyxel::Image::from_image(filename, include_colors)
            .map(Self::wrap)
            .map_err(|e| vm.new_value_error(e))
    }

    #[pygetset]
    fn width(&self) -> u32 {
        unsafe { &*self.inner }.width()
    }

    #[pygetset]
    fn height(&self) -> u32 {
        unsafe { &*self.inner }.height()
    }

    #[pymethod]
    fn set(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        let x = i(&a[0], vm)?;
        let y = i(&a[1], vm)?;
        let data = extract_str_vec(&a[2], vm)?;
        let data_refs: Vec<&str> = data.iter().map(|s| s.as_str()).collect();
        unsafe { &mut *self.inner }.set(x, y, &data_refs);
        Ok(())
    }

    #[pymethod]
    fn load(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        let x = i(&a[0], vm)?;
        let y = i(&a[1], vm)?;
        let filename = s(&a[2]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
        let include_colors = args
            .kwargs
            .get("include_colors")
            .map(to_bool)
            .or_else(|| ob(a, 3, vm));
        unsafe { &mut *self.inner }
            .load(x, y, filename, include_colors)
            .map_err(|e| vm.new_value_error(e))
    }

    #[pymethod]
    fn save(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        let filename = s(&a[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
        let scale = u(&a[1], vm)?;
        unsafe { &mut *self.inner }
            .save(filename, scale)
            .map_err(|e| vm.new_value_error(e))
    }

    #[pymethod]
    fn cls(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        unsafe { &mut *self.inner }.clear(c(&args.args[0], vm)?);
        Ok(())
    }

    #[pymethod]
    fn pget(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<u8> {
        let a = &args.args;
        Ok(unsafe { &mut *self.inner }.get_pixel(f(&a[0], vm)?, f(&a[1], vm)?))
    }

    #[pymethod]
    fn pset(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        unsafe { &mut *self.inner }.set_pixel(f(&a[0], vm)?, f(&a[1], vm)?, c(&a[2], vm)?);
        Ok(())
    }

    #[pymethod]
    fn line(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.img_mut().draw_line(
            f(&a[0], vm)?,
            f(&a[1], vm)?,
            f(&a[2], vm)?,
            f(&a[3], vm)?,
            c(&a[4], vm)?,
        );
        Ok(())
    }

    #[pymethod]
    fn rect(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.img_mut().draw_rect(
            f(&a[0], vm)?,
            f(&a[1], vm)?,
            f(&a[2], vm)?,
            f(&a[3], vm)?,
            c(&a[4], vm)?,
        );
        Ok(())
    }

    #[pymethod]
    fn rectb(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.img_mut().draw_rect_border(
            f(&a[0], vm)?,
            f(&a[1], vm)?,
            f(&a[2], vm)?,
            f(&a[3], vm)?,
            c(&a[4], vm)?,
        );
        Ok(())
    }

    #[pymethod]
    fn circ(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.img_mut()
            .draw_circle(f(&a[0], vm)?, f(&a[1], vm)?, f(&a[2], vm)?, c(&a[3], vm)?);
        Ok(())
    }

    #[pymethod]
    fn circb(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.img_mut().draw_circle_border(
            f(&a[0], vm)?,
            f(&a[1], vm)?,
            f(&a[2], vm)?,
            c(&a[3], vm)?,
        );
        Ok(())
    }

    #[pymethod]
    fn elli(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.img_mut().draw_ellipse(
            f(&a[0], vm)?,
            f(&a[1], vm)?,
            f(&a[2], vm)?,
            f(&a[3], vm)?,
            c(&a[4], vm)?,
        );
        Ok(())
    }

    #[pymethod]
    fn ellib(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.img_mut().draw_ellipse_border(
            f(&a[0], vm)?,
            f(&a[1], vm)?,
            f(&a[2], vm)?,
            f(&a[3], vm)?,
            c(&a[4], vm)?,
        );
        Ok(())
    }

    #[pymethod]
    fn tri(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.img_mut().draw_triangle(
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

    #[pymethod]
    fn trib(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.img_mut().draw_triangle_border(
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

    #[pymethod]
    fn fill(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.img_mut()
            .flood_fill(f(&a[0], vm)?, f(&a[1], vm)?, c(&a[2], vm)?);
        Ok(())
    }

    #[pymethod]
    fn blt(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        let x = f(&a[0], vm)?;
        let y = f(&a[1], vm)?;
        let u_coord = f(&a[3], vm)?;
        let v_coord = f(&a[4], vm)?;
        let w = f(&a[5], vm)?;
        let h = f(&a[6], vm)?;
        let colkey = oc(a, 7, vm)?;
        let rotate = of(a, 8, vm)?;
        let scale = of(a, 9, vm)?;

        // img arg: int index or Image object
        let img_obj = &a[2];
        let img_ptr = if let Ok(idx) = u(img_obj, vm) {
            *pyxel::images()
                .get(idx as usize)
                .ok_or_else(|| vm.new_value_error("invalid image index".into()))?
        } else if let Some(img) = img_obj.payload::<PyImage>() {
            img.inner
        } else {
            return Err(vm.new_type_error("expected int or Image".into()));
        };
        unsafe {
            self.img_mut()
                .draw_image(x, y, img_ptr, u_coord, v_coord, w, h, colkey, rotate, scale);
        }
        Ok(())
    }

    #[pymethod]
    fn clip(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        if a.is_empty() {
            self.img_mut().reset_clip_rect();
        } else {
            self.img_mut().set_clip_rect(
                f(&a[0], vm)?,
                f(&a[1], vm)?,
                f(&a[2], vm)?,
                f(&a[3], vm)?,
            );
        }
        Ok(())
    }

    #[pymethod]
    fn text(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        let x = f(&a[0], vm)?;
        let y = f(&a[1], vm)?;
        let text_str = s(&a[2]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
        let col = c(&a[3], vm)?;
        let font = a
            .get(4)
            .and_then(|o| o.payload::<crate::font_wrapper::PyFont>())
            .map(|f| f.inner);
        self.img_mut().draw_text(x, y, text_str, col, font);
        Ok(())
    }

    #[pymethod]
    fn camera(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        if a.is_empty() {
            self.img_mut().reset_draw_offset();
        } else {
            self.img_mut().set_draw_offset(f(&a[0], vm)?, f(&a[1], vm)?);
        }
        Ok(())
    }

    #[pymethod]
    fn pal(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        if a.is_empty() {
            self.img_mut().reset_color_map();
        } else {
            self.img_mut().map_color(c(&a[0], vm)?, c(&a[1], vm)?);
        }
        Ok(())
    }

    #[pymethod]
    fn dither(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        self.img_mut().set_dithering(f(&args.args[0], vm)?);
        Ok(())
    }

    #[pymethod]
    fn bltm(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        let x = f(&a[0], vm)?;
        let y = f(&a[1], vm)?;
        let u_coord = f(&a[3], vm)?;
        let v_coord = f(&a[4], vm)?;
        let w = f(&a[5], vm)?;
        let h = f(&a[6], vm)?;
        let colkey = oc(a, 7, vm)?;
        let rotate = of(a, 8, vm)?;
        let scale = of(a, 9, vm)?;

        let tm_obj = &a[2];
        let tm_ptr = if let Ok(idx) = u(tm_obj, vm) {
            *pyxel::tilemaps()
                .get(idx as usize)
                .ok_or_else(|| vm.new_value_error("invalid tilemap index".into()))?
        } else if let Some(tm) = tm_obj.payload::<crate::tilemap_wrapper::PyTilemap>() {
            tm.inner
        } else {
            return Err(vm.new_type_error("expected int or Tilemap".into()));
        };
        unsafe {
            self.img_mut()
                .draw_tilemap(x, y, tm_ptr, u_coord, v_coord, w, h, colkey, rotate, scale);
        }
        Ok(())
    }

    #[pymethod]
    fn blt3d(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        let x = f(&a[0], vm)?;
        let y = f(&a[1], vm)?;
        let w = f(&a[2], vm)?;
        let h = f(&a[3], vm)?;
        let img_obj = &a[4];
        let pos = extract_f32_tuple3(&a[5], vm)?;
        let rot = extract_f32_tuple3(&a[6], vm)?;
        let fov = of(a, 7, vm)?;
        let colkey = oc(a, 8, vm)?;

        let img_ptr = if let Ok(idx) = u(img_obj, vm) {
            *pyxel::images()
                .get(idx as usize)
                .ok_or_else(|| vm.new_value_error("invalid image index".into()))?
        } else if let Some(img) = img_obj.payload::<PyImage>() {
            img.inner
        } else {
            return Err(vm.new_type_error("expected int or Image".into()));
        };
        unsafe {
            self.img_mut()
                .draw_image_3d(x, y, w, h, img_ptr, pos, rot, fov, colkey);
        }
        Ok(())
    }

    #[pymethod]
    fn bltm3d(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        let x = f(&a[0], vm)?;
        let y = f(&a[1], vm)?;
        let w = f(&a[2], vm)?;
        let h = f(&a[3], vm)?;
        let tm_obj = &a[4];
        let pos = extract_f32_tuple3(&a[5], vm)?;
        let rot = extract_f32_tuple3(&a[6], vm)?;
        let fov = of(a, 7, vm)?;
        let colkey = oc(a, 8, vm)?;

        let tm_ptr = if let Ok(idx) = u(tm_obj, vm) {
            *pyxel::tilemaps()
                .get(idx as usize)
                .ok_or_else(|| vm.new_value_error("invalid tilemap index".into()))?
        } else if let Some(tm) = tm_obj.payload::<crate::tilemap_wrapper::PyTilemap>() {
            tm.inner
        } else {
            return Err(vm.new_type_error("expected int or Tilemap".into()));
        };
        unsafe {
            self.img_mut()
                .draw_tilemap_3d(x, y, w, h, tm_ptr, pos, rot, fov, colkey);
        }
        Ok(())
    }
}

impl Constructor for PyImage {
    type Args = (u32, u32);

    fn py_new(cls: PyTypeRef, (width, height): Self::Args, vm: &VirtualMachine) -> PyResult {
        Self::wrap(pyxel::Image::new(width, height))
            .into_ref_with_type(vm, cls)
            .map(Into::into)
    }
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

// Helper to extract Vec<String> from a Python list or tuple
pub fn extract_str_vec(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<Vec<String>> {
    use rustpython_vm::builtins::{PyList, PyTuple};
    let items: Vec<PyObjectRef> = if let Some(list) = obj.payload::<PyList>() {
        list.borrow_vec().to_vec()
    } else if let Some(tup) = obj.payload::<PyTuple>() {
        tup.as_slice().to_vec()
    } else {
        return Err(vm.new_type_error("expected list or tuple".into()));
    };
    let mut result = Vec::with_capacity(items.len());
    for item in &items {
        let val = s(item).ok_or_else(|| vm.new_type_error("expected str in list".into()))?;
        result.push(val.to_owned());
    }
    Ok(result)
}
