use rustpython_vm::builtins::PyList;
use rustpython_vm::function::FuncArgs;
use rustpython_vm::{pyclass, PyObjectRef, PyPayload, PyResult, VirtualMachine};

use crate::helpers::*;

#[pyclass(module = "pyxel", name = "Music")]
#[derive(Debug, PyPayload)]
pub struct PyMusic {
    pub inner: *mut pyxel::Music,
}

unsafe impl Send for PyMusic {}
unsafe impl Sync for PyMusic {}

impl PyMusic {
    pub fn wrap(inner: *mut pyxel::Music) -> Self {
        Self { inner }
    }

    fn mus(&self) -> &pyxel::Music {
        unsafe { &*self.inner }
    }

    #[allow(clippy::mut_from_ref)]
    fn mus_mut(&self) -> &mut pyxel::Music {
        unsafe { &mut *self.inner }
    }
}

// Extract Vec<u32> from a Python list of ints
fn extract_u32_vec(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<Vec<u32>> {
    let list = obj
        .payload::<PyList>()
        .ok_or_else(|| vm.new_type_error("expected list of int".into()))?;
    let items = list.borrow_vec();
    items.iter().map(|item| u(item, vm)).collect()
}

#[pyclass]
impl PyMusic {
    // Property: seqs as list of lists
    #[pygetset]
    fn seqs(&self, vm: &VirtualMachine) -> PyObjectRef {
        let outer: Vec<PyObjectRef> = self
            .mus()
            .seqs
            .iter()
            .map(|seq| {
                let inner: Vec<PyObjectRef> = seq.iter().map(|&v| vm.new_pyobj(v)).collect();
                vm.new_pyobj(inner)
            })
            .collect();
        vm.new_pyobj(outer)
    }

    // set(*seqs) — variable number of list args
    #[pymethod]
    fn set(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let seqs: Vec<Vec<u32>> = args
            .args
            .iter()
            .map(|a| extract_u32_vec(a, vm))
            .collect::<PyResult<_>>()?;
        self.mus_mut().set(&seqs);
        Ok(())
    }

    #[pymethod]
    fn save(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        let filename = s(&a[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
        let sec = f(&a[1], vm)?;
        let ffmpeg = ob(a, 2, vm);
        self.mus_mut()
            .save(filename, sec, ffmpeg)
            .map_err(|e| vm.new_value_error(e))
    }
}
