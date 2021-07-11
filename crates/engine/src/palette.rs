use crate::settings::{COLOR_COUNT, MAX_COLOR_COUNT};

pub type Color = u8;
pub type Rgb24 = u32;

#[derive(Debug)]
pub struct Palette {
    render_colors: [Color; MAX_COLOR_COUNT as usize],
    display_colors: [Rgb24; MAX_COLOR_COUNT as usize],
}

impl Palette {
    pub fn new() -> Palette {
        assert!(MAX_COLOR_COUNT <= Color::MAX as u32 + 1);

        let mut palette = Palette {
            render_colors: [0; MAX_COLOR_COUNT as usize],
            display_colors: [0; MAX_COLOR_COUNT as usize],
        };

        palette.reset_render_colors();

        palette
    }

    pub fn render_color(&self, original_color: Color) -> Color {
        self.render_colors[original_color as usize]
    }

    pub fn set_render_color(&mut self, original_color: Color, render_color: Color) {
        self.render_colors[original_color as usize] = render_color;
    }

    pub fn reset_render_colors(&mut self) {
        for i in 0..MAX_COLOR_COUNT {
            self.render_colors[i as usize] = i as Color;
        }
    }

    pub fn display_color(&self, render_color: Color) -> Rgb24 {
        self.display_colors[render_color as usize]
    }

    pub fn set_display_color(&mut self, render_color: Color, display_color: Rgb24) {
        self.display_colors[render_color as usize] = display_color;
    }

    pub fn set_display_colors(&mut self, display_colors: &[Rgb24]) {
        for (i, &display_color) in display_colors.iter().enumerate() {
            self.set_display_color(i as Color, display_color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let palette = Palette::new();

        for i in 0..MAX_COLOR_COUNT {
            assert_eq!(palette.render_color(i as Color), i as Color);
            assert_eq!(palette.display_color(i as Color), 0);
        }
    }

    #[test]
    fn set_render_color() {
        let mut palette = Palette::new();

        for i in 0..MAX_COLOR_COUNT {
            assert_eq!(palette.render_color(i as Color), i as Color);
        }

        for i in 0..MAX_COLOR_COUNT {
            palette.set_render_color(i as Color, ((i + 1) % MAX_COLOR_COUNT) as Color);
        }

        for i in 0..MAX_COLOR_COUNT {
            assert_eq!(
                palette.render_color(i as Color),
                ((i + 1) % MAX_COLOR_COUNT) as Color
            );
        }
    }

    #[test]
    fn reset_render_colors() {
        let mut palette = Palette::new();

        for i in 0..MAX_COLOR_COUNT {
            palette.set_render_color(i as Color, ((i + 1) % MAX_COLOR_COUNT) as Color);
        }

        palette.reset_render_colors();

        for i in 0..MAX_COLOR_COUNT {
            assert_eq!(palette.render_color(i as Color), i as Color);
        }
    }

    #[test]
    fn set_display_color() {
        let mut palette = Palette::new();

        for i in 0..MAX_COLOR_COUNT {
            assert_eq!(palette.display_color(i as Color), 0);
        }

        for i in 0..MAX_COLOR_COUNT {
            palette.set_display_color(i as Color, (0x111111 * i) as Rgb24);
        }

        for i in 0..MAX_COLOR_COUNT {
            assert_eq!(palette.display_color(i as Color), (0x111111 * i) as Rgb24);
        }
    }

    #[test]
    fn set_display_colors() {
        let mut palette = Palette::new();

        let rgbs: [Rgb24; COLOR_COUNT as usize] = [
            0x000000, 0x111111, 0x222222, 0x333333, 0x444444, 0x555555, 0x666666, 0x777777,
            0x888888, 0x999999, 0xaaaaaa, 0xbbbbbb, 0xcccccc, 0xdddddd, 0xeeeeee, 0xffffff,
        ];

        palette.set_display_colors(&rgbs);

        for i in 0..16 {
            assert_eq!(palette.display_color(i as Color), rgbs[i]);
        }
    }
}
