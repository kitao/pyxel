use std::sync::Once;

use pyo3::prelude::*;

static NOISE_ONCE: Once = Once::new();
static WAVEFORM_ONCE: Once = Once::new();

wrap_as_python_sequence!(
    Wavetable,
    *mut pyxel::Tone,
    (|inner: &*mut pyxel::Tone| unsafe { &**inner }.wavetable.len()),
    pyxel::ToneSample,
    (|inner: &*mut pyxel::Tone, index| unsafe { &**inner }.wavetable[index]),
    pyxel::ToneSample,
    (|inner: &*mut pyxel::Tone, index, value| unsafe { &mut **inner }.wavetable[index] = value),
    Vec<pyxel::ToneSample>,
    (|inner: &*mut pyxel::Tone, list| unsafe { &mut **inner }.wavetable = list),
    (|inner: &*mut pyxel::Tone| unsafe { &**inner }.wavetable.clone())
);

#[pyclass(from_py_object)]
#[derive(Clone, Copy)]
pub struct Tone {
    pub(crate) inner: *mut pyxel::Tone,
}

unsafe impl Send for Tone {}
unsafe impl Sync for Tone {}

impl Tone {
    pub fn wrap(inner: *mut pyxel::Tone) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Tone {
    #[new]
    fn new() -> Self {
        Self::wrap(pyxel::Tone::new())
    }

    #[getter]
    fn mode(&self) -> u32 {
        unsafe { &*self.inner }.mode.into()
    }

    #[setter]
    fn set_mode(&self, mode: u32) {
        unsafe { &mut *self.inner }.mode = pyxel::ToneMode::from(mode);
    }

    #[getter]
    fn noise(&self) -> u32 {
        NOISE_ONCE.call_once(|| {
            println!("Tone.noise is deprecated. Use Tone.mode instead.");
        });

        unsafe { &*self.inner }.mode.into()
    }

    #[setter]
    fn set_noise(&self, mode: u32) {
        NOISE_ONCE.call_once(|| {
            println!("Tone.noise is deprecated. Use Tone.mode instead.");
        });

        unsafe { &mut *self.inner }.mode = pyxel::ToneMode::from(mode);
    }

    #[getter]
    fn sample_bits(&self) -> pyxel::ToneSample {
        unsafe { &*self.inner }.sample_bits
    }

    #[setter]
    fn set_sample_bits(&self, sample_bits: pyxel::ToneSample) {
        unsafe { &mut *self.inner }.sample_bits = sample_bits;
    }

    #[getter]
    fn wavetable(&self) -> Wavetable {
        Wavetable::wrap(self.inner)
    }

    #[getter]
    fn waveform(&self) -> Wavetable {
        WAVEFORM_ONCE.call_once(|| {
            println!("Tone.waveform is deprecated. Use Tone.wavetable instead.");
        });

        Wavetable::wrap(self.inner)
    }

    #[getter]
    fn gain(&self) -> pyxel::ToneGain {
        unsafe { &*self.inner }.gain
    }

    #[setter]
    fn set_gain(&self, gain: pyxel::ToneGain) {
        unsafe { &mut *self.inner }.gain = gain;
    }
}

pub fn add_tone_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Tone>()?;
    Ok(())
}
