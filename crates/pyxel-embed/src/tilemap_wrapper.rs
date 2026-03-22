use rustpython_vm::builtins::{PyTuple, PyTypeRef};
use rustpython_vm::function::FuncArgs;
use rustpython_vm::{pyclass, PyObjectRef, PyPayload, PyResult, VirtualMachine};

use crate::helpers::*;
use crate::image_wrapper::PyImage;

// Extract a Tile (u8, u8) from a Python tuple
fn tile(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<pyxel::Tile> {
    let tup = obj
        .payload::<PyTuple>()
        .ok_or_else(|| vm.new_type_error("expected tuple for tile".into()))?;
    let items = tup.as_slice();
    if items.len() != 2 {
        return Err(vm.new_value_error("tile must be a 2-element tuple".into()));
    }
    Ok((c(&items[0], vm)?, c(&items[1], vm)?))
}

// Extract an optional Tile from args at index
fn otile(a: &[PyObjectRef], i: usize, vm: &VirtualMachine) -> PyResult<Option<pyxel::Tile>> {
    match a.get(i) {
        Some(o) if !vm.is_none(o) => Ok(Some(tile(o, vm)?)),
        _ => Ok(None),
    }
}

// Extract a Vec<Tile> from a Python list
fn tile_vec(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<Vec<pyxel::Tile>> {
    use rustpython_vm::builtins::PyList;
    let list = obj
        .payload::<PyList>()
        .ok_or_else(|| vm.new_type_error("expected list of tiles".into()))?;
    let items = list.borrow_vec();
    items.iter().map(|item| tile(item, vm)).collect()
}

#[pyclass(module = "pyxel", name = "Tilemap")]
#[derive(Debug, PyPayload)]
pub struct PyTilemap {
    pub inner: *mut pyxel::Tilemap,
}

unsafe impl Send for PyTilemap {}
unsafe impl Sync for PyTilemap {}

impl PyTilemap {
    pub fn wrap(inner: *mut pyxel::Tilemap) -> Self {
        Self { inner }
    }

    fn tm(&self) -> &pyxel::Tilemap {
        unsafe { &*self.inner }
    }

    #[allow(clippy::mut_from_ref)]
    fn tm_mut(&self) -> &mut pyxel::Tilemap {
        unsafe { &mut *self.inner }
    }
}

fn tile_to_pyobj(t: pyxel::Tile, vm: &VirtualMachine) -> PyObjectRef {
    vm.new_pyobj((vm.new_pyobj(t.0), vm.new_pyobj(t.1)))
}

#[pyclass]
impl PyTilemap {
    #[pyclassmethod]
    fn from_tmx(_cls: PyTypeRef, args: FuncArgs, vm: &VirtualMachine) -> PyResult<Self> {
        let a = &args.args;
        let filename = s(&a[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
        let layer = u(&a[1], vm)?;
        pyxel::Tilemap::from_tmx(filename, layer)
            .map(Self::wrap)
            .map_err(|e| vm.new_value_error(e))
    }

    // Properties
    #[pygetset]
    fn width(&self) -> u32 {
        self.tm().width()
    }

    #[pygetset]
    fn height(&self) -> u32 {
        self.tm().height()
    }

    #[pygetset]
    fn imgsrc(&self, vm: &VirtualMachine) -> PyObjectRef {
        match &self.tm().imgsrc {
            pyxel::ImageSource::Index(index) => vm.new_pyobj(*index),
            pyxel::ImageSource::Image(image) => vm.new_pyobj(PyImage::wrap(*image)),
        }
    }

    #[pygetset(setter)]
    fn set_imgsrc(&self, value: u32) {
        self.tm_mut().imgsrc = pyxel::ImageSource::Index(value);
    }

    // Methods
    #[pymethod]
    fn set(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        let x = f(&a[0], vm)? as i32;
        let y = f(&a[1], vm)? as i32;
        let data = crate::image_wrapper::extract_str_vec(&a[2], vm)?;
        let refs: Vec<&str> = data.iter().map(|s| s.as_str()).collect();
        self.tm_mut().set(x, y, &refs);
        Ok(())
    }

    #[pymethod]
    fn load(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        let x = f(&a[0], vm)? as i32;
        let y = f(&a[1], vm)? as i32;
        let filename = s(&a[2]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
        let layer = u(&a[3], vm)?;
        self.tm_mut()
            .load(x, y, filename, layer)
            .map_err(|e| vm.new_value_error(e))
    }

    #[pymethod]
    fn clip(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        if a.is_empty() {
            self.tm_mut().reset_clip_rect();
        } else if a.len() == 4 {
            self.tm_mut()
                .set_clip_rect(f(&a[0], vm)?, f(&a[1], vm)?, f(&a[2], vm)?, f(&a[3], vm)?);
        } else {
            return Err(vm.new_type_error("clip() takes 0 or 4 arguments".into()));
        }
        Ok(())
    }

    #[pymethod]
    fn camera(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        if a.is_empty() {
            self.tm_mut().reset_draw_offset();
        } else if a.len() == 2 {
            self.tm_mut().set_draw_offset(f(&a[0], vm)?, f(&a[1], vm)?);
        } else {
            return Err(vm.new_type_error("camera() takes 0 or 2 arguments".into()));
        }
        Ok(())
    }

    #[pymethod]
    fn cls(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        self.tm_mut().clear(tile(&args.args[0], vm)?);
        Ok(())
    }

    #[pymethod]
    fn pget(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        let a = &args.args;
        let t = self.tm_mut().get_tile(f(&a[0], vm)?, f(&a[1], vm)?);
        Ok(tile_to_pyobj(t, vm))
    }

    #[pymethod]
    fn pset(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.tm_mut()
            .set_tile(f(&a[0], vm)?, f(&a[1], vm)?, tile(&a[2], vm)?);
        Ok(())
    }

    #[pymethod]
    fn line(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.tm_mut().draw_line(
            f(&a[0], vm)?,
            f(&a[1], vm)?,
            f(&a[2], vm)?,
            f(&a[3], vm)?,
            tile(&a[4], vm)?,
        );
        Ok(())
    }

    #[pymethod]
    fn rect(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.tm_mut().draw_rect(
            f(&a[0], vm)?,
            f(&a[1], vm)?,
            f(&a[2], vm)?,
            f(&a[3], vm)?,
            tile(&a[4], vm)?,
        );
        Ok(())
    }

    #[pymethod]
    fn rectb(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.tm_mut().draw_rect_border(
            f(&a[0], vm)?,
            f(&a[1], vm)?,
            f(&a[2], vm)?,
            f(&a[3], vm)?,
            tile(&a[4], vm)?,
        );
        Ok(())
    }

    #[pymethod]
    fn circ(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.tm_mut().draw_circle(
            f(&a[0], vm)?,
            f(&a[1], vm)?,
            f(&a[2], vm)?,
            tile(&a[3], vm)?,
        );
        Ok(())
    }

    #[pymethod]
    fn circb(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.tm_mut().draw_circle_border(
            f(&a[0], vm)?,
            f(&a[1], vm)?,
            f(&a[2], vm)?,
            tile(&a[3], vm)?,
        );
        Ok(())
    }

    #[pymethod]
    fn elli(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.tm_mut().draw_ellipse(
            f(&a[0], vm)?,
            f(&a[1], vm)?,
            f(&a[2], vm)?,
            f(&a[3], vm)?,
            tile(&a[4], vm)?,
        );
        Ok(())
    }

    #[pymethod]
    fn ellib(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.tm_mut().draw_ellipse_border(
            f(&a[0], vm)?,
            f(&a[1], vm)?,
            f(&a[2], vm)?,
            f(&a[3], vm)?,
            tile(&a[4], vm)?,
        );
        Ok(())
    }

    #[pymethod]
    fn tri(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.tm_mut().draw_triangle(
            f(&a[0], vm)?,
            f(&a[1], vm)?,
            f(&a[2], vm)?,
            f(&a[3], vm)?,
            f(&a[4], vm)?,
            f(&a[5], vm)?,
            tile(&a[6], vm)?,
        );
        Ok(())
    }

    #[pymethod]
    fn trib(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.tm_mut().draw_triangle_border(
            f(&a[0], vm)?,
            f(&a[1], vm)?,
            f(&a[2], vm)?,
            f(&a[3], vm)?,
            f(&a[4], vm)?,
            f(&a[5], vm)?,
            tile(&a[6], vm)?,
        );
        Ok(())
    }

    #[pymethod]
    fn fill(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        self.tm_mut()
            .flood_fill(f(&a[0], vm)?, f(&a[1], vm)?, tile(&a[2], vm)?);
        Ok(())
    }

    #[pymethod]
    fn collide(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        let a = &args.args;
        let (dx, dy) = self.tm_mut().collide(
            f(&a[0], vm)?,
            f(&a[1], vm)?,
            f(&a[2], vm)?,
            f(&a[3], vm)?,
            f(&a[4], vm)?,
            f(&a[5], vm)?,
            &tile_vec(&a[6], vm)?,
        );
        Ok(vm.new_pyobj((vm.new_pyobj(dx), vm.new_pyobj(dy))))
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
        let tilekey = otile(a, 7, vm)?;
        let rotate = of(a, 8, vm)?;
        let scale = of(a, 9, vm)?;

        // tm arg: int index or Tilemap object
        let tm_obj = &a[2];
        let tm_ptr = if let Ok(idx) = u(tm_obj, vm) {
            *pyxel::tilemaps()
                .get(idx as usize)
                .ok_or_else(|| vm.new_value_error("invalid tilemap index".into()))?
        } else if let Some(tm) = tm_obj.payload::<PyTilemap>() {
            tm.inner
        } else {
            return Err(vm.new_type_error("expected int or Tilemap".into()));
        };
        unsafe {
            self.tm_mut()
                .draw_tilemap(x, y, tm_ptr, u_coord, v_coord, w, h, tilekey, rotate, scale);
        }
        Ok(())
    }
}
