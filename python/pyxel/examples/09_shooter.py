# title: Pyxel Shooter
# author: Takashi Kitao
# desc: A Pyxel shoot'em up game example
# site: https://github.com/kitao/pyxel
# license: MIT
# version: 1.0

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


def update_entities(entities):
    for entity in entities:
        entity.update()


def draw_entities(entities):
    for entity in entities:
        entity.draw()


def cleanup_entities(entities):
    for i in range(len(entities) - 1, -1, -1):
        if not entities[i].is_alive:
            del entities[i]


def load_bgm(msc, filename, snd1, snd2, snd3):
    import json

    with open(filename, "rt") as file:
        bgm = json.loads(file.read())
        pyxel.sounds[snd1].set(*bgm[0])
        pyxel.sounds[snd2].set(*bgm[1])
        pyxel.sounds[snd3].set(*bgm[2])
        pyxel.musics[msc].set([snd1], [snd2], [snd3])


class Background:
    def __init__(self):
        self.stars = []
        for i in range(NUM_STARS):
            self.stars.append(
                (
                    pyxel.rndi(0, pyxel.width - 1),
                    pyxel.rndi(0, pyxel.height - 1),
                    pyxel.rndf(1, 2.5),
                )
            )

    def update(self):
        for i, (x, y, speed) in enumerate(self.stars):
            y += speed
            if y >= pyxel.height:
                y -= pyxel.height
            self.stars[i] = (x, y, speed)

    def draw(self):
        for x, y, speed in self.stars:
            pyxel.pset(x, y, STAR_COLOR_HIGH if speed > 1.8 else STAR_COLOR_LOW)


class Player:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.w = PLAYER_WIDTH
        self.h = PLAYER_HEIGHT
        self.is_alive = True

    def update(self):
        if pyxel.btn(pyxel.KEY_LEFT) or pyxel.btn(pyxel.GAMEPAD1_BUTTON_DPAD_LEFT):
            self.x -= PLAYER_SPEED
        if pyxel.btn(pyxel.KEY_RIGHT) or pyxel.btn(pyxel.GAMEPAD1_BUTTON_DPAD_RIGHT):
            self.x += PLAYER_SPEED
        if pyxel.btn(pyxel.KEY_UP) or pyxel.btn(pyxel.GAMEPAD1_BUTTON_DPAD_UP):
            self.y -= PLAYER_SPEED
        if pyxel.btn(pyxel.KEY_DOWN) or pyxel.btn(pyxel.GAMEPAD1_BUTTON_DPAD_DOWN):
            self.y += PLAYER_SPEED

        self.x = max(self.x, 0)
        self.x = min(self.x, pyxel.width - self.w)
        self.y = max(self.y, 0)
        self.y = min(self.y, pyxel.height - self.h)

        if pyxel.btnp(pyxel.KEY_SPACE) or pyxel.btnp(pyxel.GAMEPAD1_BUTTON_A):
            Bullet(
                self.x + (PLAYER_WIDTH - BULLET_WIDTH) / 2, self.y - BULLET_HEIGHT / 2
            )
            pyxel.play(3, 0)

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
        self.timer_offset = pyxel.rndi(0, 59)
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

        self.init_image()
        self.init_sound()

        self.scene = SCENE_TITLE
        self.score = 0
        self.background = Background()
        self.player = Player(pyxel.width / 2, pyxel.height - 20)

        pyxel.playm(0, loop=True)
        pyxel.run(self.update, self.draw)

    def init_image(self):
        # Set player image
        pyxel.images[0].set(
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

        # Set enemy image
        pyxel.images[0].set(
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

    def init_sound(self):
        # Set sound effects
        pyxel.sounds[0].set("a3a2c1a1", "p", "7", "s", 5)
        pyxel.sounds[1].set("a3a2c2c2", "n", "7742", "s", 10)

        # Set title music
        a1 = "t128 @2 o3 q8 l8 x0:765"
        a2 = "edcr<ab>cr d2<g4>g<b& ba&a2r4"
        a3 = ">dc4<b2r>"
        a4 = ">d4<b.r16>dg+dg+"

        b1 = "t128 @0 o1 v7 l16"
        b2 = "q5aar4aar4<q8a4> q5ggr4ggr4<q8g4> q5ffr4ffr4<q8f4>"
        b3 = "q5ggr4ggr4<q8g4>"
        b4 = "q5ggr4ggr4g+g+<g+8"

        c1 = "t128 @3 l8 x0:21"
        c2 = "q1o2v4crrcrr>>q2x0a#16a#16a#<<"
        c3 = "q1o2v4crrcrr>>q2x0a#16a#16<<q1o2v4c"

        pyxel.sounds[2].mml(a1 + a2 + a3 + a2 + a4)
        pyxel.sounds[3].mml(b1 + b2 + b3 + b2 + b4)
        pyxel.sounds[4].mml(c1 + c2 * 3 + c3)
        pyxel.musics[0].set([2], [3], [4])

        # Set play music
        a1 = "t150 @2 o2 q8 l16 x0:765"
        a2 = "e8>e<e4r>c8<e4&er ag+abaf+8r r8a4>d8"
        a3 = "ed8c8<b8a8.g+f+q6e8q8e8 f+8.r>f+4<a4b>cd<f+"
        a4 = "c4&cr<ag+a.r.>c<ba>ce rd<a8.>f+8.<b.r32>e8dc<bq6e"

        b1 = "t150 @0 o1 v7 l16 q7"
        b2 = "<ar>ea<ar>ea<ar>ea<ar>ea"
        b3 = "dra>d<dra>d<dra>d<dra>d<"
        b4 = "dra>d<dra>d<er>eeeeer"

        c1 = "t150 @3 l16 x0:21 x1:52"
        c2 = "q2x0o3 a#ra#a# a#ra#a# a#ra#a# a#ra#a#"
        c3 = "q2x0o3 a#ra#a# a#ra#a# a#ra#a# o2x1 a#a# q1o2v5cr"

        pyxel.sounds[5].mml(a1 + a2 + a3 + a2 + a4)
        pyxel.sounds[6].mml(b1 + (b2 + b3) * 3 + b2 + b4)
        pyxel.sounds[7].mml(c1 + c2 * 3 + c3)
        pyxel.musics[1].set([5], [6], [7])

        # You can also use 8bit BGM generator for music:
        #   load_bgm(0, "assets/bgm_title.json", 2, 3, 4)
        #   load_bgm(1, "assets/bgm_play.json", 5, 6, 7)

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
        if pyxel.btnp(pyxel.KEY_RETURN) or pyxel.btnp(pyxel.GAMEPAD1_BUTTON_X):
            self.scene = SCENE_PLAY
            pyxel.playm(1, loop=True)

    def update_play_scene(self):
        if pyxel.frame_count % 6 == 0:
            Enemy(pyxel.rndi(0, pyxel.width - ENEMY_WIDTH), 0)

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
                    pyxel.play(2, 1, resume=True)
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
                pyxel.stop()
                pyxel.play(3, 1)
                self.scene = SCENE_GAMEOVER

        self.player.update()

        update_entities(bullets)
        update_entities(enemies)
        update_entities(blasts)

        cleanup_entities(enemies)
        cleanup_entities(bullets)
        cleanup_entities(blasts)

    def update_gameover_scene(self):
        update_entities(bullets)
        update_entities(enemies)
        update_entities(blasts)

        cleanup_entities(enemies)
        cleanup_entities(bullets)
        cleanup_entities(blasts)

        if pyxel.btnp(pyxel.KEY_RETURN) or pyxel.btnp(pyxel.GAMEPAD1_BUTTON_X):
            self.scene = SCENE_PLAY
            self.player.x = pyxel.width / 2
            self.player.y = pyxel.height - 20
            self.score = 0

            enemies.clear()
            bullets.clear()
            blasts.clear()

            pyxel.playm(1, loop=True)

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

        draw_entities(bullets)
        draw_entities(enemies)
        draw_entities(blasts)

    def draw_gameover_scene(self):
        draw_entities(bullets)
        draw_entities(enemies)
        draw_entities(blasts)

        pyxel.text(43, 66, "GAME OVER", 8)
        pyxel.text(31, 126, "- PRESS ENTER -", 13)


App()
