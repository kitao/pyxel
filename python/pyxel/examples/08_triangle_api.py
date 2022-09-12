import pyxel


class App:
    def __init__(self):
        pyxel.init(200, 150, title="Pyxel Triangle API")
        self.triangles = [(100, 24, 7, 143, 193, 143, 7)]
        pyxel.cls(13)
        pyxel.text(6, 6, "tri(x1,y1,x2,y2,x3,y3,col)", 7)
        pyxel.text(6, 14, "trib(x1,y1,x2,y2,x3,y3,col)", 7)
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

    def draw(self):
        if self.triangles:
            triangle = self.triangles.pop(0)
            self.draw_triangle(*triangle)

    def draw_triangle(self, x1, y1, x2, y2, x3, y3, n):
        if n == 0:
            return
        col = n + 7
        if n % 2 == 0:
            pyxel.tri(x1, y1, x2, y2, x3, y3, col)
        else:
            pyxel.trib(x1, y1, x2, y2, x3, y3, col)
        h1 = (x1 + x2) / 2
        w1 = (y1 + y2) / 2
        h2 = (x2 + x3) / 2
        w2 = (y2 + y3) / 2
        h3 = (x3 + x1) / 2
        w3 = (y3 + y1) / 2
        self.triangles.append((x1, y1, h1, w1, h3, w3, n - 1))
        self.triangles.append((h1, w1, x2, y2, h2, w2, n - 1))
        self.triangles.append((h3, w3, h2, w2, x3, y3, n - 1))


App()
