from random import random

import pyxel

SCENE_TITLE = 0
SCENE_PLAY = 1
SCENE_GAMEOVER = 2

NUM_STARS = 100
STAR_COLOR_HIGH = 12
STAR_COLOR_LOW = 5

PLAYER_WIDTH = 8
PLAYER_HEIGHT = 8
PLAYER_SPEED = 2

BULLET_WIDTH = 2
BULLET_HEIGHT = 8
BULLET_COLOR = 11
BULLET_SPEED = 4

ENEMY_WIDTH = 8
ENEMY_HEIGHT = 8
ENEMY_SPEED = 1.5

BLAST_START_RADIUS = 1
BLAST_END_RADIUS = 8
BLAST_COLOR_IN = 7
BLAST_COLOR_OUT = 10

enemies = []
bullets = []
blasts = []


def update_list(list):
    for elem in list:
        elem.update()


def draw_list(list):
    for elem in list:
        elem.draw()


def cleanup_list(list):
    i = 0
    while i < len(list):
        elem = list[i]
        if not elem.is_alive:
            list.pop(i)
        else:
            i += 1


class Background:
    def __init__(self):
        self.stars = []
        for i in range(NUM_STARS):
            self.stars.append(
                (random() * pyxel.width, random() * pyxel.height, random() * 1.5 + 1)
            )

    def update(self):
        for i, (x, y, speed) in enumerate(self.stars):
            y += speed
            if y >= pyxel.height:
                y -= pyxel.height
            self.stars[i] = (x, y, speed)

    def draw(self):
        for (x, y, speed) in self.stars:
            pyxel.pset(x, y, STAR_COLOR_HIGH if speed > 1.8 else STAR_COLOR_LOW)


class Player:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.w = PLAYER_WIDTH
        self.h = PLAYER_HEIGHT
        self.is_alive = True

    def update(self):
        if pyxel.btn(pyxel.KEY_LEFT):
            self.x -= PLAYER_SPEED
        if pyxel.btn(pyxel.KEY_RIGHT):
            self.x += PLAYER_SPEED
        if pyxel.btn(pyxel.KEY_UP):
            self.y -= PLAYER_SPEED
        if pyxel.btn(pyxel.KEY_DOWN):
            self.y += PLAYER_SPEED
        self.x = max(self.x, 0)
        self.x = min(self.x, pyxel.width - self.w)
        self.y = max(self.y, 0)
        self.y = min(self.y, pyxel.height - self.h)

        if pyxel.btnp(pyxel.KEY_SPACE):
            Bullet(
                self.x + (PLAYER_WIDTH - BULLET_WIDTH) / 2, self.y - BULLET_HEIGHT / 2
            )
            pyxel.play(0, 0)

    def draw(self):
        pyxel.blt(self.x, self.y, 0, 0, 0, self.w, self.h, 0)


class Bullet:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.w = BULLET_WIDTH
        self.h = BULLET_HEIGHT
        self.is_alive = True
        bullets.append(self)

    def update(self):
        self.y -= BULLET_SPEED
        if self.y + self.h - 1 < 0:
            self.is_alive = False

    def draw(self):
        pyxel.rect(self.x, self.y, self.w, self.h, BULLET_COLOR)


class Enemy:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.w = ENEMY_WIDTH
        self.h = ENEMY_HEIGHT
        self.dir = 1
        self.timer_offset = int(random() * 60)
        self.is_alive = True
        enemies.append(self)

    def update(self):
        if (pyxel.frame_count + self.timer_offset) % 60 < 30:
            self.x += ENEMY_SPEED
            self.dir = 1
        else:
            self.x -= ENEMY_SPEED
            self.dir = -1
        self.y += ENEMY_SPEED
        if self.y > pyxel.height - 1:
            self.is_alive = False

    def draw(self):
        pyxel.blt(self.x, self.y, 0, 8, 0, self.w * self.dir, self.h, 0)


class Blast:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.radius = BLAST_START_RADIUS
        self.is_alive = True
        blasts.append(self)

    def update(self):
        self.radius += 1
        if self.radius > BLAST_END_RADIUS:
            self.is_alive = False

    def draw(self):
        pyxel.circ(self.x, self.y, self.radius, BLAST_COLOR_IN)
        pyxel.circb(self.x, self.y, self.radius, BLAST_COLOR_OUT)


class App:
    def __init__(self):
        pyxel.init(120, 160, title="Pyxel Shooter")
        pyxel.image(0).set(
            0,
            0,
            [
                "00c00c00",
                "0c7007c0",
                "0c7007c0",
                "c703b07c",
                "77033077",
                "785cc587",
                "85c77c58",
                "0c0880c0",
            ],
        )
        pyxel.image(0).set(
            8,
            0,
            [
                "00088000",
                "00ee1200",
                "08e2b180",
                "02882820",
                "00222200",
                "00012280",
                "08208008",
                "80008000",
            ],
        )
        pyxel.sound(0).set("a3a2c1a1", "p", "7", "s", 5)
        pyxel.sound(1).set("a3a2c2c2", "n", "7742", "s", 10)
        self.scene = SCENE_TITLE
        self.score = 0
        self.background = Background()
        self.player = Player(pyxel.width / 2, pyxel.height - 20)
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btn(pyxel.KEY_Q):
            pyxel.quit()

        self.background.update()
        if self.scene == SCENE_TITLE:
            self.update_title_scene()
        elif self.scene == SCENE_PLAY:
            self.update_play_scene()
        elif self.scene == SCENE_GAMEOVER:
            self.update_gameover_scene()

    def update_title_scene(self):
        if pyxel.btnp(pyxel.KEY_RETURN):
            self.scene = SCENE_PLAY

    def update_play_scene(self):
        if pyxel.frame_count % 6 == 0:
            Enemy(random() * (pyxel.width - PLAYER_WIDTH), 0)

        for enemy in enemies:
            for bullet in bullets:
                if (
                    enemy.x + enemy.w > bullet.x
                    and bullet.x + bullet.w > enemy.x
                    and enemy.y + enemy.h > bullet.y
                    and bullet.y + bullet.h > enemy.y
                ):
                    enemy.is_alive = False
                    bullet.is_alive = False
                    blasts.append(
                        Blast(enemy.x + ENEMY_WIDTH / 2, enemy.y + ENEMY_HEIGHT / 2)
                    )
                    pyxel.play(1, 1)
                    self.score += 10

        for enemy in enemies:
            if (
                self.player.x + self.player.w > enemy.x
                and enemy.x + enemy.w > self.player.x
                and self.player.y + self.player.h > enemy.y
                and enemy.y + enemy.h > self.player.y
            ):
                enemy.is_alive = False
                blasts.append(
                    Blast(
                        self.player.x + PLAYER_WIDTH / 2,
                        self.player.y + PLAYER_HEIGHT / 2,
                    )
                )
                pyxel.play(1, 1)
                self.scene = SCENE_GAMEOVER

        self.player.update()
        update_list(bullets)
        update_list(enemies)
        update_list(blasts)
        cleanup_list(enemies)
        cleanup_list(bullets)
        cleanup_list(blasts)

    def update_gameover_scene(self):
        update_list(bullets)
        update_list(enemies)
        update_list(blasts)
        cleanup_list(enemies)
        cleanup_list(bullets)
        cleanup_list(blasts)

        if pyxel.btnp(pyxel.KEY_RETURN):
            self.scene = SCENE_PLAY
            self.player.x = pyxel.width / 2
            self.player.y = pyxel.height - 20
            self.score = 0
            enemies.clear()
            bullets.clear()
            blasts.clear()

    def draw(self):
        pyxel.cls(0)
        self.background.draw()
        if self.scene == SCENE_TITLE:
            self.draw_title_scene()
        elif self.scene == SCENE_PLAY:
            self.draw_play_scene()
        elif self.scene == SCENE_GAMEOVER:
            self.draw_gameover_scene()
        pyxel.text(39, 4, f"SCORE {self.score:5}", 7)

    def draw_title_scene(self):
        pyxel.text(35, 66, "Pyxel Shooter", pyxel.frame_count % 16)
        pyxel.text(31, 126, "- PRESS ENTER -", 13)

    def draw_play_scene(self):
        self.player.draw()
        draw_list(bullets)
        draw_list(enemies)
        draw_list(blasts)

    def draw_gameover_scene(self):
        draw_list(bullets)
        draw_list(enemies)
        draw_list(blasts)
        pyxel.text(43, 66, "GAME OVER", 8)
        pyxel.text(31, 126, "- PRESS ENTER -", 13)


App()
