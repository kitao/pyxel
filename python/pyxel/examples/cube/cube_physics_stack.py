import pyxel
from cube_physics_camera import OrbitCamera

from pyxel.cube import (
    Collider,
    Mat4,
    Node,
    Shading,
    Vec3,
)

# Carnival-style 2-high pyramid. Vertical chains of three or more cans
# collapse under the single-pass resolver (stable stacks are out of
# scope, cube-design.md § 16); a 2-high interlocked pyramid settles
# with ~0.1 sink and no lateral drift.
CAN_LAYOUT = [
    Vec3(-0.85, 0.4, 0),
    Vec3(0.0, 0.4, 0),
    Vec3(0.85, 0.4, 0),
    Vec3(-0.425, 1.2, 0),
    Vec3(0.425, 1.2, 0),
]


class Floor(Node):
    def __init__(self):
        super().__init__()
        self.size = Vec3(20, 0.5, 20)
        self.transform = Mat4.from_translation(Vec3(0, -0.25, 0))
        self.collider = Collider(size=self.size, mass=0.0, friction=0.7)

    def on_draw(self):
        self.box(Mat4.IDENTITY, self.size, 3)


class Can(Node):
    def __init__(self, pos: Vec3):
        super().__init__()
        self.transform = Mat4.from_translation(pos)
        self.size = Vec3(0.8, 0.8, 0.8)
        # Stacked AABB-vs-AABB contacts are notoriously hard to settle
        # under single-pass resolution; restitution > 0 keeps bouncing
        # forever and a high friction value swings sideways across
        # the rest frame. Both go to zero so the stack damps quickly.
        self.collider = Collider(
            size=self.size, mass=1.0, restitution=0.0, friction=0.0
        )

    def on_update(self):
        self.collider.velocity += Vec3(0, -0.02, 0)

    def on_collide(self, other, contact):
        push = Mat4.from_translation(contact.normal * contact.depth)
        self.transform = push * self.transform
        self.collider.velocity += contact.delta_velocity

    def on_draw(self):
        self.box(Mat4.IDENTITY, self.size, 8)


class Bullet(Node):
    def __init__(self, pos: Vec3, vel: Vec3):
        super().__init__()
        self.transform = Mat4.from_translation(pos)
        self.collider = Collider(radius=0.3, mass=2.0, restitution=0.4)
        self.collider.velocity = vel
        self.ttl = 200

    def on_update(self):
        self.collider.velocity += Vec3(0, -0.005, 0)
        self.ttl -= 1
        if self.ttl <= 0:
            self.destroy()

    def on_collide(self, other, contact):
        push = Mat4.from_translation(contact.normal * contact.depth)
        self.transform = push * self.transform
        self.collider.velocity += contact.delta_velocity

    def on_draw(self):
        self.sphere(Vec3.ZERO, 0.3, 14)


class App:
    def __init__(self):
        pyxel.init(160, 120, title="Cube Physics: Stack")
        pyxel.mouse(True)
        self.scene = Node()
        self.scene.shading = Shading([pyxel.colors[i] for i in range(16)])
        self.scene.shading.direction = Vec3(0.4, -0.8, 0.2)
        self.scene.add_child(Floor())
        for pos in CAN_LAYOUT:
            self.scene.add_child(Can(pos))
        self.orbit = OrbitCamera(target=Vec3(0, 1, 0), pitch_deg=15, radius=10)
        self.orbit.camera.clear_color = 1
        self.scene.camera = self.orbit.camera
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q) or pyxel.btnp(pyxel.KEY_ESCAPE):
            pyxel.quit()
        if pyxel.btnp(pyxel.KEY_SPACE):
            self.scene.add_child(Bullet(Vec3(0, 1.0, 8), Vec3(0, 0, -0.4)))
        self.orbit.update()
        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, 160, 120)


if __name__ == "__main__":
    App()
