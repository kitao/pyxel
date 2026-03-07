import pyxel

HUD_HEIGHT = 16


class App:
    def __init__(self):
        pyxel.init(200, 150, title="Perspective")
        pyxel.load("assets/sample.pyxres")

        self.cam_x = 64
        self.cam_y = 128
        self.cam_z = 32
        self.rot_x = 40
        self.rot_y = -90
        self.rot_z = 0
        self.fov = 90
        self.speed = 1.5

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_ESCAPE):
            pyxel.quit()

        # Look (Arrows + QE)
        if pyxel.btn(pyxel.KEY_LEFT):
            self.rot_y -= 2
        if pyxel.btn(pyxel.KEY_RIGHT):
            self.rot_y += 2

        if pyxel.btn(pyxel.KEY_UP):
            self.rot_x = min(90, self.rot_x + 2)
        if pyxel.btn(pyxel.KEY_DOWN):
            self.rot_x = max(1, self.rot_x - 2)

        if pyxel.btn(pyxel.KEY_Q):
            self.rot_z = max(self.rot_z - 2, -30)
        elif pyxel.btn(pyxel.KEY_E):
            self.rot_z = min(self.rot_z + 2, 30)
        else:
            self.rot_z *= 0.85

        # Move (WASD + RF)
        fwd_x = pyxel.cos(self.rot_y) * self.speed
        fwd_y = pyxel.sin(self.rot_y) * self.speed
        right_x = pyxel.cos(self.rot_y + 90) * self.speed
        right_y = pyxel.sin(self.rot_y + 90) * self.speed

        if pyxel.btn(pyxel.KEY_W):
            self.cam_x += fwd_x
            self.cam_y += fwd_y
        if pyxel.btn(pyxel.KEY_S):
            self.cam_x -= fwd_x
            self.cam_y -= fwd_y

        if pyxel.btn(pyxel.KEY_A):
            self.cam_x -= right_x
            self.cam_y -= right_y
        if pyxel.btn(pyxel.KEY_D):
            self.cam_x += right_x
            self.cam_y += right_y

        if pyxel.btn(pyxel.KEY_R):
            self.cam_z = min(self.cam_z + 1, 120)
        if pyxel.btn(pyxel.KEY_F):
            self.cam_z = max(self.cam_z - 1, 10)

        # FOV (TG)
        if pyxel.btn(pyxel.KEY_T):
            self.fov = min(self.fov + 2, 150)
        if pyxel.btn(pyxel.KEY_G):
            self.fov = max(self.fov - 2, 30)

    def draw(self):
        pyxel.cls(0)

        # Perspective views
        cam = (self.cam_x, self.cam_y, self.cam_z)
        rot = (self.rot_x, self.rot_y, self.rot_z)
        hw = pyxel.width // 2
        vh = pyxel.height - HUD_HEIGHT
        pyxel.blt3d(0, HUD_HEIGHT, hw, vh, 0, cam, rot, fov=self.fov)
        pyxel.bltm3d(hw + 2, HUD_HEIGHT, hw, vh, 0, cam, rot, fov=self.fov)

        # Divider
        pyxel.line(hw, HUD_HEIGHT, hw, pyxel.height - 1, 0)
        pyxel.line(hw + 1, HUD_HEIGHT, hw + 1, pyxel.height - 1, 0)

        # HUD
        pyxel.text(4, 2, "blt3d", 10)
        pyxel.text(hw + 6, 2, "bltm3d", 10)
        pyxel.text(4, 9, "WASD:Move Arrows:Look RF:Alt QE:Tilt TG:FOV", 13)


App()
