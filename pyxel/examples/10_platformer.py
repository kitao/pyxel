import math

import pyxel

CHARA_WIDTH = 8
CHARA_HEIGHT = 8
SCROLL_BORDER_X = 80

BRICK_TILE_X = 4

TILE_SPACE = (0, 0)
TILE_FLOOR = (1, 0)
TILE_SPAWN1 = (0, 1)
TILE_SPAWN2 = (1, 1)
TILE_SPAWN3 = (2, 1)

enemy_list = []
scroll_x = 0
player = None


def get_tile(x, y):
    return pyxel.tilemap(0).pget(x, y)


def detect_collision(x, y, dy):
    x1 = x // 8
    y1 = y // 8
    x2 = (x + CHARA_WIDTH - 1) // 8
    y2 = (y + CHARA_HEIGHT - 1) // 8

    for i in range(y1, y2 + 1):
        for j in range(x1, x2 + 1):
            if get_tile(j, i)[0] >= BRICK_TILE_X:
                return True

    if dy > 0 and y % 8 == 1:
        for i in range(x1, x2 + 1):
            if get_tile(i, y1 + 1) == TILE_FLOOR:
                return True

    return False


def react_on_collision(x, y, dx, dy):
    abs_dx = abs(dx)
    abs_dy = abs(dy)

    if abs_dx > abs_dy:
        sign = 1 if dx > 0 else -1
        for i in range(abs_dx):
            if detect_collision(x + sign, y, dy):
                break
            x += sign

        sign = 1 if dy > 0 else -1
        for i in range(abs_dy):
            if detect_collision(x, y + sign, dy):
                break
            y += sign
    else:
        sign = 1 if dy > 0 else -1
        for i in range(abs_dy):
            if detect_collision(x, y + sign, dy):
                break
            y += sign

        sign = 1 if dx > 0 else -1
        for i in range(abs_dx):
            if detect_collision(x + sign, y, dy):
                break
            x += sign

    return x, y, dx, dy


def is_impassable(x, y):
    tile = get_tile(x // 8, y // 8)
    return tile == TILE_FLOOR or tile[0] >= BRICK_TILE_X


def spawn_enemy(scroll_left, scroll_right):
    scroll_left = math.ceil(scroll_left / 8)
    scroll_right = math.floor(scroll_right / 8)

    for x in range(scroll_left, scroll_right + 1):
        for y in range(16):
            val = get_tile(x, y)
            if val == TILE_SPAWN1:
                enemy_list.append(Enemy1(x * 8, y * 8))
            elif val == TILE_SPAWN2:
                enemy_list.append(Enemy2(x * 8, y * 8))
            elif val == TILE_SPAWN3:
                enemy_list.append(Enemy3(x * 8, y * 8))


def cleanup_list(list):
    i = 0
    while i < len(list):
        elem = list[i]
        if not elem.alive:
            list.pop(i)
        else:
            i += 1


class Player:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.dx = 0
        self.dy = 0
        self.direction = 1
        self.is_falling = False

    def update(self):
        global scroll_x

        if pyxel.btn(pyxel.KEY_LEFT):
            self.dx = -2
            self.direction = -1

        if pyxel.btn(pyxel.KEY_RIGHT):
            self.dx = 2
            self.direction = 1

        self.dy = min(self.dy + 1, 3)

        if pyxel.btnp(pyxel.KEY_SPACE):
            self.dy = -6

        last_y = self.y
        self.x, self.y, self.dx, self.dy = react_on_collision(
            self.x, self.y, self.dx, self.dy
        )
        self.is_falling = self.y > last_y

        if self.x < scroll_x:
            self.x = scroll_x

        if self.y < 0:
            self.y = 0

        self.dx = int(self.dx * 0.8)

        if self.x > scroll_x + SCROLL_BORDER_X:
            last_scroll_x = scroll_x
            scroll_x = min(self.x - SCROLL_BORDER_X, 240 * 8)

            spawn_enemy(last_scroll_x + 128, scroll_x + 127)

    def draw(self):
        u = (2 if self.is_falling else pyxel.frame_count // 3 % 2) * 8
        w = 8 if self.direction > 0 else -8
        pyxel.blt(self.x, self.y, 0, u, 16, w, 8, 13)


class Enemy1:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.dx = 0
        self.dy = 0
        self.direction = 1
        self.is_falling = True
        self.alive = True

    def update(self):
        self.dx = self.direction
        self.dy = min(self.dy + 1, 3)

        if is_impassable(self.x, self.y + 8) or is_impassable(self.x + 7, self.y + 8):
            if self.is_falling:
                self.is_falling = False
                if player.x < self.x:
                    self.direction = -1
                else:
                    self.direction = 1
            elif self.direction < 0 and is_impassable(self.x - 1, self.y + 4):
                self.direction = 1
            elif self.direction > 0 and is_impassable(self.x + 8, self.y + 4):
                self.direction = -1
        else:
            self.is_falling = True

        self.x, self.y, self.dx, self.dy = react_on_collision(
            self.x, self.y, self.dx, self.dy
        )

        if self.y > 128:
            self.x = 64
            self.y = -8

    def draw(self):
        u = pyxel.frame_count // 4 % 2 * 8
        w = 8 if self.direction > 0 else -8
        pyxel.blt(self.x, self.y, 0, u, 24, w, 8, 13)


class Enemy2:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.dx = 0
        self.dy = 0
        self.direction = 1
        self.alive = True

    def update(self):
        self.dx = self.direction
        self.dy = min(self.dy + 1, 3)

        if is_impassable(self.x, self.y + 8) or is_impassable(self.x + 7, self.y + 8):
            if self.direction < 0 and (
                is_impassable(self.x - 1, self.y + 4)
                or not is_impassable(self.x - 1, self.y + 8)
            ):
                self.direction = 1
            elif self.direction > 0 and (
                is_impassable(self.x + 8, self.y + 4)
                or not is_impassable(self.x + 7, self.y + 8)
            ):
                self.direction = -1

        self.x, self.y, self.dx, self.dy = react_on_collision(
            self.x, self.y, self.dx, self.dy
        )

    def draw(self):
        u = pyxel.frame_count // 4 % 2 * 8 + 16
        w = 8 if self.direction > 0 else -8
        pyxel.blt(self.x, self.y, 0, u, 24, w, 8, 13)


class Enemy3:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.rest_time = 0
        self.alive = True

    def update(self):
        if self.rest_time > 0:
            self.rest_time -= 1

        if self.rest_time == 0:
            dx = player.x - self.x
            dy = player.y - self.y
            sq_dist = dx * dx + dy * dy

            if sq_dist < 60 * 60 and sq_dist > 0:
                dist = math.sqrt(sq_dist)
                enemy_list.append(Enemy3Bullet(self.x, self.y, dx / dist, dy / dist))
                self.rest_time = 60

    def draw(self):
        u = pyxel.frame_count // 8 % 2 * 8
        pyxel.blt(self.x, self.y, 0, u, 32, 8, 8, 13)


class Enemy3Bullet:
    def __init__(self, x, y, dx, dy):
        self.x = x
        self.y = y
        self.dx = dx
        self.dy = dy
        self.alive = True

    def update(self):
        self.x += self.dx
        self.y += self.dy

    def draw(self):
        u = pyxel.frame_count // 2 % 2 * 8 + 16
        pyxel.blt(self.x, self.y, 0, u, 32, 8, 8, 13)


class App:
    def __init__(self):
        pyxel.init(128, 128, title="Pyxel Platformer")

        pyxel.load("assets/platformer.pyxres")

        # make enemy spawn images invisible
        pyxel.image(0).rect(0, 8, 24, 8, 13)

        global player
        player = Player(0, 0)

        spawn_enemy(0, 127)

        pyxel.run(self.update, self.draw)

    def update(self):
        player.update()

        for enemy in enemy_list:
            if abs(player.x - enemy.x) < 6 and abs(player.y - enemy.y) < 6:
                game_over()
                return

            enemy.update()

            if enemy.x < scroll_x - 8 or enemy.x > scroll_x + 160 or enemy.y > 160:
                enemy.alive = False

        cleanup_list(enemy_list)

    def draw(self):
        pyxel.cls(0)

        pyxel.camera()
        pyxel.bltm(0, 0, 0, (scroll_x // 4) % 128, 128, 128, 128)
        pyxel.bltm(0, 0, 0, scroll_x, 0, 128, 128, 13)

        pyxel.camera(scroll_x, 0)
        player.draw()
        for enemy in enemy_list:
            enemy.draw()


def game_over():
    global scroll_x, enemy_list

    scroll_x = 0
    player.x = 0
    player.y = 0
    player.dx = 0
    player.dy = 0

    enemy_list = []
    spawn_enemy(0, 127)


App()
