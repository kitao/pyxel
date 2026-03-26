import os

import pyxel


# Image class
class TestImage:
    def test_new_dimensions(self):
        img = pyxel.Image(64, 48)
        assert img.width == 64
        assert img.height == 48

    def test_pset_pget(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.pset(3, 3, 8)
        assert img.pget(3, 3) == 8

    def test_set_data(self):
        img = pyxel.Image(4, 2)
        img.set(0, 0, ["0123", "4567"])
        assert img.pget(0, 0) == 0
        assert img.pget(3, 0) == 3
        assert img.pget(0, 1) == 4

    def test_clear(self):
        img = pyxel.Image(8, 8)
        img.pset(0, 0, 7)
        img.cls(0)
        assert img.pget(0, 0) == 0

    def test_blt_with_int(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.blt(0, 0, 0, 0, 0, 8, 8)

    def test_blt_with_image_instance(self):
        src = pyxel.Image(16, 16)
        src.cls(0)
        src.pset(0, 0, 5)
        dst = pyxel.Image(16, 16)
        dst.cls(0)
        dst.blt(0, 0, src, 0, 0, 8, 8)
        assert dst.pget(0, 0) == 5

    def test_bltm_with_int(self):
        img = pyxel.Image(64, 64)
        img.cls(0)
        img.bltm(0, 0, 0, 0, 0, 64, 64)

    def test_bltm_with_tilemap_instance(self):
        img = pyxel.Image(64, 64)
        img.cls(0)
        tm = pyxel.Tilemap(8, 8, 0)
        img.bltm(0, 0, tm, 0, 0, 64, 64)

    def test_from_image(self, assets_dir):
        img = pyxel.Image.from_image(os.path.join(assets_dir, "cat_16x16.png"))
        assert img.width == 16
        assert img.height == 16

    def test_load_image_file(self, assets_dir):
        img = pyxel.Image(32, 32)
        img.load(0, 0, os.path.join(assets_dir, "cat_16x16.png"))
        # Verify something was loaded (not all zeros)
        has_nonzero = any(img.pget(x, 0) != 0 for x in range(16))
        assert has_nonzero

    def test_save(self, tmp_path):
        img = pyxel.Image(8, 8)
        img.cls(0)
        img.pset(0, 0, 7)
        path = str(tmp_path / "test_img.png")
        img.save(path, 1)
        assert os.path.exists(path)

    def test_line(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.line(0, 0, 15, 0, 7)
        assert img.pget(0, 0) == 7
        assert img.pget(8, 0) == 7
        assert img.pget(0, 8) == 0  # Not on the line

    def test_rect(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.rect(2, 2, 4, 4, 5)
        assert img.pget(3, 3) == 5  # Inside
        assert img.pget(0, 0) == 0  # Outside

    def test_rectb(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.rectb(2, 2, 4, 4, 5)
        assert img.pget(2, 2) == 5  # Border
        assert img.pget(3, 3) == 0  # Inside hollow

    def test_circ(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.circ(16, 16, 5, 8)
        assert img.pget(16, 16) == 8  # Center

    def test_circb(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.circb(16, 16, 5, 8)
        assert img.pget(16, 16) == 0  # Center is hollow

    def test_elli(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.elli(8, 8, 16, 8, 3)
        assert img.pget(16, 12) == 3  # Inside ellipse center area

    def test_ellib(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.ellib(8, 8, 16, 8, 3)
        # Center should be hollow
        assert img.pget(16, 12) == 0

    def test_tri(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.tri(8, 0, 0, 15, 15, 15, 9)
        assert img.pget(8, 8) == 9  # Inside triangle

    def test_trib(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.trib(8, 0, 0, 15, 15, 15, 9)
        assert img.pget(8, 8) == 0  # Inside is hollow

    def test_fill(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.rect(2, 2, 8, 8, 5)
        img.fill(4, 4, 10)
        assert img.pget(4, 4) == 10

    def test_text(self):
        img = pyxel.Image(64, 16)
        img.cls(0)
        img.text(0, 0, "A", 7)
        # At least one pixel should be drawn
        has_text = any(img.pget(x, y) == 7 for x in range(4) for y in range(6))
        assert has_text

    def test_clip_restricts_drawing(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.clip(4, 4, 8, 8)
        img.rect(0, 0, 16, 16, 7)  # Try to fill entire image
        img.clip()  # Reset
        assert img.pget(0, 0) == 0  # Outside clip area
        assert img.pget(6, 6) == 7  # Inside clip area

    def test_camera_offsets_drawing(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.camera(10, 10)
        img.pset(10, 10, 7)  # With camera offset, draws at (0, 0)
        img.camera()  # Reset
        assert img.pget(0, 0) == 7

    def test_pal_color_replacement(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.pal(7, 8)  # Replace color 7 with 8 when drawing
        img.pset(0, 0, 7)
        img.pal()  # Reset
        assert img.pget(0, 0) == 8

    def test_dither(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.dither(0.5)
        img.rect(0, 0, 16, 16, 7)
        img.dither(1.0)  # Reset
        # With 50% dither, some pixels should be drawn, some not
        drawn = sum(1 for x in range(16) for y in range(16) if img.pget(x, y) == 7)
        assert 0 < drawn < 256  # Some but not all

    def test_blt3d(self):
        img = pyxel.Image(64, 64)
        img.cls(0)
        img.blt3d(0, 0, 64, 64, 0, (0, 0, 10), (45, 0, 0))

    def test_bltm3d(self):
        img = pyxel.Image(64, 64)
        img.cls(0)
        tm = pyxel.Tilemap(8, 8, 0)
        img.bltm3d(0, 0, 64, 64, tm, (0, 0, 10), (45, 0, 0))
