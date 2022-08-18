use std::cell::RefCell;
use std::mem::transmute;
use std::os::raw::c_int;

use crate::{PyxelCallback, System};

#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern "C" fn();

extern "C" {
    pub fn emscripten_set_main_loop(
        func: em_callback_func,
        fps: c_int,
        simulate_infinite_loop: c_int,
    );
    pub fn emscripten_cancel_main_loop();
}

thread_local! {
    static PYXEL_CALLBACK: RefCell<Option<Box<dyn PyxelCallback>>> = RefCell::new(None);
}

pub fn set_main_loop_callback<T: PyxelCallback>(callback: T) {
    println!("for buying time");
    println!("for buying time");
    println!("for buying time");
    println!("for buying time");
    println!("for buying time");

    PYXEL_CALLBACK.with(|d| {
        *d.borrow_mut() = Some(unsafe {
            transmute::<Box<dyn PyxelCallback>, Box<dyn PyxelCallback>>(Box::new(callback))
        });
    });

    unsafe extern "C" fn main_loop<T: PyxelCallback>() {
        PYXEL_CALLBACK.with(|d| {
            if let Some(callback) = &mut *d.borrow_mut() {
                System::instance().run_one_frame(&mut **callback);
            }
        });
    }

    unsafe {
        emscripten_set_main_loop(main_loop::<T>, 0, 1);
    }
}

pub fn cancel_main_loop() {
    // Should I call this function in quit()?
    unsafe {
        emscripten_cancel_main_loop();
    }
}
