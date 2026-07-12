use crate::canvas::Canvas;
use crate::image::{rgb24_to_rgb8, Color, Image, RcImage};
use crate::key::{
    Key, GAMEPAD1_BUTTON_A, GAMEPAD1_BUTTON_B, GAMEPAD1_BUTTON_BACK, GAMEPAD1_BUTTON_DPAD_DOWN,
    GAMEPAD1_BUTTON_DPAD_LEFT, GAMEPAD1_BUTTON_DPAD_RIGHT, GAMEPAD1_BUTTON_DPAD_UP,
    GAMEPAD1_BUTTON_X, GAMEPAD1_BUTTON_Y, KEY_0, KEY_1, KEY_2, KEY_3, KEY_8, KEY_9, KEY_ALT, KEY_R,
    KEY_RETURN, KEY_SHIFT,
};
use crate::platform::{self, Event};
use crate::profiler::Profiler;
use crate::pyxel::{self, Pyxel};
#[cfg(not(target_os = "emscripten"))]
use crate::settings::WINDOW_TO_DISPLAY_RATIO;
use crate::settings::{MAX_FRAME_DELAY_MS, NUM_MEASURE_FRAMES, NUM_SCREEN_MODES};
use crate::window_watcher::WindowWatcher;

pub trait PyxelCallback {
    fn update(&mut self);
    fn draw(&mut self);
}

#[derive(Clone, Copy)]
enum LifecycleAction {
    Quit,
    Restart,
}

pub struct System {
    fps: u32,
    frame_ms: f32,
    quit_key: Key,
    paused: bool,
    fps_profiler: Profiler,
    update_profiler: Profiler,
    draw_profiler: Profiler,
    perf_monitor_enabled: bool,
    integer_scale_enabled: bool,
    window_watcher: WindowWatcher,
    event_buf: Vec<Event>,
    pub screen_x: i32,
    pub screen_y: i32,
    pub screen_scale: f32,
    pub screen_mode: u32,
}

impl System {
    pub fn new(fps: u32, quit_key: Key, headless: bool) -> Self {
        Self {
            fps,
            frame_ms: 1000.0 / fps as f32,
            quit_key,
            paused: false,
            fps_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            update_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            draw_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            perf_monitor_enabled: false,
            integer_scale_enabled: false,
            window_watcher: if headless {
                WindowWatcher::new_headless()
            } else {
                WindowWatcher::new()
            },
            event_buf: Vec::new(),
            screen_x: 0,
            screen_y: 0,
            screen_scale: 0.0,
            screen_mode: 0,
        }
    }
}

impl Pyxel {
    // Main loop

    pub fn run<T: PyxelCallback>(mut callback: T) {
        let (fps, frame_ms) = {
            let pyxel = pyxel::pyxel();
            (pyxel.system.fps, pyxel.system.frame_ms)
        };

        platform::run_frame_loop(fps, move |delta_ms| {
            let ticks = platform::ticks();
            {
                let mut pyxel = pyxel::pyxel();
                pyxel.system.fps_profiler.end(ticks);
                pyxel.system.fps_profiler.start(ticks);
            }

            let update_count = if delta_ms > MAX_FRAME_DELAY_MS as f32 {
                1
            } else {
                (delta_ms / frame_ms) as u32
            };
            for _ in 1..update_count {
                Self::run_update_frame(&mut callback);
                *pyxel::frame_count() += 1;
            }

            Self::run_update_frame(&mut callback);
            Self::run_draw_frame(&mut callback);
            *pyxel::frame_count() += 1;
        });
    }

    pub fn show_screen() {
        struct App {
            image: RcImage,
        }

        impl PyxelCallback for App {
            fn update(&mut self) {}
            fn draw(&mut self) {
                rc_mut!(pyxel::screen()).draw_image(
                    0.0,
                    0.0,
                    &self.image,
                    0.0,
                    0.0,
                    *pyxel::width() as f32,
                    *pyxel::height() as f32,
                    None,
                    None,
                    None,
                );
            }
        }

        let image = Image::new(*pyxel::width(), *pyxel::height());
        rc_mut!(image).draw_image(
            0.0,
            0.0,
            &pyxel::screen(),
            0.0,
            0.0,
            *pyxel::width() as f32,
            *pyxel::height() as f32,
            None,
            None,
            None,
        );

        Self::run(App { image });
    }

    // Window & screen

    pub fn flip_screen() {
        let fps = {
            let mut pyxel = pyxel::pyxel();
            pyxel.system.update_profiler.end(platform::ticks());
            pyxel.draw_frame(None);
            pyxel.system.fps
        };
        *pyxel::frame_count() += 1;

        platform::step_frame(fps);

        let ticks = platform::ticks();
        let action = {
            let mut pyxel = pyxel::pyxel();
            pyxel.system.fps_profiler.end(ticks);
            pyxel.system.fps_profiler.start(ticks);
            let (_, action) = pyxel.begin_update_frame();
            action
        };

        if let Some(action) = action {
            Self::perform_lifecycle_action(action);
        }
    }

    pub fn quit() {
        platform::quit();
    }

    pub fn restart() {
        #[cfg(not(target_os = "emscripten"))]
        if let Some(mut callback) = pyxel::reset_callback().take() {
            let window_state = pyxel::pyxel().system.window_watcher.state_string();
            platform::close_audio();
            callback(window_state);
        }

        #[cfg(target_os = "emscripten")]
        {
            use std::os::raw::c_char;

            extern "C" {
                fn emscripten_run_script(script: *const c_char);
            }

            // SAFETY: Emscripten provides this symbol, and the C string is static
            // and NUL-terminated for the duration of the call.
            unsafe {
                emscripten_run_script(c"resetPyxel();".as_ptr());
            }
        }
    }

    fn perform_lifecycle_action(action: LifecycleAction) {
        match action {
            LifecycleAction::Quit => Self::quit(),
            LifecycleAction::Restart => Self::restart(),
        }
    }

    pub fn set_title(&self, title: &str) {
        if *pyxel::is_headless() {
            return;
        }

        platform::set_window_title(title);
    }

    // Convert icon pattern data into scaled RGBA pixels.
    pub fn set_icon<S: AsRef<str>>(
        &self,
        data: &[S],
        scale: u32,
        transparent: Option<Color>,
    ) -> Result<(), String> {
        let rc = Image::from_data(data, "icon")?;
        if scale == 0 {
            return Err("scale must be greater than 0".to_string());
        }
        let colors = pyxel::colors();
        let image = rc_ref!(rc);
        let width = image.width();
        let height = image.height();
        let image_data = &image.canvas.data;
        let scaled_width = width
            .checked_mul(scale)
            .ok_or("icon dimensions overflow after scaling")?;
        let scaled_height = height
            .checked_mul(scale)
            .ok_or("icon dimensions overflow after scaling")?;
        let rgba_capacity = (scaled_width as usize)
            .checked_mul(scaled_height as usize)
            .and_then(|size| size.checked_mul(4))
            .ok_or("icon dimensions overflow after scaling")?;

        for (index, &color) in image_data.iter().enumerate() {
            if color as usize >= colors.len() {
                return Err(format!(
                    "Invalid icon data at row {}, column {}: color {color} exceeds palette size {}",
                    index / width as usize,
                    index % width as usize,
                    colors.len()
                ));
            }
        }

        if *pyxel::is_headless() {
            return Ok(());
        }

        let mut rgba: Vec<u8> = Vec::with_capacity(rgba_capacity);

        for y in 0..height {
            for _sy in 0..scale {
                for x in 0..width {
                    let color = image_data[(width * y + x) as usize];
                    let (r, g, b) = rgb24_to_rgb8(colors[color as usize]);
                    let a = if Some(color) == transparent { 0 } else { 0xff };
                    for _sx in 0..scale {
                        rgba.push(r);
                        rgba.push(g);
                        rgba.push(b);
                        rgba.push(a);
                    }
                }
            }
        }

        platform::set_window_icon(scaled_width, scaled_height, &rgba);
        Ok(())
    }

    // Screen configuration

    pub fn set_perf_monitor(&mut self, enabled: bool) {
        self.system.perf_monitor_enabled = enabled;
    }

    pub fn set_integer_scale(&mut self, enabled: bool) {
        self.system.integer_scale_enabled = enabled;
    }

    pub fn set_screen_mode(&mut self, screen_mode: u32) {
        self.system.screen_mode = screen_mode;
    }

    pub fn set_fullscreen(&self, enabled: bool) {
        if *pyxel::is_headless() {
            return;
        }

        platform::set_fullscreen(enabled);
    }

    // Resize screen resources while keeping window scaling coherent.
    pub fn set_screen_size(&mut self, width: u32, height: u32) -> Result<(), String> {
        pyxel::validate_resize_params(width, height, *pyxel::is_headless())?;

        *pyxel::width() = width;
        *pyxel::height() = height;

        rc_mut!(pyxel::screen()).canvas = Canvas::new(width, height);

        if let Some(graphics) = &mut self.graphics {
            graphics.invalidate_screen_texture();
        }

        self.reset_screencast();

        if !*pyxel::is_headless() {
            // Keep the current effective scale when possible, so the window
            // follows a resize naturally in windowed mode on native platforms.
            // Cap the scale so the new window fits within the display.
            #[cfg(not(target_os = "emscripten"))]
            if !platform::is_fullscreen() {
                let (display_w, display_h) = platform::display_size();
                let max_scale = f32::min(
                    display_w as f32 * WINDOW_TO_DISPLAY_RATIO / width as f32,
                    display_h as f32 * WINDOW_TO_DISPLAY_RATIO / height as f32,
                )
                .max(1.0);
                let scale = self.system.screen_scale.max(1.0).min(max_scale);
                let new_window_w = (width as f32 * scale).round() as u32;
                let new_window_h = (height as f32 * scale).round() as u32;
                platform::set_window_size(new_window_w, new_window_h);
            }
            self.update_screen_params();
        }
        Ok(())
    }

    // Event & input processing

    // Poll platform events and update input/window state.
    fn process_events(&mut self) -> Option<LifecycleAction> {
        let mut lifecycle_action = None;
        if platform::is_sigint_received() {
            lifecycle_action = Some(LifecycleAction::Quit);
        }

        self.start_input_frame();

        platform::poll_events(&mut self.system.event_buf);
        let mut events = std::mem::take(&mut self.system.event_buf);

        for event in events.drain(..) {
            match event {
                Event::WindowShown => {
                    self.system.paused = false;
                    platform::pause_audio(false);
                }
                Event::WindowHidden => {
                    self.system.paused = true;
                    platform::pause_audio(true);
                }
                Event::KeyPressed { key } => self.press_key(key),
                Event::KeyReleased { key } => self.release_key(key),
                Event::KeyValueChanged { key, value } => self.set_key_value(key, value),
                Event::TextInput { text } => self.add_input_text(&text),
                Event::FileDropped { filename } => self.add_dropped_file(&filename),
                Event::Quit => lifecycle_action = Some(LifecycleAction::Quit),
            }
        }

        // Return the buffer for reuse
        self.system.event_buf = events;
        lifecycle_action
    }

    // Handle hidden capture and display shortcuts before regular update code.
    fn check_special_input(&mut self) -> Option<LifecycleAction> {
        if self.is_button_pressed(self.system.quit_key, None, None) {
            self.reset_key(self.system.quit_key);
            return Some(LifecycleAction::Quit);
        } else if self.is_button_down(KEY_ALT) {
            if self.is_button_down(KEY_SHIFT) {
                if self.is_button_pressed(KEY_0, None, None) {
                    self.reset_key(KEY_0);
                    self.dump_palette();
                } else {
                    for i in 0..=8 {
                        if self.is_button_pressed(KEY_1 + i, None, None) {
                            self.reset_key(KEY_1 + i);
                            self.dump_image_bank(i);
                        }
                    }
                }
            } else if self.is_button_pressed(KEY_1, None, None) {
                self.reset_key(KEY_1);
                if let Err(e) = self.save_screenshot(None, None) {
                    println!("{e}");
                }
            } else if self.is_button_pressed(KEY_2, None, None) {
                self.reset_key(KEY_2);
                self.reset_screencast();
            } else if self.is_button_pressed(KEY_3, None, None) {
                self.reset_key(KEY_3);
                if let Err(e) = self.save_screencast(None, None) {
                    println!("{e}");
                }
            } else if self.is_button_pressed(KEY_8, None, None) {
                self.reset_key(KEY_8);
                self.set_integer_scale(!self.system.integer_scale_enabled);
            } else if self.is_button_pressed(KEY_9, None, None) {
                self.reset_key(KEY_9);
                self.set_screen_mode((self.system.screen_mode + 1) % NUM_SCREEN_MODES);
            } else if self.is_button_pressed(KEY_0, None, None) {
                self.reset_key(KEY_0);
                self.set_perf_monitor(!self.system.perf_monitor_enabled);
            } else if self.is_button_pressed(KEY_R, None, None) {
                self.reset_key(KEY_R);
                return Some(LifecycleAction::Restart);
            } else if self.is_button_pressed(KEY_RETURN, None, None) {
                self.reset_key(KEY_RETURN);
                self.set_fullscreen(!platform::is_fullscreen());
            }
        } else if self.is_button_down(GAMEPAD1_BUTTON_A)
            && self.is_button_down(GAMEPAD1_BUTTON_B)
            && self.is_button_down(GAMEPAD1_BUTTON_X)
            && self.is_button_down(GAMEPAD1_BUTTON_Y)
        {
            if self.is_button_pressed(GAMEPAD1_BUTTON_BACK, None, None) {
                self.reset_key(GAMEPAD1_BUTTON_BACK);
                return Some(LifecycleAction::Restart);
            } else if self.is_button_pressed(GAMEPAD1_BUTTON_DPAD_LEFT, None, None) {
                self.reset_key(GAMEPAD1_BUTTON_DPAD_LEFT);
                self.set_integer_scale(!self.system.integer_scale_enabled);
            } else if self.is_button_pressed(GAMEPAD1_BUTTON_DPAD_RIGHT, None, None) {
                self.reset_key(GAMEPAD1_BUTTON_DPAD_RIGHT);
                self.set_screen_mode((self.system.screen_mode + 1) % NUM_SCREEN_MODES);
            } else if self.is_button_pressed(GAMEPAD1_BUTTON_DPAD_UP, None, None) {
                self.reset_key(GAMEPAD1_BUTTON_DPAD_UP);
                self.set_perf_monitor(!self.system.perf_monitor_enabled);
            } else if self.is_button_pressed(GAMEPAD1_BUTTON_DPAD_DOWN, None, None) {
                self.reset_key(GAMEPAD1_BUTTON_DPAD_DOWN);
                self.set_fullscreen(!platform::is_fullscreen());
            }
        }
        None
    }

    // Frame lifecycle

    pub(crate) fn update_screen_params(&mut self) {
        let (window_width, window_height) = platform::window_size();
        let w = *pyxel::width() as f32;
        let h = *pyxel::height() as f32;

        let mut scale = f32::min(window_width as f32 / w, window_height as f32 / h);
        if self.system.integer_scale_enabled {
            scale = scale.floor();
        }
        self.system.screen_scale = scale.max(1.0);

        self.system.screen_x = (window_width as i32 - (w * self.system.screen_scale) as i32) / 2;
        self.system.screen_y = (window_height as i32 - (h * self.system.screen_scale) as i32) / 2;
    }

    fn run_update_frame(callback: &mut dyn PyxelCallback) {
        let (should_update, action) = pyxel::pyxel().begin_update_frame();
        if let Some(action) = action {
            Self::perform_lifecycle_action(action);
            return;
        }
        if !should_update {
            return;
        }

        // Public callbacks re-enter Pyxel, so no singleton borrow may span this call.
        callback.update();
        pyxel::pyxel().finish_update_frame();
    }

    fn begin_update_frame(&mut self) -> (bool, Option<LifecycleAction>) {
        self.system.update_profiler.start(platform::ticks());

        if let Some(action) = self.process_events() {
            return (false, Some(action));
        }

        if self.system.paused {
            return (false, None);
        }

        if let Some(action) = self.check_special_input() {
            return (false, Some(action));
        }
        (true, None)
    }

    fn finish_update_frame(&mut self) {
        self.system.update_profiler.end(platform::ticks());
    }

    // Rendering & UI

    // Draw frame metrics while preserving the caller's screen state.
    fn draw_perf_monitor(&self) {
        if !self.system.perf_monitor_enabled {
            return;
        }

        let screen_rc = pyxel::screen().clone();
        let mut screen = rc_mut!(screen_rc);
        let clip_rect = screen.canvas.clip_rect;
        let camera_x = screen.canvas.camera_x;
        let camera_y = screen.canvas.camera_y;
        let palette1 = screen.palette[1];
        let palette2 = screen.palette[2];
        let alpha = screen.canvas.alpha;

        screen.reset_clip_rect();
        screen.reset_camera();
        screen.map_color(1, 1);
        screen.map_color(2, 9);
        screen.set_dithering(1.0);

        let fps = format!("{:.2}", self.system.fps_profiler.average_fps());
        screen.draw_text(1.0, 0.0, &fps, 1, None);
        screen.draw_text(0.0, 0.0, &fps, 2, None);

        let update_time = format!("{:.2}", self.system.update_profiler.average_time());
        screen.draw_text(1.0, 6.0, &update_time, 1, None);
        screen.draw_text(0.0, 6.0, &update_time, 2, None);

        let draw_time = format!("{:.2}", self.system.draw_profiler.average_time());
        screen.draw_text(1.0, 12.0, &draw_time, 1, None);
        screen.draw_text(0.0, 12.0, &draw_time, 2, None);

        screen.canvas.clip_rect = clip_rect;
        screen.canvas.camera_x = camera_x;
        screen.canvas.camera_y = camera_y;
        screen.map_color(1, palette1);
        screen.map_color(2, palette2);
        screen.set_dithering(alpha);
    }

    // Draw the custom cursor while preserving screen state.
    fn draw_cursor(&self) {
        let x = *pyxel::mouse_x();
        let y = *pyxel::mouse_y();

        platform::set_mouse_visible(
            x < 0 || x >= *pyxel::width() as i32 || y < 0 || y >= *pyxel::height() as i32,
        );

        if !self.is_mouse_visible() {
            return;
        }

        let width = rc_ref!(pyxel::cursor_image()).width() as i32;
        let height = rc_ref!(pyxel::cursor_image()).height() as i32;

        if x <= -width
            || x >= *pyxel::width() as i32
            || y <= -height
            || y >= *pyxel::height() as i32
        {
            return;
        }

        let screen_rc = pyxel::screen().clone();
        let mut screen = rc_mut!(screen_rc);
        let clip_rect = screen.canvas.clip_rect;
        let camera_x = screen.canvas.camera_x;
        let camera_y = screen.canvas.camera_y;
        let palette = screen.palette;

        screen.reset_clip_rect();
        screen.reset_camera();
        screen.draw_image(
            x as f32,
            y as f32,
            &pyxel::cursor_image(),
            0.0,
            0.0,
            width as f32,
            height as f32,
            Some(0),
            None,
            None,
        );

        screen.canvas.clip_rect = clip_rect;
        screen.canvas.camera_x = camera_x;
        screen.canvas.camera_y = camera_y;
        screen.palette = palette;
    }

    fn draw_frame(&mut self, callback: Option<&mut dyn PyxelCallback>) {
        if !self.begin_draw_frame() {
            return;
        }

        if let Some(callback) = callback {
            callback.draw();
        }

        self.finish_draw_frame();
    }

    fn run_draw_frame(callback: &mut dyn PyxelCallback) {
        let should_draw = pyxel::pyxel().begin_draw_frame();
        if !should_draw {
            return;
        }

        // Public callbacks re-enter Pyxel, so no singleton borrow may span this call.
        callback.draw();
        pyxel::pyxel().finish_draw_frame();
    }

    fn begin_draw_frame(&mut self) -> bool {
        self.system.draw_profiler.start(platform::ticks());

        if self.system.paused {
            return false;
        }

        self.update_screen_params();
        true
    }

    fn finish_draw_frame(&mut self) {
        self.system.window_watcher.update();
        self.draw_perf_monitor();
        self.draw_cursor();
        self.render_screen();
        self.capture_screen();

        self.system.draw_profiler.end(platform::ticks());
    }
}
