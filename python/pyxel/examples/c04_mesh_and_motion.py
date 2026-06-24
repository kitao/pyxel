import pyxel
from pyxel.cube import Camera, Mat4, Mesh, Node, Shading, Vec3

ACTOR_COUNT = 3


class Actor(Node):
    def __init__(self, mesh, index):
        super().__init__()
        self.index = index
        self.phase = index * 360.0 / ACTOR_COUNT

        self.model = Node.from_mesh(mesh)
        self.model.play_motion(
            mesh.motions[0],
            speed=0.65 + index * 0.25,
            start_frame=mesh.motions[0].length * index / ACTOR_COUNT,
        )
        self.add_child(self.model)

    def on_update(self):
        t = pyxel.frame_count
        angle = self.phase + t * 0.9
        pos = Vec3(pyxel.sin(angle) * 2.2, 0.0, pyxel.cos(angle) * 2.2)
        spin = Mat4.from_euler(Vec3(0.0, 180.0 - angle + t * 1.4, 0.0))
        scale = Mat4.from_scale(Vec3(0.8, 0.8, 0.8))
        self.transform = Mat4.from_translation(pos) * spin * scale


class Floor(Node):
    def on_draw(self):
        self.shaded(False)
        for i in range(11):
            u = -5.0 + i
            self.line(Vec3(u, 0.0, -5.0), Vec3(u, 0.0, 5.0), 5)
            self.line(Vec3(-5.0, 0.0, u), Vec3(5.0, 0.0, u), 5)


class Scene(Node):
    def __init__(self, mesh):
        super().__init__()

        self.shading = Shading(pyxel.colors)
        self.shading.direction = Vec3(0.4, -1.2, -0.8).normalize()

        self.camera = Camera()
        self.camera.clear_color = 1
        self.camera.transform = Mat4.look_at(Vec3(0.0, 3.4, 6.0), Vec3(0.0, 0.7, 0.0))

        self.add_child(Floor())
        for i in range(ACTOR_COUNT):
            self.add_child(Actor(mesh, i))


class App:
    def __init__(self):
        pyxel.init(240, 180, title="Mesh and Motion")

        mesh = Mesh.from_glb("assets/cube_actor.glb", colkey=0, fps=30.0)
        self.motion_name = mesh.motions[0].name
        self.scene = Scene(mesh)

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, pyxel.width, pyxel.height)

        pyxel.text(5, 5, f"Mesh.from_glb / Motion: {self.motion_name}", 7)


App()
