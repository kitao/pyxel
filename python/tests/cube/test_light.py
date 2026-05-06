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
        light = Light()
        light.ambient = 0.3
        assert isclose(light.ambient, 0.3, abs_tol=1e-6)

    def test_set_intensity(self):
        light = Light()
        light.intensity = 0.7
        assert isclose(light.intensity, 0.7, abs_tol=1e-6)

    def test_set_direction(self):
        light = Light()
        light.direction = Vec3(1, 0, 0)
        assert light.direction == Vec3(1, 0, 0)


class TestRepr:
    def test_includes_attributes(self):
        light = Light()
        light.ambient = 0.5
        light.intensity = 0.8
        text = repr(light)
        assert "0.5" in text
        assert "0.8" in text
        assert "direction=" in text
