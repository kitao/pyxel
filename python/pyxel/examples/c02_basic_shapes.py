import pyxel
from pyxel.cube import Camera, Mat4, Node, Shading, Vec3

cat_image = None


class Label(Node):
    caption = ""

    def on_draw(self):
        self.text(Vec3(0, 0.9, 0), self.caption, 7)


class Shape(Node):
    solid_method = ""
    wire_method = ""

    def __init__(self, pos):
        super().__init__()
        self.pos = pos
        self.label = Label()
        self.add_child(self.label)

    def on_update(self):
        spin = Mat4.from_euler(Vec3(0, pyxel.frame_count * 2.0, 0))
        self.transform = Mat4.from_translation(self.pos) * spin

    def on_draw(self):
        if self.wire_method and pyxel.btn(pyxel.KEY_SPACE):
            self.label.caption = self.wire_method
            self.draw_wire()
        else:
            self.label.caption = self.solid_method
            self.draw_solid()


class PsetShape(Shape):
    solid_method = "pset"

    def draw_solid(self):
        self.pset(Vec3.ZERO, 8)


class LineShape(Shape):
    solid_method = "line"

    def draw_solid(self):
        self.line(Vec3(-0.5, 0, 0), Vec3(0.5, 0, 0), 14)


class TriShape(Shape):
    solid_method, wire_method = "tri", "trib"

    def draw_solid(self):
        self.tri(Vec3(0, 0.5, 0), Vec3(-0.5, -0.35, 0), Vec3(0.5, -0.35, 0), 9)

    def draw_wire(self):
        self.trib(Vec3(0, 0.5, 0), Vec3(-0.5, -0.35, 0), Vec3(0.5, -0.35, 0), 9)


class RectShape(Shape):
    solid_method, wire_method = "rect", "rectb"

    def draw_solid(self):
        self.rect(Mat4.IDENTITY, 1.0, 0.7, 10)

    def draw_wire(self):
        self.rectb(Mat4.IDENTITY, 1.0, 0.7, 10)


class ElliShape(Shape):
    solid_method, wire_method = "elli", "ellib"

    def draw_solid(self):
        self.elli(Mat4.IDENTITY, 1.0, 0.7, 11)

    def draw_wire(self):
        self.ellib(Mat4.IDENTITY, 1.0, 0.7, 11)


class CircShape(Shape):
    solid_method, wire_method = "circ", "circb"

    def draw_solid(self):
        self.circ(Vec3.ZERO, 0.5, 3)

    def draw_wire(self):
        self.circb(Vec3.ZERO, 0.5, 3)


class SpriteShape(Shape):
    solid_method = "sprite"

    def draw_solid(self):
        uvs = ((0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0))
        self.sprite(Vec3.ZERO, cat_image, uvs, 1.0, 1.0)


class BoxShape(Shape):
    solid_method, wire_method = "box", "boxb"

    def draw_solid(self):
        self.box(Mat4.IDENTITY, Vec3(0.85, 0.85, 0.85), cat_image)

    def draw_wire(self):
        self.boxb(Mat4.IDENTITY, Vec3(0.85, 0.85, 0.85), 12)


class SphereShape(Shape):
    solid_method, wire_method = "sphere", "sphereb"

    def draw_solid(self):
        self.sphere(Vec3.ZERO, 0.5, cat_image)

    def draw_wire(self):
        self.sphereb(Vec3.ZERO, 0.5, 2)


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

        global cat_image
        cat_image = pyxel.Image.from_image("assets/cat_16x16.png")

        self.scene = Scene()

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, pyxel.width, pyxel.height)


App()
