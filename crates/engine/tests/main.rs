extern crate pyxel;

pub fn main() {
    pyxel::init_system("test", 100, 200);
    pyxel::system().run();
}
