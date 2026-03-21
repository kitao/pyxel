use std::sync::Once;

use rustpython_vm::function::FuncArgs;
use rustpython_vm::{pyclass, PyObjectRef, PyPayload, PyResult, VirtualMachine};

use crate::helpers::*;

static OLD_MML_ONCE: Once = Once::new();

#[pyclass(module = "pyxel", name = "Sound")]
#[derive(Debug, PyPayload)]
pub struct PySound {
    pub inner: *mut pyxel::Sound,
}

unsafe impl Send for PySound {}
unsafe impl Sync for PySound {}

impl PySound {
    pub fn wrap(inner: *mut pyxel::Sound) -> Self {
        Self { inner }
    }

    fn snd(&self) -> &pyxel::Sound {
        unsafe { &*self.inner }
    }

    #[allow(clippy::mut_from_ref)]
    fn snd_mut(&self) -> &mut pyxel::Sound {
        unsafe { &mut *self.inner }
    }
}

#[pyclass]
impl PySound {
    // Properties (live wrappers)
    #[pygetset]
    fn notes(&self, vm: &VirtualMachine) -> PyObjectRef {
        vm.new_pyobj(crate::seq_wrapper::PyNotes::wrap(self.inner))
    }

    #[pygetset]
    fn tones(&self, vm: &VirtualMachine) -> PyObjectRef {
        vm.new_pyobj(crate::seq_wrapper::PyTones::wrap(self.inner))
    }

    #[pygetset]
    fn volumes(&self, vm: &VirtualMachine) -> PyObjectRef {
        vm.new_pyobj(crate::seq_wrapper::PyVolumes::wrap(self.inner))
    }

    #[pygetset]
    fn effects(&self, vm: &VirtualMachine) -> PyObjectRef {
        vm.new_pyobj(crate::seq_wrapper::PyEffects::wrap(self.inner))
    }

    #[pygetset]
    fn speed(&self) -> u16 {
        self.snd().speed
    }

    #[pygetset(setter)]
    fn set_speed(&self, speed: u32) {
        self.snd_mut().speed = speed as u16;
    }

    // Methods
    #[pymethod]
    fn set(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        // Support both positional and keyword arguments
        let get_str = |idx: usize, name: &str| -> PyResult<&str> {
            a.get(idx)
                .and_then(|o| s(o))
                .or_else(|| args.kwargs.get(name).and_then(|o| s(o)))
                .ok_or_else(|| vm.new_type_error(format!("expected str for '{name}'")))
        };
        let notes = get_str(0, "notes")?;
        let tones = get_str(1, "tones")?;
        let volumes = get_str(2, "volumes")?;
        let effects = get_str(3, "effects")?;
        let speed = a
            .get(4)
            .map(|o| u(o, vm))
            .or_else(|| args.kwargs.get("speed").map(|o| u(o, vm)))
            .ok_or_else(|| vm.new_type_error("expected int for 'speed'".into()))??
            as u16;
        self.snd_mut()
            .set(notes, tones, volumes, effects, speed)
            .map_err(|e| vm.new_value_error(e))
    }

    #[pymethod]
    fn set_notes(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let notes = s(&args.args[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
        self.snd_mut()
            .set_notes(notes)
            .map_err(|e| vm.new_value_error(e))
    }

    #[pymethod]
    fn set_tones(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let tones = s(&args.args[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
        self.snd_mut()
            .set_tones(tones)
            .map_err(|e| vm.new_value_error(e))
    }

    #[pymethod]
    fn set_volumes(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let volumes = s(&args.args[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
        self.snd_mut()
            .set_volumes(volumes)
            .map_err(|e| vm.new_value_error(e))
    }

    #[pymethod]
    fn set_effects(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let effects = s(&args.args[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
        self.snd_mut()
            .set_effects(effects)
            .map_err(|e| vm.new_value_error(e))
    }

    #[pymethod]
    fn mml(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        match args.args.first() {
            Some(obj) if !vm.is_none(obj) => {
                let code =
                    s(obj).ok_or_else(|| vm.new_type_error("expected str or None".into()))?;
                if code.contains('x') || code.contains('X') || code.contains('~') {
                    OLD_MML_ONCE.call_once(|| {
                        println!("Old MML syntax is deprecated. Use new syntax instead.");
                    });
                    self.snd_mut()
                        .old_mml(code)
                        .map_err(|e| vm.new_value_error(e))
                } else {
                    self.snd_mut()
                        .set_mml(code)
                        .map_err(|e| vm.new_value_error(e))
                }
            }
            _ => {
                self.snd_mut().clear_mml();
                Ok(())
            }
        }
    }

    #[pymethod]
    fn pcm(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        match args.args.first() {
            Some(obj) if !vm.is_none(obj) => {
                let filename =
                    s(obj).ok_or_else(|| vm.new_type_error("expected str or None".into()))?;
                self.snd_mut()
                    .load_pcm(filename)
                    .map_err(|e| vm.new_value_error(e))
            }
            _ => {
                self.snd_mut().clear_pcm();
                Ok(())
            }
        }
    }

    #[pymethod]
    fn save(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        let filename = s(&a[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
        let sec = f(&a[1], vm)?;
        let ffmpeg = ob(a, 2, vm);
        self.snd_mut()
            .save(filename, sec, ffmpeg)
            .map_err(|e| vm.new_value_error(e))
    }

    #[pymethod]
    fn total_sec(&self) -> Option<f32> {
        self.snd().total_seconds()
    }
}
