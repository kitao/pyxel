import unittest

import pyxel


class TestPyxel(unittest.TestCase):
    def setUpClass():
        pyxel.init(300, 300, "hoge")

    def test_title(self):
        pyxel.title("hoge")

    def test_palette(self):
        self.assertEqual(pyxel.palette[3], 3)

        pyxel.palette[3] = 5
        self.assertEqual(pyxel.palette[3], 5)

    def test_cls(self):
        pyxel.cls(3)

    def test_text_input(self):
        pyxel.text_input
