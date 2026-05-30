import pyxel
from pyxel.cube import Camera, Mat4, Node, Shading, Vec3

# A specimen-book style display of cube draw primitives, arranged in a
# 3x3 grid. Each specimen spins around its local Y axis. Holding SPACE
# overlays the wireframe variant on the six pair primitives (tri/trib,
# rect/rectb, elli/ellib, circ/circb, box/boxb, sphere/sphereb). The
# mouse cursor movement orbits the scene camera around the grid center.

GRID_SPACING = 1.5
SPEC_SCALE = 0.7
LABEL_OFFSET_Y = 0.9
ROT_SPEED = 2.0  # deg/frame
CAM_DIST = 6.0
CAM_YAW_LIMIT = 45.0
CAM_PITCH_LIMIT = 30.0
MOUSE_SENS = 0.5
SPRITE_UVS = ((0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0))


class Spinner(Node):
    def on_update(self):
        self.transform = Mat4.from_euler(Vec3(0, pyxel.frame_count * ROT_SPEED, 0))


class PsetSpinner(Spinner):
    def on_draw(self):
        self.pset(Vec3.ZERO, 11)


class LineSpinner(Spinner):
    def on_draw(self):
        half = SPEC_SCALE / 2
        self.line(Vec3(-half, 0, 0), Vec3(half, 0, 0), 7)


class TriSpinner(Spinner):
    def on_draw(self):
        s = SPEC_SCALE / 2
        p1, p2, p3 = Vec3(0, s, 0), Vec3(-s, -s * 0.7, 0), Vec3(s, -s * 0.7, 0)
        self.tri(p1, p2, p3, 8)
        if pyxel.btn(pyxel.KEY_SPACE):
            self.trib(p1, p2, p3, 7)


class RectSpinner(Spinner):
    def on_draw(self):
        w, h = SPEC_SCALE, SPEC_SCALE * 0.7
        self.rect(Mat4.IDENTITY, w, h, 9)
        if pyxel.btn(pyxel.KEY_SPACE):
            self.rectb(Mat4.IDENTITY, w, h, 7)


class ElliSpinner(Spinner):
    def on_draw(self):
        w, h = SPEC_SCALE, SPEC_SCALE * 0.7
        self.elli(Mat4.IDENTITY, w, h, 10)
        if pyxel.btn(pyxel.KEY_SPACE):
            self.ellib(Mat4.IDENTITY, w, h, 7)


class CircSpinner(Spinner):
    def on_draw(self):
        r = SPEC_SCALE / 2
        self.circ(Vec3.ZERO, r, 11)
        if pyxel.btn(pyxel.KEY_SPACE):
            self.circb(Vec3.ZERO, r, 7)


class BoxSpinner(Spinner):
    def on_draw(self):
        size = Vec3(SPEC_SCALE, SPEC_SCALE, SPEC_SCALE)
        self.box(Mat4.IDENTITY, size, pyxel.images[0])
        if pyxel.btn(pyxel.KEY_SPACE):
            self.boxb(Mat4.IDENTITY, size, 7)


class SphereSpinner(Spinner):
    def on_draw(self):
        r = SPEC_SCALE / 2
        self.sphere(Vec3.ZERO, r, pyxel.images[0])
        if pyxel.btn(pyxel.KEY_SPACE):
            self.sphereb(Vec3.ZERO, r, 7)


class SpriteSpinner(Spinner):
    def on_draw(self):
        self.sprite(Vec3.ZERO, pyxel.images[0], SPRITE_UVS, SPEC_SCALE, SPEC_SCALE)


class Cell(Node):
    def __init__(self, pos: Vec3, label: str, spinner: Spinner):
        super().__init__()
        self.transform = Mat4.from_translation(pos)
        self._label = label
        self.add_child(spinner)

    def on_draw(self):
        self.text(Vec3(0, LABEL_OFFSET_Y, 0), self._label, 7)


CELL_SPECS = [
    # (grid_x, grid_y, label, spinner class)
    (-1, +1, "pset", PsetSpinner),
    (0, +1, "line", LineSpinner),
    (+1, +1, "tri", TriSpinner),
    (-1, 0, "rect", RectSpinner),
    (0, 0, "sprite", SpriteSpinner),
    (+1, 0, "elli", ElliSpinner),
    (-1, -1, "circ", CircSpinner),
    (0, -1, "box", BoxSpinner),
    (+1, -1, "sphere", SphereSpinner),
]


class Scene(Node):
    def __init__(self):
        super().__init__()
        self.camera = Camera()
        self.shading = Shading(pyxel.colors)
        self.shading.direction = Vec3(0.5, -1.5, -1.0).normalize()
        self._yaw = 0.0
        self._pitch = 0.0
        self._mouse_prev_x = pyxel.mouse_x
        self._mouse_prev_y = pyxel.mouse_y
        self._refresh_camera()

    def _refresh_camera(self):
        eye = Vec3(
            CAM_DIST * pyxel.sin(self._yaw) * pyxel.cos(self._pitch),
            CAM_DIST * pyxel.sin(self._pitch),
            CAM_DIST * pyxel.cos(self._yaw) * pyxel.cos(self._pitch),
        )
        self.camera.transform = Mat4.look_at(eye, Vec3.ZERO)

    def on_update(self):
        dx = pyxel.mouse_x - self._mouse_prev_x
        dy = pyxel.mouse_y - self._mouse_prev_y
        self._yaw = max(-CAM_YAW_LIMIT, min(CAM_YAW_LIMIT, self._yaw - dx * MOUSE_SENS))
        self._pitch = max(
            -CAM_PITCH_LIMIT, min(CAM_PITCH_LIMIT, self._pitch + dy * MOUSE_SENS)
        )
        self._refresh_camera()
        self._mouse_prev_x = pyxel.mouse_x
        self._mouse_prev_y = pyxel.mouse_y


class App:
    def __init__(self):
        pyxel.init(192, 192, title="Cube Basic Shapes")
        self._populate_texture()

        self.scene = Scene()
        for gx, gy, label, spinner_cls in CELL_SPECS:
            pos = Vec3(gx * GRID_SPACING, gy * GRID_SPACING, 0)
            self.scene.add_child(Cell(pos, label, spinner_cls()))

        pyxel.run(self.update, self.draw)

    def _populate_texture(self):
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
        self.scene.draw(0, 0, pyxel.width, pyxel.height, self.scene.camera, clear_color=1)


App()
