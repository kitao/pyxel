use std::ffi::CString;
use std::mem;
use std::ptr;

use glow::Context;

use crate::sdl2_sys::*;

static mut WINDOW: *mut SDL_Window = ptr::null_mut();
static mut GL: *mut Context = ptr::null_mut();
//static mut RENDERER: *mut SDL_Renderer = ptr::null_mut();

pub(crate) fn window() -> &'static mut SDL_Window {
    unsafe {
        assert!(!WINDOW.is_null(), "Pyxel is not initialized");
        &mut *WINDOW
    }
}

pub fn gl() -> &'static mut Context {
    unsafe {
        assert!(!GL.is_null(), "Pyxel is not initialized");
        &mut *GL
    }
}

pub fn create_window(title: &str, width: u32, height: u32) {
    unsafe {
        let title = CString::new(title).unwrap();
        WINDOW = SDL_CreateWindow(
            title.as_ptr(),
            SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            width as i32,
            height as i32,
            SDL_WINDOW_OPENGL,
        );
        if WINDOW.is_null() {
            panic!("Failed to create window");
        }
        /*RENDERER = SDL_CreateRenderer(WINDOW, -1, 0);
        if RENDERER.is_null() {
            panic!("Failed to create renderer");
        }*/
        if SDL_GL_CreateContext(WINDOW).is_null() {
            panic!("Failed to create OpenGL context");
        }
        GL = mem::transmute(Box::new(Context::from_loader_function(|s| {
            SDL_GL_GetProcAddress(s.as_ptr() as *const _) as *const _
        })));
    }

    /*
    let sdl_context = sdl2::init().unwrap();
    let sdl_event_pump = sdl_context.event_pump().unwrap();
    let sdl_timer = sdl_context.timer().unwrap();
    let sdl_video = sdl_context.video().unwrap();
    let sdl_display_mode = sdl_video.desktop_display_mode(0).unwrap();
    let display_scale = u32::max(
        match display_scale {
            DisplayScale::Scale(scale) => scale,
            DisplayScale::Ratio(ratio) => {
                (f64::min(
                    sdl_display_mode.w as f64 / screen_width as f64,
                    sdl_display_mode.h as f64 / screen_height as f64,
                ) * ratio) as u32
            }
        },
        1,
    );
    let watch_info_file = Self::watch_info_file();
    let sdl_window = Self::load_watch_info(&watch_info_file).map_or_else(
        || {
            sdl_video
                .window(
                    title,
                    screen_width * display_scale,
                    screen_height * display_scale,
                )
                .position_centered()
                .resizable()
                .build()
                .unwrap()
        },
        |window_state| {
            sdl_video
                .window(title, window_state.width, window_state.height)
                .position(window_state.x, window_state.y)
                .resizable()
                .build()
                .unwrap()
        },
    );
    let mut sdl_canvas = sdl_window.into_canvas().present_vsync().build().unwrap();
    sdl_canvas
        .window_mut()
        .set_minimum_size(screen_width, screen_height)
        .unwrap();
    let sdl_texture = sdl_canvas
        .texture_creator()
        .create_texture_streaming(SdlPixelFormat::RGB24, screen_width, screen_height)
        .unwrap();
    let sdl_game_controller = sdl_context.game_controller().map_or_else(
        |_| {
            println!("Unable to initialize the game controller subsystem");
            None
        },
        |sdl_game_controller| Some(sdl_game_controller),
    );
    let sdl_audio = sdl_context.audio().map_or_else(
        |_| {
            println!("Unable to initialize the audio subsystem");
            None
        },
        |sdl_audio| Some(sdl_audio),
    );
    sdl_hint::set("SDL_MOUSE_FOCUS_CLICKTHROUGH", "1");
    Self::set_instance(Self {
        sdl_context,
        sdl_event_pump,
        sdl_timer,
        sdl_canvas,
        sdl_texture,
        sdl_game_controller,
        sdl_game_controller_states: Vec::new(),
        sdl_audio,
        sdl_audio_device: None,
        screen_width,
        screen_height,
        mouse_x: i32::MIN,
        mouse_y: i32::MIN,
        watch_info_file,
        window_state: WindowState::default(),
        #[cfg(target_os = "emscripten")]
        virtual_gamepad_states: [false; 8],
    });
    Self::instance().save_watch_info();
    */
}

pub fn set_window_title(title: &str) {
    let title = CString::new(title).unwrap();
    unsafe {
        SDL_SetWindowTitle(window(), title.as_ptr());
    }
}

pub fn set_window_icon(image: &[Vec<u32>]) {
    /*
    let mut sdl_surface =
        SdlSurface::new(width * scale, height * scale, SdlPixelFormat::RGBA32).unwrap();
    let pitch = sdl_surface.pitch();
    sdl_surface.with_lock_mut(|buffer: &mut [u8]| {
        for y in 0..height * scale {
            for x in 0..width * scale {
                let color = image[(width * (y / scale) + x / scale) as usize];
                let rgb = colors[color as usize];
                let offset = (y * pitch + x * 4) as usize;
                buffer[offset] = ((rgb >> 16) & 0xff) as u8;
                buffer[offset + 1] = ((rgb >> 8) & 0xff) as u8;
                buffer[offset + 2] = (rgb & 0xff) as u8;
                buffer[offset + 3] = if color > 0 { 0xff } else { 0x00 };
            }
        }
    });
    self.sdl_canvas.window_mut().set_icon(&sdl_surface);
    */
}

pub fn is_fullscreen() -> bool {
    (unsafe { SDL_GetWindowFlags(window()) } & SDL_WINDOW_FULLSCREEN) != 0
}

pub fn set_fullscreen(fullscreen: bool) {
    let fullscreen = if fullscreen {
        SDL_WINDOW_FULLSCREEN_DESKTOP
    } else {
        0
    };
    unsafe {
        SDL_SetWindowFullscreen(window(), fullscreen);
    }
}
