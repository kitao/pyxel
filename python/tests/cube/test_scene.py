from pyxel.cube import Camera, ColorRamp, Light, Mat4, Scene, Vec3


class TestDefault:
    def test_construction(self):
        s = Scene()
        assert isinstance(s.light, Light)
        assert isinstance(s.color_ramp, ColorRamp)
        assert "Scene(" in repr(s)


class TestAttributes:
    def test_set_light(self):
        s = Scene()
        new_light = Light()
        new_light.intensity = 0.5
        s.light = new_light
        # Reassignment surfaces a fresh Light wrapping the underlying Rc
        # state. Confirm the value round-trips.
        assert s.light.intensity == 0.5

    def test_set_color_ramp(self):
        s = Scene()
        new_ramp = ColorRamp()
        s.color_ramp = new_ramp
        assert s.color_ramp[0, 15] == 0


class TestQueue:
    def test_clear_queue_only(self):
        s = Scene()
        s.pset(Vec3.ZERO, 7)
        # repr exposes queue length for skeleton verification
        assert "queued=1" in repr(s)
        s.clear()
        assert "queued=0" in repr(s)

    def test_clear_with_color(self):
        # col-specified clear should not crash even before rasterizer is ready
        s = Scene()
        s.pset(Vec3.ZERO, 7)
        s.clear(0)
        assert "queued=0" in repr(s)


class TestDrawCommands:
    def setup_method(self):
        self.s = Scene()

    def test_pset(self):
        self.s.pset(Vec3.ZERO, 7)
        assert "queued=1" in repr(self.s)

    def test_line(self):
        self.s.line(Vec3.ZERO, Vec3(1, 0, 0), 7)
        assert "queued=1" in repr(self.s)

    def test_tri_trib(self):
        self.s.tri(Vec3.ZERO, Vec3(1, 0, 0), Vec3(0, 1, 0), 7)
        self.s.trib(Vec3.ZERO, Vec3(1, 0, 0), Vec3(0, 1, 0), 8)
        assert "queued=2" in repr(self.s)

    def test_circ_circb(self):
        self.s.circ(Vec3.ZERO, 1.0, 7)
        self.s.circb(Vec3.ZERO, 1.0, 8)
        assert "queued=2" in repr(self.s)

    def test_text(self):
        self.s.text(Vec3.ZERO, "hello", 7)
        assert "queued=1" in repr(self.s)

    def test_rect_family(self):
        m = Mat4.IDENTITY
        self.s.rect(m, 1.0, 1.0, 7)
        self.s.rectb(m, 1.0, 1.0, 8)
        self.s.elli(m, 1.0, 1.0, 9)
        self.s.ellib(m, 1.0, 1.0, 10)
        assert "queued=4" in repr(self.s)


class TestRender:
    def test_render_no_op_skeleton(self):
        # render is a stub in the skeleton phase; calling it must not crash
        # and must not consume the queue.
        s = Scene()
        s.pset(Vec3.ZERO, 7)
        cam = Camera()
        s.render(0, 0, 256, 192, cam)
        assert "queued=1" in repr(s)
