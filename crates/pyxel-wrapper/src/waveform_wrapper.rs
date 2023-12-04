use pyo3::prelude::*;
use pyxel::WaveformTable;

wrap_as_python_list!(
    Table,
    pyxel::SharedWaveform,
    (|inner: &pyxel::SharedWaveform| inner.lock().table.len()),
    pyxel::Amp4,
    (|inner: &pyxel::SharedWaveform, index| inner.lock().table[index]),
    (|inner: &pyxel::SharedWaveform, index, value| inner.lock().table[index] = value),
    pyxel::WaveformTable,
    (|inner: &pyxel::SharedWaveform, list| inner.lock().table = list),
    (|inner: &pyxel::SharedWaveform| inner.lock().table.clone())
);

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
    pub fn table(&self) -> Table {
        Table::wrap(self.inner.clone())
    }
}

pub fn add_waveform_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Waveform>()?;
    Ok(())
}
