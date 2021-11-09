use pyxel::{Pyxel, PyxelCallback};

pub struct App {
    x: f64,
    y: f64,
}

impl App {
    fn new(pyxel: &mut Pyxel) -> App {
        let app = App { x: 0.0, y: 0.0 };

        pyxel.mouse(true);
        pyxel.set_btnv(pyxel::MOUSE_POS_X, 0);
        pyxel.set_btnv(pyxel::MOUSE_POS_Y, 0);

        pyxel.image(0).lock().set(
            0,
            0,
            &[
                "00011000", "00010100", "00010010", "00010010", "00010100", "00010000", "01110000",
                "01100000",
            ],
        );

        pyxel.sound(0).lock().set(
            "e2e2c2g1 g1g1c2e2 d2d2d2g2 g2g2rr c2c2a1e1 e1e1a1c2 b1b1b1e2 e2e2rr",
            "p",
            "6",
            "vffn fnff vffs vfnn",
            25,
        );

        pyxel.sound(1).lock().set(
            "r a1b1c2 b1b1c2d2 g2g2g2g2 c2c2d2e2 f2f2f2e2 f2e2d2c2 d2d2d2d2 g2g2r r ",
            "s",
            "6",
            "nnff vfff vvvv vfff svff vfff vvvv svnn",
            25,
        );

        pyxel.sound(2).lock().set(
            "c1g1c1g1 c1g1c1g1 b0g1b0g1 b0g1b0g1 a0e1a0e1 a0e1a0e1 g0d1g0d1 g0d1g0d1",
            "t",
            "7",
            "n",
            25,
        );

        pyxel.sound(3).lock().set(
            "f0c1f0c1 g0d1g0d1 c1g1c1g1 a0e1a0e1 f0c1f0c1 f0c1f0c1 g0d1g0d1 g0d1g0d1",
            "t",
            "7",
            "n",
            25,
        );

        pyxel.sound(4).lock().set(
            "f0ra4r f0ra4r f0ra4r f0f0a4r",
            "n",
            "6622 6622 6622 6422",
            "f",
            25,
        );

        pyxel.play(0, &[0, 1], true);
        pyxel.play(1, &[2, 3], true);
        pyxel.play(2, &[4], true);

        app
    }
}

impl PyxelCallback for App {
    fn update(&mut self, pyxel: &mut Pyxel) {
        if pyxel.frame_count() < 60 {
            self.x += (pyxel.frame_count() % 2) as f64;
            self.y -= 1.0;
        }

        if pyxel.btnp(pyxel::KEY_Q, None, None) {
            pyxel.quit();
        }
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        pyxel.cls(3);
        pyxel.pset(self.x, 20.0, 7);
        pyxel.rect(self.x + 10.0, 25.0, 15.0, 10.0, 8);
        pyxel.rectb(self.x + 15.0, 45.0, 15.0, 10.0, pyxel::COLOR_WHITE);

        pyxel.blt(0.0, 0.0, 0, 0.0, 0.0, 8.0, 8.0, None);
    }
}

pub fn main() {
    let mut pyxel = Pyxel::new(200, 150, Some("Hello, Pyxel in Rust!"), None, None, None);
    let mut app = App::new(&mut pyxel);

    pyxel.run(&mut app);
}
