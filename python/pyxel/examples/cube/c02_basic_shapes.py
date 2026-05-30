import pyxel
from pyxel.cube import Camera, Mat4, Node, Shading, Vec3

# A specimen-book style display of cube draw primitives, arranged in a
# 3x3 grid with the center cell empty. Each specimen spins around its
# local Y axis. Holding SPACE overlays the wireframe variant on the six
# pair primitives (tri/trib, rect/rectb, elli/ellib, circ/circb,
# box/boxb, sphere/sphereb). The mouse drag orbits the camera around
# the grid center.

GRID_SPACING = 1.5
SPEC_SCALE = 0.7
LABEL_OFFSET_Y = 0.9
ROT_SPEED = 2.0  # deg/frame
CAM_DIST = 6.0
CAM_YAW_LIMIT = 45.0
CAM_PITCH_LIMIT = 30.0
MOUSE_SENS = 0.5


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


class Cell(Node):
    def __init__(self, pos: Vec3, label: str, spinner: Spinner):
        super().__init__()
        self.transform = Mat4.from_translation(pos)
        self._label = label
        self.add_child(spinner)

    def on_draw(self):
        self.text(Vec3(0, LABEL_OFFSET_Y, 0), self._label, 7)


CELL_SPECS = [
    # (grid_x, grid_y, label, spinner class) — center (0, 0) is empty.
    (-1, +1, "pset", PsetSpinner),
    (0, +1, "line", LineSpinner),
    (+1, +1, "tri", TriSpinner),
    (-1, 0, "rect", RectSpinner),
    (+1, 0, "elli", ElliSpinner),
    (-1, -1, "circ", CircSpinner),
    (0, -1, "box", BoxSpinner),
    (+1, -1, "sphere", SphereSpinner),
]


class App:
    def __init__(self):
        pyxel.init(192, 192, title="Cube Basic Shapes")
        pyxel.mouse(True)

        self._populate_texture()

        self.root = Node()
        self.root.shading = Shading(pyxel.colors)
        self.root.shading.direction = Vec3(0.5, -1.5, -1.0).normalize()

        for gx, gy, label, spinner_cls in CELL_SPECS:
            pos = Vec3(gx * GRID_SPACING, gy * GRID_SPACING, 0)
            self.root.add_child(Cell(pos, label, spinner_cls()))

        self.camera = Camera()
        self.cam_yaw = 0.0
        self.cam_pitch = 0.0
        self.mouse_prev_x = pyxel.mouse_x
        self.mouse_prev_y = pyxel.mouse_y
        self._update_camera_transform()

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

    def _update_camera_transform(self):
        eye = Vec3(
            CAM_DIST * pyxel.sin(self.cam_yaw) * pyxel.cos(self.cam_pitch),
            CAM_DIST * pyxel.sin(self.cam_pitch),
            CAM_DIST * pyxel.cos(self.cam_yaw) * pyxel.cos(self.cam_pitch),
        )
        self.camera.transform = Mat4.look_at(eye, Vec3.ZERO)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        dx = pyxel.mouse_x - self.mouse_prev_x
        dy = pyxel.mouse_y - self.mouse_prev_y
        self.cam_yaw = max(
            -CAM_YAW_LIMIT, min(CAM_YAW_LIMIT, self.cam_yaw - dx * MOUSE_SENS)
        )
        self.cam_pitch = max(
            -CAM_PITCH_LIMIT,
            min(CAM_PITCH_LIMIT, self.cam_pitch - dy * MOUSE_SENS),
        )
        self._update_camera_transform()
        self.mouse_prev_x = pyxel.mouse_x
        self.mouse_prev_y = pyxel.mouse_y

        self.root.update()

    def draw(self):
        self.root.draw(0, 0, pyxel.width, pyxel.height, self.camera, clear_color=1)


App()
