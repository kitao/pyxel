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


def _slope_mesh() -> Mesh:
    # 6x6 grid sloped along +X. Y drops as X increases.
    verts: list[float] = []
    indices: list[int] = []
    nx, nz = 6, 6
    extent = 12.0
    step = extent / (nx - 1)
    for iz in range(nz):
        for ix in range(nx):
            x = -extent / 2 + ix * step
            z = -extent / 2 + iz * step
            y = -x * 0.4
            verts.extend([x, y, z])
    for iz in range(nz - 1):
        for ix in range(nx - 1):
            i = iz * nx + ix
            indices.extend([i, i + nx, i + 1, i + 1, i + nx, i + nx + 1])
    geom = Geometry(positions=verts, indices=indices)
    return Mesh(
        geometries=[geom],
        transforms=[Mat4.IDENTITY],
        parents=[-1],
        col_img=3,
    )


class Floor(Node):
    def __init__(self):
        super().__init__()
        self.mesh_asset = _slope_mesh()
        self.collider = Collider(mesh=self.mesh_asset, mass=0.0, friction=0.6)

    def on_draw(self):
        self.mesh(Mat4.IDENTITY, self.mesh_asset)


class Ball(Node):
    def __init__(self):
        super().__init__()
        self.transform = Mat4.from_translation(Vec3(-5, 6, 0))
        self.collider = Collider(
            radius=0.6, mass=1.0, rolls=True, restitution=0.4, friction=0.4
        )

    def on_update(self):
        self.collider.velocity += Vec3(0, -0.02, 0)

    def on_collide(self, other, contact):
        # World-space push-back: Mat4.translate is local-frame (spec
        # § 5.6), so a rotating ball would bend the push vector through
        # its own basis and drift sideways across the slope. Compose a
        # world translation by left-multiplying with from_translation.
        push = Mat4.from_translation(contact.normal * contact.depth)
        self.transform = push * self.transform * contact.delta_rotation
        self.collider.velocity += contact.delta_velocity
        self.collider.angular_velocity += contact.delta_angular_velocity

    def on_draw(self):
        self.sphere(Vec3.ZERO, 0.6, 8)


class App:
    def __init__(self):
        pyxel.init(160, 120, title="Cube Physics: Terrain")
        pyxel.mouse(True)
        self.scene = Scene()
        self.scene.clear_color = 1
        self.scene.shading = Shading([pyxel.colors[i] for i in range(16)])
        self.scene.shading.direction = Vec3(0.4, -0.8, 0.2)
        self.scene.add_child(Floor())
        self.scene.add_child(Ball())
        self.orbit = OrbitCamera(target=Vec3(0, 0, 0), pitch_deg=25, radius=18)
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q) or pyxel.btnp(pyxel.KEY_ESCAPE):
            pyxel.quit()
        self.orbit.update()
        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, 160, 120, self.orbit.camera)


# `if __name__` guard so the headless audit script can import the
# Ball / Floor classes without triggering pyxel.run.
if __name__ == "__main__":
    App()
