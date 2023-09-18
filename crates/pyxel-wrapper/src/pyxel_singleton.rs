use std::mem::transmute;
use std::ptr::null_mut;

use pyxel::Pyxel;

static mut PYXEL: *mut Pyxel = null_mut();

pub fn pyxel() -> &'static mut Pyxel {
    unsafe {
        if PYXEL != null_mut() {
            &mut *PYXEL
        } else {
            panic!("Pyxel not initialized");
        }
    }
}

pub fn set_pyxel_instance(pyxel: Pyxel) {
    unsafe {
        PYXEL = transmute(Box::new(pyxel));
    }
}
