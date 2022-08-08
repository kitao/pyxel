import unittest

import pyxel


class TestPyxel(unittest.TestCase):
    def setUpClass():
        pyxel.init(300, 300, "hoge")

    def test_title(self):
        pyxel.title("hoge")

    def test_colors(self):
        self.assertEqual(pyxel.colors[0], 0)

        pyxel.colors[0] = 0x112233
        self.assertEqual(pyxel.colors[0], 0x112233)

    def test_cls(self):
        pyxel.cls(3)

    def test_input_text(self):
        pyxel.input_text
