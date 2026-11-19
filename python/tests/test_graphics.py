import pytest
import pyxel


class TestPsetPget:
    def test_pset_pget_multiple_colors(self):
        pyxel.cls(0)
        for col in range(16):
            pyxel.pset(col, 0, col)
        for col in range(16):
            assert pyxel.pget(col, 0) == col


class TestDrawingPrimitives:
    def test_elli(self):
        pyxel.cls(0)
        pyxel.elli(50, 50, 20, 10, 7)
        assert pyxel.pget(60, 55) == 7

    def test_ellib(self):
        pyxel.cls(0)
        pyxel.ellib(50, 50, 20, 10, 7)
        assert pyxel.pget(60, 50) == 7
        assert pyxel.pget(60, 55) == 0

    def test_fill(self):
        pyxel.cls(0)
        pyxel.rect(10, 10, 20, 20, 5)
        pyxel.fill(15, 15, 8)
        assert pyxel.pget(15, 15) == 8
        assert pyxel.pget(0, 0) == 0

    def test_fill_bounded_by_different_color(self):
        pyxel.cls(0)
        pyxel.rectb(10, 10, 10, 10, 5)
        pyxel.fill(15, 15, 8)
        assert pyxel.pget(15, 15) == 8
        assert pyxel.pget(0, 0) == 0


class TestDrawingState:
    def test_clip_reset(self):
        pyxel.cls(0)
        pyxel.clip(10, 10, 5, 5)
        pyxel.clip()
        pyxel.pset(0, 0, 7)
        assert pyxel.pget(0, 0) == 7

    def test_camera_reset(self):
        pyxel.cls(0)
        pyxel.camera(10, 10)
        pyxel.camera()
        pyxel.pset(5, 5, 7)
        assert pyxel.pget(5, 5) == 7

    def test_pal_reset(self):
        pyxel.cls(0)
        pyxel.pal(7, 8)
        pyxel.pal()
        pyxel.pset(0, 0, 7)
        assert pyxel.pget(0, 0) == 7


class TestBlt:
    def test_blt_with_scale(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(0, 0, 7)
        pyxel.blt(0, 0, 0, 0, 0, 1, 1, scale=4)
        # At the origin, clipping leaves four painted pixels.
        drawn = sum(1 for x in range(8) for y in range(8) if pyxel.pget(x, y) == 7)
        assert drawn == 4


class TestBlitClipping:
    @pytest.mark.parametrize("left", [31, 32, 33])
    def test_clip_across_a_long_scaled_row(self, left):
        source = pyxel.Image(64, 1)
        for x in range(64):
            source.pset(x, 0, 1 + x % 15)
        target = pyxel.Image(96, 1)
        target.blt(3, 0, source, 0, 0, 64, 1, scale=1.5)
        expected = [target.pget(x, 0) for x in range(left, 96)]

        target.cls(0)
        target.clip(left, 0, 96 - left, 1)
        target.blt(3, 0, source, 0, 0, 64, 1, scale=1.5)
        assert [target.pget(x, 0) for x in range(left, 96)] == expected

    @pytest.mark.parametrize("tilemap", [False, True])
    @pytest.mark.parametrize("perspective", [False, True])
    def test_clip_preserves_sampled_pixels(self, tilemap, perspective):
        image = pyxel.Image(32, 32)
        for y in range(32):
            for x in range(32):
                image.pset(x, y, (x * 3 + y * 7) % 16)
        tiles = pyxel.Tilemap(4, 4, image)
        for y in range(4):
            for x in range(4):
                tiles.pset(x, y, (x, y))

        target = pyxel.Image(32, 32)
        source = tiles if tilemap else image
        if perspective:
            draw = target.bltm3d if tilemap else target.blt3d
            args = (0, 0, 32, 32, source, (15.5, 11.5, 7), (30, 0, 0), 60)
            left = 3
        else:
            draw = target.bltm if tilemap else target.blt
            args = (
                (4, 4, source, 2, 1, 4, 4, None, 45, 0.5)
                if tilemap
                else (4, 4, source, 2, 1, 8, 4, None, 0, 1.5)
            )
            left = 5 if tilemap else 9

        target.cls(15)
        draw(*args)
        expected = [
            target.pget(x, y) if x >= left else 15 for y in range(32) for x in range(32)
        ]
        target.cls(15)
        target.clip(left, 0, 32 - left, 32)
        draw(*args)
        target.clip()
        assert [target.pget(x, y) for y in range(32) for x in range(32)] == expected


class TestBltm:
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

    def test_bltm_scale(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(0, 0, 7)
        pyxel.tilemaps[0].cls((0, 0))
        pyxel.tilemaps[0].pset(0, 0, (0, 0))

        pyxel.bltm(0, 0, 0, 0, 0, 1, 1, scale=4)
        drawn = sum(1 for x in range(8) for y in range(8) if pyxel.pget(x, y) == 7)
        assert drawn == 4

    def test_bltm_scale_skips_tiles_outside_the_image(self):
        pyxel.cls(3)
        pyxel.tilemaps[0].cls((255, 255))

        pyxel.bltm(0, 0, 0, 0, 0, 8, 8, scale=2)
        assert pyxel.pget(4, 4) == 3


class TestBlt3d:
    def test_blt3d_with_image_instance(self):
        pyxel.cls(0)
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.rect(0, 0, 16, 16, 5)
        pyxel.blt3d(0, 0, 160, 120, img, (0, 0, 10), (0, 30, 0))
        assert any(pyxel.pget(x, y) == 5 for x in range(160) for y in range(120))

    def test_bltm3d_with_tilemap_instance(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].rect(0, 0, 8, 8, 14)
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.rect(0, 0, 8, 8, (0, 0))

        pyxel.bltm3d(0, 0, 160, 120, tm, (0, 0, 10), (0, 30, 0))
        assert any(pyxel.pget(x, y) == 14 for x in range(160) for y in range(120))

    def test_blt3d_with_fov(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].rect(0, 0, 16, 16, 9)
        pyxel.blt3d(0, 0, 160, 120, 0, (0, 0, 10), (0, 30, 0), fov=60.0)
        narrow = [pyxel.pget(x, y) for x in range(160) for y in range(120)]

        pyxel.cls(0)
        pyxel.blt3d(0, 0, 160, 120, 0, (0, 0, 10), (0, 30, 0), fov=90.0)
        wide = [pyxel.pget(x, y) for x in range(160) for y in range(120)]
        assert 9 in narrow
        assert wide != narrow

    def test_blt3d_with_colkey(self):
        pyxel.cls(3)
        pyxel.images[0].cls(0)
        pyxel.images[0].rect(0, 0, 8, 8, 7)
        pyxel.blt3d(0, 0, 160, 120, 0, (0, 0, 10), (0, 30, 0), colkey=0)
        assert any(pyxel.pget(x, y) == 7 for x in range(160) for y in range(120))
        assert not any(pyxel.pget(x, y) == 0 for x in range(160) for y in range(120))

    @pytest.mark.parametrize("draw", [pyxel.blt3d, pyxel.bltm3d])
    def test_fov_and_colkey(self, draw):
        pyxel.cls(3)
        pyxel.images[0].cls(0)
        pyxel.images[0].line(0, 0, 0, 7, 6)
        pyxel.tilemaps[0].pset(0, 0, (0, 0))

        # A 90-degree view from four units up spans the eight-pixel source tile.
        draw(0, 0, 8, 8, 0, (3.5, 3.5, 4), (0, 0, 0), fov=90, colkey=0)
        assert [pyxel.pget(x, 3) for x in range(8)] == [6, 3, 3, 3, 3, 3, 3, 3]


class TestText:
    def test_text_empty_string(self):
        pyxel.cls(0)
        pyxel.text(0, 0, "", 7)
        drawn = sum(1 for x in range(10) for y in range(10) if pyxel.pget(x, y) == 7)
        assert drawn == 0


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

    def test_dither_negative_alpha_behaves_as_zero(self):
        # All pattern thresholds are nonnegative.
        pyxel.cls(0)
        pyxel.dither(-0.5)
        pyxel.rect(0, 0, 20, 20, 7)
        pyxel.dither(1.0)
        drawn = sum(1 for x in range(20) for y in range(20) if pyxel.pget(x, y) == 7)
        assert drawn == 0

    def test_dither_above_one_behaves_as_one(self):
        # Alpha >= 1 bypasses the dither pattern.
        pyxel.cls(0)
        pyxel.dither(1.5)
        pyxel.rect(0, 0, 20, 20, 7)
        drawn = sum(1 for x in range(20) for y in range(20) if pyxel.pget(x, y) == 7)
        assert drawn == 400

    def test_clip_and_camera_interaction(self):
        pyxel.cls(0)
        pyxel.camera(10, 10)
        pyxel.clip(0, 0, 5, 5)
        pyxel.rect(10, 10, 20, 20, 7)
        pyxel.clip()
        pyxel.camera()
        assert pyxel.pget(2, 2) == 7
        assert pyxel.pget(10, 10) == 0


class TestDeprecatedAccessors:
    def test_image_function_aliases_bank_entry(self, capfd):
        result = pyxel.image(0)  # type: ignore[attr-defined]
        assert isinstance(result, pyxel.Image)

        original = pyxel.images[0].pget(0, 0)
        try:
            pyxel.images[0].pset(0, 0, 0)
            result.pset(0, 0, 7)
            assert pyxel.images[0].pget(0, 0) == 7
        finally:
            pyxel.images[0].pset(0, 0, original)

        out = capfd.readouterr().out
        assert out == "pyxel.image(img) is deprecated. Use pyxel.images[img] instead.\n"

    def test_tilemap_function_aliases_bank_entry(self, capfd):
        result = pyxel.tilemap(0)  # type: ignore[attr-defined]
        assert isinstance(result, pyxel.Tilemap)

        original = pyxel.tilemaps[0].pget(0, 0)
        try:
            pyxel.tilemaps[0].pset(0, 0, (0, 0))
            result.pset(0, 0, (3, 4))
            assert pyxel.tilemaps[0].pget(0, 0) == (3, 4)
        finally:
            pyxel.tilemaps[0].pset(0, 0, original)

        out = capfd.readouterr().out
        assert (
            out == "pyxel.tilemap(tm) is deprecated. Use pyxel.tilemaps[tm] instead.\n"
        )
