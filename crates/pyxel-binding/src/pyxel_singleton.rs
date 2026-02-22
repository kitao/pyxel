use std::mem::transmute;
use std::ptr::null_mut;

use pyxel::Pyxel;

static mut PYXEL: *mut Pyxel = null_mut();

pub fn pyxel() -> &'static mut Pyxel {
    unsafe {
        if PYXEL.is_null() {
            panic!("Pyxel not initialized");
        } else {
            &mut *PYXEL
        }
    }
}

pub fn set_pyxel_instance(pyxel: Pyxel) {
    unsafe {
        PYXEL = transmute::<Box<Pyxel>, *mut Pyxel>(Box::new(pyxel));
    }
}
