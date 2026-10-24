import pytest
from pyxel import Image
from pyxel.cube import Camera, Collider, Mat4, Mesh, Node, Primitive, Vec3


def pixels(node):
    camera = Camera()
    camera.transform = Mat4.look_at(Vec3(0, 0, 6), Vec3.ZERO)
    camera.ortho_size = 8
    node.camera = camera
    target = Image(32, 32)
    node.draw(0, 0, 32, 32, target)
    return [target.pget(x, y) for y in range(32) for x in range(32)]


def test_opacity_default_and_clamping():
    node = Node()
    assert node.opacity == 1
    node.opacity = -1
    assert node.opacity == 0
    node.opacity = 2
    assert node.opacity == 1
    node.opacity = 0.25
    assert node.opacity == 0.25


@pytest.mark.parametrize(
    "command",
    [
        "pset",
        "line",
        "tri",
        "trib",
        "rect",
        "rectb",
        "circ",
        "circb",
        "elli",
        "ellib",
        "box",
        "boxb",
        "sphere",
        "sphereb",
        "plane",
        "sprite",
        "prim",
        "text",
    ],
)
def test_zero_opacity_hides_each_draw_family(command):
    texture = Image(2, 2)
    texture.cls(8)
    uvs = ((0, 0), (1, 0), (1, 1), (0, 1))
    triangle = (Vec3(-1, -1, 0), Vec3(1, -1, 0), Vec3(0, 1, 0))
    arguments = {
        "pset": (Vec3.ZERO, 8),
        "line": (Vec3.LEFT, Vec3.RIGHT, 8),
        "tri": (*triangle, 8),
        "trib": (*triangle, 8),
        "rect": (Mat4.IDENTITY, 2, 2, 8),
        "rectb": (Mat4.IDENTITY, 2, 2, 8),
        "circ": (Vec3.ZERO, 1, 8),
        "circb": (Vec3.ZERO, 1, 8),
        "elli": (Mat4.IDENTITY, 2, 2, 8),
        "ellib": (Mat4.IDENTITY, 2, 2, 8),
        "box": (Mat4.IDENTITY, Vec3(2, 2, 2), 8),
        "boxb": (Mat4.IDENTITY, Vec3(2, 2, 2), 8),
        "sphere": (Vec3.ZERO, 1, 8),
        "sphereb": (Vec3.ZERO, 1, 8),
        "plane": (Mat4.IDENTITY, texture, uvs, 2, 2),
        "sprite": (Vec3.ZERO, texture, uvs, 2, 2),
        "prim": (Mat4.IDENTITY, Primitive.plane(2, 2), 8),
        "text": (Vec3.ZERO, "X", 8),
    }

    class Shape(Node):
        def on_draw(self):
            getattr(self, command)(*arguments[command])

    shape = Shape()
    assert 8 in pixels(shape)

    shape.opacity = 0
    assert set(pixels(shape)) == {0}


def test_ancestor_opacity_and_dither_multiply_once_per_command():
    class Shape(Node):
        def __init__(self, alpha):
            self.alpha = alpha

        def on_draw(self):
            self.dither(self.alpha)
            self.rect(Mat4.from_translation(Vec3(-2, 0, 0)), 2, 2, 8)
            self.rect(Mat4.from_translation(Vec3(2, 0, 0)), 2, 2, 8)

    parent = Node()
    parent.opacity = 0.5
    shape = Shape(0.5)
    shape.opacity = 0.5
    parent.add_child(shape)
    # Direct subtree drawing must still include the outside parent's opacity.
    actual = pixels(shape)
    expected = pixels(Shape(0.125))
    assert 8 in expected
    assert actual == expected

    shape.alpha = 1
    assert pixels(shape) == pixels(Shape(0.25))


def test_instance_opacity_does_not_change_shared_mesh_or_sibling():
    mesh = Mesh([Primitive.plane(2, 2)], [Mat4.IDENTITY], [-1], col_img=8)
    root = Node()
    a, b = Node.from_mesh(mesh), Node.from_mesh(mesh)
    a.transform = Mat4.from_translation(Vec3(-2, 0, 0))
    b.transform = Mat4.from_translation(Vec3(2, 0, 0))
    root.add_child(a)
    root.add_child(b)
    baseline = pixels(root)

    a.opacity = 0
    faded = pixels(root)
    assert any(baseline[y * 32 + x] for y in range(32) for x in range(16))
    assert all(faded[y * 32 + x] == 0 for y in range(32) for x in range(16))
    assert [faded[y * 32 + x] for y in range(32) for x in range(16, 32)] == [
        baseline[y * 32 + x] for y in range(32) for x in range(16, 32)
    ]

    a.opacity = 1
    assert pixels(root) == baseline


def test_zero_opacity_keeps_update_collision_and_draw_callbacks():
    events = []

    class Probe(Node):
        def on_update(self):
            events.append("update")

        def on_collide(self, other, contact):
            events.append("collide")

        def on_draw(self):
            events.append("draw")
            self.rect(Mat4.IDENTITY, 2, 2, 8)

    root = Node()
    probe, other = Probe(), Node()
    probe.collider = Collider(radius=1, trigger=True, mass=0)
    other.collider = Collider(radius=1, mass=0)
    root.add_child(probe)
    root.add_child(other)
    probe.opacity = 0
    root.update()
    assert set(pixels(root)) == {0}
    assert events == ["update", "collide", "draw"]
