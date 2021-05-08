pub fn main() {
    pyxel::init(200, 150, Some("hello"), None);
    pyxel::cls(3);
    pyxel::pset(10, 20, 8);
    pyxel::run();
}
