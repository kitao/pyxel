import pytest
import pyxel
from _assertions import raises_exact  # type: ignore[reportMissingImports]
from pyxel.cube import Camera, Mat4, Node, Primitive, Shading, Vec3


class TestDefault:
    def test_construction(self):
        s = Shading(palette())
        assert repr(s) == "Shading(16 x 4)"

    def test_direction_default(self):
        s = Shading(palette())
        assert s.direction == Vec3(0, -1, 0)


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
            _ = s[0, 4]
        with raises_exact(IndexError, "Shading index out of range"):
            s[0, 4] = (5, 7)

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
    for level in range(4):
        scene.shading[source, level] = (mapped, mapped)
    target = pyxel.Image(64, 64)

    scene.draw(0, 0, 64, 64, target)

    expected = 2 if source == 0 else mapped if shaded else source
    assert target.pget(32, 32) == expected


class TestBuild:
    def test_build_resets_modifications(self):
        pal = palette()
        s = Shading(pal)
        initial = [[s[col, level] for level in range(4)] for col in range(len(pal))]
        for col in range(len(pal)):
            for level in range(4):
                s[col, level] = (99, 99)
        s.build(pal)
        assert [
            [s[col, level] for level in range(4)] for col in range(len(pal))
        ] == initial


GB_PALETTE = [0x000000, 0x555555, 0xAAAAAA, 0xFFFFFF]


# Flat ramps are valid when the palette has no darker or brighter color.
@pytest.mark.parametrize("pal_name", ["pyxel_default", "gb_monochrome"])
class TestRampInvariants:
    def test_ramp_is_monotone(self, pal_name):
        pal = _pal(pal_name)
        s = Shading(pal)
        for col in range(len(pal)):
            cells = [s[col, lv] for lv in range(4)]
            ls = [_entry_luma(pal, p, q) for p, q in cells]
            assert ls[0] <= ls[1] + 1e-6, f"{pal_name} col {col} lv0>lv1"
            assert ls[1] <= ls[2] + 1e-6, f"{pal_name} col {col} lv1>lv2"
            assert ls[2] <= ls[3] + 1e-6, f"{pal_name} col {col} lv2>lv3"

    def test_shade_levels_below_or_equal_base(self, pal_name):
        pal = _pal(pal_name)
        s = Shading(pal)
        for col in range(len(pal)):
            base = _linear_luma(pal[col])
            for lv in (0, 1):
                p, q = s[col, lv]
                assert _entry_luma(pal, p, q) < base + 1e-6, (
                    f"{pal_name} col {col} lv{lv} brighter than base"
                )

    def test_highlight_above_or_equal_base(self, pal_name):
        pal = _pal(pal_name)
        s = Shading(pal)
        for col in range(len(pal)):
            base = _linear_luma(pal[col])
            p, q = s[col, 3]
            assert _entry_luma(pal, p, q) >= base - 1e-6, (
                f"{pal_name} col {col} lv3 darker than base"
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
