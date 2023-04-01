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
        self.font_bounding_box = [0, 0, 0, 0]
        self.fonts = self._parse_bdf(bdf_filename)
        self.screen_ptr = pyxel.screen.data_ptr()
        self.screen_width = pyxel.width

    def _parse_bdf(self, bdf_filename):
        fonts = {}
        code = None
        bitmap = None
        dwidth = 0
        with open(bdf_filename, "r") as f:
            for line in f:
                if line.startswith("FONTBOUNDINGBOX"):
                    self.font_bounding_box = list(map(int, line.split()[1:]))
                elif line.startswith("ENCODING"):
                    code = int(line.split()[1])
                elif line.startswith("DWIDTH"):
                    dwidth = int(line.split()[1])
                elif line.startswith("BBX"):
                    bbx = tuple(map(int, line.split()[1:]))
                elif line.startswith("BITMAP"):
                    bitmap = []
                elif line.startswith("ENDCHAR"):
                    fonts[code] = (
                        dwidth,
                        bbx,
                        bitmap,
                    )
                    bitmap = None
                elif bitmap is not None:
                    hex_string = line.strip()
                    bin_string = bin(int(hex_string, 16))[2:].zfill(len(hex_string) * 4)
                    bitmap.append(int(bin_string[::-1], 2))
        return fonts

    def _draw_font(self, x, y, font, color):
        dwidth, bbx, bitmap = font
        screen_ptr = self.screen_ptr
        screen_width = self.screen_width
        x = x + self.font_bounding_box[2] + bbx[2]
        y = y + self.font_bounding_box[1] + self.font_bounding_box[3] - bbx[1] - bbx[3]
        for j in range(bbx[1]):
            for i in range(bbx[0]):
                if (bitmap[j] >> i) & 1:
                    screen_ptr[(y + j) * screen_width + x + i] = color

    def draw_text(self, x, y, text, color=7, border_color=None, spacing=0):
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
            x += font[0] + spacing


pyxel.init(128, 128, title="Bitmap Font")
pyxel.load("assets/sample.pyxres")
umplus10 = BDFRenderer("assets/umplus_j10r.bdf")
umplus12 = BDFRenderer("assets/umplus_j12r.bdf")

pyxel.cls(1)
pyxel.blt(0, 0, 1, 0, 0, 128, 128)
umplus10.draw_text(24, 8, "Pyxel♪", 8)
umplus12.draw_text(4, 98, "気軽に楽しく", 7, 5)
umplus12.draw_text(4, 113, "プログラミング！", 7, 5)
pyxel.show()
