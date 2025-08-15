use std::os::raw::{c_char, c_int, c_void};
// 追加インポート（最小）
use std::{ffi::CString, ptr};

use glow::Context;

use crate::event::Event;
use crate::platform::GLProfile;

// ===== Emscripten FFI =====
#[allow(non_camel_case_types)]
type EMSCRIPTEN_WEBGL_CONTEXT_HANDLE = i32;

extern "C" {
    // time / exit
    fn emscripten_get_now() -> f64;
    fn emscripten_force_exit(status: c_int);

    // canvas size（target = NULL で Module.canvas を使う）
    fn emscripten_set_canvas_element_size(target: *const c_char, w: c_int, h: c_int) -> c_int;
    fn emscripten_get_canvas_element_size(
        target: *const c_char,
        w: *mut c_int,
        h: *mut c_int,
    ) -> c_int;

    // WebGL
    fn emscripten_webgl_init_context_attributes(attr: *mut EmscriptenWebGLContextAttributes);
    fn emscripten_webgl_create_context(
        target: *const c_char,
        attr: *const EmscriptenWebGLContextAttributes,
    ) -> EMSCRIPTEN_WEBGL_CONTEXT_HANDLE;
    fn emscripten_webgl_make_context_current(ctx: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE) -> c_int;

    // GL loader
    fn emscripten_GetProcAddress(name: *const c_char) -> *const c_void;

    // JS 側ブリッジ：次の rAF まで同期的に待つ（Asyncify で実装）
    fn js_wait_vsync();
}

// Emscripten の属性最小ミラー
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct EmscriptenWebGLContextAttributes {
    alpha: c_int,
    depth: c_int,
    stencil: c_int,
    antialias: c_int,
    premultipliedAlpha: c_int,
    preserveDrawingBuffer: c_int,
    preferLowPowerToHighPerformance: c_int,
    failIfMajorPerformanceCaveat: c_int,
    majorVersion: c_int,
    minorVersion: c_int,
    enableExtensionsByDefault: c_int,
    explicitSwapControl: c_int,
    renderViaOffscreenBackBuffer: c_int,
}

// グローバル（削除前）
// static GL_CTX: OnceCell<*mut Context> = OnceCell::new();
// static WEBGL_HANDLE: OnceCell<EMSCRIPTEN_WEBGL_CONTEXT_HANDLE> = OnceCell::new();
// static FRAME_CB: OnceCell<RefCell<Option<Box<dyn FnMut(f32)>>>> = OnceCell::new();
// static AUDIO_CB_I16: OnceCell<RefCell<Option<Box<dyn FnMut(&mut [i16])>>>> = OnceCell::new();

// 代替：インスタンスへの生ポインタ（コールバック入口用）
static mut PLATFORM_HTML5_INSTANCE: *mut PlatformHtml5 = std::ptr::null_mut();

unsafe extern "C" fn callback_wrapper<F: FnMut()>(arg: *mut c_void) {
    (*arg.cast::<F>())();
}

pub struct PlatformHtml5 {
    width: u32,
    height: u32,
    paused_audio: bool,
    // ここから元 static をメンバ化
    gl_ctx: Option<Context>,
    webgl_handle: Option<EMSCRIPTEN_WEBGL_CONTEXT_HANDLE>,
    frame_cb: Option<Box<dyn FnMut(f32)>>,
    audio_cb_i16: Option<Box<dyn FnMut(&mut [i16])>>,
}

impl PlatformHtml5 {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            paused_audio: false,
            gl_ctx: None,
            webgl_handle: None,
            frame_cb: None,
            audio_cb_i16: None,
        }
    }

    // JS コールバックから参照させたいインスタンスで呼ぶ
    pub fn set_as_global(&mut self) {
        unsafe {
            PLATFORM_HTML5_INSTANCE = self as *mut _;
        }
    }

    //
    // Core
    //
    pub fn init(&mut self) {
        unsafe {
            let mut attr = EmscriptenWebGLContextAttributes::default();
            emscripten_webgl_init_context_attributes(&mut attr);
            attr.majorVersion = 2;
            attr.minorVersion = 0;
            attr.alpha = 0;
            attr.antialias = 0;

            let ctx = emscripten_webgl_create_context(ptr::null(), &attr);
            assert!(ctx > 0, "failed to create WebGL context");
            emscripten_webgl_make_context_current(ctx);
            self.webgl_handle = Some(ctx);

            if self.gl_ctx.is_none() {
                let loader = |s: &str| {
                    let c = CString::new(s).unwrap();
                    emscripten_GetProcAddress(c.as_ptr()) as *const _
                };
                let gl = Context::from_loader_function(loader);
                self.gl_ctx = Some(gl);
            }
        }
    }

    pub fn quit(&mut self) {
        unsafe { emscripten_force_exit(0) };
    }

    pub fn ticks(&self) -> u32 {
        // 既存仕様に合わせて *1000（emscripten_get_now は ms 返却だが元コード踏襲）
        unsafe { (emscripten_get_now() * 1000.0) as u32 }
    }

    //
    // Window
    //
    pub fn init_window(&mut self, _title: &str, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        unsafe {
            emscripten_set_canvas_element_size(ptr::null(), width as c_int, height as c_int);
        }
    }

    pub fn window_pos(&mut self) -> (i32, i32) {
        (0, 0)
    }

    pub fn set_window_pos(&mut self, _x: i32, _y: i32) {}

    pub fn window_size(&mut self) -> (u32, u32) {
        let mut w: c_int = 0;
        let mut h: c_int = 0;
        unsafe {
            emscripten_get_canvas_element_size(ptr::null(), &mut w, &mut h);
        }
        (w as u32, h as u32)
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        unsafe {
            emscripten_set_canvas_element_size(ptr::null(), width as c_int, height as c_int);
        }
    }

    pub fn set_window_title(&mut self, _title: &str) {}
    pub fn set_window_icon(&mut self, _width: u32, _height: u32, _rgba: &[u8]) {}

    pub fn is_fullscreen(&mut self) -> bool {
        false
    }
    pub fn set_fullscreen(&mut self, _enabled: bool) {}

    pub fn set_mouse_pos(&mut self, _x: i32, _y: i32) {}
    pub fn set_mouse_visible(&self, _visible: bool) {}

    pub fn display_size(&self) -> (u32, u32) {
        let mut w: c_int = 0;
        let mut h: c_int = 0;
        unsafe {
            emscripten_get_canvas_element_size(ptr::null(), &mut w, &mut h);
        }
        (w as u32, h as u32)
    }

    //
    // Audio
    //
    pub fn start_audio<F: FnMut(&mut [i16]) + 'static>(
        &mut self,
        _sample_rate: u32,
        _buffer_size: u32,
        callback: F,
    ) {
        self.audio_cb_i16 = Some(Box::new(callback));
    }

    pub fn pause_audio(&mut self, paused: bool) {
        self.paused_audio = paused;
    }

    //
    // Frame
    //
    /*pub fn run_frame_loop<F: FnMut(f32)>(&mut self, _fps: u32, callback: F) {
        self.frame_cb = Some(Box::new(callback));
    }*/

    pub fn present_frame(&mut self) {
        unsafe {
            js_wait_vsync();
        }
    }

    pub fn poll_events(&mut self) -> Vec<Event> {
        Vec::new()
    }

    pub fn gl_profile(&self) -> GLProfile {
        GLProfile::Gles
    }

    pub fn gl_context(&mut self) -> &mut Context {
        self.gl_ctx.as_mut().expect("GL context not initialized")
    }

    /*pub fn export_file(filename: &str) {
        let filename = CString::new(filename).unwrap();
        extern "C" {
            fn expoert_file(filename: *const c_char);
        }
    }*/
}

// ===== JS が呼ぶ入口（最小） =====

#[no_mangle]
pub extern "C" fn on_frame_js(_timestamp_ms: c_int) {
    unsafe {
        if !PLATFORM_HTML5_INSTANCE.is_null() {
            let inst = &mut *PLATFORM_HTML5_INSTANCE;
            if let Some(cb) = inst.frame_cb.as_mut() {
                cb(0.0);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn fill_audio_i16(out: *mut i16, frames: c_int) {
    if out.is_null() || frames <= 0 {
        return;
    }
    unsafe {
        if !PLATFORM_HTML5_INSTANCE.is_null() {
            let inst = &mut *PLATFORM_HTML5_INSTANCE;
            if let Some(cb) = inst.audio_cb_i16.as_mut() {
                let len = frames as usize * 2;
                let buf = std::slice::from_raw_parts_mut(out, len);
                cb(buf);
            }
        }
    }
}
