import pyxel
from pyxel.cube import Camera, Model, Scene, Vec3


class TestSceneDraw:
    def test_draw_produces_pixels(self):
        """Scene.draw should write pixels into the screen canvas."""
        pyxel.cls(0)
        scene = Scene()
        model = Model.cube(7)
        scene.add(model, pos=Vec3(0.0, 0.0, 0.0))
        cam = Camera(Vec3(0.0, -3.0, 2.0), Vec3(0.0, 0.0, 0.0))
        scene.draw(0, 0, 160, 120, cam)
        non_zero = sum(
            1 for x in range(160) for y in range(120) if pyxel.pget(x, y) != 0
        )
        assert non_zero > 0, "Scene.draw should produce visible pixels"

    def test_draw_with_offset(self):
        """Scene.draw at offset should not write outside the viewport area."""
        pyxel.cls(0)
        scene = Scene()
        model = Model.cube(7)
        scene.add(model, pos=Vec3(0.0, 0.0, 0.0))
        cam = Camera(Vec3(0.0, -3.0, 2.0), Vec3(0.0, 0.0, 0.0))
        scene.draw(80, 60, 40, 30, cam)
        assert pyxel.pget(0, 0) == 0
        assert pyxel.pget(79, 59) == 0

    def test_empty_scene(self):
        """Empty scene should not produce any pixels."""
        pyxel.cls(5)
        scene = Scene()
        cam = Camera(Vec3(0.0, -3.0, 2.0), Vec3(0.0, 0.0, 0.0))
        scene.draw(0, 0, 160, 120, cam)
        assert pyxel.pget(80, 60) == 5

    def test_zbuffer_ordering(self):
        """Closer object should occlude farther object."""
        pyxel.cls(0)
        scene = Scene()
        far_cube = Model.cube(8)
        scene.add(far_cube, pos=Vec3(0.0, 0.0, 0.0))
        near_cube = Model.cube(11)
        scene.add(near_cube, pos=Vec3(0.0, -1.5, 0.0))
        cam = Camera(Vec3(0.0, -5.0, 2.0), Vec3(0.0, 0.0, 0.0))
        scene.draw(0, 0, 160, 120, cam)
        center_col = pyxel.pget(80, 60)
        assert center_col != 0, "Center should have a rendered pixel"


class TestModel:
    def test_cube_creates_model(self):
        model = Model.cube(7)
        assert model is not None

    def test_plane_creates_model(self):
        model = Model.plane(3)
        assert model is not None

    def test_tri_adds_face(self):
        model = Model()
        model.tri(
            Vec3(0.0, 0.0, 0.0),
            Vec3(1.0, 0.0, 0.0),
            Vec3(0.0, 1.0, 0.0),
            7,
        )
        scene = Scene()
        scene.add(model)
        cam = Camera(Vec3(0.0, 0.0, 5.0), Vec3(0.0, 0.0, 0.0))
        scene.draw(0, 0, 10, 10, cam)
