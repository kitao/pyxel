use std::mem::transmute;
use std::ptr::null_mut;

use pyxel_engine as pyxel;

static mut PYXEL: *mut pyxel::Pyxel = null_mut();

pub fn pyxel() -> &'static mut pyxel::Pyxel {
    unsafe {
        if is_pyxel_initialized() {
            &mut *PYXEL
        } else {
            panic!("Pyxel not initialized");
        }
    }
}

pub fn is_pyxel_initialized() -> bool {
    unsafe { PYXEL != null_mut() }
}

pub fn set_pyxel_instance(pyxel: pyxel::Pyxel) {
    unsafe {
        PYXEL = transmute(Box::new(pyxel));
    }
}
