import pyxel
from cube_physics_camera import OrbitCamera

from pyxel.cube import (
    Collider,
    Geometry,
    Mat4,
    Mesh,
    Node,
    Scene,
    Shading,
    Vec3,
)


def _stage_mesh() -> Mesh:
    verts = [
        -8.0,
        0.0,
        -8.0,
        8.0,
        0.0,
        -8.0,
        -8.0,
        0.0,
        8.0,
        8.0,
        0.0,
        8.0,
    ]
    indices = [0, 1, 2, 1, 3, 2]
    geom = Geometry(positions=verts, indices=indices)
    return Mesh(
        geometries=[geom],
        transforms=[Mat4.IDENTITY],
        parents=[-1],
        col_img=11,
    )


class Stage(Node):
    def __init__(self):
        super().__init__()
        self.mesh_asset = _stage_mesh()
        self.collider = Collider(mesh=self.mesh_asset, mass=0.0, friction=0.5)

    def on_draw(self):
        self.mesh(Mat4.IDENTITY, self.mesh_asset)


class Wall(Node):
    def __new__(cls, *args, **kwargs):
        return super().__new__(cls)

    def __init__(self, pos: Vec3, size: Vec3):
        super().__init__()
        self.transform = Mat4.from_translation(pos)
        self.size = size
        self.collider = Collider(size=size, mass=0.0, friction=0.5)

    def on_draw(self):
        self.box(Mat4.IDENTITY, self.size, 5)


class MovingPlatform(Node):
    def __init__(self):
        super().__init__()
        self.transform = Mat4.from_translation(Vec3(0, 0.3, 4))
        self.size = Vec3(3, 0.6, 3)
        self.collider = Collider(size=self.size, mass=0.0, velocity=Vec3(0.05, 0, 0))
        self.frame = 0

    def on_update(self):
        self.frame += 1
        if self.frame % 120 == 0:
            v = self.collider.velocity
            self.collider.velocity = Vec3(-v.x, 0, 0)

    def on_draw(self):
        self.box(Mat4.IDENTITY, self.size, 10)


class Character(Node):
    def __init__(self):
        super().__init__()
        self.transform = Mat4.from_translation(Vec3(0, 1.0, -3))
        self.collider = Collider(radius=0.6, mass=1.0, rolls=False, friction=0.5)

    def on_update(self):
        v = Vec3(0, self.collider.velocity.y - 0.02, 0)
        if pyxel.btn(pyxel.KEY_W):
            v += Vec3(0, 0, -0.1)
        if pyxel.btn(pyxel.KEY_S):
            v += Vec3(0, 0, 0.1)
        if pyxel.btn(pyxel.KEY_A):
            v += Vec3(-0.1, 0, 0)
        if pyxel.btn(pyxel.KEY_D):
            v += Vec3(0.1, 0, 0)
        self.collider.velocity = v

    def on_collide(self, other, contact):
        # World-space push-back (see cube_physics_terrain.py comment).
        push = Mat4.from_translation(contact.normal * contact.depth)
        self.transform = push * self.transform
        self.collider.velocity += contact.delta_velocity

    def on_draw(self):
        self.sphere(Vec3.ZERO, 0.6, 14)


class App:
    def __init__(self):
        pyxel.init(160, 120, title="Cube Physics: Character")
        pyxel.mouse(True)
        self.scene = Scene()
        self.scene.clear_color = 1
        self.scene.shading = Shading([pyxel.colors[i] for i in range(16)])
        self.scene.shading.direction = Vec3(0.4, -0.8, 0.2)
        self.scene.add_child(Stage())
        self.scene.add_child(Wall(Vec3(-7, 1.0, 0), Vec3(0.4, 2.0, 14)))
        self.scene.add_child(Wall(Vec3(7, 1.0, 0), Vec3(0.4, 2.0, 14)))
        self.scene.add_child(Wall(Vec3(0, 1.0, -7), Vec3(14, 2.0, 0.4)))
        self.scene.add_child(Wall(Vec3(0, 1.0, 7), Vec3(14, 2.0, 0.4)))
        self.scene.add_child(MovingPlatform())
        self.scene.add_child(Character())
        self.orbit = OrbitCamera(target=Vec3(0, 1, 0), pitch_deg=35, radius=18)
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q) or pyxel.btnp(pyxel.KEY_ESCAPE):
            pyxel.quit()
        self.orbit.update()
        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, 160, 120, self.orbit.camera)


if __name__ == "__main__":
    App()
