use crate::settings::{Color, Rgb24, COLOR_COUNT, RGB24_MAX_VALUE};
use crate::Palette;

#[derive(Debug)]
pub struct ColorPalette {
    render_colors: [Color; COLOR_COUNT],
    display_colors: [Rgb24; COLOR_COUNT],
}

impl Palette<Color> for ColorPalette {
    #[inline]
    fn get_render_value(&self, original_value: Color) -> Color {
        self.render_colors[original_value as usize]
    }
}

impl ColorPalette {
    pub fn new() -> ColorPalette {
        let mut palette = ColorPalette {
            render_colors: [0; COLOR_COUNT],
            display_colors: [0; COLOR_COUNT],
        };

        palette.reset_render_colors();

        palette
    }

    #[inline]
    pub fn get_render_color(&self, original_color: Color) -> Color {
        self.get_render_value(original_color)
    }

    #[inline]
    pub fn set_render_color(&mut self, original_color: Color, render_color: Color) {
        assert!((render_color as usize) < COLOR_COUNT);

        self.render_colors[original_color as usize] = render_color;
    }

    #[inline]
    pub fn reset_render_colors(&mut self) {
        for i in 0..COLOR_COUNT {
            self.render_colors[i] = i as Color;
        }
    }

    #[inline]
    pub fn get_display_color(&self, render_color: Color) -> Rgb24 {
        self.display_colors[render_color as usize]
    }

    #[inline]
    pub fn set_display_color(&mut self, render_color: Color, display_color: Rgb24) {
        assert!(display_color <= RGB24_MAX_VALUE);

        self.display_colors[render_color as usize] = display_color;
    }

    #[inline]
    pub fn set_display_colors(&mut self, display_colors: &[Rgb24]) {
        assert!(display_colors.len() == COLOR_COUNT);

        for (i, &display_color) in display_colors.iter().enumerate() {
            self.set_display_color(i as Color, display_color);
        }
    }
}

#[cfg(test)]
mod color_palette_tests {
    use super::*;

    #[test]
    fn new() {
        let palette = ColorPalette::new();

        for i in 0..COLOR_COUNT {
            assert_eq!(palette.get_render_color(i as Color), i as Color);
            assert_eq!(palette.get_display_color(i as Color), 0);
        }
    }

    #[test]
    fn set_render_color() {
        let mut palette = ColorPalette::new();

        for i in 0..COLOR_COUNT {
            assert_eq!(palette.get_render_color(i as Color), i as Color);
        }

        for i in 0..COLOR_COUNT {
            palette.set_render_color(i as Color, ((i + 1) % COLOR_COUNT) as Color);
        }

        for i in 0..COLOR_COUNT {
            assert_eq!(
                palette.get_render_color(i as Color),
                ((i + 1) % COLOR_COUNT) as Color
            );
        }
    }

    #[test]
    #[should_panic]
    fn set_render_color_panic() {
        let mut palette = ColorPalette::new();

        palette.set_render_color(0, COLOR_COUNT as Color);
    }

    #[test]
    fn reset_render_colors() {
        let mut palette = ColorPalette::new();

        for i in 0..COLOR_COUNT {
            palette.set_render_color(i as Color, ((i + 1) % COLOR_COUNT) as Color);
        }

        palette.reset_render_colors();

        for i in 0..COLOR_COUNT {
            assert_eq!(palette.get_render_color(i as Color), i as Color);
        }
    }

    #[test]
    fn set_display_color() {
        let mut palette = ColorPalette::new();

        for i in 0..COLOR_COUNT {
            assert_eq!(palette.get_display_color(i as Color), 0);
        }

        for i in 0..COLOR_COUNT {
            palette.set_display_color(i as Color, (0x111111 * i) as Rgb24);
        }

        for i in 0..COLOR_COUNT {
            assert_eq!(
                palette.get_display_color(i as Color),
                (0x111111 * i) as Rgb24
            );
        }
    }

    #[test]
    #[should_panic]
    fn set_display_color_panic() {
        let mut palette = ColorPalette::new();

        palette.set_display_color(0, RGB24_MAX_VALUE + 1);
    }

    #[test]
    fn set_display_colors() {
        let mut palette = ColorPalette::new();

        palette.set_display_colors(&[
            0x111111, 0x222222, 0x333333, 0x444444, 0x555555, 0x666666, 0x777777, 0x888888,
            0x999999, 0xaaaaaa, 0xbbbbbb, 0xcccccc, 0xdddddd, 0xeeeeee, 0xffffff, 0x000000,
        ]);

        for i in 0..COLOR_COUNT {
            assert_eq!(
                palette.get_display_color(i as Color),
                (0x111111 * ((i + 1) % COLOR_COUNT)) as Rgb24
            );
        }
    }

    #[test]
    #[should_panic]
    fn set_display_colors_panic() {
        let mut palette = ColorPalette::new();

        palette.set_display_colors(&[
            0x111111, 0x222222, 0x333333, 0x444444, 0x555555, 0x666666, 0x777777, 0x888888,
            0x999999, 0xaaaaaa, 0xbbbbbb, 0xcccccc, 0xdddddd, 0xeeeeee, 0xffffff,
        ]);
    }
}
