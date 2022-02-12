import pyxel

pyxel.init(64, 64, title="Perlin Noise")
while True:
    pyxel.cls(0)
    for y in range(64):
        for x in range(64):
            n = pyxel.noise(x / 10, y / 10, pyxel.frame_count / 40)
            if n > 0.4:
                col = 7
            elif n > 0:
                col = 6
            elif n > -0.4:
                col = 12
            else:
                col = 0
            pyxel.pset(x, y, col)
    pyxel.flip()
