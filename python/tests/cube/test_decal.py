import math

import pytest
from pyxel import Image
from pyxel.cube import Camera, Mat4, Node, Shading, Vec3

UVS = ((0, 0), (1, 0), (0, 1), (1, 1))


class Drawing(Node):
    def __init__(self, draw):
        super().__init__()
        self.draw_commands = draw

    def on_draw(self):
        self.draw_commands(self)


def render(*nodes, perspective=False, viewport=(0, 0, 64, 64), target=None):
    root = Node()
    root.camera = Camera()
    root.camera.transform = Mat4.from_translation(Vec3(0, 0, 10))
    if not perspective:
        root.camera.ortho_size = 8

    for node in nodes:
        root.add_child(node)

    target = target if target is not None else Image(64, 64)
    root.draw(*viewport, target=target)
    return target


def translated(x=0, y=0, z=0):
    return Mat4.from_translation(Vec3(x, y, z))


@pytest.mark.parametrize("perspective", [False, True])
def test_decal_paints_only_existing_surfaces_inside_volume(perspective):
    def draw(node):
        # All surfaces have no collider. Near and far receivers both take paint;
        # a surface behind the volume and one in front of it keep their colors.
        node.rect(translated(-2.4, z=0.2), 1.2, 3, 3)
        node.rect(translated(-0.8, z=-1.5), 1.2, 3, 4)
        node.rect(translated(0.8, z=-4), 1.2, 3, 5)
        node.rect(translated(2.4, z=2), 1.2, 3, 6)

    before = render(Drawing(draw), perspective=perspective)

    def project(node):
        node.decal(4)
        node.rect(translated(z=1), 8, 8, 8)

    after = render(Drawing(draw), Drawing(project), perspective=perspective)
    counts = {color: 0 for color in (0, 3, 4, 5, 6)}
    for y in range(64):
        for x in range(64):
            color = before.pget(x, y)
            counts[color] += 1
            assert after.pget(x, y) == (8 if color in (3, 4) else color)

    assert all(counts.values())


@pytest.mark.parametrize("shape", ["rect", "elli", "plane"])
def test_decal_uses_rotated_parent_frame_and_clips_at_receiver_edge(shape):
    def receiver(node):
        node.rect(Mat4.IDENTITY, 4, 4, 3)

    def project(node):
        node.decal(3)
        if shape == "plane":
            image = Image(2, 2)
            image.cls(8)
            node.plane(Mat4.IDENTITY, image, UVS, 2, 2)
        else:
            getattr(node, shape)(Mat4.IDENTITY, 2, 2, 8)

    parent = Node()
    parent.transform = translated(-2, z=0.5) * Mat4.from_euler(Vec3(0, -90, 0))
    parent.add_child(Drawing(project))
    result = render(Drawing(receiver), parent)
    # The projector points along +X, through the vertical receiver. Its source
    # rectangle is edge-on to the camera and would normally cover no area.
    assert result.pget(20, 32) == 8
    assert result.pget(36, 32) == 8
    assert result.pget(44, 32) == 3
    assert result.pget(10, 32) == 0
    assert result.pget(24, 18) == 3


@pytest.mark.parametrize("reset", ["omitted", "none"])
def test_decal_zero_keeps_mode_and_reset_restores_normal_drawing(reset):
    def draw(node):
        node.decal(0)
        node.rect(translated(-2), 2, 2, 8)
        if reset == "omitted":
            node.decal()
        else:
            node.decal(None)
        node.rect(translated(2), 2, 2, 7)

    result = render(Drawing(draw))
    assert result.pget(16, 32) == 0
    assert result.pget(48, 32) == 7


def test_decal_state_resets_for_children_and_siblings():
    parent = Drawing(lambda node: node.decal(0))
    parent.add_child(Drawing(lambda node: node.rect(translated(-2), 2, 2, 7)))
    sibling = Drawing(lambda node: node.rect(translated(2), 2, 2, 8))
    result = render(parent, sibling)
    assert result.pget(16, 32) == 7
    assert result.pget(48, 32) == 8


@pytest.mark.parametrize("front", [False, True])
def test_decal_preserves_receiver_depth_and_ignores_depth_overrides(front):
    def draw(node):
        node.rect(Mat4.IDENTITY, 4, 4, 3)

        node.decal(4)
        node.depth_test(False)
        node.depth_write(True)
        node.depth_offset(-100)
        node.rect(translated(z=2), 4, 4, 8)

        node.decal()
        node.depth_test(True)
        node.depth_offset(0)
        node.rect(translated(z=1 if front else -1), 2, 2, 7)

    result = render(Drawing(draw))
    assert result.pget(32, 32) == (7 if front else 8)


def test_decal_before_receiver_has_no_effect():
    def draw(node):
        node.decal(4)
        node.elli(translated(z=1), 4, 4, 8)
        node.decal()
        node.rect(Mat4.IDENTITY, 4, 4, 3)

    assert render(Drawing(draw)).pget(32, 32) == 3


@pytest.mark.parametrize("mode", [None, 0, 4])
def test_decal_leaves_linework_and_solids_unchanged(mode):
    def draw(node):
        node.decal(mode)
        node.rectb(translated(-2, 2), 2, 2, 7)
        node.ellib(translated(2, 2), 2, 2, 8)
        node.box(translated(-2, -2), Vec3(2, 2, 2), 9)
        node.tri(Vec3(1, -3, 0), Vec3(3, -3, 0), Vec3(2, -1, 0), 10)

    result = render(Drawing(draw))

    # Exact comparison ensures decal neither suppresses nor projects these calls.
    # Capture normal rendering by clearing the mode immediately after its setter.
    class Ordinary(Drawing):
        def decal(self, distance=None):
            super().decal()

    expected = render(Ordinary(draw))
    assert bytes(result.data_ptr()) == bytes(expected.data_ptr())
    assert result.pget(16, 48) == 9


@pytest.mark.parametrize("perspective", [False, True])
def test_decal_texture_uvs_colkey_and_shading_match_plane(perspective):
    image = Image(4, 4)
    for y in range(4):
        for x in range(4):
            image.pset(x, y, y * 4 + x)

    # Non-rectangular UVs exercise both triangles, not just bilinear interpolation.
    uvs = ((0, 0), (1, 0.25), (0.25, 1), (0.75, 0.75))

    shading = Shading([0] * 16)
    for color in range(16):
        for level in range(Shading.LEVEL_COUNT):
            shading[color, level] = ((color + 1) % 16, (color + 2) % 16)

    def ordinary(node):
        node.rect(Mat4.IDENTITY, 6, 6, 3)
        node.depth_test(False)
        node.plane(Mat4.IDENTITY, image, uvs, 4, 4, colkey=5)

    def projected(node):
        node.rect(Mat4.IDENTITY, 6, 6, 3)
        node.decal(2)
        node.plane(translated(z=1), image, uvs, 4, 4, colkey=5)

    a, b = Drawing(ordinary), Drawing(projected)
    a.shading = b.shading = shading
    expected = render(a, perspective=perspective)
    result = render(b, perspective=perspective)
    assert bytes(result.data_ptr()) == bytes(expected.data_ptr())


def test_decal_dither_masks_paint_without_erasing_receiver():
    def draw(node):
        node.rect(Mat4.IDENTITY, 4, 4, 3)
        node.decal(2)
        node.dither(0.5)
        node.rect(translated(z=1), 4, 4, 8)

    image = render(Drawing(draw))
    colors = [image.pget(x, y) for y in range(24, 40) for x in range(24, 40)]
    assert colors.count(3) == colors.count(8) == 128


@pytest.mark.parametrize("distance", [math.nan, math.inf, -math.inf])
def test_nonfinite_decal_distance_draws_nothing(distance):
    def draw(node):
        node.rect(Mat4.IDENTITY, 4, 4, 3)
        node.decal(distance)
        node.rect(translated(z=1), 4, 4, 8)

    assert render(Drawing(draw)).pget(32, 32) == 3


def test_negative_decal_distance_reverses_projection():
    def draw(node):
        node.rect(Mat4.IDENTITY, 4, 4, 3)
        node.decal(-2)
        node.rect(translated(z=-1), 4, 4, 8)

    assert render(Drawing(draw)).pget(32, 32) == 8


def test_decal_viewport_offset_clipping_and_eye_plane_crossing():
    def draw(node):
        node.rect(Mat4.IDENTITY, 100, 100, 3)
        node.decal(20)
        node.rect(translated(z=11), 100, 100, 8)

    result = render(Drawing(draw), perspective=True, viewport=(-5, 7, 30, 20))
    for y in range(64):
        for x in range(64):
            assert result.pget(x, y) == (8 if x < 25 and 7 <= y < 27 else 0)


def test_decal_can_sample_its_render_target():
    target = Image(64, 64)

    def draw(node):
        node.rect(translated(-2), 4, 8, 3)
        node.rect(translated(2), 4, 8, 8)
        node.decal(2)
        node.plane(translated(z=1), target, ((1, 0), (0, 0), (1, 1), (0, 1)), 8, 8)

    result = render(Drawing(draw), target=target)
    assert result.pget(16, 32) == 8
    assert result.pget(48, 32) == 3


@pytest.mark.parametrize("shape", ["rect", "elli"])
def test_decal_keeps_the_original_shape_footprint(shape):
    def draw(node, projection):
        node.rect(Mat4.IDENTITY, 8, 8, 3)
        if projection:
            node.decal(2)
        else:
            node.depth_test(False)
        getattr(node, shape)(translated(z=1 if projection else 0), 6.5, 4.5, 8)

    expected = render(Drawing(lambda node: draw(node, False)))
    result = render(Drawing(lambda node: draw(node, True)))
    assert bytes(result.data_ptr()) == bytes(expected.data_ptr())


@pytest.mark.parametrize("scale, expected", [(0, 3), (1, 3), (2, 8)])
def test_decal_distance_follows_matrix_scale(scale, expected):
    def receiver(node):
        node.rect(Mat4.IDENTITY, 4, 4, 3)

    def project(node):
        node.decal(1)
        node.rect(Mat4.from_scale(Vec3.ONE * scale), 4, 4, 8)

    parent = Node()
    parent.transform = translated(z=1.5)
    parent.add_child(Drawing(project))
    assert render(Drawing(receiver), parent).pget(32, 32) == expected
