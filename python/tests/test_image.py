import subprocess
import sys

import PIL.Image
import pytest
import pyxel
from _assertions import raises_exact  # type: ignore[reportMissingImports]


class TestImageCreation:
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
        assert list(img.data_ptr()) == [0, 1, 2, 3, 4, 5, 6, 7]

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


class TestImageDrawing:
    def test_bltm_camera_offset_does_not_wrap(self):
        source = pyxel.Image(8, 8)
        source.cls(7)
        tilemap = pyxel.Tilemap(17, 1, source)
        img = pyxel.Image(8, 8)
        img.camera(-2_147_483_648, 0)
        img.bltm(2_147_483_520, 0, tilemap, 0, 0, 136, 8)
        assert list(img.data_ptr()) == [0] * 64

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
        assert img.pget(2, 2) == 5
        assert img.pget(4, 4) == 0

    def test_circ(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.circ(16, 16, 5, 8)
        assert img.pget(16, 16) == 8

    def test_circb(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.circb(16, 16, 5, 8)
        assert img.pget(16, 11) == 8
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
        assert img.pget(16, 8) == 3
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
        assert img.pget(8, 0) == 9
        assert img.pget(0, 15) == 9
        assert img.pget(15, 15) == 9
        assert img.pget(8, 8) == 0

    def test_dithered_fill_preserves_connected_region(self):
        code = """
import pyxel

rows = ["11111111", "10010001", "10010001", "10000001",
        "11111111", "10000001", "10000001", "11111111"]

for alpha in (0.0, 0.25, 0.5, 1.0, float("nan")):
    actual, expected = pyxel.Image(8, 8), pyxel.Image(8, 8)
    for img in (actual, expected):
        img.set(0, 0, rows)
        img.dither(alpha)

    expected.rect(1, 1, 6, 3, 7)
    expected.dither(1)
    expected.rect(3, 1, 1, 2, 1)

    actual.clip(1, 1, 6, 6)
    actual.camera(1, 1)
    actual.fill(2, 2, 7)
    assert list(actual.data_ptr()) == list(expected.data_ptr()), alpha
"""

        result = subprocess.run(
            [sys.executable, "-c", code],
            capture_output=True,
            text=True,
            timeout=10,
            check=False,
        )
        assert result.returncode == 0, result.stderr

    def test_text_with_font(self, assets_dir):
        img = pyxel.Image(64, 32)
        img.cls(0)
        font = pyxel.Font(str(assets_dir / "umplus_j10r.bdf"))
        img.text(0, 0, "A", 7, font)
        assert [img.pget(x, 6) for x in range(6)] == [7, 7, 7, 7, 7, 0]


class TestImageBlt:
    @pytest.mark.parametrize(
        ("method", "rotate", "source_pos"),
        [
            ("blt", 0, 0),
            ("blt", 30, 0),
            ("blt", 0, -1),
            ("blt", 30, -1),
            ("bltm", 0, 0),
            ("bltm", 30, 0),
            ("blt3d", 0, 0),
            ("bltm3d", 0, 0),
        ],
    )
    @pytest.mark.parametrize("bank_source", [False, True])
    def test_self_blit_matches_source_snapshot(
        self, method, rotate, source_pos, bank_source
    ):
        actual, expected, snapshot = [pyxel.Image(16, 16) for _ in range(3)]
        for img in (actual, expected, snapshot):
            for y in range(16):
                for x in range(16):
                    img.pset(x, y, (x + y * 3) % 16)

        original_bank = pyxel.images[0]
        try:
            for dst, src in ((expected, snapshot), (actual, actual)):
                if bank_source:
                    pyxel.images[0] = src
                    src = 0
                if method.startswith("bltm"):
                    src = pyxel.Tilemap(2, 2, src)

                dst.clip(2, 2, 12, 12)
                dst.camera(1, 1)
                dst.pal(7, 8)
                dst.dither(0.5)

                if method.endswith("3d"):
                    getattr(dst, method)(
                        0, 0, 16, 16, src, (8, 8, 10), (0, 30, 0), colkey=0
                    )
                else:
                    getattr(dst, method)(
                        3,
                        3,
                        src,
                        source_pos,
                        source_pos,
                        -8,
                        8,
                        colkey=0 if source_pos == 0 else None,
                        rotate=rotate,
                    )

                dst.clip()
                dst.camera()

            assert list(expected.data_ptr()) != list(snapshot.data_ptr())
            assert list(actual.data_ptr()) == list(expected.data_ptr())
        finally:
            pyxel.images[0] = original_bank

    def test_blt_preserves_uncopied_area(self):
        src = pyxel.Image(8, 8)
        src.cls(5)
        dst = pyxel.Image(16, 16)
        dst.cls(3)
        dst.blt(0, 0, src, 0, 0, 8, 8)
        assert dst.pget(0, 0) == 5
        assert dst.pget(10, 10) == 3

    def test_blt_with_colkey(self):
        src = pyxel.Image(8, 8)
        src.cls(0)
        src.pset(1, 0, 5)
        dst = pyxel.Image(8, 8)
        dst.cls(3)
        dst.blt(0, 0, src, 0, 0, 8, 8, colkey=0)
        assert dst.pget(0, 0) == 3
        assert dst.pget(1, 0) == 5

    def test_blt_with_rotate(self):
        src = pyxel.Image(8, 8)
        src.cls(0)
        src.rect(0, 0, 8, 8, 7)
        dst = pyxel.Image(32, 32)
        dst.cls(0)
        dst.blt(8, 8, src, 0, 0, 8, 8, rotate=45)
        # (7, 11) is outside the unrotated 8x8 box and painted only when rotated
        assert dst.pget(7, 11) == 7

    def test_blt_with_scale(self):
        src = pyxel.Image(8, 8)
        src.cls(0)
        src.pset(0, 0, 7)
        dst = pyxel.Image(32, 32)
        dst.cls(0)
        dst.blt(0, 0, src, 0, 0, 1, 1, scale=4)
        # At the origin, clipping leaves four painted pixels.
        drawn = sum(1 for x in range(8) for y in range(8) if dst.pget(x, y) == 7)
        assert drawn == 4

    def test_bltm_with_int(self):
        pyxel.tilemaps[0].cls((0, 0))
        pyxel.tilemaps[0].pset(0, 0, (0, 0))
        pyxel.images[0].cls(0)
        pyxel.images[0].rect(0, 0, 8, 8, 7)
        img = pyxel.Image(64, 64)
        img.cls(0)

        img.bltm(0, 0, 0, 0, 0, 64, 64)
        assert img.pget(0, 0) == 7

    def test_bltm_with_tilemap_instance(self):
        pyxel.images[0].cls(0)
        pyxel.images[0].rect(0, 0, 8, 8, 5)
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        tm.pset(0, 0, (0, 0))
        img = pyxel.Image(64, 64)
        img.cls(0)

        img.bltm(0, 0, tm, 0, 0, 64, 64)
        assert img.pget(0, 0) == 5

    def test_blt3d(self):
        pyxel.images[0].cls(0)
        pyxel.images[0].rect(0, 0, 16, 16, 7)
        img = pyxel.Image(64, 64)
        img.cls(0)
        img.blt3d(0, 0, 64, 64, 0, (0, 0, 10), (0, 30, 0))
        assert any(img.pget(x, y) == 7 for x in range(64) for y in range(64))

    def test_bltm3d(self):
        pyxel.images[0].cls(0)
        pyxel.images[0].rect(0, 0, 8, 8, 12)
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        tm.rect(0, 0, 8, 8, (0, 0))
        img = pyxel.Image(64, 64)
        img.cls(0)

        img.bltm3d(0, 0, 64, 64, tm, (0, 0, 10), (0, 30, 0))
        assert any(img.pget(x, y) == 12 for x in range(64) for y in range(64))


class TestImageState:
    @pytest.mark.parametrize(
        ("clip_rect", "bounds"),
        [
            ((4, 4, 8, 8), (4, 4, 12, 12)),
            ((-2147483648, 0, 0, 1), (0, 0, 0, 0)),
            ((0, -2147483648, 1, 0), (0, 0, 0, 0)),
            ((1, 1, 4294967295, 4294967295), (1, 1, 16, 16)),
        ],
    )
    def test_clip_restricts_drawing(self, clip_rect, bounds):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.clip(*clip_rect)
        img.rect(0, 0, 16, 16, 7)
        img.clip()

        left, top, right, bottom = bounds
        assert list(img.data_ptr()) == [
            7 if left <= x < right and top <= y < bottom else 0
            for y in range(16)
            for x in range(16)
        ]

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
        # dither(0.5) deterministically draws exactly half of the 256 pixels.
        assert drawn == 128


class TestImageIO:
    def test_from_image(self, assets_dir):
        img = pyxel.Image.from_image(str(assets_dir / "cat_16x16.png"))
        assert img.width == 16
        assert img.height == 16

    def test_save_with_scale(self, tmp_path):
        img = pyxel.Image(8, 8)
        img.cls(0)
        img.rect(0, 0, 8, 8, 7)

        path1 = str(tmp_path / "scale1.png")
        path2 = str(tmp_path / "scale4.png")
        img.save(path1, 1)
        img.save(path2, 4)
        with PIL.Image.open(path1) as image1, PIL.Image.open(path2) as image4:
            assert image1.size == (8, 8)
            assert image4.size == (32, 32)

    def test_save_rejects_scaled_dimension_overflow_before_overwrite(
        self, tmp_path, panic_exception
    ):
        path = tmp_path / "existing.png"
        path.write_bytes(b"original destination")
        img = pyxel.Image(2, 2)
        with raises_exact(
            panic_exception, "scale is too large for the image dimensions"
        ):
            img.save(str(path), 2_147_483_649)
        assert path.read_bytes() == b"original destination"

    def test_from_image_with_include_colors(self, assets_dir):
        path = str(assets_dir / "cat_16x16.png")
        with PIL.Image.open(path) as source:
            expected_color = int.from_bytes(
                source.convert("RGB").getpixel((0, 0)), "big"
            )

        # include_colors replaces the whole global palette; restore it fully.
        original_colors = list(pyxel.colors)
        try:
            pyxel.colors[:] = [0]
            img = pyxel.Image.from_image(path, include_colors=True)
            assert img.width == 16
            assert pyxel.colors[img.pget(0, 0)] == expected_color
        finally:
            pyxel.colors[:] = original_colors

    def test_load_with_include_colors(self, assets_dir):
        path = str(assets_dir / "cat_16x16.png")
        with PIL.Image.open(path) as source:
            expected_color = int.from_bytes(
                source.convert("RGB").getpixel((0, 0)), "big"
            )

        original_colors = list(pyxel.colors)
        try:
            pyxel.colors[:] = [0]
            img = pyxel.Image(32, 32)
            img.load(0, 0, path, include_colors=True)
            assert pyxel.colors[img.pget(0, 0)] == expected_color
        finally:
            pyxel.colors[:] = original_colors

    def test_from_image_with_too_many_colors_keeps_palette(
        self, tmp_path, panic_exception
    ):
        path = tmp_path / "many_colors.png"
        file_image = PIL.Image.new("RGB", (32, 9))
        file_image.putdata([(i % 256, i // 256, 0) for i in range(32 * 9)])
        file_image.save(path)
        colors_before = list(pyxel.colors)

        with raises_exact(
            panic_exception, "Number of colors must be between 1 and 256"
        ):
            pyxel.Image.from_image(str(path), include_colors=True)
        assert list(pyxel.colors) == colors_before

    def test_incl_colors_deprecated(self, capfd, assets_dir):
        # incl_colors is the deprecated alias; warning fires only once per session,
        # so test both APIs in order.
        original_colors = list(pyxel.colors)
        try:
            pyxel.colors[:] = [0]
            img1 = pyxel.Image.from_image(
                str(assets_dir / "cat_16x16.png"),
                include_colors=False,
                incl_colors=True,  # type: ignore[call-arg]
            )
            assert img1.width == 16
            assert len(pyxel.colors) > 1
            out = capfd.readouterr().out
            assert (
                out == "incl_colors option is deprecated. Use include_colors instead.\n"
            )

            img2 = pyxel.Image(32, 32)
            pyxel.colors[:] = [0]
            img2.load(
                0,
                0,
                str(assets_dir / "cat_16x16.png"),
                include_colors=False,
                incl_colors=True,  # type: ignore[call-arg]
            )
            assert len(pyxel.colors) > 1
            has_nonzero = any(img2.pget(x, 0) != 0 for x in range(16))
            assert has_nonzero
        finally:
            pyxel.colors[:] = original_colors


class TestImageDataPtr:
    def test_data_ptr_keeps_image_alive(self):
        img = pyxel.Image(2, 2)
        ptr = img.data_ptr()
        assert ptr._pyxel_owner is img

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
        # Second row starts at offset = width.
        assert ptr[8] == 9
