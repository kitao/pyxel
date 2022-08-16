use std::cell::RefCell;
use std::os::raw::c_int;

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
    static MAIN_LOOP_CLOSURE: RefCell<Option<Box<dyn FnMut()>>> = RefCell::new(None);
}

pub fn set_main_loop_callback<F: FnMut()>(callback: F) {
    let callback = move || {
        println!("dummy");
    };

    MAIN_LOOP_CLOSURE.with(|d| {
        *d.borrow_mut() = Some(Box::new(callback));
    });

    unsafe extern "C" fn wrapper<F: FnMut()>() {
        MAIN_LOOP_CLOSURE.with(|z| {
            if let Some(closure) = &mut *z.borrow_mut() {
                (*closure)();
            }
        });
    }

    unsafe {
        emscripten_set_main_loop(wrapper::<F>, 0, 1);
    }
}

pub fn cancel_main_loop() {
    unsafe {
        emscripten_cancel_main_loop();
    }

    MAIN_LOOP_CLOSURE.with(|d| {
        *d.borrow_mut() = None;
    });
}
