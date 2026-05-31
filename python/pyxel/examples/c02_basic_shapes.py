import pyxel
from pyxel.cube import Camera, Mat4, Node, Shading, Vec3

# A standalone texture for the box and sphere. Image banks (pyxel.images)
# are sprite sheets, so a single texture gets its own small Image. Its
# 4-color quadrants make the rotation of the box and sphere visible.
texture = pyxel.Image(16, 16)
texture.rect(0, 0, 8, 8, 8)
texture.rect(8, 0, 8, 8, 11)
texture.rect(0, 8, 8, 8, 14)
texture.rect(8, 8, 8, 8, 7)


class Label(Node):
    def __init__(self, name: str):
        super().__init__()
        self.name = name

    def on_draw(self):
        self.text(Vec3(0, 0.9, 0), self.name, 7)


# Each shape spins around its local Y axis and carries a Label naming the
# draw API it shows. Shapes with a wireframe counterpart toggle solid<->wire
# (exclusively) while SPACE is held, updating the label to the API in use.
class Shape(Node):
    solid_name = ""

    def __init__(self, pos: Vec3):
        super().__init__()
        self.pos = pos
        self.label = Label(self.solid_name)
        self.add_child(self.label)

    def on_update(self):
        spin = Mat4.from_euler(Vec3(0, pyxel.frame_count * 2.0, 0))
        self.transform = Mat4.from_translation(self.pos) * spin


class PsetShape(Shape):
    solid_name = "pset"

    def on_draw(self):
        self.pset(Vec3.ZERO, 11)


class LineShape(Shape):
    solid_name = "line"

    def on_draw(self):
        self.line(Vec3(-0.5, 0, 0), Vec3(0.5, 0, 0), 7)


class TriShape(Shape):
    solid_name, wire_name = "tri", "trib"

    def on_draw(self):
        p1, p2, p3 = Vec3(0, 0.5, 0), Vec3(-0.5, -0.35, 0), Vec3(0.5, -0.35, 0)
        if pyxel.btn(pyxel.KEY_SPACE):
            self.label.name = self.wire_name
            self.trib(p1, p2, p3, 7)
        else:
            self.label.name = self.solid_name
            self.tri(p1, p2, p3, 8)


class RectShape(Shape):
    solid_name, wire_name = "rect", "rectb"

    def on_draw(self):
        if pyxel.btn(pyxel.KEY_SPACE):
            self.label.name = self.wire_name
            self.rectb(Mat4.IDENTITY, 1.0, 0.7, 7)
        else:
            self.label.name = self.solid_name
            self.rect(Mat4.IDENTITY, 1.0, 0.7, 9)


class ElliShape(Shape):
    solid_name, wire_name = "elli", "ellib"

    def on_draw(self):
        if pyxel.btn(pyxel.KEY_SPACE):
            self.label.name = self.wire_name
            self.ellib(Mat4.IDENTITY, 1.0, 0.7, 7)
        else:
            self.label.name = self.solid_name
            self.elli(Mat4.IDENTITY, 1.0, 0.7, 10)


class CircShape(Shape):
    solid_name, wire_name = "circ", "circb"

    def on_draw(self):
        if pyxel.btn(pyxel.KEY_SPACE):
            self.label.name = self.wire_name
            self.circb(Vec3.ZERO, 0.5, 7)
        else:
            self.label.name = self.solid_name
            self.circ(Vec3.ZERO, 0.5, 11)


class SpriteShape(Shape):
    solid_name = "sprite"

    def on_draw(self):
        uvs = ((0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0))
        self.sprite(Vec3.ZERO, texture, uvs, 1.0, 1.0)


class BoxShape(Shape):
    solid_name, wire_name = "box", "boxb"

    def on_draw(self):
        size = Vec3(0.85, 0.85, 0.85)
        if pyxel.btn(pyxel.KEY_SPACE):
            self.label.name = self.wire_name
            self.boxb(Mat4.IDENTITY, size, 7)
        else:
            self.label.name = self.solid_name
            self.box(Mat4.IDENTITY, size, texture)


class SphereShape(Shape):
    solid_name, wire_name = "sphere", "sphereb"

    def on_draw(self):
        if pyxel.btn(pyxel.KEY_SPACE):
            self.label.name = self.wire_name
            self.sphereb(Vec3.ZERO, 0.5, 7)
        else:
            self.label.name = self.solid_name
            self.sphere(Vec3.ZERO, 0.5, texture)


class Scene(Node):
    def __init__(self):
        super().__init__()

        self.shading = Shading(pyxel.colors)
        self.shading.direction = Vec3(0.5, -1.5, -1.0).normalize()

        self.camera = Camera()
        self.camera.clear_color = 1
        self.yaw = 0.0
        self.pitch = 0.0
        self.mouse_prev = None
        self.update_camera()

        # Lay shapes left-to-right, top-to-bottom in definition order on a
        # 3x3 grid (cells 2.2 apart), lowered 0.25 so labels fit above.
        shapes = [
            PsetShape,
            LineShape,
            TriShape,
            RectShape,
            ElliShape,
            CircShape,
            SpriteShape,
            BoxShape,
            SphereShape,
        ]
        for i, shape_cls in enumerate(shapes):
            col, row = i % 3, i // 3
            pos = Vec3((col - 1) * 2.2, (1 - row) * 2.2 - 0.25, 0)
            self.add_child(shape_cls(pos))

    def update_camera(self):
        eye = Vec3(
            6.0 * pyxel.sin(self.yaw) * pyxel.cos(self.pitch),
            6.0 * pyxel.sin(self.pitch),
            6.0 * pyxel.cos(self.yaw) * pyxel.cos(self.pitch),
        )
        self.camera.transform = Mat4.look_at(eye, Vec3.ZERO)

    def on_update(self):
        mx, my = pyxel.mouse_x, pyxel.mouse_y
        if self.mouse_prev is not None:
            px, py = self.mouse_prev
            self.yaw = max(-45.0, min(45.0, self.yaw - (mx - px) * 0.5))
            self.pitch = max(-30.0, min(30.0, self.pitch + (my - py) * 0.5))
            self.update_camera()
        self.mouse_prev = (mx, my)


class App:
    def __init__(self):
        pyxel.init(160, 160, title="Cube Basic Shapes")
        self.scene = Scene()
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, pyxel.width, pyxel.height)


App()
