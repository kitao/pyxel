#[inline]
pub fn pyxel() -> std::cell::RefMut<'static, pyxel::Pyxel> {
    pyxel::pyxel()
}
