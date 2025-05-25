use std::sync::Once;

use pyo3::prelude::*;

static WAVEFORM_ONCE: Once = Once::new();

wrap_as_python_list!(
    Wavetable,
    pyxel::SharedTone,
    (|inner: &pyxel::SharedTone| inner.lock().wavetable.len()),
    pyxel::WavetableValue,
    (|inner: &pyxel::SharedTone, index| inner.lock().wavetable[index]),
    pyxel::WavetableValue,
    (|inner: &pyxel::SharedTone, index, value| inner.lock().wavetable[index] = value),
    pyxel::Wavetable,
    (|inner: &pyxel::SharedTone, list| inner.lock().wavetable = list),
    (|inner: &pyxel::SharedTone| inner.lock().wavetable)
);

#[pyclass]
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
    pub fn gain(&self) -> pyxel::Gain {
        self.inner.lock().gain
    }

    #[setter]
    pub fn set_gain(&self, gain: pyxel::Gain) {
        self.inner.lock().gain = gain;
    }

    #[getter]
    pub fn noise(&self) -> u32 {
        self.inner.lock().noise.to_index()
    }

    #[setter]
    pub fn set_noise(&self, noise: u32) {
        self.inner.lock().noise = pyxel::Noise::from_index(noise);
    }

    #[getter]
    pub fn wavetable(&self) -> Wavetable {
        Wavetable::wrap(self.inner.clone())
    }

    #[getter]
    pub fn waveform(&self) -> Wavetable {
        WAVEFORM_ONCE.call_once(|| {
            println!("Tone.waveform is deprecated, use pyxel.wavetable instead.");
        });
        Wavetable::wrap(self.inner.clone())
    }
}

pub fn add_tone_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Tone>()?;
    Ok(())
}
