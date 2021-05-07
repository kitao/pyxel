use crate::imagebank::Imagebank;
use crate::palette::Palette;
use crate::settings::{
    DISPLAY_COLORS, IMAGEBANK_COUNT, IMAGEBANK_SIZE, TILEMAP_COUNT, TILEMAP_SIZE,
};
use crate::tilemap::Tilemap;

static mut INSTANCE: Option<Graphics> = None;

#[inline]
pub fn graphics() -> &'static mut Graphics {
    unsafe { INSTANCE.as_mut().expect("Graphics is not initialized") }
}

pub fn init_graphics(width: u32, height: u32) {
    unsafe {
        assert!(INSTANCE.is_none(), "Graphics is already initialized");

        INSTANCE = Some(Graphics::new(width, height));
    }
}

pub struct Graphics {
    screen: Imagebank,
    imagebanks: Vec<Imagebank>,
    tilemaps: Vec<Tilemap>,
}

impl Graphics {
    pub fn new(screen_width: u32, screen_height: u32) -> Graphics {
        let screen = Imagebank::new(screen_width, screen_height);
        let mut imagebanks = Vec::new();
        let mut tilemaps = Vec::new();

        for _ in 0..IMAGEBANK_COUNT {
            imagebanks.push(Imagebank::new(IMAGEBANK_SIZE, IMAGEBANK_SIZE));
        }

        for _ in 0..TILEMAP_COUNT {
            tilemaps.push(Tilemap::new(TILEMAP_SIZE, TILEMAP_SIZE));
        }

        let mut graphics = Graphics {
            screen: screen,
            imagebanks: imagebanks,
            tilemaps: tilemaps,
        };

        graphics.palette().set_display_colors(&DISPLAY_COLORS);

        graphics
    }

    #[inline]
    pub fn screen(&mut self) -> &mut Imagebank {
        &mut self.screen
    }

    #[inline]
    pub fn palette(&mut self) -> &mut Palette {
        self.screen.palette_mut()
    }

    #[inline]
    pub fn imagebank(&mut self, no: usize) -> &mut Imagebank {
        if no < self.imagebanks.len() {
            &mut self.imagebanks[no]
        } else {
            &mut self.screen
        }
    }

    #[inline]
    pub fn tilemap(&mut self, no: usize) -> &mut Tilemap {
        &mut self.tilemaps[no]
    }
}
