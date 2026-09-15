import math
import random

import pyxel
from pyxel.cube import Camera, Mat4, Mesh, Node, Shading, Vec3

MOTION_SPEED = 1.0
HUD_HEIGHT = 26
FLOOR_RADIUS = 2.2


class Floor(Node):
    def __init__(self, joints):
        super().__init__()
        self.joints = joints
        self.offset = 0.0
        self.tufts = []

        placement = random.Random(8)
        heading = random.Random(42)
        for _ in range(34):
            x, z = placement.uniform(-1.9, 1.9), placement.uniform(-1.9, 1.9)
            height = placement.uniform(0.09, 0.15)
            mirror = 1 if math.cos(heading.uniform(0, math.tau)) >= 0 else -1
            if abs(x) >= 0.9:
                self.tufts.append((x, z, height, mirror))

    def on_draw(self):
        self.shaded(False)
        rotation = Mat4.from_euler(Vec3(-90, 0, 0))
        diameter = FLOOR_RADIUS * 2
        inner_radius = FLOOR_RADIUS - 0.075
        self.elli(rotation, diameter, diameter, 5)
        self.elli(
            Mat4.from_translation(Vec3(0, 0.01, 0)) * rotation,
            inner_radius * 2,
            inner_radius * 2,
            3,
        )

        center = (
            sum((joint.world_transform.pos for joint in self.joints), Vec3.ZERO) / 4
        )
        shadow = Mat4.from_translation(Vec3(center.x, 0.023, center.z)) * rotation
        self.elli(shadow, 0.75, 1.45, 5)

        camera = self.effective_camera
        transform = camera.transform
        right = Vec3(transform[0, 0], 0, transform[2, 0]).normalize()
        for x, start_z, height, mirror in self.tufts:
            z = (start_z + self.offset + 2.1) % 4.2 - 2.1
            # The root crossing the inner floor's edge removes the whole tuft.
            if x**2 + z**2 > inner_radius**2:
                continue

            base = Vec3(x, 0.02, z)
            tips = [
                base + right * (dx * mirror) + Vec3(0, height * dy, 0)
                for dx, dy in [(-0.065, 0.7), (0.005, 1.0), (0.060, 0.55)]
            ]

            if not all(self.is_on_screen(vertex, camera) for vertex in [base, *tips]):
                continue
            for tip in tips:
                self.line(base, tip, 11)

    def is_on_screen(self, vertex, camera):
        local = vertex.to_local(camera.transform)
        scale = pyxel.height / camera.ortho_size
        x = pyxel.width / 2 + local.x * scale
        y = pyxel.height / 2 - local.y * scale
        return 1 <= x < pyxel.width - 1 and HUD_HEIGHT <= y < pyxel.height - 1


class App:
    def __init__(self):
        pyxel.init(320, 240, title="Mesh and Motion")

        self.scene = Node()
        self.scene.camera = Camera()
        # A light sky keeps the dark horns distinct from the background.
        self.scene.camera.clear_color = 12
        self.scene.camera.ortho_size = 3.5
        self.shading = Shading(pyxel.colors)
        self.shading.direction = Vec3(-0.5, -1.0, 0.8).normalize()

        # Gazelle by monkus (CC BY 4.0); see assets/cube_gazelle.md.
        mesh = Mesh.from_glb("../assets/cube_gazelle.glb", fps=30.0)
        self.motions = [
            next(motion for motion in mesh.motions if motion.name == name)
            for name in ["move", "run", "eat"]
        ]
        self.actor = Node.from_mesh(mesh)
        model = Node()
        model.transform = Mat4.from_translation(Vec3(0.006513, 0.03, 0.181916)) * (
            Mat4.from_scale(Vec3(1.20027, 1.20027, 1.20027))
        )
        model.add_child(self.actor)
        self.scene.add_child(model)

        self.rest_pose = []
        nodes = [self.actor]
        while nodes:
            node = nodes.pop()
            self.rest_pose.append((node, node.transform))
            nodes.extend(node.children)

        joints = [
            self.actor.find_by_name(name)[0]
            for name in ["rightFronLeg", "leftFronLeg", "rightBackLeg", "leftBackLeg"]
        ]
        self.floor = Floor(joints)
        self.scene.add_child(self.floor)
        self.yaw = 148.0
        self.pitch = math.degrees(math.atan2(1.7, 6.0))
        self.select_motion(0)
        self.update_camera()

        pyxel.run(self.update, self.draw)

    def select_motion(self, index):
        self.motion_index = index
        for node, transform in self.rest_pose:
            node.transform = transform
        self.actor.play_motion(self.motions[index], speed=MOTION_SPEED)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        for index, key in enumerate([pyxel.KEY_1, pyxel.KEY_2, pyxel.KEY_3]):
            if pyxel.btnp(key):
                self.select_motion(index)

        if pyxel.btnp(pyxel.KEY_S):
            self.scene.shading = self.shading if self.scene.shading is None else None

        turn = pyxel.btn(pyxel.KEY_RIGHT) - pyxel.btn(pyxel.KEY_LEFT)
        tilt = pyxel.btn(pyxel.KEY_UP) - pyxel.btn(pyxel.KEY_DOWN)
        self.yaw += 0.4 + turn * 2.0
        self.pitch = max(5.0, min(60.0, self.pitch + tilt))
        self.update_camera()

        speed = [0.42, 2.5, 0.0][self.motion_index]
        self.floor.offset = (self.floor.offset + speed * MOTION_SPEED / 30) % 4.2

        self.scene.update()

    def update_camera(self):
        yaw, pitch = math.radians(self.yaw), math.radians(self.pitch)
        eye = Vec3(6 * math.sin(yaw), 0.85 + 6 * math.tan(pitch), 6 * math.cos(yaw))
        self.scene.camera.transform = Mat4.look_at(eye, Vec3(0, 0.85, 0))
        # Keep the circular ground in view when looking down from above.
        extent = 0.85 * math.cos(pitch) + FLOOR_RADIUS * math.sin(pitch)
        self.scene.camera.ortho_size = max(3.5, extent * 2 + 0.2)

    def draw(self):
        self.scene.draw(0, 0, pyxel.width, pyxel.height)

        pyxel.text(8, 8, "Arrows: Rotate  S: Shading  Q: Quit", 7)
        pyxel.text(8, 18, "1: Walk  2: Run  3: Eat", 7)


App()
