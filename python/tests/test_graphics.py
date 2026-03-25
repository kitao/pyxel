import pyxel


class TestPsetPget:
    def test_pset_pget_on_screen(self):
        pyxel.cls(0)
        pyxel.pset(10, 10, 7)
        assert pyxel.pget(10, 10) == 7

    def test_cls_clears_screen(self):
        pyxel.pset(0, 0, 7)
        pyxel.cls(0)
        assert pyxel.pget(0, 0) == 0

    def test_pset_pget_on_image(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.pset(5, 5, 3)
        assert img.pget(5, 5) == 3


class TestDrawingPrimitives:
    """Verify drawing primitives run without errors."""

    def test_line(self):
        pyxel.cls(0)
        pyxel.line(0, 0, 10, 10, 7)

    def test_rect(self):
        pyxel.cls(0)
        pyxel.rect(0, 0, 10, 10, 7)

    def test_rectb(self):
        pyxel.cls(0)
        pyxel.rectb(0, 0, 10, 10, 7)

    def test_circ(self):
        pyxel.cls(0)
        pyxel.circ(50, 50, 10, 7)

    def test_circb(self):
        pyxel.cls(0)
        pyxel.circb(50, 50, 10, 7)

    def test_elli(self):
        pyxel.cls(0)
        pyxel.elli(50, 50, 20, 10, 7)

    def test_ellib(self):
        pyxel.cls(0)
        pyxel.ellib(50, 50, 20, 10, 7)

    def test_tri(self):
        pyxel.cls(0)
        pyxel.tri(10, 10, 20, 10, 15, 20, 7)

    def test_trib(self):
        pyxel.cls(0)
        pyxel.trib(10, 10, 20, 10, 15, 20, 7)

    def test_fill(self):
        pyxel.cls(0)
        pyxel.rect(10, 10, 20, 20, 5)
        pyxel.fill(15, 15, 8)


class TestDrawingState:
    def test_clip_set_and_reset(self):
        pyxel.clip(10, 10, 50, 50)
        pyxel.clip()

    def test_camera_set_and_reset(self):
        pyxel.camera(10, 20)
        pyxel.camera()

    def test_pal_set_and_reset(self):
        pyxel.pal(7, 8)
        pyxel.pal()

    def test_dither(self):
        pyxel.dither(0.5)
        pyxel.dither(1.0)


class TestBlt:
    def test_blt_with_int_img(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(0, 0, 7)
        pyxel.blt(0, 0, 0, 0, 0, 8, 8)

    def test_blt_with_image_instance(self):
        pyxel.cls(0)
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.pset(0, 0, 5)
        pyxel.blt(0, 0, img, 0, 0, 8, 8)

    def test_blt_with_colkey(self):
        pyxel.cls(0)
        pyxel.blt(0, 0, 0, 0, 0, 8, 8, colkey=0)

    def test_blt_with_rotate_scale(self):
        pyxel.cls(0)
        pyxel.blt(0, 0, 0, 0, 0, 8, 8, rotate=45, scale=2)


class TestBltm:
    def test_bltm_with_int_tm(self):
        pyxel.cls(0)
        pyxel.bltm(0, 0, 0, 0, 0, 64, 64)

    def test_bltm_with_tilemap_instance(self):
        pyxel.cls(0)
        tm = pyxel.Tilemap(32, 32, 0)
        pyxel.bltm(0, 0, tm, 0, 0, 64, 64)

    def test_bltm_with_colkey(self):
        pyxel.cls(0)
        pyxel.bltm(0, 0, 0, 0, 0, 64, 64, colkey=0)


class TestBlt3d:
    def test_blt3d_with_int_img(self):
        pyxel.cls(0)
        # blt3d(x, y, w, h, img, pos, rot, fov, colkey)
        pyxel.blt3d(0, 0, 160, 120, 0, (0, 0, 10), (45, 0, 0))

    def test_blt3d_with_image_instance(self):
        pyxel.cls(0)
        img = pyxel.Image(16, 16)
        img.cls(0)
        pyxel.blt3d(0, 0, 160, 120, img, (0, 0, 10), (45, 0, 0))

    def test_bltm3d_with_int_tm(self):
        pyxel.cls(0)
        # bltm3d(x, y, w, h, tm, pos, rot, fov, colkey)
        pyxel.bltm3d(0, 0, 160, 120, 0, (0, 0, 10), (45, 0, 0))

    def test_bltm3d_with_tilemap_instance(self):
        pyxel.cls(0)
        tm = pyxel.Tilemap(32, 32, 0)
        pyxel.bltm3d(0, 0, 160, 120, tm, (0, 0, 10), (45, 0, 0))


class TestText:
    def test_text_basic(self):
        pyxel.cls(0)
        pyxel.text(10, 10, "hello", 7)

    def test_text_with_font(self):
        import os

        pyxel.cls(0)
        assets_dir = os.path.join(
            os.path.dirname(__file__), os.pardir, "pyxel", "examples", "assets"
        )
        font = pyxel.Font(os.path.join(assets_dir, "umplus_j10r.bdf"))
        pyxel.text(10, 10, "hello", 7, font)
