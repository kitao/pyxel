use sdl2::render::WindowCanvas as SdlWindowCanvas;
use sdl2::EventPump as SdlEventPump;
use sdl2::Sdl as SdlContext;

global_instance!(System, system);

pub struct System {
    sdl_context: SdlContext,
    sdl_event_pump: SdlEventPump,
    sdl_canvas: SdlWindowCanvas,
}

pub fn init_system(name: &str, width: usize, height: usize) {
    let sdl_context = sdl2::init().unwrap();
    let sdl_event_pump = sdl_context.event_pump().unwrap();
    let sdl_video = sdl_context.video().unwrap();
    let sdl_window = sdl_video
        .window(name, width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();
    let sdl_canvas = sdl_window.into_canvas().build().unwrap();

    let system = System {
        sdl_context: sdl_context,
        sdl_event_pump: sdl_event_pump,
        sdl_canvas: sdl_canvas,
    };

    set_instance(system);
}

impl System {
    pub fn set_window_caption(&self, caption: &str) {
        //
    }

    pub fn get_window_caption(&self) -> &'static str {
        "hoge"
    }

    pub fn get_screen_size(&self) -> (usize, usize) {
        (1, 2)
    }

    pub fn is_fullscreen(&self) -> bool {
        true
    }

    pub fn set_fullscreen(&self, is_fullscreen: bool) {
        //
    }

    pub fn get_palette_color(&self, index: usize) -> u32 {
        0
    }

    pub fn set_palette_color(&self, index: usize, color: u32) {
        //
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hoge() {
        crate::init_system("test", 100, 100);
    }
}
