import pyxel
from pyxel.cube import Mat4, Node, Scene, Shading, Vec3

CUBE_COLORS = [8, 9, 10, 11, 12, 14]
CUBE_COUNT = len(CUBE_COLORS)


class Cube(Node):
    def __init__(self, index):
        super().__init__()
        self.color = CUBE_COLORS[index]
        self.phase = index * (360.0 / CUBE_COUNT)

    def on_update(self):
        t = pyxel.frame_count
        orbit = self.phase + t * 1.4
        bob = pyxel.sin(self.phase + t * 4.0) * 0.3
        pos = Vec3(
            pyxel.cos(orbit) * 2.0,
            bob,
            pyxel.sin(orbit) * 2.0,
        )
        spin = Mat4.from_euler(Vec3(t, t * 2.0, 0.0))
        self.transform = Mat4.from_translation(pos) * spin

    def on_draw(self):
        self.box(Mat4.IDENTITY, Vec3(0.6, 0.6, 0.6), self.color)


class MainScene(Scene):
    def __init__(self):
        super().__init__()

        self.shading = Shading(pyxel.colors)
        self.shading.direction = Vec3(-0.5, -1.0, -0.7).normalize()
        self.clear_color = 0
        self.camera.transform = Mat4.look_at(Vec3(0.0, 2.5, 5.0), Vec3.ZERO)

        for i in range(CUBE_COUNT):
            self.add_child(Cube(i))

    def on_draw(self):
        self.text(Vec3(0.0, 0.0, 0.0), "Hello, Pyxel Cube!", pyxel.frame_count % 16)


class App:
    def __init__(self):
        pyxel.init(160, 120, title="Hello Pyxel Cube")
        self.scene = MainScene()
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, pyxel.width, pyxel.height)


App()
