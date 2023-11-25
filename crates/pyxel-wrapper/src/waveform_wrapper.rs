use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Waveform {
    pub(crate) inner: pyxel::SharedWaveform,
}

impl Waveform {
    pub fn wrap(inner: pyxel::SharedWaveform) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Waveform {
    #[new]
    pub fn new() -> Self {
        Self::wrap(pyxel::Waveform::new())
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
    pub fn table(&self) -> pyxel::WaveformTable {
        self.inner.lock().table
    }

    #[setter]
    pub fn set_table(&self, table: pyxel::WaveformTable) {
        self.inner.lock().table = table;
    }
}

pub fn add_waveform_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Waveform>()?;
    Ok(())
}
