import pyxel

HUMAN_IMAGE = (368, 8, 16, 16)
CAR_IMAGES = [
    (240, 272, 32, 24),
    (240, 240, 32, 24),
    (336, 264, 32, 16),
    (336, 280, 32, 16),
    (288, 240, 32, 24),
]


def is_colliding(x, y):
    x1 = pyxel.floor(x) // 8
    y1 = pyxel.floor(y) // 8
    x2 = (pyxel.ceil(x) + 15) // 8
    y2 = (pyxel.ceil(y) + 15) // 8

    for yi in range(y1, y2 + 1):
        for xi in range(x1, x2 + 1):
            if pyxel.tilemaps[2].pget(xi, yi) == (2, 0):
                return True

    return False


def push_back(x, y, dx, dy):
    for _ in range(pyxel.ceil(abs(dy))):
        step = max(-1, min(1, dy))
        if is_colliding(x, y + step):
            break
        y += step
        dy -= step

    for _ in range(pyxel.ceil(abs(dx))):
        step = max(-1, min(1, dx))
        if is_colliding(x + step, y):
            break
        x += step
        dx -= step

    return x, y


class App:
    def __init__(self):
        pyxel.init(464, 256, title="Tiled Map File")

        pyxel.images[0] = pyxel.Image.from_image(
            "assets/urban_rpg.png", incl_colors=True
        )
        # This example loads a PNG file as the tileset image, but you can also use
        # Pyxel's image bank as the tileset image in the usual way. Images from the
        # image bank can be saved to the desktop with the shortcut
        # Shift+Alt(Option)+1/2/3, and loading those images into Tiled allows them to
        # be used as a tileset image.

        for i in range(3):
            pyxel.tilemaps[i] = pyxel.Tilemap.from_tmx("assets/urban_rpg.tmx", i)
        # Tiled Map Files (.tmx) can be created with Tiled, a 2D level editor. In Tiled,
        # the tile layout format must be in CSV format, and the tileset image needs to
        # be of the `Embed in map` type, and the tile size must be 8x8.

        self.player = (160, 80, 1, 0)  # (x, y, u, v)
        self.cars = [  # (x, y, dx, image)
            (128, 104, -2, 0),
            (288, 104, -2, 1),
            (416, 112, -2, 2),
            (32, 144, 2, 3),
            (64, 136, 2, 4),
            (96, 136, 2, 4),
        ]

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        # Update player
        x, y, u, v = self.player
        dx, dy = 0, 0

        if pyxel.btn(pyxel.KEY_UP):
            dy = -1
            u, v = 2, 1
        if pyxel.btn(pyxel.KEY_DOWN):
            dy = 1
            u, v = 1, 1
        if pyxel.btn(pyxel.KEY_LEFT):
            dx = -1
            u, v = 0, 1
        if pyxel.btn(pyxel.KEY_RIGHT):
            dx = 1
            u, v = 3, 1

        if v == 1:
            v += [-1, 0, -1, 1][pyxel.frame_count // 5 % 4]

        x, y = push_back(x, y, dx, dy)
        x = min(max(x, 0), pyxel.width - 16)
        y = min(max(y, 0), pyxel.height - 16)
        self.player = (x, y, u, v)

        # Update cars
        for i, car in enumerate(self.cars):
            x, y, dx, image = car
            x += dx

            if x <= -32:
                x = pyxel.tilemaps[0].width * 8
            elif x >= pyxel.tilemaps[0].width * 8:
                x = -32

            self.cars[i] = (x, y, dx, image)

    def draw(self):
        pyxel.cls(1)
        pyxel.bltm(0, 0, 0, 0, 0, pyxel.width, pyxel.height, 0)

        # Draw player
        x, y, u, v = self.player
        pyxel.blt(
            x,
            y - 1,
            0,
            HUMAN_IMAGE[0] + u * 16,
            HUMAN_IMAGE[1] + v * 16,
            HUMAN_IMAGE[2],
            HUMAN_IMAGE[3],
            0,
        )

        # Draw cars
        for car in self.cars:
            x, y, _, image = car
            u, v, w, h = CAR_IMAGES[image]
            pyxel.blt(x, y, 0, u, v, w, h, 0)

        pyxel.bltm(0, 0, 1, 0, 0, pyxel.width, pyxel.height, 0)


App()
