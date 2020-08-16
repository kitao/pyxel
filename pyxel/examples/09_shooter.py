from random import random

import pyxel

SCENE_TITLE = 0
SCENE_PLAY = 1
SCENE_GAMEOVER = 2

BACKGROUND_STAR_COUNT = 100

PLAYER_WIDTH = 8
PLAYER_HEIGHT = 8
PLAYER_SPEED = 2

BULLET_WIDTH = 2
BULLET_HEIGHT = 8
BULLET_COLOR = 14
BULLET_SPEED = 4

ENEMY_WIDTH = 8
ENEMY_HEIGHT = 8
ENEMY_SPEED = 1

BLAST_START_RADIUS = 3
BLAST_END_RADIUS = 10

enemy_list = []
bullet_list = []
blast_list = []


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
        if not elem.alive:
            list.pop(i)
        else:
            i += 1


class Background:
    def __init__(self):
        self.star_list = []
        for i in range(BACKGROUND_STAR_COUNT):
            self.star_list.append(
                (random() * pyxel.width, random() * pyxel.height, random() * 1.5 + 1)
            )

    def update(self):
        for i, (x, y, speed) in enumerate(self.star_list):
            y += speed
            if y >= pyxel.height:
                y -= pyxel.height
            self.star_list[i] = (x, y, speed)

    def draw(self):
        for (x, y, _) in self.star_list:
            pyxel.pset(x, y, 13)


class Player:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.w = PLAYER_WIDTH
        self.h = PLAYER_HEIGHT
        self.alive = True

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
        self.alive = True

        bullet_list.append(self)

    def update(self):
        self.y -= BULLET_SPEED

        if self.y + self.h - 1 < 0:
            self.alive = False

    def draw(self):
        pyxel.rect(self.x, self.y, self.w, self.h, BULLET_COLOR)


class Enemy:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.w = ENEMY_WIDTH
        self.h = ENEMY_HEIGHT
        self.alive = True
        self.offset = int(random() * 60)

        enemy_list.append(self)

    def update(self):
        if (pyxel.frame_count + self.offset) % 60 < 30:
            self.x += ENEMY_SPEED
        else:
            self.x -= ENEMY_SPEED

        self.y += ENEMY_SPEED

        if self.y > pyxel.height - 1:
            self.alive = False

    def draw(self):
        pyxel.blt(self.x, self.y, 0, 8, 0, self.w, self.h, 0)


class Blast:
    def __init__(self, x, y, col):
        self.x = x
        self.y = y
        self.col = col
        self.radius = BLAST_START_RADIUS
        self.alive = True

        blast_list.append(self)

    def update(self):
        self.radius += 1

        if self.radius > BLAST_END_RADIUS:
            self.alive = False

    def draw(self):
        pyxel.circb(self.x, self.y, self.radius, self.col)


class App:
    def __init__(self):
        pyxel.init(120, 160, caption="Pyxel Shooter")

        pyxel.image(0).set(
            0,
            0,
            [
                "00066000",
                "00677600",
                "00777700",
                "807ee708",
                "70788707",
                "76788767",
                "76777767",
                "80766708",
            ],
        )

        pyxel.image(0).set(
            8,
            0,
            [
                "00c00c00",
                "0cc99cc0",
                "cccaaccc",
                "09aaaa90",
                "09aaaa90",
                "cccaaccc",
                "0cc99cc0",
                "00c00c00",
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
        self.background.update()

        if self.scene == SCENE_TITLE:
            self.update_title_scene()
        elif self.scene == SCENE_PLAY:
            self.update_play_scene()
        elif self.scene == SCENE_GAMEOVER:
            self.update_gameover_scene()

    def update_title_scene(self):
        if pyxel.btnp(pyxel.KEY_ENTER):
            self.scene = SCENE_PLAY

    def update_play_scene(self):
        if pyxel.frame_count % 30 == 0:
            Enemy(random() * (pyxel.width - PLAYER_WIDTH), 0)

        for a in enemy_list:
            for b in bullet_list:
                if (
                    a.x + a.w > b.x
                    and b.x + b.w > a.x
                    and a.y + a.h > b.y
                    and b.y + b.h > a.y
                ):
                    a.alive = False
                    b.alive = False

                    blast_list.append(
                        Blast(a.x + ENEMY_WIDTH / 2, a.y + ENEMY_HEIGHT / 2, 8)
                    )

                    pyxel.play(1, 1)

                    self.score += 10

        for enemy in enemy_list:
            if (
                self.player.x + self.player.w > enemy.x
                and enemy.x + enemy.w > self.player.x
                and self.player.y + self.player.h > enemy.y
                and enemy.y + enemy.h > self.player.y
            ):
                enemy.alive = False

                # 自機の爆発を生成する
                blast_list.append(
                    Blast(
                        self.player.x + PLAYER_WIDTH / 2,
                        self.player.y + PLAYER_HEIGHT / 2,
                        7,
                    )
                )

                pyxel.play(1, 1)

                self.scene = SCENE_GAMEOVER

        self.player.update()
        update_list(bullet_list)
        update_list(enemy_list)
        update_list(blast_list)

        cleanup_list(enemy_list)
        cleanup_list(bullet_list)
        cleanup_list(blast_list)

    def update_gameover_scene(self):
        update_list(bullet_list)
        update_list(enemy_list)
        update_list(blast_list)

        cleanup_list(enemy_list)
        cleanup_list(bullet_list)
        cleanup_list(blast_list)

        if pyxel.btnp(pyxel.KEY_ENTER):
            self.scene = SCENE_PLAY
            self.player.x = pyxel.width / 2
            self.player.y = pyxel.height - 20
            self.score = 0

            enemy_list.clear()
            bullet_list.clear()
            blast_list.clear()

    def draw(self):
        pyxel.cls(0)

        self.background.draw()

        if self.scene == SCENE_TITLE:
            self.draw_title_scene()
        elif self.scene == SCENE_PLAY:
            self.draw_play_scene()
        elif self.scene == SCENE_GAMEOVER:
            self.draw_gameover_scene()

        pyxel.text(38, 4, "SCORE {:5}".format(self.score), 7)

    def draw_title_scene(self):
        pyxel.text(34, 70, "Pyxel Shooter", pyxel.frame_count % 16)
        pyxel.text(30, 100, "- PRESS ENTER -", 13)

    def draw_play_scene(self):
        self.player.draw()
        draw_list(bullet_list)
        draw_list(enemy_list)
        draw_list(blast_list)

    def draw_gameover_scene(self):
        draw_list(bullet_list)
        draw_list(enemy_list)
        draw_list(blast_list)

        pyxel.text(42, 70, "GAME OVER", 14)
        pyxel.text(30, 100, "- PRESS ENTER -", 13)


App()
