import unittest

import pyxel


class TestPyxel(unittest.TestCase):
    def setUpClass():
        pyxel.init(300, 300, title="hoge")

    def test_title(self):
        pyxel.title("hoge")

    def test_colors(self):
        default_colors = [
            0x000000,
            0x2B335F,
            0x7E2072,
            0x19959C,
            0x8B4852,
            0x395C98,
            0xA9C1FF,
            0xEEEEEE,
            0xD4186C,
            0xD38441,
            0xE9C35B,
            0x70C6A9,
            0x7696DE,
            0xA3A3A3,
            0xFF9798,
            0xEDC7B0,
        ]
        self.assertEqual(pyxel.colors.to_list(), default_colors)

        reduced_colors = [0x111111, 0x222222, 0x333333, 0x444444]
        pyxel.colors.from_list(reduced_colors)
        self.assertEqual(pyxel.colors.to_list(), reduced_colors)

        expanded_colors = default_colors[:] + [0xAAAAAA, 0xBBBBBB, 0xCCCCCC]
        pyxel.colors.from_list(expanded_colors)
        self.assertEqual(pyxel.colors.to_list(), expanded_colors)

        self.assertEqual(pyxel.colors[0], 0x000000)
        pyxel.colors[0] = 0x112233
        self.assertEqual(pyxel.colors[0], 0x112233)

    def test_cls(self):
        pyxel.cls(3)

    def test_input_text(self):
        pyxel.input_text
