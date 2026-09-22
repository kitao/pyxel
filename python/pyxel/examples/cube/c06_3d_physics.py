import pyxel
from pyxel.cube import Camera, Collider, Mat4, Mesh, Node, Vec3

GRAVITY = Vec3(0, -0.16, 0)
CANNON_POS = Vec3(0, 14, -105)


class Toy(Node):
    def __init__(self, mesh, pos, collider, rotation=Vec3.ZERO):
        super().__init__()
        self.transform = Mat4.from_translation(pos) * Mat4.from_euler(rotation)
        self.add_child(Node.from_mesh(mesh))
        self.collider = collider
        self.collider.rolls = True

    def on_update(self):
        self.collider.velocity += GRAVITY
        self.collider.velocity *= 0.995
        self.collider.angular_velocity *= 0.98

        pos = self.transform.pos
        if pos.y < -80:
            self.destroy()

    def on_collide(self, other, contact):
        push = Mat4.from_translation(contact.normal * contact.depth)
        self.transform = push * self.transform

        self.collider.velocity += contact.delta_velocity
        self.collider.angular_velocity += contact.delta_angular_velocity

        if contact.delta_velocity.length() > 1.5 and pyxel.play_pos(1) is None:
            pyxel.play(1, 1)


class Animal(Toy):
    def __init__(self, mesh, kind, pos, velocity):
        if kind == 0:
            collider = Collider(radius=10, mass=3, restitution=0.65)
        elif kind == 1:
            collider = Collider(size=Vec3(0, 18, 0), radius=8, mass=6, restitution=0.1)
        else:
            collider = Collider(size=Vec3(12, 7, 12), radius=4, mass=9, restitution=0.2)

        collider.velocity = velocity
        rotation = Vec3(-90, 0, 0) if kind == 1 else Vec3(0, 180, 0)
        super().__init__(mesh, pos, collider, rotation)
        self.age = 0

    def on_update(self):
        super().on_update()
        self.age += 1
        if self.age > 450:
            self.destroy()


class Shadows(Node):
    def on_draw(self):
        for node in self.parent.children:
            if not isinstance(node, Toy) or not node.visible:
                continue

            pos = node.transform.pos
            if pos.y < -10:
                continue

            self.decal(pos.y + 26)
            self.elli(
                Mat4.from_translation(pos + Vec3(0, 15, 0))
                * Mat4.from_euler(Vec3(-90, 0, 0)),
                18,
                18,
                9,
            )


class Scene(Node):
    def __init__(self, meshes):
        super().__init__()

        self.meshes = meshes
        self.kind = 0
        self.angle = 14
        self.pitch = 30
        self.cooldown = 0

        self.camera = Camera()
        self.camera.fov = 56
        self.camera.near = 2
        self.camera.transform = Mat4.look_at(Vec3(45, 53, -195), Vec3(0, 70, 80))

        stage = Node.from_mesh(meshes["stage"])
        stage.collider = Collider(mesh=meshes["stage"])
        self.add_child(stage)
        # Draw shadows after the stage, before the moving objects.
        self.add_child(Shadows())

        self.cannon = Node.from_mesh(meshes["cannon"])
        self.cannon.transform = Mat4.from_translation(CANNON_POS)
        self.add_child(self.cannon)

        self.loaded_animals = []
        for name in ["bird", "seal", "pig"]:
            animal = Node.from_mesh(meshes[name])
            self.loaded_animals.append(animal)
            self.add_child(animal)

        # Collision shapes: (size, radius, mass)
        shapes = {
            "block": (Vec3(20, 20, 20), 0, 1),
            "plank": (Vec3(56, 8, 20), 0, 2),
            "ball": (Vec3.ZERO, 8, 1),
            "domino": (Vec3(16, 28, 6), 0, 0.5),
            "roller": (Vec3(0, 16, 0), 7, 2),
        }

        # Place toys at the stage's model markers
        for marker in stage.find_by_name("Toys")[0].children:
            size, radius, mass = shapes[marker.name]
            collider = Collider(size=size, radius=radius, mass=mass, friction=0.2)
            self.add_child(
                Toy(meshes[marker.name], marker.world_transform.pos, collider)
            )

    def on_update(self):
        turn = int(pyxel.btn(pyxel.KEY_RIGHT)) - int(pyxel.btn(pyxel.KEY_LEFT))
        self.angle = pyxel.clamp(self.angle + turn, -60, 60)
        tilt = int(pyxel.btn(pyxel.KEY_UP)) - int(pyxel.btn(pyxel.KEY_DOWN))
        self.pitch = pyxel.clamp(self.pitch + tilt, 10, 60)
        self.cooldown = max(0, self.cooldown - 1)
        self.cannon.transform = Mat4.from_translation(CANNON_POS) * Mat4.from_euler(
            Vec3(-self.pitch, -self.angle, 0)
        )

        for index, key in enumerate([pyxel.KEY_1, pyxel.KEY_2, pyxel.KEY_3]):
            if pyxel.btnp(key):
                self.kind = index

        if pyxel.btnp(pyxel.KEY_SPACE) and self.cooldown == 0:
            name = ["bird", "seal", "pig"][self.kind]
            direction = -self.cannon.forward
            self.add_child(
                Animal(
                    self.meshes[name],
                    self.kind,
                    CANNON_POS + direction * 32,
                    direction * 8.5,
                )
            )
            self.cooldown = 45
            pyxel.play(0, 0)

        for index, animal in enumerate(self.loaded_animals):
            animal.visible = index == self.kind and self.cooldown == 0
            animal.transform = Mat4.from_translation(
                CANNON_POS - self.cannon.forward * 32
            )
            rotation = Vec3(-90, 0, 0) if index == 1 else Vec3(0, 180, 0)
            animal.transform *= Mat4.from_euler(rotation)

    def on_draw(self):
        # Draw the start of the flight path
        pos = CANNON_POS - self.cannon.forward * 32
        velocity = -self.cannon.forward * 8.5
        for time in range(1, 22):
            velocity = (velocity + GRAVITY) * 0.995
            pos += velocity
            if time % 3 == 0:
                self.elli(Mat4.from_translation(pos), 2, 2, 7)


class App:
    def __init__(self):
        pyxel.init(320, 240, title="3D Physics")

        self.meshes = {
            name: Mesh.from_glb(f"assets/toy_{name}.glb")
            for name in [
                "stage",
                "block",
                "plank",
                "domino",
                "ball",
                "roller",
                "cannon",
                "bird",
                "seal",
                "pig",
            ]
        }
        self.scene = Scene(self.meshes)

        pyxel.sounds[0].mml(
            "T240 Q100 @2 V92 O4 @ENV1{127,4,90,12,0} @GLI1{-700,12} G12"
        )
        pyxel.sounds[1].mml("T240 Q100 @0 V88 O5 @ENV1{127,2,48,10,0} @GLI1{900,6} C16")

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        if pyxel.btnp(pyxel.KEY_R):
            pyxel.stop()
            self.scene = Scene(self.meshes)

        self.scene.update()

    def draw(self):
        pyxel.cls(6)
        self.scene.draw(0, 0, pyxel.width, pyxel.height)

        pyxel.text(10, 9, "ROLLING PALS", 4)

        pyxel.rect(0, 218, 320, 22, 4)
        for index, label in enumerate(["1:BIRD", "2:SEAL", "3:PIG"]):
            pyxel.text(
                18 + index * 65, 222, label, 7 if index == self.scene.kind else 15
            )
        pyxel.text(241, 222, f"TILT {self.scene.pitch}", 15)
        pyxel.text(20, 232, "LEFT/RIGHT:AIM UP/DOWN:TILT SPACE:FIRE R:RESET", 15)


App()
