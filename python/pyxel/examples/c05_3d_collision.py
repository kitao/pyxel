import pyxel
from pyxel.cube import Camera, Collider, Mat4, Mesh, Node, Primitive, Shading, Vec3

WIDTH = 240
HEIGHT = 180
GRAVITY = -0.03
JUMP_SPEED = 0.42
MOVE_SPEED = 0.13
TURN_SPEED = 4.0
PLAYER_START = Vec3(0.0, 1.2, 5.0)


def pressed(*keys):
    return any(pyxel.btn(key) for key in keys)


def pressedp(*keys):
    return any(pyxel.btnp(key) for key in keys)


def make_quad_mesh(corners, color):
    positions = []
    for v in corners:
        positions += [v.x, v.y, v.z]
    primitive = Primitive(
        Primitive.MODE_TRIANGLES,
        positions,
        [0, 1, 2, 1, 3, 2],
        cull=Primitive.CULL_BACK,
    )
    primitive.compute_normals()
    return Mesh(
        primitives=[primitive],
        transforms=[Mat4.IDENTITY],
        parents=[-1],
        col_img=color,
    )


class TerrainPatch(Node):
    def __init__(self, corners, color, outline=1):
        super().__init__()
        self.corners = corners
        self.outline = outline
        self.mesh = make_quad_mesh(corners, color)
        self.collider = Collider(mesh=self.mesh, mass=0.0)
        self.add_child(Node.from_mesh(self.mesh))

    def on_draw(self):
        edge_points = [self.corners[i] for i in [0, 1, 3, 2]]
        for p, q in zip(edge_points, edge_points[1:] + edge_points[:1]):
            self.line(p, q, self.outline)


class MarkerBox(Node):
    def __init__(self, pos, size, color, outline=1):
        super().__init__()
        self.transform = Mat4.from_translation(pos)
        self.size = size
        self.color = color
        self.outline = outline

    def on_draw(self):
        self.box(Mat4.IDENTITY, self.size, self.color)
        self.boxb(Mat4.IDENTITY, self.size, self.outline)


class MovingPlatform(Node):
    def __init__(self):
        super().__init__()
        self.size = Vec3(2.4, 0.35, 4.4)
        self.transform = Mat4.from_translation(Vec3(0.0, 1.0 - self.size.y * 0.5, -7.3))
        self.direction = 1.0
        self.delta = Vec3.ZERO
        self.collider = Collider(size=self.size, mass=0.0)

    def on_update(self):
        if abs(self.transform.pos.x) > 1.2:
            self.direction *= -1.0
        self.delta = Vec3(0.03 * self.direction, 0.0, 0.0)
        self.collider.velocity = self.delta

    def on_draw(self):
        self.box(Mat4.IDENTITY, self.size, 10)
        self.boxb(Mat4.IDENTITY, self.size, 1)


class Goal(Node):
    def __init__(self, pos):
        super().__init__()
        self.pos = pos
        self.collider = Collider(radius=0.65, trigger=True, mass=0.0)

    def on_update(self):
        spin = Mat4.from_euler(Vec3(0.0, pyxel.frame_count * 4.0, 0.0))
        self.transform = Mat4.from_translation(self.pos) * spin

    def on_draw(self):
        self.box(Mat4.IDENTITY, Vec3(0.75, 0.75, 0.75), 10)
        self.boxb(Mat4.IDENTITY, Vec3(0.9, 0.9, 0.9), 7)


class Player(Node):
    def __init__(self, mesh):
        super().__init__()
        self.transform = Mat4.from_translation(PLAYER_START)
        self.collider = Collider(size=Vec3(0.0, 0.9, 0.0), radius=0.32, mass=1.0)
        self.yaw = 0.0
        self.on_floor = False
        self.reached_goal = False
        self.walking = False
        self.motion = mesh.motions[0] if mesh.motions else None
        self.model = Node.from_mesh(mesh)
        self.model.transform = Mat4.from_translation(
            Vec3(0.0, -0.4, 0.0)
        ) * Mat4.from_scale(Vec3(0.37, 0.37, 0.37))
        if self.motion is not None:
            self.model.apply_motion(self.motion, 0.0)
        self.add_child(self.model)

    def reset(self):
        self.transform = Mat4.from_translation(PLAYER_START)
        self.collider.velocity = Vec3.ZERO
        self.yaw = 0.0
        self.on_floor = False
        self.reached_goal = False

    def facing(self):
        return Vec3(pyxel.sin(self.yaw), 0.0, -pyxel.cos(self.yaw))

    def set_walking(self, enabled):
        if self.motion is None or self.walking == enabled:
            return
        self.walking = enabled
        if enabled:
            self.model.play_motion(self.motion, speed=1.3)
        else:
            self.model.stop_motion()
            self.model.apply_motion(self.motion, 0.0)

    def on_update(self):
        if pressed(pyxel.KEY_LEFT, pyxel.KEY_A):
            self.yaw -= TURN_SPEED
        if pressed(pyxel.KEY_RIGHT, pyxel.KEY_D):
            self.yaw += TURN_SPEED

        forward = self.facing()
        move = Vec3.ZERO
        if pressed(pyxel.KEY_UP, pyxel.KEY_W):
            move += forward
        if pressed(pyxel.KEY_DOWN, pyxel.KEY_S):
            move -= forward

        v = self.collider.velocity
        if move.length() > 0:
            move = move.normalize() * MOVE_SPEED
            vx, vz = move.x, move.z
        else:
            vx, vz = v.x * 0.75, v.z * 0.75

        vy = max(v.y + GRAVITY, -0.55)
        if self.on_floor and pressedp(pyxel.KEY_SPACE, pyxel.GAMEPAD1_BUTTON_A):
            vy = JUMP_SPEED
            pyxel.play(0, 0)

        self.on_floor = False
        self.collider.velocity = Vec3(vx, vy, vz)
        self.transform = Mat4.from_translation(self.transform.pos) * Mat4.from_euler(
            Vec3(0.0, self.yaw, 0.0)
        )
        self.set_walking(move.length() > 0)

        if self.transform.pos.y < -4.0:
            self.reset()

    def on_collide(self, other, contact):
        if isinstance(other, Goal):
            self.reached_goal = True
            return

        push = Mat4.from_translation(contact.normal * contact.depth)
        self.transform = push * self.transform

        if contact.normal.y > 0.45:
            self.on_floor = True
            if isinstance(other, MovingPlatform):
                self.transform = Mat4.from_translation(other.delta) * self.transform
            if self.collider.velocity.y < 0:
                self.collider.velocity = Vec3(
                    self.collider.velocity.x, 0.0, self.collider.velocity.z
                )


class Scene(Node):
    def __init__(self, player_mesh):
        super().__init__()
        self.shading = Shading(pyxel.colors)
        self.shading.direction = Vec3(0.4, -1.0, -0.3).normalize()

        self.camera = Camera()
        self.camera.clear_color = 12

        self.player = Player(player_mesh)
        self.platform = MovingPlatform()
        self.goal = Goal(Vec3(0.0, 2.35, -11.4))

        self.build_stage()
        self.add_child(self.platform)
        self.add_child(self.goal)
        self.add_child(self.player)
        self.update_camera()

    def build_stage(self):
        # Main route
        self.add_child(
            TerrainPatch(
                [
                    Vec3(-5.6, 0.0, 6.4),
                    Vec3(5.6, 0.0, 6.4),
                    Vec3(-5.6, 0.0, 0.7),
                    Vec3(5.6, 0.0, 0.7),
                ],
                3,
            )
        )
        self.add_child(
            TerrainPatch(
                [
                    Vec3(-1.55, 0.0, 0.7),
                    Vec3(1.55, 0.0, 0.7),
                    Vec3(-1.55, 1.0, -2.4),
                    Vec3(1.55, 1.0, -2.4),
                ],
                11,
            )
        )
        self.add_child(
            TerrainPatch(
                [
                    Vec3(-4.2, 1.0, -2.4),
                    Vec3(4.2, 1.0, -2.4),
                    Vec3(-4.2, 1.0, -5.4),
                    Vec3(4.2, 1.0, -5.4),
                ],
                5,
            )
        )
        self.add_child(
            TerrainPatch(
                [
                    Vec3(-1.8, 1.15, -10.5),
                    Vec3(1.8, 1.15, -10.5),
                    Vec3(-1.8, 1.15, -12.5),
                    Vec3(1.8, 1.15, -12.5),
                ],
                5,
            )
        )

        # Surface markers
        for z in [5.2, 3.9, 2.6, 1.3]:
            self.add_child(MarkerBox(Vec3(0.0, 0.035, z), Vec3(1.25, 0.06, 0.45), 9))
        for z in [-3.4, -4.6]:
            self.add_child(MarkerBox(Vec3(0.0, 1.035, z), Vec3(1.1, 0.06, 0.38), 9))
        for x, z in [(-4.2, 4.5), (4.2, 3.0), (-3.4, -3.2), (3.4, -4.6)]:
            self.add_child(MarkerBox(Vec3(x, 0.45, z), Vec3(0.35, 0.9, 0.35), 4))

    def update_camera(self):
        target = self.player.transform.pos + Vec3(0.0, 0.75, 0.0)
        back = self.player.facing() * -6.6
        eye = target + back + Vec3(0.0, 3.8, 0.0)
        self.camera.transform = Mat4.look_at(eye, target)


class App:
    def __init__(self):
        pyxel.init(WIDTH, HEIGHT, title="3D Collision")
        pyxel.sounds[0].set("c3g3c4", "t", "654", "nnf", 4)
        mesh = Mesh.from_glb("assets/cube_actor.glb", colkey=0, fps=30.0)
        self.scene = Scene(mesh)
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()
        if pyxel.btnp(pyxel.KEY_R):
            self.scene.player.reset()

        self.scene.update()
        self.scene.update_camera()

    def draw(self):
        self.scene.draw(0, 0, WIDTH, HEIGHT)

        if self.scene.player.reached_goal:
            pyxel.text(8, 8, "GOAL! Press R", 10)
        else:
            pyxel.text(8, 8, "Up/W: Move  Left/Right: Turn  Space: Jump", 7)


App()
