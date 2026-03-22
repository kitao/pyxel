// pyxel-embed: RustPython embedded Pyxel runtime

mod audio_wrapper;
mod channel_wrapper;
mod font_wrapper;
mod graphics_wrapper;
mod helpers;
mod image_wrapper;
mod input_wrapper;
mod math_wrapper;
mod music_wrapper;
mod resource_wrapper;
mod seq_wrapper;
mod sound_wrapper;
mod system_wrapper;
mod tilemap_wrapper;
mod tone_wrapper;
mod variable_wrapper;

use rustpython_vm as vm;
pub use vm::Interpreter;

/// Capture the original working directory before any chdir happens.
/// Must be called at the very start of main().
pub fn save_original_cwd() {
    system_wrapper::set_original_cwd();
}

pub fn create_interpreter() -> Interpreter {
    rustpython::InterpreterConfig::new()
        .init_stdlib()
        .init_hook(Box::new(|vm| {
            vm.add_native_module("pyxel".to_owned(), Box::new(pyxel_module::make_module));
        }))
        .interpreter()
}

pub fn exec_source(interp: &Interpreter, source: &str) {
    exec_source_with_file(interp, source, None);
}

pub fn exec_source_with_file(interp: &Interpreter, source: &str, file_path: Option<&str>) {
    interp.enter(|vm| {
        let scope = vm.new_scope_with_builtins();
        if let Some(path) = file_path {
            scope
                .globals
                .set_item("__file__", vm.new_pyobj(path), vm)
                .ok();
        }
        let filename = file_path.unwrap_or("<script>");
        let code = match vm.compile(source, vm::compiler::Mode::Exec, filename.to_owned()) {
            Ok(code) => code,
            Err(err) => {
                let exc = vm.new_syntax_error(&err, Some(source));
                vm.print_exception(exc);
                return;
            }
        };
        if let Err(exc) = vm.run_code_obj(code, scope) {
            let mut msg = String::new();
            vm.write_exception(&mut msg, &exc).ok();
            eprintln!("[pyxel-embed] Python exception:\n{msg}");
        }
    });
}

#[vm::pymodule]
mod pyxel_module {
    use super::vm::function::FuncArgs;
    use super::vm::{PyResult, VirtualMachine};

    type A = FuncArgs;
    type V = VirtualMachine;
    type R = PyResult<()>;

    // System
    #[pyfunction]
    fn init(a: A, v: &V) -> R {
        super::system_wrapper::init(a, v)
    }
    #[pyfunction]
    fn run(update: super::vm::PyObjectRef, draw: super::vm::PyObjectRef, v: &V) {
        super::system_wrapper::run(update, draw, v)
    }
    #[pyfunction]
    fn show() {
        super::system_wrapper::show()
    }
    #[pyfunction]
    fn flip() {
        super::system_wrapper::flip()
    }
    #[pyfunction]
    fn quit() {
        super::system_wrapper::quit()
    }
    #[pyfunction]
    fn reset() {
        super::system_wrapper::reset()
    }
    #[pyfunction]
    fn title(a: A, v: &V) -> R {
        super::system_wrapper::title(a, v)
    }
    #[pyfunction]
    fn icon(a: A, v: &V) -> R {
        super::system_wrapper::icon(a, v)
    }
    #[pyfunction]
    fn perf_monitor(a: A, v: &V) -> R {
        super::system_wrapper::perf_monitor(a, v)
    }
    #[pyfunction]
    fn integer_scale(a: A, v: &V) -> R {
        super::system_wrapper::integer_scale(a, v)
    }
    #[pyfunction]
    fn screen_mode(a: A, v: &V) -> R {
        super::system_wrapper::screen_mode(a, v)
    }
    #[pyfunction]
    fn fullscreen(a: A, v: &V) -> R {
        super::system_wrapper::fullscreen(a, v)
    }

    // Graphics
    #[pyfunction]
    fn cls(a: A, v: &V) -> R {
        super::graphics_wrapper::cls(a, v)
    }
    #[pyfunction]
    fn pget(a: A, v: &V) -> PyResult<u8> {
        super::graphics_wrapper::pget(a, v)
    }
    #[pyfunction]
    fn pset(a: A, v: &V) -> R {
        super::graphics_wrapper::pset(a, v)
    }
    #[pyfunction]
    fn line(a: A, v: &V) -> R {
        super::graphics_wrapper::line(a, v)
    }
    #[pyfunction]
    fn rect(a: A, v: &V) -> R {
        super::graphics_wrapper::rect(a, v)
    }
    #[pyfunction]
    fn rectb(a: A, v: &V) -> R {
        super::graphics_wrapper::rectb(a, v)
    }
    #[pyfunction]
    fn circ(a: A, v: &V) -> R {
        super::graphics_wrapper::circ(a, v)
    }
    #[pyfunction]
    fn circb(a: A, v: &V) -> R {
        super::graphics_wrapper::circb(a, v)
    }
    #[pyfunction]
    fn elli(a: A, v: &V) -> R {
        super::graphics_wrapper::elli(a, v)
    }
    #[pyfunction]
    fn ellib(a: A, v: &V) -> R {
        super::graphics_wrapper::ellib(a, v)
    }
    #[pyfunction]
    fn tri(a: A, v: &V) -> R {
        super::graphics_wrapper::tri(a, v)
    }
    #[pyfunction]
    fn trib(a: A, v: &V) -> R {
        super::graphics_wrapper::trib(a, v)
    }
    #[pyfunction]
    fn fill(a: A, v: &V) -> R {
        super::graphics_wrapper::fill(a, v)
    }
    #[pyfunction]
    fn blt(a: A, v: &V) -> R {
        super::graphics_wrapper::blt(a, v)
    }
    #[pyfunction]
    fn bltm(a: A, v: &V) -> R {
        super::graphics_wrapper::bltm(a, v)
    }
    #[pyfunction]
    fn blt3d(a: A, v: &V) -> R {
        super::graphics_wrapper::blt3d(a, v)
    }
    #[pyfunction]
    fn bltm3d(a: A, v: &V) -> R {
        super::graphics_wrapper::bltm3d(a, v)
    }
    #[pyfunction]
    fn text(a: A, v: &V) -> R {
        super::graphics_wrapper::text(a, v)
    }
    #[pyfunction]
    fn clip(a: A, v: &V) -> R {
        super::graphics_wrapper::clip(a, v)
    }
    #[pyfunction]
    fn camera(a: A, v: &V) -> R {
        super::graphics_wrapper::camera(a, v)
    }
    #[pyfunction]
    fn pal(a: A, v: &V) -> R {
        super::graphics_wrapper::pal(a, v)
    }
    #[pyfunction]
    fn dither(a: A, v: &V) -> R {
        super::graphics_wrapper::dither(a, v)
    }

    // Input
    #[pyfunction]
    fn btn(a: A, v: &V) -> PyResult<bool> {
        super::input_wrapper::btn(a, v)
    }
    #[pyfunction]
    fn btnp(a: A, v: &V) -> PyResult<bool> {
        super::input_wrapper::btnp(a, v)
    }
    #[pyfunction]
    fn btnr(a: A, v: &V) -> PyResult<bool> {
        super::input_wrapper::btnr(a, v)
    }
    #[pyfunction]
    fn btnv(a: A, v: &V) -> PyResult<i32> {
        super::input_wrapper::btnv(a, v)
    }
    #[pyfunction]
    fn mouse(visible: bool) {
        super::input_wrapper::mouse(visible)
    }
    #[pyfunction]
    fn warp_mouse(a: A, v: &V) -> R {
        super::input_wrapper::warp_mouse(a, v)
    }

    // Math
    #[pyfunction]
    fn ceil(a: A, v: &V) -> PyResult<i32> {
        super::math_wrapper::ceil(a, v)
    }
    #[pyfunction]
    fn floor(a: A, v: &V) -> PyResult<i32> {
        super::math_wrapper::floor(a, v)
    }
    #[pyfunction]
    fn sqrt(a: A, v: &V) -> PyResult<f64> {
        super::math_wrapper::sqrt(a, v)
    }
    #[pyfunction]
    fn sin(a: A, v: &V) -> PyResult<f64> {
        super::math_wrapper::sin(a, v)
    }
    #[pyfunction]
    fn cos(a: A, v: &V) -> PyResult<f64> {
        super::math_wrapper::cos(a, v)
    }
    #[pyfunction]
    fn atan2(a: A, v: &V) -> PyResult<f64> {
        super::math_wrapper::atan2(a, v)
    }
    #[pyfunction]
    fn rseed(seed: u32) {
        super::math_wrapper::rseed(seed)
    }
    #[pyfunction]
    fn rndi(a: i32, b: i32) -> i32 {
        super::math_wrapper::rndi(a, b)
    }
    #[pyfunction]
    fn rndf(a: A, v: &V) -> PyResult<f64> {
        super::math_wrapper::rndf(a, v)
    }
    #[pyfunction]
    fn nseed(seed: u32) {
        super::math_wrapper::nseed(seed)
    }
    #[pyfunction]
    fn noise(a: A, v: &V) -> PyResult<f64> {
        super::math_wrapper::noise(a, v)
    }
    #[pyfunction]
    fn clamp(a: A, v: &V) -> PyResult<super::vm::PyObjectRef> {
        super::math_wrapper::clamp(a, v)
    }
    #[pyfunction]
    fn sgn(a: A, v: &V) -> PyResult<super::vm::PyObjectRef> {
        super::math_wrapper::sgn(a, v)
    }

    // Audio
    #[pyfunction]
    fn play(a: A, v: &V) -> R {
        super::audio_wrapper::play(a, v)
    }
    #[pyfunction]
    fn playm(a: A, v: &V) -> R {
        super::audio_wrapper::playm(a, v)
    }
    #[pyfunction]
    fn stop(a: A, v: &V) -> R {
        super::audio_wrapper::stop(a, v)
    }
    #[pyfunction]
    fn play_pos(a: A, v: &V) -> PyResult<super::vm::PyObjectRef> {
        super::audio_wrapper::play_pos(a, v)
    }
    #[pyfunction]
    fn gen_bgm(a: A, v: &V) -> PyResult<super::vm::PyObjectRef> {
        super::audio_wrapper::gen_bgm(a, v)
    }

    // Resource
    #[pyfunction]
    fn load(a: A, v: &V) -> R {
        super::resource_wrapper::load(a, v)
    }
    #[pyfunction]
    fn save(a: A, v: &V) -> R {
        super::resource_wrapper::save(a, v)
    }
    #[pyfunction]
    fn load_pal(a: A, v: &V) -> R {
        super::resource_wrapper::load_pal(a, v)
    }
    #[pyfunction]
    fn save_pal(a: A, v: &V) -> R {
        super::resource_wrapper::save_pal(a, v)
    }
    #[pyfunction]
    fn screenshot(a: A, v: &V) -> R {
        super::resource_wrapper::screenshot(a, v)
    }
    #[pyfunction]
    fn screencast(a: A, v: &V) -> R {
        super::resource_wrapper::screencast(a, v)
    }
    #[pyfunction]
    fn reset_screencast() {
        super::resource_wrapper::reset_screencast()
    }
    #[pyfunction]
    fn user_data_dir(a: A, v: &V) -> PyResult<String> {
        super::resource_wrapper::user_data_dir(a, v)
    }

    // _Colors collection class (impl in variable_wrapper.rs)
    #[pyattr]
    #[allow(non_snake_case)]
    fn _Colors(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::variable_wrapper::PyColors::make_class(&vm.ctx)
    }

    // Image class (impl in image_wrapper.rs)
    #[pyattr]
    #[allow(non_snake_case)]
    fn Image(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::image_wrapper::PyImage::make_class(&vm.ctx)
    }

    // _Images collection class (impl in variable_wrapper.rs)
    #[pyattr]
    #[allow(non_snake_case)]
    fn _Images(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::variable_wrapper::PyImages::make_class(&vm.ctx)
    }

    // Sound class (impl in sound_wrapper.rs)
    #[pyattr]
    #[allow(non_snake_case)]
    fn Sound(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::sound_wrapper::PySound::make_class(&vm.ctx)
    }

    // _Sounds collection class (impl in variable_wrapper.rs)
    #[pyattr]
    #[allow(non_snake_case)]
    fn _Sounds(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::variable_wrapper::PySounds::make_class(&vm.ctx)
    }

    // Tilemap class (impl in tilemap_wrapper.rs)
    #[pyattr]
    #[allow(non_snake_case)]
    fn Tilemap(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::tilemap_wrapper::PyTilemap::make_class(&vm.ctx)
    }

    // _Tilemaps collection class (impl in variable_wrapper.rs)
    #[pyattr]
    #[allow(non_snake_case)]
    fn _Tilemaps(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::variable_wrapper::PyTilemaps::make_class(&vm.ctx)
    }

    // Music class (impl in music_wrapper.rs)
    #[pyattr]
    #[allow(non_snake_case)]
    fn Music(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::music_wrapper::PyMusic::make_class(&vm.ctx)
    }

    // _Musics collection class (impl in variable_wrapper.rs)
    #[pyattr]
    #[allow(non_snake_case)]
    fn _Musics(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::variable_wrapper::PyMusics::make_class(&vm.ctx)
    }

    // Channel class (impl in channel_wrapper.rs)
    #[pyattr]
    #[allow(non_snake_case)]
    fn Channel(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::channel_wrapper::PyChannel::make_class(&vm.ctx)
    }

    // _Channels collection class (impl in variable_wrapper.rs)
    #[pyattr]
    #[allow(non_snake_case)]
    fn _Channels(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::variable_wrapper::PyChannels::make_class(&vm.ctx)
    }

    // Font class (impl in font_wrapper.rs)
    #[pyattr]
    #[allow(non_snake_case)]
    fn Font(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::font_wrapper::PyFont::make_class(&vm.ctx)
    }

    // Tone class (impl in tone_wrapper.rs)
    #[pyattr]
    #[allow(non_snake_case)]
    fn Tone(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::tone_wrapper::PyTone::make_class(&vm.ctx)
    }

    // _Tones collection class (impl in variable_wrapper.rs)
    #[pyattr]
    #[allow(non_snake_case)]
    fn _Tones(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::variable_wrapper::PyTones::make_class(&vm.ctx)
    }

    // Live sequence wrapper types
    #[pyattr]
    #[allow(non_snake_case)]
    fn _Notes(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::seq_wrapper::PyNotes::make_class(&vm.ctx)
    }
    #[pyattr]
    #[allow(non_snake_case)]
    fn _Tones_Seq(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::seq_wrapper::PyTones::make_class(&vm.ctx)
    }
    #[pyattr]
    #[allow(non_snake_case)]
    fn _Volumes(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::seq_wrapper::PyVolumes::make_class(&vm.ctx)
    }
    #[pyattr]
    #[allow(non_snake_case)]
    fn _Effects(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::seq_wrapper::PyEffects::make_class(&vm.ctx)
    }
    #[pyattr]
    #[allow(non_snake_case)]
    fn _Wavetable(vm: &V) -> super::vm::builtins::PyTypeRef {
        use super::vm::class::PyClassImpl;
        super::seq_wrapper::PyWavetable::make_class(&vm.ctx)
    }

    // Dynamic variables via __getattr__
    #[pyfunction]
    fn __getattr__(name: super::vm::builtins::PyStrRef, v: &V) -> PyResult<super::vm::PyObjectRef> {
        super::variable_wrapper::module_getattr(name, v)
    }

    // Settings constants
    #[pyattr]
    const VERSION: &str = pyxel::VERSION;
    #[pyattr]
    const NUM_COLORS: u32 = pyxel::NUM_COLORS;
    #[pyattr]
    const NUM_IMAGES: u32 = pyxel::NUM_IMAGES;
    #[pyattr]
    const IMAGE_SIZE: u32 = pyxel::IMAGE_SIZE;
    #[pyattr]
    const NUM_TILEMAPS: u32 = pyxel::NUM_TILEMAPS;
    #[pyattr]
    const TILEMAP_SIZE: u32 = pyxel::TILEMAP_SIZE;
    #[pyattr]
    const TILE_SIZE: u32 = pyxel::TILE_SIZE;
    #[pyattr]
    #[allow(non_snake_case)]
    fn DEFAULT_COLORS(vm: &V) -> super::vm::PyObjectRef {
        let colors: Vec<super::vm::PyObjectRef> = pyxel::DEFAULT_COLORS
            .iter()
            .map(|&c| vm.new_pyobj(c))
            .collect();
        vm.new_pyobj(colors)
    }
    #[pyattr]
    const COLOR_BLACK: u8 = pyxel::COLOR_BLACK;
    #[pyattr]
    const COLOR_NAVY: u8 = pyxel::COLOR_NAVY;
    #[pyattr]
    const COLOR_PURPLE: u8 = pyxel::COLOR_PURPLE;
    #[pyattr]
    const COLOR_GREEN: u8 = pyxel::COLOR_GREEN;
    #[pyattr]
    const COLOR_BROWN: u8 = pyxel::COLOR_BROWN;
    #[pyattr]
    const COLOR_DARK_BLUE: u8 = pyxel::COLOR_DARK_BLUE;
    #[pyattr]
    const COLOR_LIGHT_BLUE: u8 = pyxel::COLOR_LIGHT_BLUE;
    #[pyattr]
    const COLOR_WHITE: u8 = pyxel::COLOR_WHITE;
    #[pyattr]
    const COLOR_RED: u8 = pyxel::COLOR_RED;
    #[pyattr]
    const COLOR_ORANGE: u8 = pyxel::COLOR_ORANGE;
    #[pyattr]
    const COLOR_YELLOW: u8 = pyxel::COLOR_YELLOW;
    #[pyattr]
    const COLOR_LIME: u8 = pyxel::COLOR_LIME;
    #[pyattr]
    const COLOR_CYAN: u8 = pyxel::COLOR_CYAN;
    #[pyattr]
    const COLOR_GRAY: u8 = pyxel::COLOR_GRAY;
    #[pyattr]
    const COLOR_PINK: u8 = pyxel::COLOR_PINK;
    #[pyattr]
    const COLOR_PEACH: u8 = pyxel::COLOR_PEACH;
    #[pyattr]
    const FONT_WIDTH: u32 = pyxel::FONT_WIDTH;
    #[pyattr]
    const FONT_HEIGHT: u32 = pyxel::FONT_HEIGHT;
    #[pyattr]
    const NUM_CHANNELS: u32 = pyxel::NUM_CHANNELS;
    #[pyattr]
    const NUM_TONES: u32 = pyxel::NUM_TONES;
    #[pyattr]
    const NUM_SOUNDS: u32 = pyxel::NUM_SOUNDS;
    #[pyattr]
    const NUM_MUSICS: u32 = pyxel::NUM_MUSICS;

    // Tone constants
    #[pyattr]
    const TONE_TRIANGLE: u8 = pyxel::TONE_TRIANGLE;
    #[pyattr]
    const TONE_SQUARE: u8 = pyxel::TONE_SQUARE;
    #[pyattr]
    const TONE_PULSE: u8 = pyxel::TONE_PULSE;
    #[pyattr]
    const TONE_NOISE: u8 = pyxel::TONE_NOISE;

    // Effect constants
    #[pyattr]
    const EFFECT_NONE: u8 = pyxel::EFFECT_NONE;
    #[pyattr]
    const EFFECT_SLIDE: u8 = pyxel::EFFECT_SLIDE;
    #[pyattr]
    const EFFECT_VIBRATO: u8 = pyxel::EFFECT_VIBRATO;
    #[pyattr]
    const EFFECT_FADEOUT: u8 = pyxel::EFFECT_FADEOUT;
    #[pyattr]
    const EFFECT_HALF_FADEOUT: u8 = pyxel::EFFECT_HALF_FADEOUT;
    #[pyattr]
    const EFFECT_QUARTER_FADEOUT: u8 = pyxel::EFFECT_QUARTER_FADEOUT;

    // Key constants
    #[pyattr]
    const KEY_UNKNOWN: u32 = pyxel::KEY_UNKNOWN;
    #[pyattr]
    const KEY_BACKSPACE: u32 = pyxel::KEY_BACKSPACE;
    #[pyattr]
    const KEY_TAB: u32 = pyxel::KEY_TAB;
    #[pyattr]
    const KEY_RETURN: u32 = pyxel::KEY_RETURN;
    #[pyattr]
    const KEY_ESCAPE: u32 = pyxel::KEY_ESCAPE;
    #[pyattr]
    const KEY_SPACE: u32 = pyxel::KEY_SPACE;
    #[pyattr]
    const KEY_EXCLAIM: u32 = pyxel::KEY_EXCLAIM;
    #[pyattr]
    const KEY_QUOTEDBL: u32 = pyxel::KEY_QUOTEDBL;
    #[pyattr]
    const KEY_HASH: u32 = pyxel::KEY_HASH;
    #[pyattr]
    const KEY_DOLLAR: u32 = pyxel::KEY_DOLLAR;
    #[pyattr]
    const KEY_PERCENT: u32 = pyxel::KEY_PERCENT;
    #[pyattr]
    const KEY_AMPERSAND: u32 = pyxel::KEY_AMPERSAND;
    #[pyattr]
    const KEY_QUOTE: u32 = pyxel::KEY_QUOTE;
    #[pyattr]
    const KEY_LEFTPAREN: u32 = pyxel::KEY_LEFTPAREN;
    #[pyattr]
    const KEY_RIGHTPAREN: u32 = pyxel::KEY_RIGHTPAREN;
    #[pyattr]
    const KEY_ASTERISK: u32 = pyxel::KEY_ASTERISK;
    #[pyattr]
    const KEY_PLUS: u32 = pyxel::KEY_PLUS;
    #[pyattr]
    const KEY_COMMA: u32 = pyxel::KEY_COMMA;
    #[pyattr]
    const KEY_MINUS: u32 = pyxel::KEY_MINUS;
    #[pyattr]
    const KEY_PERIOD: u32 = pyxel::KEY_PERIOD;
    #[pyattr]
    const KEY_SLASH: u32 = pyxel::KEY_SLASH;
    #[pyattr]
    const KEY_0: u32 = pyxel::KEY_0;
    #[pyattr]
    const KEY_1: u32 = pyxel::KEY_1;
    #[pyattr]
    const KEY_2: u32 = pyxel::KEY_2;
    #[pyattr]
    const KEY_3: u32 = pyxel::KEY_3;
    #[pyattr]
    const KEY_4: u32 = pyxel::KEY_4;
    #[pyattr]
    const KEY_5: u32 = pyxel::KEY_5;
    #[pyattr]
    const KEY_6: u32 = pyxel::KEY_6;
    #[pyattr]
    const KEY_7: u32 = pyxel::KEY_7;
    #[pyattr]
    const KEY_8: u32 = pyxel::KEY_8;
    #[pyattr]
    const KEY_9: u32 = pyxel::KEY_9;
    #[pyattr]
    const KEY_COLON: u32 = pyxel::KEY_COLON;
    #[pyattr]
    const KEY_SEMICOLON: u32 = pyxel::KEY_SEMICOLON;
    #[pyattr]
    const KEY_LESS: u32 = pyxel::KEY_LESS;
    #[pyattr]
    const KEY_EQUALS: u32 = pyxel::KEY_EQUALS;
    #[pyattr]
    const KEY_GREATER: u32 = pyxel::KEY_GREATER;
    #[pyattr]
    const KEY_QUESTION: u32 = pyxel::KEY_QUESTION;
    #[pyattr]
    const KEY_AT: u32 = pyxel::KEY_AT;
    #[pyattr]
    const KEY_LEFTBRACKET: u32 = pyxel::KEY_LEFTBRACKET;
    #[pyattr]
    const KEY_BACKSLASH: u32 = pyxel::KEY_BACKSLASH;
    #[pyattr]
    const KEY_RIGHTBRACKET: u32 = pyxel::KEY_RIGHTBRACKET;
    #[pyattr]
    const KEY_CARET: u32 = pyxel::KEY_CARET;
    #[pyattr]
    const KEY_UNDERSCORE: u32 = pyxel::KEY_UNDERSCORE;
    #[pyattr]
    const KEY_BACKQUOTE: u32 = pyxel::KEY_BACKQUOTE;
    #[pyattr]
    const KEY_A: u32 = pyxel::KEY_A;
    #[pyattr]
    const KEY_B: u32 = pyxel::KEY_B;
    #[pyattr]
    const KEY_C: u32 = pyxel::KEY_C;
    #[pyattr]
    const KEY_D: u32 = pyxel::KEY_D;
    #[pyattr]
    const KEY_E: u32 = pyxel::KEY_E;
    #[pyattr]
    const KEY_F: u32 = pyxel::KEY_F;
    #[pyattr]
    const KEY_G: u32 = pyxel::KEY_G;
    #[pyattr]
    const KEY_H: u32 = pyxel::KEY_H;
    #[pyattr]
    const KEY_I: u32 = pyxel::KEY_I;
    #[pyattr]
    const KEY_J: u32 = pyxel::KEY_J;
    #[pyattr]
    const KEY_K: u32 = pyxel::KEY_K;
    #[pyattr]
    const KEY_L: u32 = pyxel::KEY_L;
    #[pyattr]
    const KEY_M: u32 = pyxel::KEY_M;
    #[pyattr]
    const KEY_N: u32 = pyxel::KEY_N;
    #[pyattr]
    const KEY_O: u32 = pyxel::KEY_O;
    #[pyattr]
    const KEY_P: u32 = pyxel::KEY_P;
    #[pyattr]
    const KEY_Q: u32 = pyxel::KEY_Q;
    #[pyattr]
    const KEY_R: u32 = pyxel::KEY_R;
    #[pyattr]
    const KEY_S: u32 = pyxel::KEY_S;
    #[pyattr]
    const KEY_T: u32 = pyxel::KEY_T;
    #[pyattr]
    const KEY_U: u32 = pyxel::KEY_U;
    #[pyattr]
    const KEY_V: u32 = pyxel::KEY_V;
    #[pyattr]
    const KEY_W: u32 = pyxel::KEY_W;
    #[pyattr]
    const KEY_X: u32 = pyxel::KEY_X;
    #[pyattr]
    const KEY_Y: u32 = pyxel::KEY_Y;
    #[pyattr]
    const KEY_Z: u32 = pyxel::KEY_Z;
    #[pyattr]
    const KEY_DELETE: u32 = pyxel::KEY_DELETE;
    #[pyattr]
    const KEY_CAPSLOCK: u32 = pyxel::KEY_CAPSLOCK;
    #[pyattr]
    const KEY_F1: u32 = pyxel::KEY_F1;
    #[pyattr]
    const KEY_F2: u32 = pyxel::KEY_F2;
    #[pyattr]
    const KEY_F3: u32 = pyxel::KEY_F3;
    #[pyattr]
    const KEY_F4: u32 = pyxel::KEY_F4;
    #[pyattr]
    const KEY_F5: u32 = pyxel::KEY_F5;
    #[pyattr]
    const KEY_F6: u32 = pyxel::KEY_F6;
    #[pyattr]
    const KEY_F7: u32 = pyxel::KEY_F7;
    #[pyattr]
    const KEY_F8: u32 = pyxel::KEY_F8;
    #[pyattr]
    const KEY_F9: u32 = pyxel::KEY_F9;
    #[pyattr]
    const KEY_F10: u32 = pyxel::KEY_F10;
    #[pyattr]
    const KEY_F11: u32 = pyxel::KEY_F11;
    #[pyattr]
    const KEY_F12: u32 = pyxel::KEY_F12;
    #[pyattr]
    const KEY_PRINTSCREEN: u32 = pyxel::KEY_PRINTSCREEN;
    #[pyattr]
    const KEY_SCROLLLOCK: u32 = pyxel::KEY_SCROLLLOCK;
    #[pyattr]
    const KEY_PAUSE: u32 = pyxel::KEY_PAUSE;
    #[pyattr]
    const KEY_INSERT: u32 = pyxel::KEY_INSERT;
    #[pyattr]
    const KEY_HOME: u32 = pyxel::KEY_HOME;
    #[pyattr]
    const KEY_PAGEUP: u32 = pyxel::KEY_PAGEUP;
    #[pyattr]
    const KEY_END: u32 = pyxel::KEY_END;
    #[pyattr]
    const KEY_PAGEDOWN: u32 = pyxel::KEY_PAGEDOWN;
    #[pyattr]
    const KEY_RIGHT: u32 = pyxel::KEY_RIGHT;
    #[pyattr]
    const KEY_LEFT: u32 = pyxel::KEY_LEFT;
    #[pyattr]
    const KEY_DOWN: u32 = pyxel::KEY_DOWN;
    #[pyattr]
    const KEY_UP: u32 = pyxel::KEY_UP;
    #[pyattr]
    const KEY_LCTRL: u32 = pyxel::KEY_LCTRL;
    #[pyattr]
    const KEY_LSHIFT: u32 = pyxel::KEY_LSHIFT;
    #[pyattr]
    const KEY_LALT: u32 = pyxel::KEY_LALT;
    #[pyattr]
    const KEY_LGUI: u32 = pyxel::KEY_LGUI;
    #[pyattr]
    const KEY_RCTRL: u32 = pyxel::KEY_RCTRL;
    #[pyattr]
    const KEY_RSHIFT: u32 = pyxel::KEY_RSHIFT;
    #[pyattr]
    const KEY_RALT: u32 = pyxel::KEY_RALT;
    #[pyattr]
    const KEY_RGUI: u32 = pyxel::KEY_RGUI;
    #[pyattr]
    const KEY_NONE: u32 = pyxel::KEY_NONE;
    #[pyattr]
    const KEY_SHIFT: u32 = pyxel::KEY_SHIFT;
    #[pyattr]
    const KEY_CTRL: u32 = pyxel::KEY_CTRL;
    #[pyattr]
    const KEY_ALT: u32 = pyxel::KEY_ALT;
    #[pyattr]
    const KEY_GUI: u32 = pyxel::KEY_GUI;

    // Mouse constants
    #[pyattr]
    const MOUSE_POS_X: u32 = pyxel::MOUSE_POS_X;
    #[pyattr]
    const MOUSE_POS_Y: u32 = pyxel::MOUSE_POS_Y;
    #[pyattr]
    const MOUSE_WHEEL_X: u32 = pyxel::MOUSE_WHEEL_X;
    #[pyattr]
    const MOUSE_WHEEL_Y: u32 = pyxel::MOUSE_WHEEL_Y;
    #[pyattr]
    const MOUSE_BUTTON_LEFT: u32 = pyxel::MOUSE_BUTTON_LEFT;
    #[pyattr]
    const MOUSE_BUTTON_MIDDLE: u32 = pyxel::MOUSE_BUTTON_MIDDLE;
    #[pyattr]
    const MOUSE_BUTTON_RIGHT: u32 = pyxel::MOUSE_BUTTON_RIGHT;
    #[pyattr]
    const MOUSE_BUTTON_X1: u32 = pyxel::MOUSE_BUTTON_X1;
    #[pyattr]
    const MOUSE_BUTTON_X2: u32 = pyxel::MOUSE_BUTTON_X2;

    // Gamepad 1 constants
    #[pyattr]
    const GAMEPAD1_AXIS_LEFTX: u32 = pyxel::GAMEPAD1_AXIS_LEFTX;
    #[pyattr]
    const GAMEPAD1_AXIS_LEFTY: u32 = pyxel::GAMEPAD1_AXIS_LEFTY;
    #[pyattr]
    const GAMEPAD1_AXIS_RIGHTX: u32 = pyxel::GAMEPAD1_AXIS_RIGHTX;
    #[pyattr]
    const GAMEPAD1_AXIS_RIGHTY: u32 = pyxel::GAMEPAD1_AXIS_RIGHTY;
    #[pyattr]
    const GAMEPAD1_AXIS_TRIGGERLEFT: u32 = pyxel::GAMEPAD1_AXIS_TRIGGERLEFT;
    #[pyattr]
    const GAMEPAD1_AXIS_TRIGGERRIGHT: u32 = pyxel::GAMEPAD1_AXIS_TRIGGERRIGHT;
    #[pyattr]
    const GAMEPAD1_BUTTON_A: u32 = pyxel::GAMEPAD1_BUTTON_A;
    #[pyattr]
    const GAMEPAD1_BUTTON_B: u32 = pyxel::GAMEPAD1_BUTTON_B;
    #[pyattr]
    const GAMEPAD1_BUTTON_X: u32 = pyxel::GAMEPAD1_BUTTON_X;
    #[pyattr]
    const GAMEPAD1_BUTTON_Y: u32 = pyxel::GAMEPAD1_BUTTON_Y;
    #[pyattr]
    const GAMEPAD1_BUTTON_BACK: u32 = pyxel::GAMEPAD1_BUTTON_BACK;
    #[pyattr]
    const GAMEPAD1_BUTTON_GUIDE: u32 = pyxel::GAMEPAD1_BUTTON_GUIDE;
    #[pyattr]
    const GAMEPAD1_BUTTON_START: u32 = pyxel::GAMEPAD1_BUTTON_START;
    #[pyattr]
    const GAMEPAD1_BUTTON_LEFTSTICK: u32 = pyxel::GAMEPAD1_BUTTON_LEFTSTICK;
    #[pyattr]
    const GAMEPAD1_BUTTON_RIGHTSTICK: u32 = pyxel::GAMEPAD1_BUTTON_RIGHTSTICK;
    #[pyattr]
    const GAMEPAD1_BUTTON_LEFTSHOULDER: u32 = pyxel::GAMEPAD1_BUTTON_LEFTSHOULDER;
    #[pyattr]
    const GAMEPAD1_BUTTON_RIGHTSHOULDER: u32 = pyxel::GAMEPAD1_BUTTON_RIGHTSHOULDER;
    #[pyattr]
    const GAMEPAD1_BUTTON_DPAD_UP: u32 = pyxel::GAMEPAD1_BUTTON_DPAD_UP;
    #[pyattr]
    const GAMEPAD1_BUTTON_DPAD_DOWN: u32 = pyxel::GAMEPAD1_BUTTON_DPAD_DOWN;
    #[pyattr]
    const GAMEPAD1_BUTTON_DPAD_LEFT: u32 = pyxel::GAMEPAD1_BUTTON_DPAD_LEFT;
    #[pyattr]
    const GAMEPAD1_BUTTON_DPAD_RIGHT: u32 = pyxel::GAMEPAD1_BUTTON_DPAD_RIGHT;

    // Gamepad 2 constants
    #[pyattr]
    const GAMEPAD2_AXIS_LEFTX: u32 = pyxel::GAMEPAD2_AXIS_LEFTX;
    #[pyattr]
    const GAMEPAD2_AXIS_LEFTY: u32 = pyxel::GAMEPAD2_AXIS_LEFTY;
    #[pyattr]
    const GAMEPAD2_AXIS_RIGHTX: u32 = pyxel::GAMEPAD2_AXIS_RIGHTX;
    #[pyattr]
    const GAMEPAD2_AXIS_RIGHTY: u32 = pyxel::GAMEPAD2_AXIS_RIGHTY;
    #[pyattr]
    const GAMEPAD2_AXIS_TRIGGERLEFT: u32 = pyxel::GAMEPAD2_AXIS_TRIGGERLEFT;
    #[pyattr]
    const GAMEPAD2_AXIS_TRIGGERRIGHT: u32 = pyxel::GAMEPAD2_AXIS_TRIGGERRIGHT;
    #[pyattr]
    const GAMEPAD2_BUTTON_A: u32 = pyxel::GAMEPAD2_BUTTON_A;
    #[pyattr]
    const GAMEPAD2_BUTTON_B: u32 = pyxel::GAMEPAD2_BUTTON_B;
    #[pyattr]
    const GAMEPAD2_BUTTON_X: u32 = pyxel::GAMEPAD2_BUTTON_X;
    #[pyattr]
    const GAMEPAD2_BUTTON_Y: u32 = pyxel::GAMEPAD2_BUTTON_Y;
    #[pyattr]
    const GAMEPAD2_BUTTON_BACK: u32 = pyxel::GAMEPAD2_BUTTON_BACK;
    #[pyattr]
    const GAMEPAD2_BUTTON_GUIDE: u32 = pyxel::GAMEPAD2_BUTTON_GUIDE;
    #[pyattr]
    const GAMEPAD2_BUTTON_START: u32 = pyxel::GAMEPAD2_BUTTON_START;
    #[pyattr]
    const GAMEPAD2_BUTTON_LEFTSTICK: u32 = pyxel::GAMEPAD2_BUTTON_LEFTSTICK;
    #[pyattr]
    const GAMEPAD2_BUTTON_RIGHTSTICK: u32 = pyxel::GAMEPAD2_BUTTON_RIGHTSTICK;
    #[pyattr]
    const GAMEPAD2_BUTTON_LEFTSHOULDER: u32 = pyxel::GAMEPAD2_BUTTON_LEFTSHOULDER;
    #[pyattr]
    const GAMEPAD2_BUTTON_RIGHTSHOULDER: u32 = pyxel::GAMEPAD2_BUTTON_RIGHTSHOULDER;
    #[pyattr]
    const GAMEPAD2_BUTTON_DPAD_UP: u32 = pyxel::GAMEPAD2_BUTTON_DPAD_UP;
    #[pyattr]
    const GAMEPAD2_BUTTON_DPAD_DOWN: u32 = pyxel::GAMEPAD2_BUTTON_DPAD_DOWN;
    #[pyattr]
    const GAMEPAD2_BUTTON_DPAD_LEFT: u32 = pyxel::GAMEPAD2_BUTTON_DPAD_LEFT;
    #[pyattr]
    const GAMEPAD2_BUTTON_DPAD_RIGHT: u32 = pyxel::GAMEPAD2_BUTTON_DPAD_RIGHT;

    // Gamepad 3 constants
    #[pyattr]
    const GAMEPAD3_AXIS_LEFTX: u32 = pyxel::GAMEPAD3_AXIS_LEFTX;
    #[pyattr]
    const GAMEPAD3_AXIS_LEFTY: u32 = pyxel::GAMEPAD3_AXIS_LEFTY;
    #[pyattr]
    const GAMEPAD3_AXIS_RIGHTX: u32 = pyxel::GAMEPAD3_AXIS_RIGHTX;
    #[pyattr]
    const GAMEPAD3_AXIS_RIGHTY: u32 = pyxel::GAMEPAD3_AXIS_RIGHTY;
    #[pyattr]
    const GAMEPAD3_AXIS_TRIGGERLEFT: u32 = pyxel::GAMEPAD3_AXIS_TRIGGERLEFT;
    #[pyattr]
    const GAMEPAD3_AXIS_TRIGGERRIGHT: u32 = pyxel::GAMEPAD3_AXIS_TRIGGERRIGHT;
    #[pyattr]
    const GAMEPAD3_BUTTON_A: u32 = pyxel::GAMEPAD3_BUTTON_A;
    #[pyattr]
    const GAMEPAD3_BUTTON_B: u32 = pyxel::GAMEPAD3_BUTTON_B;
    #[pyattr]
    const GAMEPAD3_BUTTON_X: u32 = pyxel::GAMEPAD3_BUTTON_X;
    #[pyattr]
    const GAMEPAD3_BUTTON_Y: u32 = pyxel::GAMEPAD3_BUTTON_Y;
    #[pyattr]
    const GAMEPAD3_BUTTON_BACK: u32 = pyxel::GAMEPAD3_BUTTON_BACK;
    #[pyattr]
    const GAMEPAD3_BUTTON_GUIDE: u32 = pyxel::GAMEPAD3_BUTTON_GUIDE;
    #[pyattr]
    const GAMEPAD3_BUTTON_START: u32 = pyxel::GAMEPAD3_BUTTON_START;
    #[pyattr]
    const GAMEPAD3_BUTTON_LEFTSTICK: u32 = pyxel::GAMEPAD3_BUTTON_LEFTSTICK;
    #[pyattr]
    const GAMEPAD3_BUTTON_RIGHTSTICK: u32 = pyxel::GAMEPAD3_BUTTON_RIGHTSTICK;
    #[pyattr]
    const GAMEPAD3_BUTTON_LEFTSHOULDER: u32 = pyxel::GAMEPAD3_BUTTON_LEFTSHOULDER;
    #[pyattr]
    const GAMEPAD3_BUTTON_RIGHTSHOULDER: u32 = pyxel::GAMEPAD3_BUTTON_RIGHTSHOULDER;
    #[pyattr]
    const GAMEPAD3_BUTTON_DPAD_UP: u32 = pyxel::GAMEPAD3_BUTTON_DPAD_UP;
    #[pyattr]
    const GAMEPAD3_BUTTON_DPAD_DOWN: u32 = pyxel::GAMEPAD3_BUTTON_DPAD_DOWN;
    #[pyattr]
    const GAMEPAD3_BUTTON_DPAD_LEFT: u32 = pyxel::GAMEPAD3_BUTTON_DPAD_LEFT;
    #[pyattr]
    const GAMEPAD3_BUTTON_DPAD_RIGHT: u32 = pyxel::GAMEPAD3_BUTTON_DPAD_RIGHT;

    // Gamepad 4 constants
    #[pyattr]
    const GAMEPAD4_AXIS_LEFTX: u32 = pyxel::GAMEPAD4_AXIS_LEFTX;
    #[pyattr]
    const GAMEPAD4_AXIS_LEFTY: u32 = pyxel::GAMEPAD4_AXIS_LEFTY;
    #[pyattr]
    const GAMEPAD4_AXIS_RIGHTX: u32 = pyxel::GAMEPAD4_AXIS_RIGHTX;
    #[pyattr]
    const GAMEPAD4_AXIS_RIGHTY: u32 = pyxel::GAMEPAD4_AXIS_RIGHTY;
    #[pyattr]
    const GAMEPAD4_AXIS_TRIGGERLEFT: u32 = pyxel::GAMEPAD4_AXIS_TRIGGERLEFT;
    #[pyattr]
    const GAMEPAD4_AXIS_TRIGGERRIGHT: u32 = pyxel::GAMEPAD4_AXIS_TRIGGERRIGHT;
    #[pyattr]
    const GAMEPAD4_BUTTON_A: u32 = pyxel::GAMEPAD4_BUTTON_A;
    #[pyattr]
    const GAMEPAD4_BUTTON_B: u32 = pyxel::GAMEPAD4_BUTTON_B;
    #[pyattr]
    const GAMEPAD4_BUTTON_X: u32 = pyxel::GAMEPAD4_BUTTON_X;
    #[pyattr]
    const GAMEPAD4_BUTTON_Y: u32 = pyxel::GAMEPAD4_BUTTON_Y;
    #[pyattr]
    const GAMEPAD4_BUTTON_BACK: u32 = pyxel::GAMEPAD4_BUTTON_BACK;
    #[pyattr]
    const GAMEPAD4_BUTTON_GUIDE: u32 = pyxel::GAMEPAD4_BUTTON_GUIDE;
    #[pyattr]
    const GAMEPAD4_BUTTON_START: u32 = pyxel::GAMEPAD4_BUTTON_START;
    #[pyattr]
    const GAMEPAD4_BUTTON_LEFTSTICK: u32 = pyxel::GAMEPAD4_BUTTON_LEFTSTICK;
    #[pyattr]
    const GAMEPAD4_BUTTON_RIGHTSTICK: u32 = pyxel::GAMEPAD4_BUTTON_RIGHTSTICK;
    #[pyattr]
    const GAMEPAD4_BUTTON_LEFTSHOULDER: u32 = pyxel::GAMEPAD4_BUTTON_LEFTSHOULDER;
    #[pyattr]
    const GAMEPAD4_BUTTON_RIGHTSHOULDER: u32 = pyxel::GAMEPAD4_BUTTON_RIGHTSHOULDER;
    #[pyattr]
    const GAMEPAD4_BUTTON_DPAD_UP: u32 = pyxel::GAMEPAD4_BUTTON_DPAD_UP;
    #[pyattr]
    const GAMEPAD4_BUTTON_DPAD_DOWN: u32 = pyxel::GAMEPAD4_BUTTON_DPAD_DOWN;
    #[pyattr]
    const GAMEPAD4_BUTTON_DPAD_LEFT: u32 = pyxel::GAMEPAD4_BUTTON_DPAD_LEFT;
    #[pyattr]
    const GAMEPAD4_BUTTON_DPAD_RIGHT: u32 = pyxel::GAMEPAD4_BUTTON_DPAD_RIGHT;

    // String constants
    #[pyattr]
    const WINDOW_STATE_ENV: &str = pyxel::WINDOW_STATE_ENV;
    #[pyattr]
    const APP_FILE_EXTENSION: &str = pyxel::APP_FILE_EXTENSION;
    #[pyattr]
    const APP_STARTUP_SCRIPT_FILE: &str = pyxel::APP_STARTUP_SCRIPT_FILE;
    #[pyattr]
    const RESOURCE_FILE_EXTENSION: &str = pyxel::RESOURCE_FILE_EXTENSION;
    #[pyattr]
    const PALETTE_FILE_EXTENSION: &str = pyxel::PALETTE_FILE_EXTENSION;
    #[pyattr]
    const BASE_DIR: &str = pyxel::BASE_DIR;

    // Utility functions
    #[pyfunction]
    fn _pid_exists(pid: u32) -> bool {
        #[cfg(not(target_os = "windows"))]
        unsafe {
            libc::kill(pid as i32, 0) == 0
        }
        #[cfg(target_os = "windows")]
        {
            let _ = pid;
            false
        }
    }
}
