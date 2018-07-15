import pyxel

PADDLE_WIDTH = 32
PADDLE_HEIGHT = 4
PADDLE_HIT_HEIGHT = 20
PADDLE_SPEED = 5
PADDLE_DECEL_RATE = 0.8

BALL_RADIUS = 3
BALL_SPEED_X = 1
BALL_SPEED_Y = -6
BALL_SPEED_UP = 0.1

ORBIT_COUNT = 30

HIT_NONE = 0
HIT_WALL = 1
HIT_PADDLE = 2


class App():
    def __init__(self):
        pyxel.init(160, 120, caption='Simple Game')

        pyxel.sound(0).set('g1', 't', '7', 'f', 10)
        pyxel.sound(1).set('c2', 'p', '6', 'f', 12)

        self.paddle_x = (pyxel.width - PADDLE_WIDTH) // 2
        self.paddle_y = pyxel.height - PADDLE_HEIGHT - 2
        self.paddle_vx = 0

        self.orbit_x = [0] * ORBIT_COUNT
        self.orbit_y = [0] * ORBIT_COUNT

        self.reset_game()

        pyxel.run(self.update, self.draw)

    def reset_game(self):
        self.score = 0

        self.ball_x = self.paddle_x + PADDLE_WIDTH // 2
        self.ball_y = self.paddle_y - BALL_RADIUS
        self.ball_vx = BALL_SPEED_X
        self.ball_vy = BALL_SPEED_Y

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        # update paddle
        if pyxel.btn(pyxel.KEY_LEFT):
            self.paddle_vx = -PADDLE_SPEED

        if pyxel.btn(pyxel.KEY_RIGHT):
            self.paddle_vx = PADDLE_SPEED

        self.paddle_x += self.paddle_vx
        self.paddle_vx *= PADDLE_DECEL_RATE

        if self.paddle_x < 0:
            self.paddle_x = 0

        if self.paddle_x > pyxel.width - PADDLE_WIDTH:
            self.paddle_x = pyxel.width - PADDLE_WIDTH

        # updat ball
        x = self.ball_x
        y = self.ball_y
        vx = self.ball_vx
        vy = self.ball_vy

        (x, y, vx, vy, hit) = self.update_ball(x + vx, y + vy, vx, vy)
        if hit == HIT_WALL:
            pyxel.play(0, 0)
        elif hit == HIT_PADDLE:
            self.score += 1
            pyxel.play(0, 1)

        self.ball_x = x
        self.ball_y = y
        self.ball_vx = vx
        self.ball_vy = vy

        if self.ball_y > pyxel.height + 300:
            self.reset_game()

        # update orbits
        for i in range(ORBIT_COUNT):
            (x, y, vx, vy, hit) = self.update_ball(x + vx, y + vy, vx, vy)
            self.orbit_x[i] = x
            self.orbit_y[i] = y

    def update_ball(self, x, y, vx, vy):
        hit = HIT_NONE

        if x <= BALL_RADIUS:
            x = BALL_RADIUS
            vx = abs(vx)
            hit = HIT_WALL

        if x >= pyxel.width - BALL_RADIUS:
            x = pyxel.width - BALL_RADIUS
            vx = -abs(vx)
            hit = HIT_WALL

        if y <= BALL_RADIUS:
            y = BALL_RADIUS
            vy = abs(vy)
            hit = HIT_WALL

        if (y + BALL_RADIUS >= self.paddle_y
                and y - BALL_RADIUS <= self.paddle_y + PADDLE_HIT_HEIGHT
                and x + BALL_RADIUS >= self.paddle_x
                and x - BALL_RADIUS <= self.paddle_x + PADDLE_WIDTH):
            vx = (x - (self.paddle_x + PADDLE_WIDTH // 2)) / 3
            y = self.paddle_y - BALL_RADIUS
            vy = -abs(vy) - BALL_SPEED_UP
            hit = HIT_PADDLE

        return (x, y, vx, vy, hit)

    def draw(self):
        pyxel.cls(5)

        for i in range(1, ORBIT_COUNT):
            pyxel.circb(self.orbit_x[i], self.orbit_y[i], BALL_RADIUS - 1, 13)

        pyxel.rect(self.paddle_x, self.paddle_y,
                   self.paddle_x + PADDLE_WIDTH - 1,
                   self.paddle_y + PADDLE_HEIGHT - 1, 12)

        pyxel.circ(self.ball_x, self.ball_y, BALL_RADIUS, 14)

        pyxel.text(4, 4, 'SCORE {}'.format(self.score), 11)


App()
