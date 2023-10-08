use std::ffi::CString;
use std::mem::transmute;

use glow::Context as GlowContext;

use crate::platform::platform;
use crate::sdl2_sys::*;

pub fn init_window(title: &str, width: u32, height: u32) -> (*mut SDL_Window, *mut GlowContext) {
    let window;
    let gl;
    unsafe {
        let title = CString::new(title).unwrap();
        window = SDL_CreateWindow(
            title.as_ptr(),
            SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            width as i32,
            height as i32,
            SDL_WINDOW_OPENGL | SDL_WINDOW_RESIZABLE,
        );
        if window.is_null() {
            panic!("Failed to create window");
        }
        if SDL_GL_CreateContext(window).is_null() {
            panic!("Failed to create OpenGL context");
        }
        gl = transmute(Box::new(GlowContext::from_loader_function(|s| {
            SDL_GL_GetProcAddress(s.as_ptr() as *const _) as *const _
        })));
    }
    (window, gl)

    /*
    let watch_info_file = Self::watch_info_file();
    let sdl_window = Self::load_watch_info(&watch_info_file).map_or_else(
        || {
            sdl_video
                .position_centered()
                .resizable()
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
    sdl_hint::set("SDL_MOUSE_FOCUS_CLICKTHROUGH", "1");
    Self::set_instance(Self {
        #[cfg(target_os = "emscripten")]
        virtual_gamepad_states: [false; 8],
    });
    Self::instance().save_watch_info();
    */
}

pub unsafe fn glow_context() -> &'static mut GlowContext {
    &mut *platform().glow_context
}

pub fn swap_window() {
    unsafe {
        SDL_GL_SwapWindow(platform().window);
    }
}

pub fn set_window_title(title: &str) {
    let title = CString::new(title).unwrap();
    unsafe {
        SDL_SetWindowTitle(platform().window, title.as_ptr());
    }
}

pub fn set_window_icon(width: u32, height: u32, rgba_data: &[u8]) {
    unsafe {
        let surface = SDL_CreateRGBSurfaceWithFormat(
            0,
            width as i32,
            height as i32,
            32,
            SDL_PIXELFORMAT_RGBA32,
        );
        let pixels = (*surface).pixels as *mut u8;
        let pitch = (*surface).pitch as u32;
        for y in 0..height {
            let src_offset = (width * y * 4) as usize;
            let dst_pixels = pixels.add((pitch * y) as usize);
            for x in 0..width * 4 {
                *(dst_pixels.add(x as usize)) = rgba_data[src_offset + x as usize];
            }
        }
        SDL_SetWindowIcon(platform().window, surface);
        SDL_FreeSurface(surface);
    }
}

pub fn window_pos() -> (i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    unsafe {
        SDL_GetWindowPosition(platform().window, &mut x as *mut i32, &mut y as *mut i32);
    }
    (x, y)
}

pub fn set_window_pos(x: i32, y: i32) {
    unsafe {
        SDL_SetWindowPosition(platform().window, x, y);
    }
}

pub fn window_size() -> (u32, u32) {
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    unsafe {
        SDL_GetWindowSize(
            platform().window,
            &mut width as *mut i32,
            &mut height as *mut i32,
        );
    }
    (width as u32, height as u32)
}

pub fn set_window_size(width: u32, height: u32) {
    unsafe {
        SDL_SetWindowSize(platform().window, width as i32, height as i32);
    }
}

pub fn is_fullscreen() -> bool {
    (unsafe { SDL_GetWindowFlags(platform().window) } & SDL_WINDOW_FULLSCREEN) != 0
}

pub fn set_fullscreen(full: bool) {
    let full = if full {
        SDL_WINDOW_FULLSCREEN_DESKTOP
    } else {
        0
    };
    unsafe {
        SDL_SetWindowFullscreen(platform().window, full);
    }
}

pub fn set_mouse_visible(visible: bool) {
    let visible = if visible { SDL_ENABLE } else { SDL_DISABLE };
    unsafe {
        SDL_ShowCursor(visible as i32);
    }
}

pub fn set_mouse_pos(x: i32, y: i32) {
    let (window_x, window_y) = window_pos();
    unsafe {
        SDL_WarpMouseGlobal(window_x + x, window_y + y);
    }
}
