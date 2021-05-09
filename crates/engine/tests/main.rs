use pyxel::*;

pub struct App {
    x: i32,
    y: i32,
}

impl App {
    fn new() -> App {
        App { x: 0, y: 0 }
    }
}

impl PyxelCallback for App {
    fn update(&mut self, pyxel: &mut Pyxel) {
        self.x += 1;
        self.y -= 1;
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        pyxel.cls(3);
        pyxel.pset(10, 20, 8);
    }
}

pub fn main() {
    let mut pyxel = Pyxel::init(200, 150, Some("Hello"), None, None);
    let mut app = App::new();

    pyxel.run(&mut app);
}
