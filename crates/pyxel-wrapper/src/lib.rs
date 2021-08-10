use pyo3::prelude::*;

use pyxel::Pyxel;

static mut INSTANCE: *mut Pyxel = 0 as *mut Pyxel;

pub fn instance() -> &'static mut Pyxel {
    unsafe {
        if INSTANCE != 0 as *mut Pyxel {
            &mut *INSTANCE
        } else {
            panic!("Pyxel is not initialized");
        }
    }
}

#[pyfunction(
    args = "*",
    title = "None",
    scale = "None",
    fps = "None",
    quit_key = "None"
)]
//#[pyo3(text_signature = "(width, height, / title, [scale, fps, quit_key)")]
fn init(
    width: u32,
    height: u32,
    title: Option<&str>,
    scale: Option<u32>,
    fps: Option<u32>,
    quit_key: Option<pyxel::Key>,
) {
    *instance() = pyxel::Pyxel::new(width, height, title, scale, fps, quit_key);
}

#[pymodule]
fn pyxel_extension(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("TEST", 1234)?;

    m.add_function(wrap_pyfunction!(init, m)?)?;

    Ok(())
}
