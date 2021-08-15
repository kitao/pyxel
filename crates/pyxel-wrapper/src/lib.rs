mod channel_wrapper;
mod constant_wrapper;
mod graphics_wrapper;
mod image_wrapper;
mod music_wrapper;
mod sound_wrapper;
mod system_wrapper;
mod tilemap_wrapper;
#[allow(non_snake_case)]
mod variable_wrapper;

use std::cmp::max;
use std::mem::transmute;

use pyo3::prelude::*;
use pyxel::Pyxel;

use crate::constant_wrapper::add_module_constants;
use crate::graphics_wrapper::add_graphics_functions;
use crate::system_wrapper::add_system_functions;
use crate::variable_wrapper::add_module_variables;

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

pub fn set_instance(pyxel: Pyxel) {
    unsafe {
        INSTANCE = transmute(Box::new(pyxel));
    }
}

pub fn i32_to_u32(x: i32) -> u32 {
    max(x, 0) as u32
}

#[pymodule]
fn pyxel_wrapper(_py: Python, m: &PyModule) -> PyResult<()> {
    add_module_constants(m)?;
    add_module_variables(m)?;
    add_system_functions(m)?;
    add_graphics_functions(m)?;

    Ok(())
}
