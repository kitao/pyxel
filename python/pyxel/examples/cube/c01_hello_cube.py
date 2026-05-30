import pyxel
from pyxel.cube import Camera, Mat4, Node, Shading, Vec3

CUBE_COLORS = [8, 9, 10, 11, 12, 14]
CUBE_COUNT = len(CUBE_COLORS)


class Cube(Node):
    def __init__(self, index):
        super().__init__()
        self.color = CUBE_COLORS[index]
        self.phase = index * (360.0 / CUBE_COUNT)

    def on_update(self):
        t = pyxel.frame_count
        orbit = self.phase + t * 2.0
        pos = Vec3(
            pyxel.cos(orbit) * 2.0,
            pyxel.sin(self.phase + t * 4.0) * 0.5 + 0.4,
            pyxel.sin(orbit) * 2.0,
        )
        spin = Mat4.from_euler(Vec3(t * 3.0, t * 5.0, 0.0))
        self.transform = Mat4.from_translation(pos) * spin

    def on_draw(self):
        self.box(Mat4.IDENTITY, Vec3(0.6, 0.6, 0.6), self.color)


class Label(Node):
    def on_draw(self):
        self.depth_test(False)
        self.text(Vec3.ZERO, "Hello, Pyxel Cube!", pyxel.frame_count % 16)


class Scene(Node):
    def __init__(self):
        super().__init__()

        self.shading = Shading(pyxel.colors)
        self.shading.direction = Vec3(0.5, -1.5, -1.0).normalize()

        self.camera = Camera()
        self.camera.transform = Mat4.look_at(Vec3(0.0, 3.5, 4.0), Vec3.ZERO)

        for i in range(CUBE_COUNT):
            self.add_child(Cube(i))
        self.add_child(Label())


class App:
    def __init__(self):
        pyxel.init(160, 120, title="Hello Pyxel Cube")
        self.scene = Scene()
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, pyxel.width, pyxel.height, clear_color=0)


App()
