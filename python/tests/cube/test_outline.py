import pytest
import pyxel
from pyxel.cube import Camera, Mat4, Node, Vec3


def make_scene(width=48, height=40, camera_z=80):
    scene = Node()
    scene.camera = Camera()
    scene.camera.transform = Mat4.look_at(Vec3(0, 0, camera_z), Vec3.ZERO)
    scene.camera.clear_color = 1
    scene.camera.ortho_size = height
    return scene, pyxel.Image(width, height)


def draw(scene, target):
    scene.draw(0, 0, target.width, target.height, target)
    return bytes(target.data_ptr())


def test_line_and_solid_share_one_outer_contour():
    class Shapes(Node):
        def on_draw(self):
            self.shaded(False)
            self.line(Vec3(-18, -7, 0), Vec3(18, 7, 0), 8)
            self.box(Mat4.IDENTITY, Vec3(10, 10, 3), 8)

    scene, target = make_scene()
    shapes = Shapes()
    scene.add_child(shapes)
    normal = draw(scene, target)
    shapes.outline = 0
    pixels = draw(scene, target)
    body = {i for i, color in enumerate(normal) if color == 8}
    width = target.width
    border = {j for i in body for j in (i - 1, i + 1, i - width, i + width)} - body
    assert pixels == bytes(
        0 if i in border else color for i, color in enumerate(normal)
    )


def test_dither_fades_the_shape_and_its_contour_together():
    class Shape(Node):
        def on_draw(self):
            self.shaded(False)
            self.rect(Mat4.IDENTITY, 16, 12, 8)

    scene, target = make_scene()
    shape = Shape()
    shape.outline = 7
    scene.add_child(shape)
    full = draw(scene, target)
    shape.opacity = 0.5
    faded = draw(scene, target)
    assert faded == bytes(
        color if ((i % target.width) + i // target.width) % 2 else 1
        for i, color in enumerate(full)
    )


class Layer(Node):
    def __init__(self, size, z, color):
        super().__init__()
        self.transform = Mat4.from_translation(Vec3(0, 0, z))
        self.size = size
        self.color = color
        self.test_depth = True
        self.write_depth = True

    def on_draw(self):
        self.shaded(False)
        self.depth_test(self.test_depth)
        self.depth_write(self.write_depth)
        self.rect(Mat4.IDENTITY, self.size, self.size, self.color)


@pytest.mark.parametrize("front_first", [False, True])
def test_front_outline_preserves_the_view_through_dither_holes(front_first):
    scene, target = make_scene()
    back = Layer(8, 0, 8)
    back.outline = 7
    front = Layer(24, 3, 4)
    front.opacity = 0.5
    for node in (front, back) if front_first else (back, front):
        scene.add_child(node)
    original = draw(scene, target)
    assert 7 in original
    front.outline = 10
    outlined = draw(scene, target)
    assert 10 in outlined
    assert all(
        a == b or (a == 1 and b == 10) for a, b in zip(original, outlined, strict=True)
    )


@pytest.mark.parametrize("disable", ["test", "write"])
def test_outline_follows_the_body_when_depth_is_disabled(disable):
    scene, target = make_scene()
    large = Layer(24, 3, 4)
    small = Layer(8, 4, 8)
    small.outline = 7
    scene.add_child(large)
    scene.add_child(small)
    expected = draw(scene, target)
    assert expected.count(7) == 32
    small.transform = Mat4.IDENTITY
    assert 7 not in draw(scene, target)
    if disable == "test":
        small.test_depth = False
    else:
        large.write_depth = False
    assert draw(scene, target) == expected


def test_an_occluded_dither_layer_preserves_the_visible_silhouettes():
    scene, target = make_scene()
    back = Layer(8, 0, 8)
    back.outline = 7
    middle = Layer(24, 2, 4)
    front = Layer(24, 3, 4)
    for layer in (middle, front):
        layer.opacity = 0.5
        layer.outline = 10
    for layer in (back, middle, front):
        scene.add_child(layer)
    middle.visible = False
    expected = draw(scene, target)
    assert expected.count(7) == 16
    middle.visible = True
    assert draw(scene, target) == expected


def test_depth_writes_survive_a_color_only_overlay():
    scene, target = make_scene()
    wall = Layer(24, 3, 4)
    overlay = Layer(8, 5, 8)
    overlay.outline = 7
    overlay.write_depth = False
    scene.add_child(wall)
    scene.add_child(overlay)
    expected = draw(scene, target)
    assert expected.count(7) == 32
    scene.add_child(Layer(24, 0, 6))
    assert draw(scene, target) == expected


def test_outline_respects_target_dither_when_the_canvas_suppresses_writes():
    scene, target = make_scene()
    shape = Layer(12, 0, 8)
    shape.outline = 7
    scene.add_child(shape)
    target.cls(3)
    target.dither(float("nan"))
    assert draw(scene, target) == bytes([3]) * (target.width * target.height)


def test_outline_at_image_edge_can_come_from_an_offscreen_point():
    class Point(Node):
        def on_draw(self):
            self.pset(Vec3(-24.5, 0, 0), 8)

    scene, target = make_scene()
    shape = Point()
    shape.outline = 7
    scene.add_child(shape)
    pixels = draw(scene, target)
    assert [
        (i % target.width, i // target.width)
        for i, color in enumerate(pixels)
        if color == 7
    ] == [(0, 20)]
    assert 8 not in pixels


def test_external_receiver_inherits_the_style_at_draw_start():
    scene, target = make_scene()
    holder = Node()
    holder.outline = 7
    receiver = Node()
    holder.add_child(receiver)

    class Relay(Node):
        def on_draw(self):
            holder.outline = 10
            receiver.rect(Mat4.IDENTITY, 10, 8, 8)

    scene.add_child(Relay())
    assert draw(scene, target).count(7) == 36
    assert draw(scene, target).count(10) == 36


def test_clipped_coplanar_faces_keep_their_colors_with_an_outline():
    class Faces(Node):
        def on_draw(self):
            self.shaded(False)
            self.tri(Vec3(-23, -14, 1), Vec3(8, 15, 1), Vec3(4, -25, 1), 3)
            self.tri(Vec3(-50, -15, 1), Vec3(-22, 44, 1), Vec3(-7, -41, 1), 9)

    scene, target = make_scene(32, 24, 70)
    faces = Faces()
    scene.add_child(faces)
    normal = draw(scene, target)
    faces.outline = 7
    outlined = draw(scene, target)
    assert 7 in outlined
    assert all(
        pixel == original or (original == 1 and pixel == 7)
        for original, pixel in zip(normal, outlined, strict=True)
    )


def test_an_unrelated_outline_preserves_clipped_face_and_wire_depth():
    class Face(Node):
        def on_draw(self):
            self.shaded(False)
            a, b, c = Vec3(-12, -13, 23), Vec3(20, 4, 39), Vec3(-34, 15, -10)
            self.tri(a, b, c, 3)
            self.trib(a, b, c, 9)

    scene, target = make_scene(32, 24, 70)
    scene.add_child(Face())
    normal = draw(scene, target)
    other = Node()
    other.outline = 7
    assert draw(scene, target) == normal


class FlatShape(Node):
    def __init__(self, command, points):
        super().__init__()
        self.command = command
        self.points = [Vec3(*point) for point in points]

    def on_draw(self):
        self.shaded(False)
        getattr(self, self.command)(*self.points, 8)


@pytest.mark.parametrize(
    ("command", "points", "border"),
    [
        ("line", ((-26, -22, 0), (-2, 22, 0)), ((24, 35),)),
        ("tri", ((-37, 12, 0), (-3, -22, 0), (-20, -17, 0)), ((24, 38), (27, 41))),
    ],
)
def test_image_clip_retains_outline_samples_from_outside(command, points, border):
    scene, target = make_scene(80, 60, 70)
    shape = FlatShape(command, points)
    shape.outline = 7
    scene.add_child(shape)
    full = draw(scene, target)
    target.clip(24, 18, 32, 24)
    clipped = draw(scene, target)
    for x, y in border:
        assert full[y * target.width + x] == clipped[y * target.width + x] == 7


@pytest.mark.parametrize(
    ("command", "points", "pixel"),
    [
        ("line", ((-6, 22, 0), (17, 12, 0)), (21, 0)),
        ("tri", ((38, -37, 0), (18, 8, 0), (5, -31, 0)), (31, 12)),
    ],
)
def test_outline_at_a_physical_edge_uses_the_original_geometry(command, points, pixel):
    scene, target = make_scene(32, 32, 70)
    shape = FlatShape(command, points)
    scene.add_child(shape)
    x, y = pixel
    assert draw(scene, target)[y * target.width + x] == 1
    shape.outline = 7
    assert draw(scene, target)[y * target.width + x] == 7


@pytest.mark.parametrize(
    ("command", "points", "pixel"),
    [
        ("line", ((0, -10, 0), (-51, 24, 0)), (5, 18)),
        ("tri", ((-43, 30, 0), (-57, 0, 0), (42, 45, 0)), (10, 18)),
    ],
)
def test_clip_touching_image_edge_preserves_other_sides(command, points, pixel):
    scene, target = make_scene(80, 60, 70)
    shape = FlatShape(command, points)
    shape.outline = 7
    scene.add_child(shape)
    target.clip(0, 18, 32, 24)
    x, y = pixel
    assert draw(scene, target)[y * target.width + x] == 1
