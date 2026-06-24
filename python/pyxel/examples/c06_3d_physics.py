import pyxel
from pyxel.cube import Camera, Collider, Mat4, Mesh, Node, Primitive, Shading, Vec3

WIDTH = 240
HEIGHT = 180
GRAVITY = Vec3(0.0, -0.025, 0.0)


def make_quad_mesh(corners, color):
    positions = []
    for v in corners:
        positions += [v.x, v.y, v.z]
    prim = Primitive(
        Primitive.MODE_TRIANGLES,
        positions,
        [0, 1, 2, 1, 3, 2],
        cull=Primitive.CULL_BACK,
    )
    prim.compute_normals()
    return Mesh(
        primitives=[prim],
        transforms=[Mat4.IDENTITY],
        parents=[-1],
        col_img=color,
    )


def apply_contact(node, contact):
    push = Mat4.from_translation(contact.normal * contact.depth)
    spin = Mat4.from_quat(contact.delta_rotation)
    node.transform = push * node.transform * spin
    node.collider.velocity += contact.delta_velocity
    node.collider.angular_velocity += contact.delta_angular_velocity


class QuadSurface(Node):
    def __init__(self, corners, color):
        super().__init__()
        self.corners = corners
        self.mesh = make_quad_mesh(corners, color)
        self.collider = Collider(mesh=self.mesh, mass=0.0, friction=0.65)
        self.add_child(Node.from_mesh(self.mesh))

    def on_draw(self):
        points = [self.corners[i] for i in [0, 1, 3, 2]]
        for p, q in zip(points, points[1:] + points[:1]):
            self.line(p, q, 1)


class Floor(Node):
    def __init__(self):
        super().__init__()
        self.size = Vec3(7.0, 0.3, 7.4)
        self.transform = Mat4.from_translation(Vec3(0.0, -0.15, -3.2))
        self.collider = Collider(size=self.size, mass=0.0, friction=0.7)

    def on_draw(self):
        self.box(Mat4.IDENTITY, self.size, 3)
        self.boxb(Mat4.IDENTITY, self.size, 1)


class Barrel(Node):
    def __init__(self, pos, color):
        super().__init__()
        self.transform = Mat4.from_translation(pos)
        self.size = Vec3(0.74, 0.88, 0.74)
        self.collider = Collider(
            size=self.size,
            radius=0.05,
            rolls=True,
            mass=0.85,
            restitution=0.12,
            friction=0.35,
        )
        self.color = color

    def on_update(self):
        self.collider.velocity += GRAVITY
        self.collider.velocity *= 0.995
        self.collider.angular_velocity *= 0.985

    def on_collide(self, other, contact):
        del other
        apply_contact(self, contact)

    def on_draw(self):
        self.box(Mat4.IDENTITY, self.size, self.color)
        self.boxb(Mat4.IDENTITY, self.size, 1)
        self.box(Mat4.from_translation(Vec3(0.0, -0.26, 0.0)), Vec3(0.8, 0.05, 0.8), 10)
        self.box(Mat4.from_translation(Vec3(0.0, 0.26, 0.0)), Vec3(0.8, 0.05, 0.8), 10)


class CapsuleBall(Node):
    def __init__(self):
        super().__init__()
        tilt = Mat4.from_axis_angle(Vec3(0.0, 0.0, 1.0), -90.0)
        self.transform = Mat4.from_translation(Vec3(0.0, 2.75, 4.6)) * tilt
        self.collider = Collider(
            size=Vec3(0.0, 1.15, 0.0),
            radius=0.34,
            rolls=True,
            mass=2.2,
            restitution=0.35,
            friction=0.45,
            velocity=Vec3(0.0, -0.02, -0.19),
        )

    def on_update(self):
        self.collider.velocity += GRAVITY

    def on_collide(self, other, contact):
        del other
        apply_contact(self, contact)

    def on_draw(self):
        self.box(Mat4.IDENTITY, Vec3(0.58, 1.15, 0.58), 14)
        self.sphere(Vec3(0.0, -0.58, 0.0), 0.34, 14)
        self.sphere(Vec3(0.0, 0.58, 0.0), 0.34, 14)
        self.boxb(Mat4.IDENTITY, Vec3(0.62, 1.18, 0.62), 1)
        self.line(Vec3(0.0, -0.88, 0.0), Vec3(0.0, 0.88, 0.0), 7)


class Scene(Node):
    def __init__(self):
        super().__init__()

        self.shading = Shading(pyxel.colors)
        self.shading.direction = Vec3(0.45, -1.0, -0.35).normalize()

        self.camera = Camera()
        self.camera.clear_color = 12
        self.camera.transform = Mat4.look_at(Vec3(0.0, 5.6, 9.5), Vec3(0.0, 1.0, -1.4))

        self.add_child(
            QuadSurface(
                [
                    Vec3(-1.35, 2.05, 5.2),
                    Vec3(1.35, 2.05, 5.2),
                    Vec3(-2.35, 0.0, 0.2),
                    Vec3(2.35, 0.0, 0.2),
                ],
                11,
            )
        )
        self.add_child(Floor())
        self.add_child(CapsuleBall())
        self.add_barrels()

    def add_barrels(self):
        spacing = 0.82
        base_y = 0.44
        for row, count in enumerate([4, 3, 2, 1]):
            y = base_y + row * 0.78
            for i in range(count):
                x = (i - (count - 1) * 0.5) * spacing
                z = -2.95 - row * 0.02
                self.add_child(Barrel(Vec3(x, y, z), 8 + row % 2))


class App:
    def __init__(self):
        pyxel.init(WIDTH, HEIGHT, title="3D Physics")
        self.scene = Scene()
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()
        if pyxel.btnp(pyxel.KEY_R) or pyxel.btnp(pyxel.KEY_SPACE):
            self.scene = Scene()

        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, WIDTH, HEIGHT)

        pyxel.text(8, 8, "Capsule vs. barrel stack  R/Space: Reset", 7)


App()
