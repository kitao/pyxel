use crate::sdl2::platform_sdl2::PlatformSdl2;
use crate::sdl2::sdl2_sys::*;

impl PlatformSdl2 {
    pub fn init(&mut self) {
        assert!(
            unsafe { SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO | SDL_INIT_GAMECONTROLLER,) } >= 0,
            "Failed to initialize SDL2"
        );
    }

    pub fn quit(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }

    pub fn ticks(&mut self) -> u32 {
        unsafe { SDL_GetTicks() }
    }

    pub fn delay(&mut self, ms: u32) {
        unsafe {
            SDL_Delay(ms);
        }
    }
}
