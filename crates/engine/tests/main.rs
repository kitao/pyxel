use pyxel::Pyxel;

pub fn main() {
    let mut pyxel = Pyxel::init(200, 150, Some("Hello"), None);

    pyxel.cls(3);
    pyxel.pset(10, 20, 8);
    pyxel.run();
}
