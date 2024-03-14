"""
Copyright (c) Cookie Yang. All right reserved.
"""
import pyxel

pyxel.init(160, 120)

def update():
    if pyxel.btnp(pyxel.KEY_R):
        pyxel.quit()

def draw():
    pyxel.cls(0)
    pyxel.rect(10, 10, 20, 20, 12)

pyxel.run(update, draw)