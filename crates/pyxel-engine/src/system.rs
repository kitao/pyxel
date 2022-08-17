use std::cmp::min;
use std::process::exit;

use crate::event::Event;
use crate::image::Image;
use crate::input::Input;
use crate::key::{KEY_0, KEY_1, KEY_2, KEY_3, KEY_ALT, KEY_RETURN};
use crate::platform::Platform;
use crate::profiler::Profiler;
use crate::resource::Resource;
use crate::settings::{BACKGROUND_COLOR, MAX_SKIP_FRAMES, NUM_MEASURE_FRAMES};
use crate::types::Key;
use crate::utils::simplify_string;
use crate::{Pyxel, PyxelCallback, COLORS, CURSOR, SCREEN};

pub struct System {
    one_frame_ms: f64,
    next_update_ms: f64,
    disable_next_frame_skip: bool,
    frame_count: u32,
    quit_key: Key,
    is_paused: bool,
    fps_profiler: Profiler,
    update_profiler: Profiler,
    draw_profiler: Profiler,
    enable_perf_monitor: bool,
}

unsafe_singleton!(System);

impl System {
    pub fn init(fps: u32, quit_key: Key) {
        Self::set_instance(Self {
            one_frame_ms: 1000.0 / fps as f64,
            next_update_ms: -1.0,
            disable_next_frame_skip: true,
            frame_count: 0,
            quit_key,
            is_paused: false,
            fps_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            update_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            draw_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            enable_perf_monitor: false,
        });
    }

    pub fn disable_next_frame_skip(&mut self) {
        self.disable_next_frame_skip = true;
    }
}

impl Pyxel {
    pub fn width(&self) -> u32 {
        SCREEN.lock().width()
    }

    pub fn height(&self) -> u32 {
        SCREEN.lock().height()
    }

    pub fn frame_count(&self) -> u32 {
        System::instance().frame_count
    }

    pub fn title(&mut self, title: &str) {
        Platform::instance().set_title(title);
    }

    pub fn icon(&mut self, data_str: &[&str], scale: u32) {
        let width = simplify_string(data_str[0]).len() as u32;
        let height = data_str.len() as u32;
        let image = Image::new(width, height);
        image.lock().set(0, 0, data_str);
        Platform::instance().set_icon(&image.lock().canvas.data, &*COLORS.lock(), scale);
    }

    pub fn is_fullscreen(&self) -> bool {
        Platform::instance().is_fullscreen()
    }

    pub fn fullscreen(&mut self, is_fullscreen: bool) {
        Platform::instance().set_fullscreen(is_fullscreen);
    }

    pub fn run<T: PyxelCallback>(&mut self, callback: &mut T) {
        #[cfg(not(target_os = "emscripten"))]
        loop {
            self.run_one_frame(callback);
            self.wait_for_update_time();
        }

        #[cfg(target_os = "emscripten")]
        {
            emscripten::set_main_loop_callback(move || {
                self.run_one_frame(callback);
            });
        }
    }

    fn run_one_frame<T: PyxelCallback>(&mut self, callback: &mut T) {
        let tick_count = Platform::instance().tick_count();
        let sleep_ms = System::instance().next_update_ms - tick_count as f64;
        if sleep_ms > 0.0 {
            //self.wait_for_update_time();
            return;
        }
        if System::instance().frame_count == 0 {
            System::instance().next_update_ms = tick_count as f64 + System::instance().one_frame_ms;
        } else {
            System::instance().fps_profiler.end(tick_count);
            System::instance().fps_profiler.start(tick_count);
            let update_count: u32;
            if System::instance().disable_next_frame_skip {
                update_count = 1;
                System::instance().disable_next_frame_skip = false;
                System::instance().next_update_ms =
                    Platform::instance().tick_count() as f64 + System::instance().one_frame_ms;
            } else {
                update_count = min(
                    (-sleep_ms / System::instance().one_frame_ms) as u32,
                    MAX_SKIP_FRAMES,
                ) + 1;
                System::instance().next_update_ms +=
                    System::instance().one_frame_ms * update_count as f64;
            }
            for _ in 1..update_count {
                self.update_frame(Some(callback));
                System::instance().frame_count += 1;
            }
        }
        self.update_frame(Some(callback));
        self.draw_frame(Some(callback));
        System::instance().frame_count += 1;
    }

    pub fn show(&mut self) {
        loop {
            self.update_frame(None);
            self.draw_frame(None);
            System::instance().frame_count += 1;
        }
    }

    pub fn flip(&mut self) {
        System::instance().frame_count += 1;
        if System::instance().next_update_ms < 0.0 {
            System::instance().next_update_ms = Platform::instance().tick_count() as f64;
        } else {
            self.wait_for_update_time();
        }
        System::instance().next_update_ms += System::instance().one_frame_ms;
        let tick_count = Platform::instance().tick_count();
        System::instance().fps_profiler.end(tick_count);
        System::instance().fps_profiler.start(tick_count);
        self.update_frame(None);
        self.draw_frame(None);
    }

    pub fn quit(&mut self) {
        exit(0);
    }

    fn update_frame(&mut self, callback: Option<&mut dyn PyxelCallback>) {
        System::instance()
            .update_profiler
            .start(Platform::instance().tick_count());
        self.process_events();
        if System::instance().is_paused {
            return;
        }
        self.check_special_input();
        if let Some(callback) = callback {
            callback.update(self);
        }
        System::instance()
            .update_profiler
            .end(Platform::instance().tick_count());
    }

    fn process_events(&mut self) {
        Input::instance().reset_input_states();
        while let Some(event) = Platform::instance().poll_event() {
            match event {
                Event::Quit => {
                    self.quit();
                }
                Event::Shown => {
                    System::instance().is_paused = false;
                    System::instance().disable_next_frame_skip = true;
                    Platform::instance().resume_audio();
                }
                Event::Hidden => {
                    System::instance().is_paused = true;
                    Platform::instance().pause_audio();
                }
                _ => {
                    if !System::instance().is_paused {
                        Input::instance()
                            .process_input_event(event, System::instance().frame_count);
                    }
                }
            }
        }
    }

    fn check_special_input(&mut self) {
        if self.btn(KEY_ALT) {
            if self.btnp(KEY_RETURN, None, None) {
                self.fullscreen(!self.is_fullscreen());
            }
            if self.btnp(KEY_0, None, None) {
                System::instance().enable_perf_monitor = !System::instance().enable_perf_monitor;
            }
            if self.btnp(KEY_1, None, None) {
                self.screenshot(None);
            }
            if self.btnp(KEY_2, None, None) {
                self.reset_capture();
            }
            if self.btnp(KEY_3, None, None) {
                self.screencast(None);
            }
        }
        if self.btnp(System::instance().quit_key, None, None) {
            self.quit();
        }
    }

    fn wait_for_update_time(&mut self) {
        loop {
            let sleep_ms =
                System::instance().next_update_ms - Platform::instance().tick_count() as f64;
            if sleep_ms <= 0.0 {
                return;
            }
            Platform::instance().sleep((sleep_ms / 2.0) as u32);
        }
    }

    fn draw_frame(&mut self, callback: Option<&mut dyn PyxelCallback>) {
        if System::instance().is_paused {
            return;
        }
        System::instance()
            .draw_profiler
            .start(Platform::instance().tick_count());
        if let Some(callback) = callback {
            callback.draw(self);
        }
        self.draw_perf_monitor();
        self.draw_cursor();
        Platform::instance().render_screen(
            &SCREEN.lock().canvas.data,
            &*COLORS.lock(),
            BACKGROUND_COLOR,
        );
        Resource::instance().capture_screen(
            &SCREEN.lock().canvas.data,
            &*COLORS.lock(),
            System::instance().frame_count,
        );
        System::instance()
            .draw_profiler
            .end(Platform::instance().tick_count());
    }

    fn draw_perf_monitor(&mut self) {
        if !System::instance().enable_perf_monitor {
            return;
        }
        let mut screen = SCREEN.lock();
        let clip_rect = screen.canvas.clip_rect;
        let camera_x = screen.canvas.camera_x;
        let camera_y = screen.canvas.camera_y;
        let palette1 = screen.palette[1];
        let palette2 = screen.palette[2];
        screen.clip0();
        screen.camera0();
        screen.pal(1, 1);
        screen.pal(2, 9);

        let fps = format!("{:.*}", 2, System::instance().fps_profiler.average_fps());
        screen.text(1.0, 0.0, &fps, 1);
        screen.text(0.0, 0.0, &fps, 2);

        let update_time = format!(
            "{:.*}",
            2,
            System::instance().update_profiler.average_time()
        );
        screen.text(1.0, 6.0, &update_time, 1);
        screen.text(0.0, 6.0, &update_time, 2);

        let draw_time = format!("{:.*}", 2, System::instance().draw_profiler.average_time());
        screen.text(1.0, 12.0, &draw_time, 1);
        screen.text(0.0, 12.0, &draw_time, 2);

        screen.canvas.clip_rect = clip_rect;
        screen.canvas.camera_x = camera_x;
        screen.canvas.camera_y = camera_y;
        screen.pal(1, palette1);
        screen.pal(2, palette2);
    }

    fn draw_cursor(&mut self) {
        let x = self.mouse_x();
        let y = self.mouse_y();
        Platform::instance()
            .show_cursor(x < 0 || x >= self.width() as i32 || y < 0 || y >= self.height() as i32);
        if !Input::instance().is_mouse_visible() {
            return;
        }
        let width = CURSOR.lock().width() as i32;
        let height = CURSOR.lock().height() as i32;
        if x <= -width || x >= self.width() as i32 || y <= -height || y >= self.height() as i32 {
            return;
        }
        let mut screen = SCREEN.lock();
        let clip_rect = screen.canvas.clip_rect;
        let camera_x = screen.canvas.camera_x;
        let camera_y = screen.canvas.camera_y;
        let palette = screen.palette;
        screen.clip0();
        screen.camera0();
        screen.pal0();
        screen.blt(
            x as f64,
            y as f64,
            CURSOR.clone(),
            0.0,
            0.0,
            width as f64,
            height as f64,
            Some(0),
        );
        screen.canvas.clip_rect = clip_rect;
        screen.canvas.camera_x = camera_x;
        screen.canvas.camera_y = camera_y;
        screen.palette = palette;
    }
}

#[cfg(target_os = "emscripten")]
mod emscripten {
    use std::cell::RefCell;
    use std::mem::transmute;
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
        let callback =
            unsafe { transmute::<Box<dyn FnMut()>, Box<dyn FnMut()>>(Box::new(callback)) };

        /*let callback = Box::new(|| {
            println!("hoge");
        });*/

        MAIN_LOOP_CLOSURE.with(|d| {
            *d.borrow_mut() = Some(callback);
        });

        unsafe extern "C" fn wrapper<F: FnMut()>() {
            MAIN_LOOP_CLOSURE.with(|z| {
                if let Some(closure) = &mut *z.borrow_mut() {
                    println!("loop!!");
                    (*closure)();
                    println!("loop end!!");
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
}
