import pyxel
from cube_physics_camera import OrbitCamera

from pyxel.cube import (
    Collider,
    Mat4,
    Node,
    Scene,
    Shading,
    Vec3,
)


class Target(Node):
    def __new__(cls, *args, **kwargs):
        return super().__new__(cls)

    def __init__(self, pos: Vec3):
        super().__init__()
        self.transform = Mat4.from_translation(pos)
        self.size = Vec3(1.0, 1.0, 1.0)
        self.collider = Collider(size=self.size, mass=0.0, trigger=True)

    def on_collide(self, other, contact):
        # First-hit self-destruct.
        self.destroy()

    def on_draw(self):
        col = 7 if not self.destroyed else 5
        self.box(Mat4.IDENTITY, self.size, col)


class Bullet(Node):
    def __new__(cls, *args, **kwargs):
        return super().__new__(cls)

    def __init__(self, pos: Vec3, vel: Vec3):
        super().__init__()
        self.transform = Mat4.from_translation(pos)
        self.collider = Collider(radius=0.2, mass=1.0)
        self.collider.velocity = vel
        self.ttl = 120

    def on_update(self):
        self.ttl -= 1
        if self.ttl <= 0:
            self.destroy()

    def on_collide(self, other, contact):
        self.destroy()

    def on_draw(self):
        self.sphere(Vec3.ZERO, 0.2, 9)


class App:
    def __init__(self):
        pyxel.init(160, 120, title="Cube Physics: Shoot")
        pyxel.mouse(True)
        self.scene = Scene()
        self.scene.clear_color = 1
        self.scene.shading = Shading([pyxel.colors[i] for i in range(16)])
        self.scene.shading.direction = Vec3(0.4, -0.8, 0.2)
        for x in (-3.0, 0.0, 3.0):
            self.scene.add_child(Target(Vec3(x, 0, 0)))
        self.orbit = OrbitCamera(target=Vec3(0, 0, 0), pitch_deg=20, radius=10)
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q) or pyxel.btnp(pyxel.KEY_ESCAPE):
            pyxel.quit()
        if pyxel.btnp(pyxel.KEY_SPACE):
            self.scene.add_child(Bullet(Vec3(0, 2, 8), Vec3(0, -0.05, -0.4)))
        self.orbit.update()
        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, 160, 120, self.orbit.camera)


if __name__ == "__main__":
    App()
