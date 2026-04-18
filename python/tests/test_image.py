from pathlib import Path

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

    def test_pset_all_colors(self):
        img = pyxel.Image(16, 16)
        for col in range(16):
            img.pset(col, 0, col)
        for col in range(16):
            assert img.pget(col, 0) == col

    def test_set_data(self):
        img = pyxel.Image(4, 2)
        img.set(0, 0, ["0123", "4567"])
        assert img.pget(0, 0) == 0
        assert img.pget(1, 0) == 1
        assert img.pget(2, 0) == 2
        assert img.pget(3, 0) == 3
        assert img.pget(0, 1) == 4
        assert img.pget(3, 1) == 7

    def test_clear(self):
        img = pyxel.Image(8, 8)
        img.pset(0, 0, 7)
        img.cls(0)
        assert img.pget(0, 0) == 0

    def test_cls_with_different_colors(self):
        img = pyxel.Image(8, 8)
        for col in [0, 5, 15]:
            img.cls(col)
            assert img.pget(0, 0) == col
            assert img.pget(4, 4) == col

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

    def test_blt_preserves_uncopied_area(self):
        src = pyxel.Image(8, 8)
        src.cls(5)
        dst = pyxel.Image(16, 16)
        dst.cls(3)
        dst.blt(0, 0, src, 0, 0, 8, 8)
        assert dst.pget(0, 0) == 5  # Copied area
        assert dst.pget(10, 10) == 3  # Uncovered area

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
        img = pyxel.Image.from_image(str(assets_dir / "cat_16x16.png"))
        assert img.width == 16
        assert img.height == 16

    def test_load_image_file(self, assets_dir):
        img = pyxel.Image(32, 32)
        img.load(0, 0, str(assets_dir / "cat_16x16.png"))
        has_nonzero = any(img.pget(x, 0) != 0 for x in range(16))
        assert has_nonzero

    def test_save(self, tmp_path):
        img = pyxel.Image(8, 8)
        img.cls(0)
        img.pset(0, 0, 7)
        path = str(tmp_path / "test_img.png")
        img.save(path, 1)
        assert Path(path).exists()
        assert Path(path).stat().st_size > 0

    def test_line(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.line(0, 0, 15, 0, 7)
        assert img.pget(0, 0) == 7
        assert img.pget(8, 0) == 7
        assert img.pget(15, 0) == 7
        assert img.pget(0, 1) == 0

    def test_rect(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.rect(2, 2, 4, 4, 5)
        assert img.pget(3, 3) == 5
        assert img.pget(0, 0) == 0

    def test_rectb(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.rectb(2, 2, 6, 6, 5)
        assert img.pget(2, 2) == 5  # Border
        assert img.pget(4, 4) == 0  # Inside hollow

    def test_circ(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.circ(16, 16, 5, 8)
        assert img.pget(16, 16) == 8

    def test_circb(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.circb(16, 16, 5, 8)
        assert img.pget(16, 16) == 0

    def test_elli(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.elli(8, 8, 16, 8, 3)
        assert img.pget(16, 12) == 3

    def test_ellib(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.ellib(8, 8, 16, 8, 3)
        assert img.pget(16, 12) == 0

    def test_tri(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.tri(8, 0, 0, 15, 15, 15, 9)
        assert img.pget(8, 8) == 9

    def test_trib(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.trib(8, 0, 0, 15, 15, 15, 9)
        assert img.pget(8, 8) == 0

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
        has_text = any(img.pget(x, y) == 7 for x in range(4) for y in range(6))
        assert has_text

    def test_clip_restricts_drawing(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.clip(4, 4, 8, 8)
        img.rect(0, 0, 16, 16, 7)
        img.clip()
        assert img.pget(0, 0) == 0  # Outside clip
        assert img.pget(6, 6) == 7  # Inside clip

    def test_camera_offsets_drawing(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.camera(10, 10)
        img.pset(10, 10, 7)
        img.camera()
        assert img.pget(0, 0) == 7

    def test_pal_color_replacement(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.pal(7, 8)
        img.pset(0, 0, 7)
        img.pal()
        assert img.pget(0, 0) == 8

    def test_dither(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.dither(0.5)
        img.rect(0, 0, 16, 16, 7)
        img.dither(1.0)
        drawn = sum(1 for x in range(16) for y in range(16) if img.pget(x, y) == 7)
        assert 0 < drawn < 256

    def test_blt3d(self):
        img = pyxel.Image(64, 64)
        img.cls(0)
        img.blt3d(0, 0, 64, 64, 0, (0, 0, 10), (45, 0, 0))

    def test_bltm3d(self):
        img = pyxel.Image(64, 64)
        img.cls(0)
        tm = pyxel.Tilemap(8, 8, 0)
        img.bltm3d(0, 0, 64, 64, tm, (0, 0, 10), (45, 0, 0))

    def test_data_ptr_read(self):
        img = pyxel.Image(8, 8)
        img.cls(0)
        img.pset(0, 0, 7)
        img.pset(1, 0, 3)
        ptr = img.data_ptr()
        assert ptr[0] == 7
        assert ptr[1] == 3
        assert ptr[2] == 0

    def test_data_ptr_write(self):
        img = pyxel.Image(8, 8)
        img.cls(0)
        ptr = img.data_ptr()
        ptr[0] = 5
        assert img.pget(0, 0) == 5

    def test_data_ptr_row_stride(self):
        img = pyxel.Image(8, 4)
        img.cls(0)
        img.pset(0, 1, 9)
        ptr = img.data_ptr()
        # Second row starts at offset = width
        assert ptr[8] == 9

    def test_from_image_with_include_colors(self, assets_dir):
        original_color0 = pyxel.colors[0]
        img = pyxel.Image.from_image(
            str(assets_dir / "cat_16x16.png"), include_colors=True
        )
        assert img.width == 16
        pyxel.colors[0] = original_color0

    def test_load_with_include_colors(self, assets_dir):
        original_color0 = pyxel.colors[0]
        img = pyxel.Image(32, 32)
        img.load(0, 0, str(assets_dir / "cat_16x16.png"), include_colors=True)
        has_nonzero = any(img.pget(x, 0) != 0 for x in range(16))
        assert has_nonzero
        pyxel.colors[0] = original_color0

    def test_blt_with_colkey(self):
        src = pyxel.Image(8, 8)
        src.cls(0)
        src.pset(1, 0, 5)
        dst = pyxel.Image(8, 8)
        dst.cls(3)
        dst.blt(0, 0, src, 0, 0, 8, 8, colkey=0)
        assert dst.pget(0, 0) == 3  # Transparent
        assert dst.pget(1, 0) == 5  # Copied

    def test_blt_with_rotate(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.blt(0, 0, 0, 0, 0, 8, 8, rotate=90)

    def test_blt_with_scale(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.blt(0, 0, 0, 0, 0, 8, 8, scale=2)

    def test_text_with_font(self, assets_dir):
        img = pyxel.Image(64, 32)
        img.cls(0)
        font = pyxel.Font(str(assets_dir / "umplus_j10r.bdf"))
        img.text(0, 0, "A", 7, font)
        has_text = any(img.pget(x, y) == 7 for x in range(20) for y in range(20))
        assert has_text
