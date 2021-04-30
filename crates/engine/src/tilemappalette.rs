use crate::settings::{Tile, TILE_COUNT};
use crate::Palette;

pub struct TilemapPalette;

impl Palette<Tile> for TilemapPalette {
    #[inline]
    fn get_render_color(&self, original_color: Tile) -> Tile {
        assert!((original_color as usize) < TILE_COUNT);

        original_color
    }
}

impl TilemapPalette {
    pub fn new() -> TilemapPalette {
        TilemapPalette {}
    }
}

#[cfg(test)]
mod tilemap_palette_tests {
    use super::*;

    #[test]
    fn get_render_color() {
        let palette = TilemapPalette::new();

        for i in 0..TILE_COUNT {
            assert_eq!(palette.get_render_color(i as Tile), i as Tile);
        }
    }

    #[test]
    #[should_panic]
    fn get_render_color_panic() {
        let palette = TilemapPalette::new();

        palette.get_render_color(TILE_COUNT as Tile);
    }
}
