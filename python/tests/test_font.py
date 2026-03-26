import os

import pyxel


# Font class
class TestFont:
    def test_bdf(self, assets_dir):
        font = pyxel.Font(os.path.join(assets_dir, "umplus_j10r.bdf"))
        assert font.text_width("A") > 0

    def test_ttf(self, assets_dir):
        font = pyxel.Font(os.path.join(assets_dir, "PixelMplus10-Regular.ttf"), 10)
        assert font.text_width("A") > 0

    def test_ttf_different_sizes(self, assets_dir):
        font_small = pyxel.Font(os.path.join(assets_dir, "PixelMplus10-Regular.ttf"), 8)
        font_large = pyxel.Font(
            os.path.join(assets_dir, "PixelMplus10-Regular.ttf"), 20
        )
        assert font_large.text_width("A") > font_small.text_width("A")

    def test_text_width_empty(self, assets_dir):
        font = pyxel.Font(os.path.join(assets_dir, "umplus_j10r.bdf"))
        assert font.text_width("") == 0

    def test_text_width_multibyte(self, assets_dir):
        font = pyxel.Font(os.path.join(assets_dir, "umplus_j10r.bdf"))
        width = font.text_width("あ")
        assert width > 0

    def test_text_width_multiline(self, assets_dir):
        font = pyxel.Font(os.path.join(assets_dir, "umplus_j10r.bdf"))
        w_single = font.text_width("AB")
        w_multi = font.text_width("AB\nA")
        # Multiline returns max line width
        assert w_multi == w_single

    def test_text_width_invisible_chars_skipped(self, assets_dir):
        font = pyxel.Font(os.path.join(assets_dir, "umplus_j10r.bdf"))
        w_plain = font.text_width("Hi")
        w_zwj = font.text_width("H\u200di")  # ZWJ
        w_vs = font.text_width("H\ufe0fi")  # Variation selector
        assert w_plain == w_zwj
        assert w_plain == w_vs

    def test_text_width_longer_string(self, assets_dir):
        font = pyxel.Font(os.path.join(assets_dir, "umplus_j10r.bdf"))
        w1 = font.text_width("A")
        w3 = font.text_width("AAA")
        assert w3 > w1

    def test_text_width_proportional(self, assets_dir):
        font = pyxel.Font(os.path.join(assets_dir, "umplus_j10r.bdf"))
        w1 = font.text_width("A")
        w2 = font.text_width("AA")
        # Two chars should be roughly double one char
        assert w2 == w1 * 2

    def test_bdf_renders_text(self, assets_dir):
        font = pyxel.Font(os.path.join(assets_dir, "umplus_j10r.bdf"))
        img = pyxel.Image(64, 32)
        img.cls(0)
        img.text(0, 0, "A", 7, font)
        has_text = any(img.pget(x, y) == 7 for x in range(20) for y in range(20))
        assert has_text
