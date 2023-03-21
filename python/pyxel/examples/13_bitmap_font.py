import pyxel


class BDFRenderer:
    BORDER_DIRECTIONS = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]

    def __init__(self, bdf_filename):
        self.fonts = self._parse_bdf(bdf_filename)
        self.screen_ptr = pyxel.screen.data_ptr()
        self.screen_width = pyxel.width

    def _parse_bdf(self, bdf_filename):
        fonts = {}
        code = None
        bitmap = None
        with open(bdf_filename, "r") as f:
            for line in f:
                if line.startswith("ENCODING"):
                    code = int(line.split()[1])
                elif line.startswith("BBX"):
                    bbx_data = list(map(int, line.split()[1:]))
                    font_width, font_height = bbx_data[0], bbx_data[1]
                elif line.startswith("BITMAP"):
                    bitmap = []
                elif line.startswith("ENDCHAR"):
                    fonts[code] = (font_width, font_height, bitmap)
                    bitmap = None
                elif bitmap is not None:
                    hex_string = line.strip()
                    bin_string = bin(int(hex_string, 16))[2:].zfill(len(hex_string) * 4)
                    bitmap.append(int(bin_string[::-1], 2))
        return fonts

    def _draw_font(self, x, y, font, color):
        font_width, font_height, bitmap = font
        screen_ptr = self.screen_ptr
        screen_width = self.screen_width
        for j in range(font_height):
            for i in range(font_width):
                if (bitmap[j] >> i) & 1:
                    screen_ptr[(y + j) * screen_width + x + i] = color

    def draw_text(self, x, y, text, color=7, border_color=None):
        for char in text:
            code = ord(char)
            if code not in self.fonts:
                continue
            font = self.fonts[code]
            if border_color is not None:
                for dx, dy in self.BORDER_DIRECTIONS:
                    self._draw_font(
                        x + dx,
                        y + dy,
                        font,
                        border_color,
                    )
            self._draw_font(x, y, font, color)
            x += font[0] + 1


pyxel.init(128, 128, title="Bitmap Font")
pyxel.load("assets/sample.pyxres")
bdf1 = BDFRenderer("assets/umplus_j10r.bdf")
bdf2 = BDFRenderer("assets/umplus_j12r.bdf")

pyxel.cls(1)
pyxel.blt(0, 0, 1, 0, 0, 128, 128)
bdf1.draw_text(26, 7, "Pyxel!︎", 1)
bdf2.draw_text(4, 97, "気軽に楽しく", 14, 0)
bdf2.draw_text(4, 112, "プログラミング！", 14, 0)
pyxel.show()
