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

    def test_pset_pget_multiple_colors(self):
        pyxel.cls(0)
        for col in range(16):
            pyxel.pset(col, 0, col)
        for col in range(16):
            assert pyxel.pget(col, 0) == col

    def test_pset_pget_on_image(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.pset(5, 5, 3)
        assert img.pget(5, 5) == 3

    def test_cls_with_different_colors(self):
        for col in [0, 5, 7, 15]:
            pyxel.cls(col)
            assert pyxel.pget(0, 0) == col
            assert pyxel.pget(80, 60) == col


class TestDrawingPrimitives:
    def test_line_horizontal(self):
        pyxel.cls(0)
        pyxel.line(0, 0, 15, 0, 7)
        assert pyxel.pget(0, 0) == 7
        assert pyxel.pget(8, 0) == 7
        assert pyxel.pget(15, 0) == 7
        assert pyxel.pget(0, 1) == 0  # Below line

    def test_line_vertical(self):
        pyxel.cls(0)
        pyxel.line(5, 0, 5, 15, 3)
        assert pyxel.pget(5, 0) == 3
        assert pyxel.pget(5, 8) == 3
        assert pyxel.pget(5, 15) == 3
        assert pyxel.pget(6, 0) == 0  # Right of line

    def test_rect(self):
        pyxel.cls(0)
        pyxel.rect(2, 2, 4, 4, 5)
        assert pyxel.pget(3, 3) == 5  # Inside
        assert pyxel.pget(0, 0) == 0  # Outside

    def test_rectb(self):
        pyxel.cls(0)
        pyxel.rectb(2, 2, 6, 6, 5)
        assert pyxel.pget(2, 2) == 5  # Corner
        assert pyxel.pget(4, 4) == 0  # Center is hollow

    def test_circ(self):
        pyxel.cls(0)
        pyxel.circ(50, 50, 10, 7)
        assert pyxel.pget(50, 50) == 7  # Center filled
        assert pyxel.pget(50, 40) == 7  # Top edge

    def test_circb(self):
        pyxel.cls(0)
        pyxel.circb(50, 50, 10, 7)
        assert pyxel.pget(50, 50) == 0  # Center is hollow
        assert pyxel.pget(50, 40) == 7  # Top edge

    def test_elli(self):
        pyxel.cls(0)
        pyxel.elli(50, 50, 20, 10, 7)
        assert pyxel.pget(60, 55) == 7  # Inside

    def test_ellib(self):
        pyxel.cls(0)
        pyxel.ellib(50, 50, 20, 10, 7)
        assert pyxel.pget(60, 55) == 0  # Inside is hollow

    def test_tri(self):
        pyxel.cls(0)
        pyxel.tri(10, 10, 20, 10, 15, 20, 7)
        assert pyxel.pget(15, 15) == 7  # Inside
        assert pyxel.pget(0, 0) == 0  # Outside

    def test_trib(self):
        pyxel.cls(0)
        pyxel.trib(10, 10, 20, 10, 15, 20, 7)
        assert pyxel.pget(15, 15) == 0  # Inside is hollow
        assert pyxel.pget(10, 10) == 7  # Vertex

    def test_fill(self):
        pyxel.cls(0)
        pyxel.rect(10, 10, 20, 20, 5)
        pyxel.fill(15, 15, 8)
        assert pyxel.pget(15, 15) == 8  # Filled area
        assert pyxel.pget(0, 0) == 0  # Outside

    def test_fill_bounded_by_different_color(self):
        pyxel.cls(0)
        pyxel.rectb(10, 10, 10, 10, 5)  # Border
        pyxel.fill(15, 15, 8)
        assert pyxel.pget(15, 15) == 8  # Inside border
        assert pyxel.pget(0, 0) == 0  # Outside border


class TestDrawingState:
    def test_clip_restricts_drawing(self):
        pyxel.cls(0)
        pyxel.clip(10, 10, 50, 50)
        pyxel.rect(0, 0, 160, 120, 7)
        pyxel.clip()
        assert pyxel.pget(0, 0) == 0  # Outside clip
        assert pyxel.pget(20, 20) == 7  # Inside clip

    def test_clip_reset(self):
        pyxel.cls(0)
        pyxel.clip(10, 10, 5, 5)
        pyxel.clip()  # Reset
        pyxel.pset(0, 0, 7)
        assert pyxel.pget(0, 0) == 7  # No clip restriction

    def test_camera_offsets_drawing(self):
        pyxel.cls(0)
        pyxel.camera(10, 10)
        pyxel.pset(10, 10, 7)
        pyxel.camera()
        assert pyxel.pget(0, 0) == 7  # (10,10) - camera(10,10) = (0,0)

    def test_camera_reset(self):
        pyxel.cls(0)
        pyxel.camera(10, 10)
        pyxel.camera()  # Reset
        pyxel.pset(5, 5, 7)
        assert pyxel.pget(5, 5) == 7

    def test_pal_color_replacement(self):
        pyxel.cls(0)
        pyxel.pal(7, 8)
        pyxel.pset(0, 0, 7)
        pyxel.pal()
        assert pyxel.pget(0, 0) == 8

    def test_pal_reset(self):
        pyxel.cls(0)
        pyxel.pal(7, 8)
        pyxel.pal()  # Reset
        pyxel.pset(0, 0, 7)
        assert pyxel.pget(0, 0) == 7

    def test_dither_half(self):
        pyxel.cls(0)
        pyxel.dither(0.5)
        pyxel.rect(0, 0, 20, 20, 7)
        pyxel.dither(1.0)
        drawn = sum(1 for x in range(20) for y in range(20) if pyxel.pget(x, y) == 7)
        # ~50% of 400 pixels = ~200 (allow wide range for dither pattern)
        assert 150 < drawn < 250


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
        assert pyxel.pget(0, 0) == 3  # Transparent (colkey=0)
        assert pyxel.pget(1, 0) == 7  # Copied

    def test_blt_with_rotate(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].rect(0, 0, 8, 8, 7)
        pyxel.blt(80, 60, 0, 0, 0, 8, 8, rotate=45)
        # After rotation, pixels should be drawn somewhere near (80, 60)
        has_drawn = any(
            pyxel.pget(x, y) == 7 for x in range(70, 90) for y in range(50, 70)
        )
        assert has_drawn

    def test_blt_with_scale(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(0, 0, 7)
        pyxel.blt(0, 0, 0, 0, 0, 1, 1, scale=4)
        # A 1x1 pixel scaled 4x should cover approximately 4x4 area
        drawn = sum(1 for x in range(8) for y in range(8) if pyxel.pget(x, y) == 7)
        assert drawn > 0


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
        pyxel.images[0].cls(0)
        pyxel.tilemaps[0].cls((0, 0))
        pyxel.bltm(0, 0, 0, 0, 0, 8, 8, colkey=0)
        # With colkey=0, black pixels are transparent, background (3) shows through
        assert pyxel.pget(0, 0) == 3

    def test_bltm_rotate(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].rect(0, 0, 8, 8, 7)
        pyxel.tilemaps[0].cls((0, 0))
        pyxel.tilemaps[0].pset(0, 0, (0, 0))
        pyxel.bltm(80, 60, 0, 0, 0, 8, 8, rotate=45)
        has_drawn = any(
            pyxel.pget(x, y) == 7 for x in range(70, 90) for y in range(50, 70)
        )
        assert has_drawn

    def test_bltm_scale(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(0, 0, 7)
        pyxel.tilemaps[0].cls((0, 0))
        pyxel.tilemaps[0].pset(0, 0, (0, 0))
        pyxel.bltm(0, 0, 0, 0, 0, 1, 1, scale=4)
        drawn = sum(1 for x in range(8) for y in range(8) if pyxel.pget(x, y) == 7)
        assert drawn > 0


class TestBlt3d:
    def test_blt3d_with_int_img(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].rect(0, 0, 16, 16, 7)
        pyxel.blt3d(0, 0, 160, 120, 0, (0, 0, 10), (0, 30, 0))
        assert any(pyxel.pget(x, y) == 7 for x in range(160) for y in range(120))

    def test_blt3d_with_image_instance(self):
        pyxel.cls(0)
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.rect(0, 0, 16, 16, 5)
        pyxel.blt3d(0, 0, 160, 120, img, (0, 0, 10), (0, 30, 0))
        assert any(pyxel.pget(x, y) == 5 for x in range(160) for y in range(120))

    def test_bltm3d_with_int_tm(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].rect(0, 0, 8, 8, 12)
        pyxel.tilemaps[0].cls((0, 0))
        pyxel.tilemaps[0].rect(0, 0, 8, 8, (0, 0))
        pyxel.bltm3d(0, 0, 160, 120, 0, (0, 0, 10), (0, 30, 0))
        assert any(pyxel.pget(x, y) == 12 for x in range(160) for y in range(120))

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
        assert any(pyxel.pget(x, y) == 9 for x in range(160) for y in range(120))

    def test_blt3d_with_colkey(self):
        pyxel.cls(3)
        pyxel.images[0].cls(0)
        pyxel.images[0].rect(0, 0, 8, 8, 7)
        pyxel.blt3d(0, 0, 160, 120, 0, (0, 0, 10), (0, 30, 0), colkey=0)
        assert any(pyxel.pget(x, y) == 7 for x in range(160) for y in range(120))

    def test_blt3d_with_fov_and_colkey(self):
        pyxel.cls(3)
        pyxel.images[0].cls(0)
        pyxel.images[0].rect(0, 0, 8, 8, 11)
        pyxel.blt3d(0, 0, 160, 120, 0, (0, 0, 10), (0, 30, 0), fov=90.0, colkey=0)
        assert any(pyxel.pget(x, y) == 11 for x in range(160) for y in range(120))

    def test_bltm3d_with_fov_and_colkey(self):
        pyxel.cls(3)
        pyxel.images[0].cls(0)
        pyxel.images[0].rect(0, 0, 8, 8, 6)
        pyxel.tilemaps[0].cls((0, 0))
        pyxel.tilemaps[0].rect(0, 0, 8, 8, (0, 0))
        pyxel.bltm3d(0, 0, 160, 120, 0, (0, 0, 10), (0, 30, 0), fov=90.0, colkey=0)
        assert any(pyxel.pget(x, y) == 6 for x in range(160) for y in range(120))


class TestText:
    def test_text_draws_pixels(self):
        pyxel.cls(0)
        pyxel.text(0, 0, "A", 7)
        has_text = any(pyxel.pget(x, y) == 7 for x in range(4) for y in range(6))
        assert has_text

    def test_text_with_font(self, assets_dir):
        pyxel.cls(0)
        font = pyxel.Font(str(assets_dir / "umplus_j10r.bdf"))
        pyxel.text(0, 0, "A", 7, font)
        has_text = any(pyxel.pget(x, y) == 7 for x in range(20) for y in range(20))
        assert has_text

    def test_text_multiline(self):
        pyxel.cls(0)
        pyxel.text(0, 0, "A\nB", 7)
        has_first = any(pyxel.pget(x, y) == 7 for x in range(4) for y in range(6))
        has_second = any(pyxel.pget(x, y) == 7 for x in range(4) for y in range(6, 12))
        assert has_first
        assert has_second

    def test_text_empty_string(self):
        pyxel.cls(0)
        pyxel.text(0, 0, "", 7)
        # No pixels should be drawn
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
        # The draw path treats alpha < pattern threshold as "skip", so any
        # negative alpha effectively behaves like alpha=0 (full transparent).
        pyxel.cls(0)
        pyxel.dither(-0.5)
        pyxel.rect(0, 0, 20, 20, 7)
        pyxel.dither(1.0)
        drawn = sum(1 for x in range(20) for y in range(20) if pyxel.pget(x, y) == 7)
        assert drawn == 0

    def test_dither_above_one_behaves_as_one(self):
        # The draw path takes the alpha >= 1.0 fast path, so any alpha > 1
        # effectively behaves like alpha=1 (full opaque).
        pyxel.cls(0)
        pyxel.dither(1.5)
        pyxel.rect(0, 0, 20, 20, 7)
        drawn = sum(1 for x in range(20) for y in range(20) if pyxel.pget(x, y) == 7)
        assert drawn == 400

    def test_pal_chained(self):
        pyxel.cls(0)
        pyxel.pal(7, 8)
        pyxel.pal(5, 9)
        pyxel.pset(0, 0, 7)
        pyxel.pset(1, 0, 5)
        pyxel.pal()
        assert pyxel.pget(0, 0) == 8
        assert pyxel.pget(1, 0) == 9

    def test_pal_triple_chain(self):
        pyxel.cls(0)
        pyxel.pal(1, 2)
        pyxel.pal(3, 4)
        pyxel.pal(5, 6)
        pyxel.pset(0, 0, 1)
        pyxel.pset(1, 0, 3)
        pyxel.pset(2, 0, 5)
        pyxel.pal()
        assert pyxel.pget(0, 0) == 2
        assert pyxel.pget(1, 0) == 4
        assert pyxel.pget(2, 0) == 6

    def test_clip_and_camera_interaction(self):
        pyxel.cls(0)
        pyxel.camera(10, 10)
        pyxel.clip(0, 0, 5, 5)
        pyxel.rect(10, 10, 20, 20, 7)
        pyxel.clip()
        pyxel.camera()
        assert pyxel.pget(2, 2) == 7  # Inside clip
        assert pyxel.pget(10, 10) == 0  # Outside clip


class TestBltFlip:
    def test_blt_negative_w_flips_horizontal(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(0, 0, 7)
        pyxel.images[0].pset(7, 0, 5)
        pyxel.blt(0, 0, 0, 0, 0, -8, 8)
        # After horizontal flip, pixel at (7,0) in source -> (0,0) on screen
        assert pyxel.pget(0, 0) == 5

    def test_blt_negative_h_flips_vertical(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(0, 0, 7)
        pyxel.images[0].pset(0, 7, 5)
        pyxel.blt(0, 0, 0, 0, 0, 8, -8)
        # After vertical flip, pixel at (0,7) in source -> (0,0) on screen
        assert pyxel.pget(0, 0) == 5

    def test_blt_negative_both_flips(self):
        pyxel.cls(0)
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(7, 7, 5)
        pyxel.blt(0, 0, 0, 0, 0, -8, -8)
        # After both flips, (7,7) -> (0,0)
        assert pyxel.pget(0, 0) == 5


class TestDeprecatedAccessors:
    def test_image_function_returns_image_instance(self, capfd):
        result = pyxel.image(0)  # type: ignore[attr-defined]
        assert isinstance(result, pyxel.Image)
        out = capfd.readouterr().out
        assert "deprecated" in out.lower()

    def test_tilemap_function_returns_tilemap_instance(self, capfd):
        result = pyxel.tilemap(0)  # type: ignore[attr-defined]
        assert isinstance(result, pyxel.Tilemap)
        out = capfd.readouterr().out
        assert "deprecated" in out.lower()
