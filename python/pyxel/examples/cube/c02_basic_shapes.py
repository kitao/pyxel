import pyxel
from pyxel.cube import Camera, Mat4, Node, Shading, Vec3

GRID_SPACING = 2.2
GRID_OFFSET_Y = 0.25  # lower grid so text-inclusive content stays centered
SHAPE_SCALE = 1.1
BOX_SCALE = SHAPE_SCALE * 0.85  # solid cube reads larger; shrink to match
LABEL_OFFSET_Y = 0.9
ROT_SPEED = 2.0  # deg/frame
CAM_DIST = 6.0
CAM_YAW_LIMIT = 45.0
CAM_PITCH_LIMIT = 30.0
MOUSE_SENS = 0.5
SPRITE_UVS = ((0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0))


class Label(Node):
    def __init__(self, name: str):
        super().__init__()
        self.name = name

    def on_draw(self):
        self.text(Vec3(0, LABEL_OFFSET_Y, 0), self.name, 7)


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
        spin = Mat4.from_euler(Vec3(0, pyxel.frame_count * ROT_SPEED, 0))
        self.transform = Mat4.from_translation(self.pos) * spin


class PsetShape(Shape):
    solid_name = "pset"

    def on_draw(self):
        self.pset(Vec3.ZERO, 11)


class LineShape(Shape):
    solid_name = "line"

    def on_draw(self):
        half = SHAPE_SCALE / 2
        self.line(Vec3(-half, 0, 0), Vec3(half, 0, 0), 7)


class TriShape(Shape):
    solid_name, wire_name = "tri", "trib"

    def on_draw(self):
        s = SHAPE_SCALE / 2
        p1, p2, p3 = Vec3(0, s, 0), Vec3(-s, -s * 0.7, 0), Vec3(s, -s * 0.7, 0)
        if pyxel.btn(pyxel.KEY_SPACE):
            self.label.name = self.wire_name
            self.trib(p1, p2, p3, 7)
        else:
            self.label.name = self.solid_name
            self.tri(p1, p2, p3, 8)


class RectShape(Shape):
    solid_name, wire_name = "rect", "rectb"

    def on_draw(self):
        w, h = SHAPE_SCALE, SHAPE_SCALE * 0.7
        if pyxel.btn(pyxel.KEY_SPACE):
            self.label.name = self.wire_name
            self.rectb(Mat4.IDENTITY, w, h, 7)
        else:
            self.label.name = self.solid_name
            self.rect(Mat4.IDENTITY, w, h, 9)


class ElliShape(Shape):
    solid_name, wire_name = "elli", "ellib"

    def on_draw(self):
        w, h = SHAPE_SCALE, SHAPE_SCALE * 0.7
        if pyxel.btn(pyxel.KEY_SPACE):
            self.label.name = self.wire_name
            self.ellib(Mat4.IDENTITY, w, h, 7)
        else:
            self.label.name = self.solid_name
            self.elli(Mat4.IDENTITY, w, h, 10)


class CircShape(Shape):
    solid_name, wire_name = "circ", "circb"

    def on_draw(self):
        r = SHAPE_SCALE / 2
        if pyxel.btn(pyxel.KEY_SPACE):
            self.label.name = self.wire_name
            self.circb(Vec3.ZERO, r, 7)
        else:
            self.label.name = self.solid_name
            self.circ(Vec3.ZERO, r, 11)


class SpriteShape(Shape):
    solid_name = "sprite"

    def on_draw(self):
        self.sprite(Vec3.ZERO, pyxel.images[0], SPRITE_UVS, SHAPE_SCALE, SHAPE_SCALE)


class BoxShape(Shape):
    solid_name, wire_name = "box", "boxb"

    def on_draw(self):
        size = Vec3(BOX_SCALE, BOX_SCALE, BOX_SCALE)
        if pyxel.btn(pyxel.KEY_SPACE):
            self.label.name = self.wire_name
            self.boxb(Mat4.IDENTITY, size, 7)
        else:
            self.label.name = self.solid_name
            self.box(Mat4.IDENTITY, size, pyxel.images[0])


class SphereShape(Shape):
    solid_name, wire_name = "sphere", "sphereb"

    def on_draw(self):
        r = SHAPE_SCALE / 2
        if pyxel.btn(pyxel.KEY_SPACE):
            self.label.name = self.wire_name
            self.sphereb(Vec3.ZERO, r, 7)
        else:
            self.label.name = self.solid_name
            self.sphere(Vec3.ZERO, r, pyxel.images[0])


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
        self.refresh_camera()

        # Lay shapes left-to-right, top-to-bottom in definition order.
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
            gx, gy = i % 3 - 1, 1 - i // 3
            pos = Vec3(gx * GRID_SPACING, gy * GRID_SPACING - GRID_OFFSET_Y, 0)
            self.add_child(shape_cls(pos))

    def refresh_camera(self):
        eye = Vec3(
            CAM_DIST * pyxel.sin(self.yaw) * pyxel.cos(self.pitch),
            CAM_DIST * pyxel.sin(self.pitch),
            CAM_DIST * pyxel.cos(self.yaw) * pyxel.cos(self.pitch),
        )
        self.camera.transform = Mat4.look_at(eye, Vec3.ZERO)

    def on_update(self):
        mx, my = pyxel.mouse_x, pyxel.mouse_y
        if self.mouse_prev is not None:
            px, py = self.mouse_prev
            dx = mx - px
            dy = my - py
            self.yaw = max(
                -CAM_YAW_LIMIT, min(CAM_YAW_LIMIT, self.yaw - dx * MOUSE_SENS)
            )
            self.pitch = max(
                -CAM_PITCH_LIMIT, min(CAM_PITCH_LIMIT, self.pitch + dy * MOUSE_SENS)
            )
            self.refresh_camera()
        self.mouse_prev = (mx, my)


class App:
    def __init__(self):
        pyxel.init(160, 160, title="Cube Basic Shapes")
        self.populate_texture()
        self.scene = Scene()
        pyxel.run(self.update, self.draw)

    def populate_texture(self):
        # 4-color quadrant pattern: makes rotation of box/sphere visible.
        img = pyxel.images[0]
        half_w = img.width // 2
        half_h = img.height // 2
        img.rect(0, 0, half_w, half_h, 8)
        img.rect(half_w, 0, half_w, half_h, 11)
        img.rect(0, half_h, half_w, half_h, 14)
        img.rect(half_w, half_h, half_w, half_h, 7)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, pyxel.width, pyxel.height)


App()
