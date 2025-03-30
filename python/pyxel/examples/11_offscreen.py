import pyxel

BORDER_OFFSETS = [(-1, 0), (1, 0), (0, -1), (0, 1)]
BG_COLOR = 5


def pset_with_border(image, x, y, col, bcol):
    for x_offset, y_offset in BORDER_OFFSETS:
        image.pset(x + x_offset, y + y_offset, bcol)

    image.pset(x, y, col)


def line_with_border(image, x1, y1, x2, y2, col, bcol):
    for x_offset, y_offset in BORDER_OFFSETS:
        image.line(
            x1 + x_offset,
            y1 + y_offset,
            x2 + x_offset,
            y2 + y_offset,
            bcol,
        )

    image.line(x1, y1, x2, y2, col)


def text_with_border(image, x, y, s, col, bcol):
    for x_offset, y_offset in BORDER_OFFSETS:
        image.text(x + x_offset, y + y_offset, s, bcol)

    image.text(x, y, s, col)


def scale_image(image, scale):
    scaled_width = image.width * scale
    scaled_height = image.height * scale
    scaled_image = pyxel.Image(scaled_width, scaled_height)

    for y in range(scaled_height):
        for x in range(scaled_width):
            color = image.pget(x // scale, y // scale)
            scaled_image.pset(x, y, color)

    return scaled_image


def make_blt_figure():
    figure = pyxel.Image(pyxel.width, pyxel.height)
    figure.cls(BG_COLOR)

    image1 = pyxel.Image(32, 24)
    image1.blt(0, 0, 0, 0, 0, 32, 24)
    image1 = scale_image(image1, 3)

    image2 = pyxel.Image(32, 24)
    image2.blt(0, 0, 0, 0, 32, 32, 24)
    image2 = scale_image(image2, 3)

    col = 7
    bcol = 0

    def draw_w_and_h(x, y):
        pset_with_border(figure, x + 47, y + 23, col, bcol)
        line_with_border(figure, x + 47, y + 47, x + 70, y + 47, col, bcol)
        text_with_border(figure, x + 58, y + 44, "w", col, bcol)
        line_with_border(figure, x + 71, y + 23, x + 71, y + 46, col, bcol)
        text_with_border(figure, x + 70, y + 33, "h", col, bcol)

    x = 10
    y = 12
    figure.blt(x, y, image1, 0, 0, image1.width, image1.height)
    text_with_border(figure, x + 1, y - 7, "Screen", col, bcol)
    text_with_border(figure, x + 38, y + 16, "(x,y)", col, bcol)
    draw_w_and_h(x, y)

    x = 116
    figure.blt(x, y, image2, 0, 0, image2.width, image2.height)
    text_with_border(figure, x + 1, y - 7, "Image Bank", col, bcol)
    text_with_border(figure, x + 38, y + 16, "(u,v)", col, bcol)
    draw_w_and_h(x, y)

    return figure


def make_bltm_figure():
    figure = pyxel.Image(pyxel.width, pyxel.height)
    figure.cls(BG_COLOR)

    image1 = pyxel.Image(32, 24)
    image1.blt(0, 0, 0, 0, 64, 32, 24)
    image1 = scale_image(image1, 3)

    image2 = pyxel.Image(32, 24)
    image2.blt(0, 0, 0, 0, 96, 32, 24)
    image2 = scale_image(image2, 3)

    x = 10
    y = 12
    col = 7
    bcol = 0
    text_with_border(figure, x + 1, y - 7, "Tilemap", col, bcol)
    figure.blt(x, y, image1, 0, 0, image1.width, image1.height)

    col = 8
    bcol = 7
    text_with_border(figure, x + 3, y + 10, "(0,0) (0,0) (1,0) (0,2)", col, bcol)
    text_with_border(figure, x + 3, y + 34, "(3,2) (0,0) (0,0) (1,2)", col, bcol)
    text_with_border(figure, x + 3, y + 58, "(3,2) (2,2) (0,0) (0,2)", col, bcol)

    x = 116
    col = 7
    bcol = 0
    text_with_border(figure, x + 1, y - 7, "Image Bank (imgsrc)", col, bcol)
    figure.blt(x, y, image2, 0, 0, image2.width, image2.height)

    return figure


class App:
    def __init__(self):
        pyxel.init(223, 92, title="Offscreen Rendering")
        pyxel.load("assets/offscreen.pyxres")

        self.blt_figure = make_blt_figure()
        self.bltm_figure = make_bltm_figure()

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

    def draw(self):
        figure = (
            self.blt_figure if (pyxel.frame_count // 120) % 2 == 0 else self.bltm_figure
        )
        pyxel.blt(0, 0, figure, 0, 0, figure.width, figure.height)


App()
