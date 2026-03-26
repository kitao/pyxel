import os

import pytest

import pyxel


@pytest.fixture(autouse=True)
def reset_drawing_state():
    yield
    pyxel.clip()
    pyxel.camera()
    pyxel.pal()
    pyxel.dither(1.0)


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
    def test_line(self):
        pyxel.cls(0)
        pyxel.line(0, 0, 15, 0, 7)
        assert pyxel.pget(0, 0) == 7
        assert pyxel.pget(8, 0) == 7
        assert pyxel.pget(0, 8) == 0

    def test_rect(self):
        pyxel.cls(0)
        pyxel.rect(2, 2, 4, 4, 5)
        assert pyxel.pget(3, 3) == 5
        assert pyxel.pget(0, 0) == 0

    def test_rectb(self):
        pyxel.cls(0)
        pyxel.rectb(2, 2, 4, 4, 5)
        assert pyxel.pget(2, 2) == 5
        assert pyxel.pget(3, 3) == 0

    def test_circ(self):
        pyxel.cls(0)
        pyxel.circ(50, 50, 10, 7)
        assert pyxel.pget(50, 50) == 7

    def test_circb(self):
        pyxel.cls(0)
        pyxel.circb(50, 50, 10, 7)
        assert pyxel.pget(50, 50) == 0

    def test_elli(self):
        pyxel.cls(0)
        pyxel.elli(50, 50, 20, 10, 7)
        assert pyxel.pget(60, 55) == 7

    def test_ellib(self):
        pyxel.cls(0)
        pyxel.ellib(50, 50, 20, 10, 7)
        assert pyxel.pget(60, 55) == 0

    def test_tri(self):
        pyxel.cls(0)
        pyxel.tri(10, 10, 20, 10, 15, 20, 7)
        assert pyxel.pget(15, 15) == 7

    def test_trib(self):
        pyxel.cls(0)
        pyxel.trib(10, 10, 20, 10, 15, 20, 7)
        assert pyxel.pget(15, 15) == 0

    def test_fill(self):
        pyxel.cls(0)
        pyxel.rect(10, 10, 20, 20, 5)
        pyxel.fill(15, 15, 8)
        assert pyxel.pget(15, 15) == 8


class TestDrawingState:
    def test_clip_restricts_drawing(self):
        pyxel.cls(0)
        pyxel.clip(10, 10, 50, 50)
        pyxel.rect(0, 0, 160, 120, 7)
        pyxel.clip()
        assert pyxel.pget(0, 0) == 0
        assert pyxel.pget(20, 20) == 7

    def test_camera_offsets_drawing(self):
        pyxel.cls(0)
        pyxel.camera(10, 10)
        pyxel.pset(10, 10, 7)
        pyxel.camera()
        assert pyxel.pget(0, 0) == 7

    def test_pal_color_replacement(self):
        pyxel.cls(0)
        pyxel.pal(7, 8)
        pyxel.pset(0, 0, 7)
        pyxel.pal()
        assert pyxel.pget(0, 0) == 8

    def test_dither(self):
        pyxel.cls(0)
        pyxel.dither(0.5)
        pyxel.rect(0, 0, 160, 120, 7)
        pyxel.dither(1.0)
        drawn = sum(1 for x in range(20) for y in range(20) if pyxel.pget(x, y) == 7)
        assert 0 < drawn < 400


class TestBlt:
    def test_blt_copies_pixel(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(0, 0, 7)
        pyxel.blt(0, 0, 0, 0, 0, 8, 8)
        assert pyxel.pget(0, 0) == 7

    def test_blt_with_image_instance(self):
        pyxel.cls(0)
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.pset(0, 0, 5)
        pyxel.blt(0, 0, img, 0, 0, 8, 8)
        assert pyxel.pget(0, 0) == 5

    def test_blt_with_colkey(self):
        pyxel.cls(3)
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(1, 0, 7)
        pyxel.blt(0, 0, 0, 0, 0, 8, 8, colkey=0)
        assert pyxel.pget(0, 0) == 3
        assert pyxel.pget(1, 0) == 7

    def test_blt_with_rotate_scale(self):
        pyxel.cls(0)
        pyxel.blt(0, 0, 0, 0, 0, 8, 8, rotate=45, scale=2)


class TestBltm:
    def test_bltm_draws_tilemap(self):
        pyxel.cls(0)
        pyxel.tilemaps[0].cls((0, 0))
        pyxel.images[0].cls(0)
        pyxel.images[0].rect(0, 0, 8, 8, 7)
        pyxel.tilemaps[0].pset(0, 0, (0, 0))
        pyxel.bltm(0, 0, 0, 0, 0, 8, 8)
        assert pyxel.pget(0, 0) == 7

    def test_bltm_with_tilemap_instance(self):
        pyxel.cls(0)
        img = pyxel.Image(256, 256)
        img.cls(0)
        img.rect(0, 0, 8, 8, 5)
        tm = pyxel.Tilemap(32, 32, img)
        tm.cls((0, 0))
        tm.pset(0, 0, (0, 0))
        pyxel.bltm(0, 0, tm, 0, 0, 8, 8)
        assert pyxel.pget(0, 0) == 5

    def test_bltm_with_colkey(self):
        pyxel.cls(3)
        pyxel.bltm(0, 0, 0, 0, 0, 8, 8, colkey=0)


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
    def test_text_draws_pixels(self):
        pyxel.cls(0)
        pyxel.text(0, 0, "A", 7)
        has_text = any(pyxel.pget(x, y) == 7 for x in range(4) for y in range(6))
        assert has_text

    def test_text_with_font(self):
        pyxel.cls(0)
        assets_dir = os.path.join(
            os.path.dirname(__file__), os.pardir, "pyxel", "examples", "assets"
        )
        font = pyxel.Font(os.path.join(assets_dir, "umplus_j10r.bdf"))
        pyxel.text(0, 0, "A", 7, font)
        has_text = any(pyxel.pget(x, y) == 7 for x in range(20) for y in range(20))
        assert has_text


class TestScreenBuffer:
    def test_screen_reflects_drawing(self):
        pyxel.cls(0)
        pyxel.pset(5, 5, 9)
        assert pyxel.screen.pget(5, 5) == 9

    def test_screen_writable(self):
        pyxel.cls(0)
        pyxel.screen.pset(10, 10, 3)
        assert pyxel.pget(10, 10) == 3

    def test_screen_dimensions_match(self):
        assert pyxel.screen.width == pyxel.width
        assert pyxel.screen.height == pyxel.height


class TestDrawingStateEdgeCases:
    def test_dither_zero_draws_nothing(self):
        pyxel.cls(0)
        pyxel.dither(0.0)
        pyxel.rect(0, 0, 160, 120, 7)
        pyxel.dither(1.0)
        drawn = sum(1 for x in range(20) for y in range(20) if pyxel.pget(x, y) == 7)
        assert drawn == 0

    def test_dither_one_draws_all(self):
        pyxel.cls(0)
        pyxel.dither(1.0)
        pyxel.rect(0, 0, 20, 20, 7)
        drawn = sum(1 for x in range(20) for y in range(20) if pyxel.pget(x, y) == 7)
        assert drawn == 400

    def test_pal_chained(self):
        pyxel.cls(0)
        pyxel.pal(7, 8)  # 7 -> 8
        pyxel.pal(5, 9)  # 5 -> 9
        pyxel.pset(0, 0, 7)
        pyxel.pset(1, 0, 5)
        pyxel.pal()
        assert pyxel.pget(0, 0) == 8
        assert pyxel.pget(1, 0) == 9

    def test_clip_and_camera_interaction(self):
        pyxel.cls(0)
        pyxel.camera(10, 10)
        pyxel.clip(0, 0, 5, 5)
        pyxel.rect(10, 10, 20, 20, 7)  # With camera offset -> draws at (0,0)
        pyxel.clip()
        pyxel.camera()
        assert pyxel.pget(2, 2) == 7  # Inside clip
        assert pyxel.pget(10, 10) == 0  # Outside clip

    def test_text_multiline(self):
        pyxel.cls(0)
        pyxel.text(0, 0, "A\nB", 7)
        # First line
        has_first = any(pyxel.pget(x, y) == 7 for x in range(4) for y in range(6))
        # Second line (6px below)
        has_second = any(
            pyxel.pget(x, y) == 7 for x in range(4) for y in range(6, 12)
        )
        assert has_first
        assert has_second


class TestBlt3dOptionalParams:
    def test_blt3d_with_fov(self):
        pyxel.cls(0)
        pyxel.blt3d(0, 0, 160, 120, 0, (0, 0, 10), (0, 0, 0), fov=60.0)

    def test_blt3d_with_colkey(self):
        pyxel.cls(0)
        pyxel.blt3d(0, 0, 160, 120, 0, (0, 0, 10), (0, 0, 0), colkey=0)

    def test_blt3d_with_fov_and_colkey(self):
        pyxel.cls(0)
        pyxel.blt3d(0, 0, 160, 120, 0, (0, 0, 10), (0, 0, 0), fov=90.0, colkey=0)

    def test_bltm3d_with_fov_and_colkey(self):
        pyxel.cls(0)
        pyxel.bltm3d(0, 0, 160, 120, 0, (0, 0, 10), (0, 0, 0), fov=90.0, colkey=0)


class TestBltRotateScale:
    def test_blt_rotate_only(self):
        pyxel.cls(0)
        pyxel.blt(0, 0, 0, 0, 0, 8, 8, rotate=90)

    def test_blt_scale_only(self):
        pyxel.cls(0)
        pyxel.blt(0, 0, 0, 0, 0, 8, 8, scale=2)

    def test_bltm_rotate_only(self):
        pyxel.cls(0)
        pyxel.bltm(0, 0, 0, 0, 0, 64, 64, rotate=45)

    def test_bltm_scale_only(self):
        pyxel.cls(0)
        pyxel.bltm(0, 0, 0, 0, 0, 64, 64, scale=2)


class TestBltFlip:
    def test_blt_negative_w_flips_horizontal(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(0, 0, 7)
        pyxel.images[0].pset(7, 0, 5)
        # Negative w flips horizontally
        pyxel.blt(0, 0, 0, 0, 0, -8, 8)
        # After horizontal flip, pixel at (7,0) in source -> (0,0) on screen
        assert pyxel.pget(0, 0) == 5

    def test_blt_negative_h_flips_vertical(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(0, 0, 7)
        pyxel.images[0].pset(0, 7, 5)
        # Negative h flips vertically
        pyxel.blt(0, 0, 0, 0, 0, 8, -8)
        # After vertical flip, pixel at (0,7) in source -> (0,0) on screen
        assert pyxel.pget(0, 0) == 5
