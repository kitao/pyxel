use crate::settings::{Tile, TILE_COUNT};
use crate::Palette;

#[derive(Debug)]
pub struct TilePalette;

impl Palette<Tile> for TilePalette {
    #[inline]
    fn get_render_value(&self, original_value: Tile) -> Tile {
        assert!((original_value as usize) < TILE_COUNT);

        original_value
    }
}

impl TilePalette {
    pub fn new() -> TilePalette {
        TilePalette {}
    }
}

#[cfg(test)]
mod tile_palette_tests {
    use super::*;

    #[test]
    fn get_render_value() {
        let palette = TilePalette::new();

        for i in 0..TILE_COUNT {
            assert_eq!(palette.get_render_value(i as Tile), i as Tile);
        }
    }

    #[test]
    #[should_panic]
    fn get_render_value_panic() {
        let palette = TilePalette::new();

        palette.get_render_value(TILE_COUNT as Tile);
    }
}
