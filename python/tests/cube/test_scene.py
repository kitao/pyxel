import pyxel

from pyxel.cube import Mat4, Node, Scene, Shading, Vec3


def palette() -> list[int]:
    return [pyxel.colors[i] for i in range(16)]


class TestDefault:
    def test_construction(self):
        s = Scene()
        assert isinstance(s.shading, Shading)
        assert s.clear_color is None
        assert "Scene(" in repr(s)

    def test_is_node_subclass(self):
        # Scene inherits Node so the entire Node API (transform, hierarchy,
        # immediate-mode draw commands, lifecycle hooks) is available.
        assert isinstance(Scene(), Node)


class TestAttributes:
    def test_set_shading(self):
        s = Scene()
        new_shading = Shading(palette())
        s.shading = new_shading
        # Shading.__getitem__ returns (primary, secondary).
        assert s.shading[0, 2] == new_shading[0, 2]

    def test_clear_color_default_none(self):
        assert Scene().clear_color is None

    def test_clear_color_round_trips(self):
        s = Scene()
        s.clear_color = 5
        assert s.clear_color == 5
        s.clear_color = None
        assert s.clear_color is None


class TestUpdate:
    def test_update_no_children(self):
        # Empty scene update must not crash.
        Scene().update()


# Immediate-mode draw commands are no-op outside an active DrawContext
# (i.e., when called outside Scene.draw). The tests confirm they do not
# crash when invoked from outside; functional rendering is exercised by
# integration tests / sample programs.
class TestImmediateDrawSafety:
    def test_pset(self):
        Scene().pset(Vec3.ZERO, 7)

    def test_line(self):
        Scene().line(Vec3.ZERO, Vec3(1, 0, 0), 7)

    def test_tri(self):
        s = Scene()
        s.tri(Vec3.ZERO, Vec3(1, 0, 0), Vec3(0, 1, 0), 7)
        s.trib(Vec3.ZERO, Vec3(1, 0, 0), Vec3(0, 1, 0), 8)

    def test_circ(self):
        s = Scene()
        s.circ(Vec3.ZERO, 1.0, 7)
        s.circb(Vec3.ZERO, 1.0, 8)

    def test_rect_family(self):
        s = Scene()
        m = Mat4.IDENTITY
        s.rect(m, 1.0, 1.0, 7)
        s.rectb(m, 1.0, 1.0, 8)
        s.elli(m, 1.0, 1.0, 9)
        s.ellib(m, 1.0, 1.0, 10)
