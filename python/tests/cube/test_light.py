from math import isclose

from pyxel.cube import Light, Vec3


class TestDefault:
    def test_ambient(self):
        assert Light().ambient == 0.0

    def test_intensity(self):
        assert Light().intensity == 1.0

    def test_direction_down(self):
        assert Light().direction == Vec3.DOWN


class TestMutation:
    def test_set_ambient(self):
        l = Light()
        l.ambient = 0.3
        assert isclose(l.ambient, 0.3, abs_tol=1e-6)

    def test_set_intensity(self):
        l = Light()
        l.intensity = 0.7
        assert isclose(l.intensity, 0.7, abs_tol=1e-6)

    def test_set_direction(self):
        l = Light()
        l.direction = Vec3(1, 0, 0)
        assert l.direction == Vec3(1, 0, 0)


class TestRepr:
    def test_includes_attributes(self):
        l = Light()
        l.ambient = 0.5
        l.intensity = 0.8
        r = repr(l)
        assert "0.5" in r
        assert "0.8" in r
        assert "direction=" in r
