import pytest
import pyxel
from _assertions import raises_exact  # type: ignore[reportMissingImports]


class TestFont:
    def test_bdf_read_error(self, tmp_path):
        path = tmp_path / "invalid.bdf"
        path.write_bytes(b"STARTFONT 2.1\n\xff\n")
        with raises_exact(Exception, f"Failed to read file '{path}'"):
            pyxel.Font(str(path))

    @pytest.mark.parametrize("custom_font", [False, True])
    def test_text_camera_offset_does_not_wrap(self, tmp_path, custom_font):
        path = tmp_path / "small.bdf"
        path.write_text(
            "FONTBOUNDINGBOX 4 1 0 0\nSTARTCHAR A\nENCODING 65\n"
            "DWIDTH 4 0\nBBX 4 1 0 0\nBITMAP\nF0\nENDCHAR\n"
        )
        font = pyxel.Font(str(path)) if custom_font else None

        img = pyxel.Image(8, 8)
        img.camera(-2_147_483_648, 0)
        img.text(2_147_483_520, 0, "A" * 40, 7, font)
        assert list(img.data_ptr()) == [0] * 64

    def test_bdf_offsets_cancel_wide_camera_position(self, tmp_path):
        path = tmp_path / "offset.bdf"
        path.write_text(
            "FONTBOUNDINGBOX 4 1 -2000000000 0\nSTARTCHAR A\nENCODING 65\n"
            "DWIDTH 4 0\nBBX 4 1 -2000000000 0\nBITMAP\nF0\nENDCHAR\n"
        )

        img = pyxel.Image(8, 2)
        img.camera(-2_000_000_000, 0)
        img.text(2_000_000_000, 0, "A", 7, pyxel.Font(str(path)))
        assert list(img.data_ptr()) == [7] * 4 + [0] * 12

    @pytest.mark.parametrize(
        "text, expected", [("AA", 4_000_000_000), ("AA\nA", 4_000_000_000), ("AB", 0)]
    )
    def test_text_width_preserves_wide_line_advances(self, tmp_path, text, expected):
        path = tmp_path / "wide.bdf"
        path.write_text(
            "FONTBOUNDINGBOX 4 1 0 0\nSTARTCHAR A\nENCODING 65\n"
            "DWIDTH 2000000000 0\nBBX 4 1 0 0\nBITMAP\nF0\nENDCHAR\n"
            "STARTCHAR B\nENCODING 66\nDWIDTH -2000000000 0\n"
            "BBX 4 1 0 0\nBITMAP\nF0\nENDCHAR\n"
        )

        font = pyxel.Font(str(path))
        assert font.text_width(text) == expected

    def test_ttf(self, assets_dir):
        font = pyxel.Font(str(assets_dir / "PixelMplus10-Regular.ttf"), 10)
        assert font.text_width("A") > 0

    def test_ttf_different_sizes(self, assets_dir):
        font_small = pyxel.Font(str(assets_dir / "PixelMplus10-Regular.ttf"), 8)
        font_large = pyxel.Font(str(assets_dir / "PixelMplus10-Regular.ttf"), 20)
        assert font_large.text_width("A") > font_small.text_width("A")

    def test_text_width_empty(self, assets_dir):
        font = pyxel.Font(str(assets_dir / "umplus_j10r.bdf"))
        assert font.text_width("") == 0

    def test_text_width_multibyte(self, assets_dir):
        font = pyxel.Font(str(assets_dir / "umplus_j10r.bdf"))
        width = font.text_width("あ")
        assert width > 0

    def test_text_width_multiline(self, assets_dir):
        font = pyxel.Font(str(assets_dir / "umplus_j10r.bdf"))
        w_single = font.text_width("AB")
        w_multi = font.text_width("AB\nA")
        # Multiline returns max line width
        assert w_multi == w_single

    def test_text_width_invisible_chars_skipped(self, assets_dir):
        font = pyxel.Font(str(assets_dir / "umplus_j10r.bdf"))
        w_plain = font.text_width("Hi")
        w_zwj = font.text_width("H\u200di")  # ZWJ
        w_vs = font.text_width("H\ufe0fi")  # Variation selector
        assert w_plain == w_zwj
        assert w_plain == w_vs

    def test_text_width_fixed_width_latin(self, assets_dir):
        font = pyxel.Font(str(assets_dir / "umplus_j10r.bdf"))
        w1 = font.text_width("A")
        w2 = font.text_width("AA")
        # This BDF has fixed-width Latin glyphs.
        assert w1 > 0
        assert w2 == w1 * 2
