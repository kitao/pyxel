use std::sync::Once;

use pyo3::prelude::*;

static NOISE_ONCE: Once = Once::new();
static WAVEFORM_ONCE: Once = Once::new();

wrap_as_python_list!(
    Wavetable,
    pyxel::SharedTone,
    (|inner: &pyxel::SharedTone| inner.lock().wavetable.len()),
    pyxel::ToneSample,
    (|inner: &pyxel::SharedTone, index| inner.lock().wavetable[index]),
    pyxel::ToneSample,
    (|inner: &pyxel::SharedTone, index, value| inner.lock().wavetable[index] = value),
    Vec<pyxel::ToneSample>,
    (|inner: &pyxel::SharedTone, list| inner.lock().wavetable = list),
    (|inner: &pyxel::SharedTone| inner.lock().wavetable.clone())
);

#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct Tone {
    pub(crate) inner: pyxel::SharedTone,
}

impl Tone {
    pub fn wrap(inner: pyxel::SharedTone) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Tone {
    #[new]
    pub fn new() -> Self {
        Self::wrap(pyxel::Tone::new())
    }

    #[getter]
    pub fn mode(&self) -> u32 {
        self.inner.lock().mode.into()
    }

    #[setter]
    pub fn set_mode(&self, mode: u32) {
        self.inner.lock().mode = pyxel::ToneMode::from(mode);
    }

    #[getter]
    pub fn noise(&self) -> u32 {
        NOISE_ONCE.call_once(|| {
            println!("Tone.noise is deprecated. Use Tone.mode instead.");
        });

        self.inner.lock().mode.into()
    }

    #[setter]
    pub fn set_noise(&self, mode: u32) {
        NOISE_ONCE.call_once(|| {
            println!("Tone.noise is deprecated. Use Tone.mode instead.");
        });

        self.inner.lock().mode = pyxel::ToneMode::from(mode);
    }

    #[getter]
    pub fn sample_bits(&self) -> pyxel::ToneSample {
        self.inner.lock().sample_bits
    }

    #[setter]
    pub fn set_sample_bits(&self, sample_bits: pyxel::ToneSample) {
        self.inner.lock().sample_bits = sample_bits;
    }

    #[getter]
    pub fn wavetable(&self) -> Wavetable {
        Wavetable::wrap(self.inner.clone())
    }

    #[getter]
    pub fn waveform(&self) -> Wavetable {
        WAVEFORM_ONCE.call_once(|| {
            println!("Tone.waveform is deprecated. Use Tone.wavetable instead.");
        });

        Wavetable::wrap(self.inner.clone())
    }

    #[getter]
    pub fn gain(&self) -> pyxel::ToneGain {
        self.inner.lock().gain
    }

    #[setter]
    pub fn set_gain(&self, gain: pyxel::ToneGain) {
        self.inner.lock().gain = gain;
    }
}

pub fn add_tone_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Tone>()?;
    Ok(())
}
