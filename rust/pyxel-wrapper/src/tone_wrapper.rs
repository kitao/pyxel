use pyo3::prelude::*;

wrap_as_python_list!(
    Waveform,
    pyxel::SharedTone,
    (|inner: &pyxel::SharedTone| inner.lock().waveform.len()),
    pyxel::Amp4,
    (|inner: &pyxel::SharedTone, index| inner.lock().waveform[index]),
    pyxel::Amp4,
    (|inner: &pyxel::SharedTone, index, value| inner.lock().waveform[index] = value),
    pyxel::Waveform,
    (|inner: &pyxel::SharedTone, list| inner.lock().waveform = list),
    (|inner: &pyxel::SharedTone| inner.lock().waveform)
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
    pub fn waveform(&self) -> Waveform {
        Waveform::wrap(self.inner.clone())
    }
}

pub fn add_tone_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Tone>()?;
    Ok(())
}
