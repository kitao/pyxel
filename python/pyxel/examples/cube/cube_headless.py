import pyxel
from pyxel.cube import Camera, Mat4, Node, Shading, Vec3

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

scene = Node()
scene.shading = Shading([pyxel.colors[i] for i in range(16)])
camera = Camera()
camera.clear_color = 0
scene.camera = camera


# Per-case helper: every check pattern is a fresh Probe node that draws
# into `scene` then dumps the resulting screen as ASCII hex.
class Probe(Node):
    def __init__(self, draw_fn):
        super().__init__()
        self._draw_fn = draw_fn

    def on_draw(self):
        self._draw_fn(self)


def run_case(label: str, draw_fn) -> None:
    # Replace any existing probe so test cases stay isolated.
    for child in list(scene.children):
        scene.remove_child(child)
    scene.add_child(Probe(draw_fn))
    scene.draw(0, 0, W, H)
    pyxel.flip()
    dump(label)


# Case 1: a single unshaded front-facing triangle, identity camera
# Unshaded so the test focuses on geometry / projection rather than the
# scene-wide shading direction.
def _tri_near(node):
    node.shaded(False)
    node.tri(Vec3(-1, -1, 0), Vec3(1, -1, 0), Vec3(0, 1, 0), 8)


camera.transform = Mat4.look_at(Vec3(0, 0, 4), Vec3.ZERO, Vec3.UP)
run_case("triangle (color 8) at z=0, camera at +Z=4 looking at origin", _tri_near)


# Case 2: same triangle, but moved further away
def _tri_far(node):
    node.shaded(False)
    node.tri(Vec3(-1, -1, -2), Vec3(1, -1, -2), Vec3(0, 1, -2), 11)


run_case("triangle (color 11) at z=-2 (further from camera)", _tri_far)


# Case 3: rect on Mat4.IDENTITY, identity camera at +Z=4
def _rect_identity(node):
    node.shaded(False)
    node.rect(Mat4.IDENTITY, 2.0, 2.0, 12)


run_case("rect 2x2 on IDENTITY (face normal +Z), color 12", _rect_identity)


# Case 4: crosshair lines at origin
def _crosshair(node):
    node.line(Vec3(-1, 0, 0), Vec3(1, 0, 0), 7)
    node.line(Vec3(0, -1, 0), Vec3(0, 1, 0), 7)


run_case("crosshair lines at origin", _crosshair)


# Case 5: screen-space text anchored at origin
run_case(
    'text "X" at origin (Vec3 anchor, screen-space glyph)',
    lambda node: node.text(Vec3.ZERO, "X", 7),
)


# Case 6: cube with shaded faces
# The scene root carries the Shading seeded at the top of this script;
# box is drawn with shading on (the default state) to exercise the
# directional shading LUT.
camera.transform = Mat4.look_at(Vec3(3, 2, 4), Vec3.ZERO, Vec3.UP)


def _box(node):
    # Filled box of size (2, 2, 2), shaded by the scene-wide Shading.
    node.box(Mat4.IDENTITY, Vec3(2, 2, 2), 7)


run_case("filled box (size 2) viewed from (3,2,4)", _box)


# Case 7: dither state (50% alpha pattern)
def _rect_dithered(node):
    node.shaded(False)
    node.dither(0.5)
    node.rect(Mat4.IDENTITY, 2.0, 2.0, 9)


run_case("rect with dither(0.5) (50% Bayer dither)", _rect_dithered)


# Case 8: depth_test state off (always on top)
def _two_rects(node):
    node.shaded(False)
    # Far rect drawn first (color 11, occupies full 2x2 area).
    node.rect(Mat4.from_translation(Vec3(0, 0, -1)), 2.0, 2.0, 11)
    # Closer rect with depth test off — overrides regardless of z.
    node.depth_test(False)
    node.rect(Mat4.IDENTITY, 1.0, 1.0, 14)


run_case("rect overdraw with depth_test(False)", _two_rects)


# Case 9: camera-facing primitive under an oblique camera
# Billboarding is baked into the camera-facing primitives (circ, circb,
# sprite, text) rather than toggled per call. Viewed from an oblique
# angle, a billboarded circle still rasterizes as a round disc instead
# of a foreshortened ellipse.
camera.transform = Mat4.look_at(Vec3(3, 2, 4), Vec3.ZERO, Vec3.UP)


def _circ_billboard(node):
    node.shaded(False)
    node.circ(Vec3.ZERO, 1.0, 13)


run_case("circ r=1 from oblique camera (billboards to a disc)", _circ_billboard)


pyxel.quit()
