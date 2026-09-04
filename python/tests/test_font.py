import pyxel
from _assertions import raises_exact  # type: ignore[reportMissingImports]


class TestFont:
    def test_bdf_read_error(self, tmp_path):
        path = tmp_path / "invalid.bdf"
        path.write_bytes(b"STARTFONT 2.1\n\xff\n")
        with raises_exact(Exception, f"Failed to read file '{path}'"):
            pyxel.Font(str(path))

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
