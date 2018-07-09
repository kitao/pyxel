import pyxel


def update():
    if pyxel.btnp(pyxel.KEY_Q):
        pyxel.quit()


def draw():
    pyxel.cls(0)
    pyxel.text(40, 60, 'Hello, Pyxel!', pyxel.frame_count % 16)


pyxel.init(128, 128, caption='Hello Pyxel')
pyxel.run(update, draw)
