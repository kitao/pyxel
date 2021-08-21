import pyxel_debug as pyxel

pyxel.init(300, 300, "hoge")

pyxel.title("hoge")
# pyxel.fullscreen()


def update():
    if pyxel.text_input:
        print(pyxel.text_input)
    pass
    # print(pyxel.frame_count)


def draw():
    pyxel.cls(3)


print(pyxel.palette[3])
pyxel.palette[3] = 5
print(pyxel.palette[3])


# pyxel.run(update, draw)
