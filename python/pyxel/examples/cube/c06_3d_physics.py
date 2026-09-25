import pyxel
from pyxel.cube import Camera, Collider, Mat4, Node, Primitive, Shading, Vec3


def box(size, cell=96):
    primitive = Primitive.box(size)

    # Box vertices group the Z, Y, X faces; keep checker squares proportional on every face.
    for vertex in range(24):
        for uv, axis in enumerate(((0, 1), (0, 2), (2, 1))[vertex // 8]):
            position = primitive.positions[vertex * 3 + axis] + size[axis] / 2
            primitive.uvs[vertex * 2 + uv] = position / (16 * cell)

    return primitive


class Body(Node):
    def __init__(self, parent, pos, size, color, mass=0, radius=0):
        super().__init__()

        self.transform = Mat4.from_translation(pos)
        self.collider = Collider(size=size, radius=radius, mass=mass, rolls=mass > 0)

        self.paint, self.transforms = color, [Mat4.IDENTITY]
        self.primitive = (
            Primitive.capsule(size.y, radius)
            if radius
            else box(size, max(size) / 2 if mass else 96)
        )

        accent = {3: 11, 6: 12, 8: 14, 9: 10, 10: 15, 11: 12, 12: 6, 14: 15}
        if mass and radius:
            self.paint = pyxel.Image(16, 16)
            self.paint.cls(color)
            if size.y:
                self.paint.rect(8, 0, 8, 8, accent[color])
                self.paint.rect(0, 8, 8, 8, accent[color])
            else:
                self.paint.rect(0, 0, 2, 16, 15)
                self.paint.rect(8, 0, 2, 16, 15)
        elif mass or isinstance(color, tuple):
            colors = color if isinstance(color, tuple) else (color, accent[color])
            self.paint = pyxel.Image(16, 16)
            for y in range(16):
                for x in range(16):
                    self.paint.pset(x, y, colors[(x + y) % 2])

        parent.add_child(self)

    def on_collide(self, other, contact):
        if self.collider.mass:
            shift = Mat4.from_translation(contact.normal * contact.depth)
            self.transform = shift * self.transform
            self.collider.velocity += contact.delta_velocity
            self.collider.angular_velocity += contact.delta_angular_velocity

    def on_draw(self):
        for transform in self.transforms:
            self.prim(transform, self.primitive, self.paint)


class Floor(Body):
    def on_draw(self):
        super().on_draw()

        self.depth_write(False)  # The floor owns depth; grid strokes only add color.
        self.depth_offset(-0.5)  # Keep the grid on the surface, without depth flicker.
        for offset in range(-2400, 2401, 400):
            self.line(Vec3(offset, 12, -2400), Vec3(offset, 12, 2400), 5)
            self.line(Vec3(-2400, 12, offset), Vec3(2400, 12, offset), 5)


class Scene(Node):
    def __init__(self):
        super().__init__()

        self.angle, self.pitch = 35, 10
        self.shading, self.camera = Shading(pyxel.colors), Camera()
        self.camera.far = 6000
        self.move_camera()

        Floor(self, Vec3(0, -12, 0), Vec3(6000, 24, 6000), 1)

        for x, y, z, width, depth in [
            (-260, 540, -25, 160, 270),
            (320, 370, -90, 180, 180),
            (-340, 320, 220, 220, 160),
            (250, 480, 195, 150, 270),
        ]:
            pillar_x = x + pyxel.sgn(x) * (width - 64) / 2
            Body(self, Vec3(pillar_x, (y - 10) / 2, z), Vec3(64, y - 10, 64), (1, 5))
            Body(self, Vec3(x, y, z), Vec3(width, 20, depth), (1, 5))

        Body(self, Vec3(-450, 280, 100), Vec3(90, 560, 40), (1, 5))
        Body(self, Vec3(-450, 280, 340), Vec3(90, 560, 40), (1, 5))

        for z, height, ledge, depth in [(-195, 570, -125, 180), (355, 600, 275, 200)]:
            Body(self, Vec3(0, (height - 10) / 2, z), Vec3(40, height - 10, 40), (1, 5))
            Body(self, Vec3(0, height, ledge), Vec3(200, 20, depth), (1, 5))

        Body(self, Vec3(0, 330, 80), Vec3(24, 24, 550), (1, 5))
        for pos, pitch, span, width, depth in [
            (Vec3(0, 330, 80), 0, 440, 24, 336),
            (Vec3(0, 16, 80), 90, 480, 40, 32),
        ]:
            rotation = Mat4.from_euler(Vec3(pitch, 0, 0))
            for angle in (0, 90):
                paddle = Body(self, pos, Vec3(span, width, depth), (2, 4))
                paddle.transform *= rotation * Mat4.from_euler(Vec3(0, 0, angle))
                paddle.collider.angular_velocity = Vec3(0, 0, -1.5)
                if angle:  # Leave the center to the other plank.
                    half = (span - width) / 2
                    paddle.primitive = box(Vec3(half, width, depth))
                    shift = Mat4.from_translation(Vec3((span + width) / 4, 0, 0))
                    paddle.transforms = [shift, shift.inverse()]

        self.reset()

    def reset(self):
        for body in self.children:
            if body.collider.mass:
                self.remove_child(body)
        self.shots, self.shot_count = [], 0

        for x, y, z, layers, palette in [
            (-235, 540, -25, 2, [8, 14, 9]),
            (200, 480, 195, 1, [11, 12, 6]),
        ]:
            for row in range(5):
                color = palette[row // 2]
                for col in range(5 - row):
                    for layer in range(layers):
                        dx = (layer - (layers - 1) / 2) * 50
                        dz = -100 + row * 25 + col * 50
                        pos = Vec3(x + dx, y + 35 + row * 50, z + dz)
                        Body(self, pos, Vec3(50, 50, 50), color, 5)

        for x, y, z, floors, palette in [
            (290, 380, -80, 3, [9, 10, 14]),
            (0, 580, -125, 2, [3, 11, 12]),
            (0, 610, 275, 1, [8, 14, 9]),
        ]:
            for floor in range(floors):
                base = y + floor * 80
                for dx, dz in [(-50, -50), (50, -50), (0, 50)]:
                    pos = Vec3(x + dx, base + 32.5, z + dz)
                    Body(self, pos, Vec3(30, 65, 30), palette[0], 3)
                Body(self, Vec3(x, base + 72.5, z), Vec3(180, 15, 160), palette[1], 8)
                Body(self, Vec3(x, base + 20, z), Vec3(100, 40, 50), palette[2], 4)

        bridge = Body(self, Vec3(-450, 570, 220), Vec3(280, 20, 90), 9, 14)
        bridge.transform *= Mat4.from_euler(Vec3(0, 90, 0))
        roller = Body(self, Vec3(-450, 615, 220), Vec3(0, 90, 0), 11, 25, 35)
        roller.transform *= Mat4.from_euler(Vec3(90, 0, 0))

        for x, y, z, color in [(290, 675, -80, 8), (0, 745, 275, 10)]:
            Body(self, Vec3(x, y, z), Vec3.ZERO, color, 60, 55)

        palette = [9, 10, 14]
        for row in range(3):  # Cross-stacked capsules.
            for side in (-1, 1):
                offset = Vec3(side * 50, 0, 0) if row != 1 else Vec3(0, 0, side * 45)
                pos = Vec3(-340, 365 + row * 70, 220) + offset
                log = Body(self, pos, Vec3(0, 110, 0), palette[row], 12, 35)
                log.transform *= Mat4.from_euler(Vec3(90, 90 if row == 1 else 0, 0))

        for floor in range(4):  # Open frame with slender supports.
            y = 490 + floor * 78
            for z in (135, 255):
                Body(self, Vec3(280, y + 32, z), Vec3(18, 64, 18), 8, 1)
            Body(self, Vec3(280, y + 71, 195), Vec3(64, 14, 180), 9, 3)

    def move_camera(self):
        target = Vec3(0, 470, 0)
        rotation = Mat4.from_euler(Vec3(-self.pitch, self.angle, 0))
        eye = target + rotation * Vec3(0, 0, 1100)
        self.camera.transform = Mat4.look_at(eye, target)

    def shoot(self):
        half_fov = self.camera.fov / 2
        scale = 2 * pyxel.sin(half_fov) / pyxel.cos(half_fov) / pyxel.height
        x = (pyxel.mouse_x - pyxel.width / 2) * scale
        y = (pyxel.height / 2 - pyxel.mouse_y) * scale
        direction = self.camera.transform.rot * Vec3(x, y, -1).normalize()
        origin = self.camera.transform.pos

        hits = self.raycast_all(origin, direction)
        hits = [hit for hit in hits if hit.node not in self.shots]
        target = hits[0].point if hits else origin + direction * 1000

        pos = origin + direction * 80 - Vec3.UP * 24
        flight = max(1, (target - pos).length() / 30)

        kind = self.shot_count % 3
        self.shot_count += 1
        size = [Vec3.ZERO, Vec3(0, 36, 0), Vec3(58, 58, 58)][kind]
        shot = Body(self, pos, size, [8, 14, 10][kind], 40, [36, 24, 0][kind])
        if kind == 1:  # A capsule approximates the oval ball for collision.
            shot.primitive = Primitive.sphere(24)
            shot.transforms = [Mat4.from_scale(Vec3(1, 1.75, 1))]

        acceleration = (
            shot.collider.gravity_direction.normalize()
            * shot.collider.gravity
            * 100
            / 30**2
        )
        drag = shot.collider.linear_damp / 30
        # Sum the decaying launch speed and accumulated gravity over the flight.
        travel = flight if drag == 0 else (1 - drag) * (1 - (1 - drag) ** flight) / drag
        fall = flight * (flight + 1) / 2 if drag == 0 else (flight - travel) / drag

        shot.collider.velocity = (target - pos - acceleration * fall) / travel
        shot.collider.angular_velocity = Vec3(3, 5, 2)

        pyxel.play(0, 0)
        self.shots.append(shot)
        if len(self.shots) > 16:
            self.shots.pop(0).destroy()


class App:
    def __init__(self):
        pyxel.init(320, 240, title="3D Physics")

        pyxel.sounds[0].mml("T200 Q100 @0 V88 O4 @ENV1{0,2,127,5,48,17,0} C8")

        self.scene = Scene()

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_R):
            self.scene.reset()

        scene = self.scene
        scene.angle += 2 * (pyxel.btn(pyxel.KEY_RIGHT) - pyxel.btn(pyxel.KEY_LEFT))
        scene.pitch += 2 * (pyxel.btn(pyxel.KEY_UP) - pyxel.btn(pyxel.KEY_DOWN))
        scene.pitch = max(-12, min(75, scene.pitch))
        scene.move_camera()

        if pyxel.btnp(pyxel.MOUSE_BUTTON_LEFT, 10, 10) or pyxel.btnp(
            pyxel.KEY_SPACE, 10, 10
        ):
            scene.shoot()

        scene.update()

    def draw(self):
        pyxel.cls(12)
        self.scene.draw(0, 0, pyxel.width, pyxel.height)

        pyxel.circb(pyxel.mouse_x, pyxel.mouse_y, 4, 7)
        pyxel.pset(pyxel.mouse_x, pyxel.mouse_y, 7)
        pyxel.text(8, 8, "Mouse: Aim  Click/Space: Fire  Arrows: Rotate  R: Reset", 7)


App()
