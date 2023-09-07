use crate::prelude::*;

pub(crate) struct Graphics {}

impl Graphics {
    pub fn new() -> Self {
        Self {}
    }
}

impl Pyxel {
    pub fn image_no(&self, image: SharedImage) -> Option<u32> {
        for (i, builtin_image) in self.images.iter().enumerate() {
            if builtin_image.data_ptr() == image.data_ptr() {
                return Some(i as u32);
            }
        }
        None
    }

    pub fn clip(&self, x: f64, y: f64, width: f64, height: f64) {
        self.screen.lock().clip(x, y, width, height);
    }

    pub fn clip0(&self) {
        self.screen.lock().clip0();
    }

    pub fn camera(&self, x: f64, y: f64) {
        self.screen.lock().camera(x, y);
    }

    pub fn camera0(&self) {
        self.screen.lock().camera0();
    }

    pub fn pal(&self, src_color: Color, dst_color: Color) {
        self.screen.lock().pal(src_color, dst_color);
    }

    pub fn pal0(&self) {
        self.screen.lock().pal0();
    }

    pub fn cls(&self, color: Color) {
        self.screen.lock().cls(color);
    }

    pub fn pget(&self, x: f64, y: f64) -> Color {
        self.screen.lock().pget(x, y)
    }

    pub fn pset(&self, x: f64, y: f64, color: Color) {
        self.screen.lock().pset(x, y, color);
    }

    pub fn line(&self, x1: f64, y1: f64, x2: f64, y2: f64, color: Color) {
        self.screen.lock().line(x1, y1, x2, y2, color);
    }

    pub fn rect(&self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.screen.lock().rect(x, y, width, height, color);
    }

    pub fn rectb(&self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.screen.lock().rectb(x, y, width, height, color);
    }

    pub fn circ(&self, x: f64, y: f64, radius: f64, color: Color) {
        self.screen.lock().circ(x, y, radius, color);
    }

    pub fn circb(&self, x: f64, y: f64, radius: f64, color: Color) {
        self.screen.lock().circb(x, y, radius, color);
    }

    pub fn elli(&self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.screen.lock().elli(x, y, width, height, color);
    }

    pub fn ellib(&self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.screen.lock().ellib(x, y, width, height, color);
    }

    pub fn tri(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, color: Color) {
        self.screen.lock().tri(x1, y1, x2, y2, x3, y3, color);
    }

    pub fn trib(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, color: Color) {
        self.screen.lock().trib(x1, y1, x2, y2, x3, y3, color);
    }

    pub fn fill(&self, x: f64, y: f64, color: Color) {
        self.screen.lock().fill(x, y, color);
    }

    pub fn blt(
        &self,
        x: f64,
        y: f64,
        image_no: u32,
        image_x: f64,
        image_y: f64,
        width: f64,
        height: f64,
        color_key: Option<Color>,
    ) {
        self.screen.lock().blt(
            x,
            y,
            self.images[image_no as usize].clone(),
            image_x,
            image_y,
            width,
            height,
            color_key,
        );
    }

    pub fn bltm(
        &self,
        x: f64,
        y: f64,
        tilemap_no: u32,
        tilemap_x: f64,
        tilemap_y: f64,
        width: f64,
        height: f64,
        color_key: Option<Color>,
    ) {
        self.screen.lock().bltm(
            x,
            y,
            self.tilemaps[tilemap_no as usize].clone(),
            tilemap_x,
            tilemap_y,
            width,
            height,
            color_key,
        );
    }

    pub fn text(&self, x: f64, y: f64, string: &str, color: Color) {
        self.screen
            .lock()
            .text(x, y, string, color, self.font.clone());
    }
}
