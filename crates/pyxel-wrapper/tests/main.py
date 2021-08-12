import pyxel_debug as pyxel

pyxel.init(300, 300, "hoge")
print(pyxel.frame_count)

pyxel.title("hoge")
# pyxel.fullscreen()


def update():
    pass
    # print(pyxel.frame_count)


def draw():
    pyxel.cls(3)


pyxel.run(update, draw)
