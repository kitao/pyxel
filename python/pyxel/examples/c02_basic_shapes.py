import pyxel
from pyxel.cube import Camera, Mat4, Node, Shading, Vec3

cat_image = None


class Shape(Node):
    solid_method = wire_method = ""

    def __init__(self, position):
        super().__init__()
        self.position = position

    def on_update(self):
        spin = Mat4.from_euler(Vec3(0, pyxel.frame_count * 2.0, 0))
        self.transform = Mat4.from_translation(self.position) * spin

    def on_draw(self):
        if self.wire_method and pyxel.btn(pyxel.KEY_SPACE):
            caption = self.wire_method
            self.draw_wire()
        else:
            caption = self.solid_method
            self.draw_solid()

        self.depth_offset(-1.0)
        self.text(Vec3(0, 0.7, 0), caption, 7)


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


class CircShape(Shape):
    solid_method, wire_method = "circ", "circb"

    def draw_solid(self):
        self.circ(Vec3.ZERO, 0.5, 3)

    def draw_wire(self):
        self.circb(Vec3.ZERO, 0.5, 3)


class ElliShape(Shape):
    solid_method, wire_method = "elli", "ellib"

    def draw_solid(self):
        self.elli(Mat4.IDENTITY, 1.0, 0.7, 11)

    def draw_wire(self):
        self.ellib(Mat4.IDENTITY, 1.0, 0.7, 11)


class PlaneShape(Shape):
    solid_method = "plane"

    def draw_solid(self):
        uvs = ((0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0))
        self.plane(Mat4.IDENTITY, cat_image, uvs, 1.0, 1.0)


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
        self.prev_mouse_pos = None
        self.update_camera()

        shapes = [
            PsetShape,
            LineShape,
            TriShape,
            RectShape,
            CircShape,
            ElliShape,
            PlaneShape,
            SpriteShape,
            BoxShape,
            SphereShape,
        ]

        for shape_index, shape_class in enumerate(shapes):
            angle = shape_index * 360.0 / len(shapes)
            position = Vec3(2.5 * pyxel.sin(angle), 2.5 * pyxel.cos(angle), 0)
            self.add_child(shape_class(position))

    def update_camera(self):
        target = Vec3(0, 0.2, 0)
        eye = target + Vec3(
            6.0 * pyxel.sin(self.yaw) * pyxel.cos(self.pitch),
            6.0 * pyxel.sin(self.pitch),
            6.0 * pyxel.cos(self.yaw) * pyxel.cos(self.pitch),
        )
        self.camera.transform = Mat4.look_at(eye, target)

    def on_update(self):
        mouse_x, mouse_y = pyxel.mouse_x, pyxel.mouse_y
        if self.prev_mouse_pos is not None:
            prev_x, prev_y = self.prev_mouse_pos
            self.yaw = max(-45.0, min(45.0, self.yaw - (mouse_x - prev_x) * 0.5))
            self.pitch = max(-30.0, min(30.0, self.pitch + (mouse_y - prev_y) * 0.5))
            self.update_camera()

        self.prev_mouse_pos = (mouse_x, mouse_y)


class App:
    def __init__(self):
        pyxel.init(240, 240, title="Basic Shapes")

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

        hud_x, hud_y = pyxel.width // 2 - 35, pyxel.height // 2 - 10

        pyxel.rect(hud_x, hud_y, 70, 20, 0)
        pyxel.text(hud_x + 3, hud_y + 3, "Mouse: Rotate", 7)
        pyxel.text(hud_x + 3, hud_y + 11, "Space: Wireframe", 7)


App()
