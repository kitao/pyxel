use rustpython_vm::builtins::PyTypeRef;
use rustpython_vm::function::FuncArgs;
use rustpython_vm::types::Constructor;
use rustpython_vm::{pyclass, PyPayload, PyResult, VirtualMachine};

use crate::helpers::*;

#[pyclass(module = "pyxel", name = "Font")]
#[derive(Debug, PyPayload)]
pub struct PyFont {
    pub inner: *mut pyxel::Font,
}

unsafe impl Send for PyFont {}
unsafe impl Sync for PyFont {}

impl PyFont {
    pub fn wrap(inner: *mut pyxel::Font) -> Self {
        Self { inner }
    }
}

#[pyclass(with(Constructor))]
impl PyFont {
    #[pymethod]
    fn text_width(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<i32> {
        let text = s(&args.args[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
        Ok(unsafe { &mut *self.inner }.text_width(text))
    }
}

impl Constructor for PyFont {
    type Args = FuncArgs;

    fn py_new(cls: PyTypeRef, args: Self::Args, vm: &VirtualMachine) -> PyResult {
        let a = &args.args;
        let filename = s(&a[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
        let font_size = of(a, 1, vm)?;
        let font = pyxel::Font::new(filename, font_size).map_err(|e| vm.new_value_error(e))?;
        Self::wrap(font).into_ref_with_type(vm, cls).map(Into::into)
    }
}
