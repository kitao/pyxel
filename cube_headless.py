import pyxel
from pyxel.cube import Camera, Mat4, Node, Scene, Vec3

W, H = 40, 30
LOG = open("/tmp/cube_check.log", "w")


def dump(label: str) -> None:
    LOG.write(f"=== {label} ===\n")
    for y in range(H):
        row = ""
        for x in range(W):
            c = pyxel.screen.pget(x, y)
            row += f"{c:x}" if c < 16 else "?"
        LOG.write(row + "\n")
    LOG.write("\n")
    LOG.flush()


pyxel.init(W, H, title="cube headless")

scene = Scene()
camera = Camera()


# Per-case helper: every check pattern is a `scene.clear_color + Node` pair
# that draws into `scene` then dumps the resulting screen as ASCII hex.
# `Node.__new__` does not accept extra positional arguments, so the draw
# callback is set after construction rather than passed through __init__.
class Probe(Node):
    def __init__(self):
        super().__init__()
        self._draw_fn = None

    def set_draw(self, draw_fn) -> None:
        self._draw_fn = draw_fn

    def on_draw(self):
        if self._draw_fn is not None:
            self._draw_fn(self)


def run_case(label: str, draw_fn) -> None:
    # Replace any existing probe so test cases stay isolated.
    for child in list(scene.children):
        scene.remove_child(child)
    scene.clear_color = 0
    probe = Probe()
    probe.set_draw(draw_fn)
    scene.add_child(probe)
    scene.draw(0, 0, W, H, camera)
    pyxel.flip()
    dump(label)


# --- case 1: a single unshaded front-facing triangle, identity camera ---
# Unshaded so the test focuses on geometry / projection rather than the
# scene-wide shading direction.
camera.transform = Mat4.look_at(Vec3(0, 0, 4), Vec3.ZERO, Vec3.UP)
run_case(
    "triangle (color 8) at z=0, camera at +Z=4 looking at origin",
    lambda node: node.tri(
        Vec3(-1, -1, 0), Vec3(1, -1, 0), Vec3(0, 1, 0), 8, shaded=False
    ),
)


# --- case 2: same triangle, but moved further away ---
run_case(
    "triangle (color 11) at z=-2 (further from camera)",
    lambda node: node.tri(
        Vec3(-1, -1, -2),
        Vec3(1, -1, -2),
        Vec3(0, 1, -2),
        11,
        shaded=False,
    ),
)


# --- case 3: rect on Mat4.IDENTITY, identity camera at +Z=4 ---
run_case(
    "rect 2x2 on IDENTITY (face normal +Z), color 12",
    lambda node: node.rect(Mat4.IDENTITY, 2.0, 2.0, 12, shaded=False),
)


# --- case 4: crosshair lines at origin ---
def _crosshair(node):
    node.line(Vec3(-1, 0, 0), Vec3(1, 0, 0), 7)
    node.line(Vec3(0, -1, 0), Vec3(0, 1, 0), 7)


run_case("crosshair lines at origin", _crosshair)


# --- case 5: screen-space text anchored at origin ---
run_case(
    'text "X" at origin (Vec3 anchor, screen-space glyph)',
    lambda node: node.text(Vec3.ZERO, "X", 7),
)


# --- case 6: cube with shaded faces ---
# Scene seeds a default Shading from the current Pyxel palette at
# construction; box is rendered with shaded=True (default) to exercise
# the directional shading LUT.
camera.transform = Mat4.look_at(Vec3(3, 2, 4), Vec3.ZERO, Vec3.UP)


def _box(node):
    # Filled box of size (2, 2, 2), shaded by the scene-wide Shading.
    node.box(Mat4.IDENTITY, Vec3(2, 2, 2), 7)


run_case("filled box (size 2) viewed from (3,2,4)", _box)


# --- case 7: per-call alpha (dither pattern) ---
run_case(
    "rect with dither_alpha=0.5 (50% Bayer dither)",
    lambda node: node.rect(
        Mat4.IDENTITY,
        2.0,
        2.0,
        9,
        shaded=False,
        dither_alpha=0.5,
    ),
)


# --- case 8: per-call depth_test off (always on top) ---
def _two_rects(node):
    # Far rect drawn first (color 11, occupies full 2x2 area).
    node.rect(
        Mat4.from_translation(Vec3(0, 0, -1)),
        2.0,
        2.0,
        11,
        shaded=False,
    )
    # Closer rect with depth_test off — overrides regardless of z.
    node.rect(Mat4.IDENTITY, 1.0, 1.0, 14, shaded=False, depth_test=False)


run_case("rect overdraw with depth_test=False", _two_rects)


# --- case 9: BILLBOARD_ON makes a tilted plane face the camera ---
# Without billboard, a 45°-tilted rect appears as a diamond; with
# BILLBOARD_ON, the rotation is overridden to face the camera so the
# rect renders as a full square.
camera.transform = Mat4.look_at(Vec3(0, 0, 4), Vec3.ZERO, Vec3.UP)
run_case(
    "rect with billboard=BILLBOARD_ON (faces camera)",
    lambda node: node.rect(
        Mat4.from_euler(Vec3(45, 30, 0)),
        2.0,
        2.0,
        13,
        shaded=False,
        billboard=Node.BILLBOARD_ON,
    ),
)


pyxel.quit()
