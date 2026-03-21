use rustpython_vm::builtins::PyTypeRef;
use rustpython_vm::types::Constructor;
use rustpython_vm::{pyclass, PyObjectRef, PyPayload, PyResult, VirtualMachine};

#[pyclass(module = "pyxel", name = "Tone")]
#[derive(Debug, PyPayload)]
pub struct PyTone {
    pub inner: *mut pyxel::Tone,
}

unsafe impl Send for PyTone {}
unsafe impl Sync for PyTone {}

impl PyTone {
    pub fn wrap(inner: *mut pyxel::Tone) -> Self {
        Self { inner }
    }

    fn tone(&self) -> &pyxel::Tone {
        unsafe { &*self.inner }
    }

    #[allow(clippy::mut_from_ref)]
    fn tone_mut(&self) -> &mut pyxel::Tone {
        unsafe { &mut *self.inner }
    }
}

#[pyclass(with(Constructor))]
impl PyTone {
    #[pygetset]
    fn mode(&self) -> u32 {
        self.tone().mode.into()
    }

    #[pygetset(setter)]
    fn set_mode(&self, mode: u32) {
        self.tone_mut().mode = pyxel::ToneMode::from(mode);
    }

    #[pygetset]
    fn sample_bits(&self) -> u32 {
        self.tone().sample_bits
    }

    #[pygetset(setter)]
    fn set_sample_bits(&self, sample_bits: u32) {
        self.tone_mut().sample_bits = sample_bits;
    }

    #[pygetset]
    fn wavetable(&self, vm: &VirtualMachine) -> PyObjectRef {
        vm.new_pyobj(crate::seq_wrapper::PyWavetable::wrap(self.inner))
    }

    #[pygetset]
    fn gain(&self) -> f32 {
        self.tone().gain
    }

    #[pygetset(setter)]
    fn set_gain(&self, gain: f32) {
        self.tone_mut().gain = gain;
    }
}

impl Constructor for PyTone {
    type Args = ();

    fn py_new(cls: PyTypeRef, _args: Self::Args, vm: &VirtualMachine) -> PyResult {
        Self::wrap(pyxel::Tone::new())
            .into_ref_with_type(vm, cls)
            .map(Into::into)
    }
}
