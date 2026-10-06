from itertools import pairwise

import pytest
import pyxel
from _assertions import raises_exact  # type: ignore[reportMissingImports]
from pyxel.cube import Camera, Mat4, Node, Primitive, Shading, Vec3


class TestDefault:
    def test_construction(self):
        s = Shading(palette())
        assert repr(s) == "Shading(16 x 8)"

    def test_level_constants_and_last_cell(self):
        assert Shading.LEVEL_COUNT == 8
        assert Shading.BASE_LEVEL == 5
        s = Shading(palette())
        s[15, Shading.LEVEL_COUNT - 1] = (6, 7)
        assert s[15, Shading.LEVEL_COUNT - 1] == (6, 7)

    def test_direction_default(self):
        s = Shading(palette())
        assert s.direction == Vec3(0.5, -1, -0.8)


class TestIndexing:
    def test_get_set(self):
        s = Shading(palette())
        s[0, 0] = (5, 7)
        assert s[0, 0] == (5, 7)

    def test_out_of_range_col(self):
        s = Shading(palette())
        with raises_exact(IndexError, "Shading index out of range"):
            _ = s[100, 0]
        with raises_exact(IndexError, "Shading index out of range"):
            s[100, 0] = (5, 7)

    def test_out_of_range_level(self):
        s = Shading(palette())
        with raises_exact(IndexError, "Shading index out of range"):
            _ = s[0, Shading.LEVEL_COUNT]
        with raises_exact(IndexError, "Shading index out of range"):
            s[0, Shading.LEVEL_COUNT] = (5, 7)

    def test_negative_col_raises(self):
        # The binding key is (usize, usize); a negative int always fails
        # unsigned extraction before the range check.
        s = Shading(palette())
        with raises_exact(OverflowError, "can't convert negative int to unsigned"):
            _ = s[-1, 0]

    def test_negative_level_raises(self):
        s = Shading(palette())
        with raises_exact(OverflowError, "can't convert negative int to unsigned"):
            _ = s[0, -1]


class TestDirectionMutate:
    def test_set_direction(self):
        s = Shading(palette())
        s.direction = Vec3(0.5, -0.5, 0.0)
        assert s.direction.x == 0.5
        assert s.direction.y == -0.5


@pytest.mark.parametrize("source,mapped", [(0, 7), (1, 0), (1, 7)])
@pytest.mark.parametrize("shaded", [False, True])
def test_texture_colkey_uses_source_color(source, mapped, shaded):
    texture = pyxel.Image(1, 1)
    texture.cls(source)
    primitive = Primitive(
        Primitive.MODE_TRIANGLES,
        [-2, -2, -4, 2, -2, -4, 0, 2, -4],
        [0, 1, 2],
        uvs=[0, 0, 1, 0, 0.5, 1],
        cull=Primitive.CULL_NONE,
    )

    class TexturedNode(Node):
        def on_draw(self):
            self.shaded(shaded)
            self.prim(Mat4.IDENTITY, primitive, texture, colkey=0)

    scene = TexturedNode()
    scene.camera = Camera()
    scene.camera.clear_color = 2
    scene.shading = Shading(palette())
    for level in range(Shading.LEVEL_COUNT):
        scene.shading[source, level] = (mapped, mapped)
    target = pyxel.Image(64, 64)

    scene.draw(0, 0, 64, 64, target)
    expected = 2 if source == 0 else mapped if shaded else source
    assert target.pget(32, 32) == expected


def _shaded_triangle_center(normal, light, scale=1, reflection=1):
    sx, sy, sz = (scale, scale, scale) if isinstance(scale, (int, float)) else scale
    sx *= reflection
    primitive = Primitive(
        Primitive.MODE_TRIANGLES,
        [
            -1.7 / sx,
            -1.6 / sy,
            -4.25 / sz,
            1.7 / sx,
            -1.6 / sy,
            -4.25 / sz,
            0,
            1.7 / sy,
            -4.25 / sz,
        ],
        [0, 1, 2],
        cull=Primitive.CULL_NONE,
        **({} if normal is None else {"normals": normal}),
    )
    transform = Mat4.from_scale(Vec3(sx, sy, sz))

    class Surface(Node):
        def on_draw(self):
            self.prim(transform, primitive, 7)

    scene = Surface()
    scene.camera = Camera()
    scene.camera.clear_color = 0
    scene.shading = Shading(palette())
    scene.shading.direction = Vec3(*light)
    # Distinct flat colors isolate shade quantization from the palette's dither.
    for level in range(Shading.LEVEL_COUNT):
        scene.shading[7, level] = (level + 1, level + 1)

    target = pyxel.Image(32, 32)
    scene.draw(0, 0, 32, 32, target)
    return target.pget(16, 16)


@pytest.mark.parametrize("scale", [2, 2e19])
def test_unit_normal_selected_sum_keeps_geometric_shading(scale):
    rotation = Mat4.from_axis_angle(Vec3(1, 1, 1), 60)
    transform = rotation * Mat4.from_scale(Vec3(scale, scale, scale))
    world_points = [(-1.7, -1.6, -4.25), (1.7, -1.6, -4.25), (0, 1.7, -4.25)]
    positions = [
        sum(rotation[row, column] * point[row] for row in range(3)) / scale
        for point in world_points
        for column in range(3)
    ]
    normal = [rotation[2, column] for column in range(3)]
    length = sum(component**2 for component in normal) ** 0.5
    primitive = Primitive(
        Primitive.MODE_TRIANGLES,
        positions,
        [0, 1, 2],
        normals=[component / length for component in normal],
        cull=Primitive.CULL_NONE,
    )

    class Surface(Node):
        def on_draw(self):
            self.prim(transform, primitive, 7)

    scene = Surface()
    scene.camera = Camera()
    scene.camera.clear_color = 0
    scene.shading = Shading(palette())
    # The world-space triangle faces +Z; opposite unit light must select the brightest level.
    scene.shading.direction = Vec3(0, 0, -1)
    for level in range(Shading.LEVEL_COUNT):
        scene.shading[7, level] = (level + 1, level + 1)

    target = pyxel.Image(32, 32)
    scene.draw(0, 0, 32, 32, target)
    assert target.pget(16, 16) == 8


@pytest.mark.parametrize("normal_scale", [1e-30, 1e-20, 1, 1e10, 1e20, 1e30])
@pytest.mark.parametrize("light_scale", [1e-30, 1e-20, 1, 1e10, 1e20, 1e30])
def test_shading_finite_magnitudes(normal_scale, light_scale):
    assert _shaded_triangle_center([0, 0, normal_scale], [0, 0, -light_scale]) == 8


@pytest.mark.parametrize("scale", [1e-30, 1e-20, 1, 1e10, 1e20, 1e30])
@pytest.mark.parametrize(
    "normal,reflection,light,expected",
    [
        (None, 1, [0, 0, -1], 8),
        ([0, 0, 1], 1, [0, 0, -1], 8),
        ([0, 0, 1], -1, [0, 0, -1], 1),
        ([0, 0, 1], -1, [0, 0, 1], 8),
    ],
)
def test_shading_oriented_cofactors(scale, normal, reflection, light, expected):
    # Local positions cancel the transform scale; reflection reverses the supplied normal.
    assert _shaded_triangle_center(normal, light, scale, reflection) == expected


@pytest.mark.parametrize(
    "normal,light,expected",
    [
        ([0, 0, 0], [0, 0, -1], 1),
        ([0, 0, 1], [0, 0, 0], 1),
        ([0, 0, -1], [0, 0, -1], 1),
        ([0, 0, 1], [0, -1, 0], 1),
        ([1, 1, 0], [0, -1, 0], 6),
    ],
)
def test_shading_degenerate_orientation(normal, light, expected):
    assert _shaded_triangle_center(normal, light) == expected


@pytest.mark.parametrize(
    "scale", [(1e30, 1e30, 1e-30), (1e-30, 1e-30, 1e30), (1e30, 1e-30, 1e30)]
)
@pytest.mark.parametrize("axis", [0, 1, 2])
def test_shading_nonuniform_cofactor_range(scale, axis):
    normal = [0, 0, 0]
    light = [0, 0, 0]
    normal[axis] = 1
    light[axis] = -1
    # Every positive diagonal cofactor preserves its selected axis, even when other axes need more range.
    assert _shaded_triangle_center(normal, light, scale) == 8


class TestBuild:
    def test_build_resets_modifications(self):
        pal = palette()
        s = Shading(pal)
        initial = [
            [s[col, level] for level in range(Shading.LEVEL_COUNT)]
            for col in range(len(pal))
        ]

        for col in range(len(pal)):
            for level in range(Shading.LEVEL_COUNT):
                s[col, level] = (99, 99)

        s.build(pal)
        assert [
            [s[col, level] for level in range(Shading.LEVEL_COUNT)]
            for col in range(len(pal))
        ] == initial


GB_PALETTE = [0x000000, 0x555555, 0xAAAAAA, 0xFFFFFF]


# Flat ramps are valid when the palette has no darker or brighter color.
@pytest.mark.parametrize("pal_name", ["pyxel_default", "gb_monochrome"])
class TestRampInvariants:
    def test_ramp_is_monotone(self, pal_name):
        pal = _pal(pal_name)
        s = Shading(pal)
        for col in range(len(pal)):
            cells = [s[col, lv] for lv in range(Shading.LEVEL_COUNT)]
            ls = [_entry_luma(pal, p, q) for p, q in cells]
            assert all(a <= b + 1e-6 for a, b in pairwise(ls)), (pal_name, col, ls)

    def test_shade_levels_below_or_equal_base(self, pal_name):
        pal = _pal(pal_name)
        s = Shading(pal)
        for col in range(len(pal)):
            base = _linear_luma(pal[col])
            for lv in range(Shading.BASE_LEVEL):
                p, q = s[col, lv]
                assert _entry_luma(pal, p, q) < base + 1e-6, (
                    f"{pal_name} col {col} lv{lv} brighter than base"
                )

    def test_highlight_above_or_equal_base(self, pal_name):
        pal = _pal(pal_name)
        s = Shading(pal)
        for col in range(len(pal)):
            base = _linear_luma(pal[col])
            for lv in range(Shading.BASE_LEVEL + 1, Shading.LEVEL_COUNT):
                p, q = s[col, lv]
                assert _entry_luma(pal, p, q) >= base - 1e-6, (
                    f"{pal_name} col {col} lv{lv} darker than base"
                )


def _entry_luma(pal: list[int], primary: int, secondary: int) -> float:
    if primary == secondary:
        return _linear_luma(pal[primary])
    return (_linear_luma(pal[primary]) + _linear_luma(pal[secondary])) / 2


def _linear_luma(rgb24: int) -> float:
    r = _srgb_to_linear(((rgb24 >> 16) & 0xFF) / 255)
    g = _srgb_to_linear(((rgb24 >> 8) & 0xFF) / 255)
    b = _srgb_to_linear((rgb24 & 0xFF) / 255)
    return 0.2126 * r + 0.7152 * g + 0.0722 * b


def _srgb_to_linear(c: float) -> float:
    if c <= 0.04045:
        return c / 12.92
    return ((c + 0.055) / 1.055) ** 2.4


def _pal(name: str) -> list[int]:
    if name == "pyxel_default":
        return palette()
    if name == "gb_monochrome":
        return GB_PALETTE
    raise KeyError(name)


def palette() -> list[int]:
    return [pyxel.colors[i] for i in range(16)]
