use rustpython_vm::builtins::{PyList, PyTypeRef};
use rustpython_vm::function::FuncArgs;
use rustpython_vm::types::Constructor;
use rustpython_vm::{pyclass, PyObjectRef, PyPayload, PyResult, VirtualMachine};

use crate::helpers::*;
use crate::sound_wrapper::PySound;

#[pyclass(module = "pyxel", name = "Channel")]
#[derive(Debug, PyPayload)]
pub struct PyChannel {
    pub inner: *mut pyxel::Channel,
}

unsafe impl Send for PyChannel {}
unsafe impl Sync for PyChannel {}

impl PyChannel {
    pub fn wrap(inner: *mut pyxel::Channel) -> Self {
        Self { inner }
    }

    fn ch(&self) -> &pyxel::Channel {
        unsafe { &*self.inner }
    }

    #[allow(clippy::mut_from_ref)]
    fn ch_mut(&self) -> &mut pyxel::Channel {
        unsafe { &mut *self.inner }
    }
}

#[pyclass(with(Constructor))]
impl PyChannel {
    #[pygetset]
    fn gain(&self) -> f32 {
        self.ch().gain
    }

    #[pygetset(setter)]
    fn set_gain(&self, gain: f32) {
        self.ch_mut().gain = gain;
    }

    #[pygetset]
    fn detune(&self) -> i32 {
        self.ch().detune
    }

    #[pygetset(setter)]
    fn set_detune(&self, detune: i32) {
        self.ch_mut().detune = detune;
    }

    #[pymethod]
    fn play(&self, args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
        let a = &args.args;
        let snd_obj = &a[0];
        let sec = kw_f32(&args, "sec", vm)?.or_else(|| {
            a.get(1)
                .and_then(|o| if vm.is_none(o) { None } else { f(o, vm).ok() })
        });
        let loop_ = args
            .kwargs
            .get("loop")
            .map(to_bool)
            .or_else(|| ob(a, 2, vm))
            .unwrap_or(false);
        let resume = args
            .kwargs
            .get("resume")
            .map(to_bool)
            .or_else(|| ob(a, 3, vm))
            .unwrap_or(false);

        if let Ok(snd_idx) = u(snd_obj, vm) {
            let sound = *pyxel::sounds()
                .get(snd_idx as usize)
                .ok_or_else(|| vm.new_value_error("invalid sound index".into()))?;
            self.ch_mut().play_sound(sound, sec, loop_, resume);
        } else if let Some(sound) = snd_obj.payload::<PySound>() {
            self.ch_mut().play_sound(sound.inner, sec, loop_, resume);
        } else if let Some(list) = snd_obj.payload::<PyList>() {
            let items = list.borrow_vec();
            if items
                .first()
                .is_some_and(|o| o.payload::<PySound>().is_some())
            {
                let sounds: Vec<*mut pyxel::Sound> = items
                    .iter()
                    .map(|item| {
                        item.payload::<PySound>()
                            .map(|s| s.inner)
                            .ok_or_else(|| vm.new_type_error("expected Sound in list".into()))
                    })
                    .collect::<PyResult<_>>()?;
                self.ch_mut().play(sounds, sec, loop_, resume);
            } else {
                let sounds: Vec<*mut pyxel::Sound> = items
                    .iter()
                    .map(|item| {
                        let idx = u(item, vm)? as usize;
                        pyxel::sounds()
                            .get(idx)
                            .copied()
                            .ok_or_else(|| vm.new_value_error("invalid sound index".into()))
                    })
                    .collect::<PyResult<_>>()?;
                self.ch_mut().play(sounds, sec, loop_, resume);
            }
        } else if let Some(code) = s(snd_obj) {
            self.ch_mut()
                .play_mml(code, sec, loop_, resume)
                .map_err(|e| vm.new_value_error(e))?;
        } else {
            return Err(vm.new_type_error("expected int, list, Sound, or str".into()));
        }
        Ok(())
    }

    #[pymethod]
    fn stop(&self) {
        self.ch_mut().stop();
    }

    #[pymethod]
    fn play_pos(&self, vm: &VirtualMachine) -> PyObjectRef {
        match self.ch_mut().play_position() {
            Some((snd, pos)) => vm.new_pyobj((vm.new_pyobj(snd), vm.new_pyobj(pos))),
            None => vm.ctx.none(),
        }
    }
}

impl Constructor for PyChannel {
    type Args = ();

    fn py_new(cls: PyTypeRef, _args: Self::Args, vm: &VirtualMachine) -> PyResult {
        Self::wrap(pyxel::Channel::new())
            .into_ref_with_type(vm, cls)
            .map(Into::into)
    }
}
