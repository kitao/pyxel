import math

import pyxel
from pyxel.cube import Camera, Mat4, Node, Primitive, Shading, Vec3

ENEMY_COLORS = [8, 9, 10, 12, 14]
LOCK_RADIUS = 13.0

LASER_ORIGIN = Vec3(0.0, 5.5, 110.0)
LASER_POINTS = 24
LASER_DURATION = 14
LASER_HALF_WIDTH = 2.42

AIM_POINT = Vec3(0.0, 11.0, 0.0)


class Enemy(Node):
    def __init__(self, index):
        super().__init__()

        self.phase = index * 1.3
        self.orbit = index * math.tau / len(ENEMY_COLORS)
        self.color = ENEMY_COLORS[index]
        self.flash_timer = 0

        # Keep the original vertices so deformation does not accumulate.
        self.body = Primitive.sphere(1.0)
        self.base_positions = list(self.body.positions)

    def on_update(self):
        frame = pyxel.frame_count
        time = frame * 0.06
        phase = self.phase
        positions = []

        for i in range(0, len(self.base_positions), 3):
            x, y, z = self.base_positions[i : i + 3]
            radius = (
                11.0
                + 2.75 * math.sin(3.0 * x + 1.7 * time + phase)
                + 2.42 * math.sin(3.5 * y + 1.3 * time + phase * 1.7)
                + 2.2 * math.sin(4.0 * z + 2.1 * time + phase * 0.6)
            )
            positions += [x * radius, y * radius, z * radius]

        self.body.positions[:] = positions
        self.body.compute_normals()

        center = Vec3(
            38.5 * math.cos(self.orbit + frame * 0.01),
            27.5 + 5.5 * math.sin(frame * 0.02 + self.phase),
            38.5 * math.sin(self.orbit + frame * 0.01),
        )
        self.transform = Mat4.from_translation(center)

        self.flash_timer = max(0, self.flash_timer - 1)

    def on_draw(self):
        color = 7 if self.flash_timer % 2 == 0 and self.flash_timer else self.color
        self.prim(Mat4.IDENTITY, self.body, color)


class Lasers(Node):
    def __init__(self, enemies, camera):
        super().__init__()

        self.age = 1

        # Spread the launch directions into a fan
        self.targets = []
        right = Vec3.RIGHT.to_world_dir(camera)
        center_index = (len(enemies) - 1) * 0.5
        for index, enemy in enumerate(enemies):
            lock_side = (index - center_index) / max(center_index, 1.0)
            target_side = (enemy.transform.pos - AIM_POINT).dot(right)
            if abs(target_side) <= 4.4:
                target_side = lock_side or 1.0
            side_sign = 1.0 if target_side >= 0.0 else -1.0
            direction = (enemy.transform.pos - LASER_ORIGIN).normalize()
            fan = side_sign * 1.2 * (0.7 + 0.3 * abs(lock_side))
            self.targets.append((enemy, (direction + right * fan).normalize()))

        # Build each strip segment from two triangles
        indices = []
        for j in range(0, (LASER_POINTS - 1) * 2, 2):
            indices += [j, j + 1, j + 2, j + 1, j + 3, j + 2]

        positions = [0.0] * (LASER_POINTS * 2 * 3)
        # Map the texture across the strip's width and along its length.
        uvs = []
        for i in range(LASER_POINTS):
            v = i / (LASER_POINTS - 1)
            uvs += [0, v, 1, v]

        self.strip = Primitive(
            Primitive.MODE_TRIANGLES,
            positions,
            indices,
            uvs=uvs,
            cull=Primitive.CULL_NONE,
        )

    def on_update(self):
        self.age += 1
        if self.age == LASER_DURATION:
            self.hit()
        elif self.age > LASER_DURATION + 8:
            self.destroy()

    def hit(self):
        pyxel.play(2, 2)
        for enemy, direction in self.targets:
            enemy.flash_timer = 12

    def on_draw(self):
        self.depth_test(False)
        self.shaded(False)

        camera = self.effective_camera.transform
        right = Vec3.RIGHT.to_world_dir(camera)
        progress = min(1.0, self.age / LASER_DURATION)
        for enemy, direction in self.targets:
            path = self.make_path(enemy.transform.pos, direction, progress)
            positions = []
            for i, point in enumerate(path):
                # Face each segment toward the camera
                tangent = path[min(i + 1, len(path) - 1)] - path[max(0, i - 1)]
                side = tangent.cross(point - camera.pos)
                side = side.normalize() if side.length() > 1e-6 else right
                positions.extend(point - side * LASER_HALF_WIDTH)
                positions.extend(point + side * LASER_HALF_WIDTH)

            self.strip.positions[:] = positions
            self.prim(Mat4.IDENTITY, self.strip, self.parent.laser_image, colkey=0)

    def make_path(self, enemy_center, direction, progress):
        to_enemy = enemy_center - LASER_ORIGIN
        step_length = to_enemy.length() * 1.15 / (LASER_POINTS - 1)
        point = LASER_ORIGIN
        path = [point]

        # Start outward, then bend toward the moving target.
        for i in range(1, LASER_POINTS):
            path_ratio = (i - 1) / (LASER_POINTS - 2)
            steer_ratio = max(0.0, (path_ratio - 0.3) / 0.7)
            steer = steer_ratio**2 * (3.0 - 2.0 * steer_ratio) * 0.35
            target_direction = (enemy_center - point).normalize()
            direction = direction.lerp(target_direction, steer).normalize()
            point += direction * step_length
            path.append(point)

        # Bend the end of the path to meet the target
        end_correction = enemy_center - path[-1]
        for i, point in enumerate(path):
            ratio = i / (LASER_POINTS - 1)
            path[i] = point + end_correction * (ratio**2.5)
        path[-1] = enemy_center

        # Reveal more of the path as the laser travels.
        visible = []
        for i in range(LASER_POINTS):
            position = progress * i
            index = min(int(position), LASER_POINTS - 2)
            visible.append(path[index].lerp(path[index + 1], position - index))

        return visible


class App(Node):
    def __init__(self):
        super().__init__()

        pyxel.init(256, 192, title="Custom Shapes")
        pyxel.mouse(False)
        self.laser_image = pyxel.Image.from_image("assets/laser.png")

        pyxel.sounds[0].mml("T240 Q100 @0 V100 O6 @ENV1{127,4,127,8,0} A16")
        pyxel.sounds[1].mml(
            "T240 Q100 @2 V104 O4 @ENV1{127,8,104,56,0} @GLI1{1900,56} @VIB1{0,8,45} E3"
        )
        pyxel.sounds[2].mml(
            "T240 Q100 @3 V112 O4 @ENV1{127,4,80,44,0} @GLI1{3600,36} C4"
        )
        # Mix growing noise with the laser's pitched sound.
        pyxel.sounds[3].mml(
            "T240 Q100 @3 V48 O7 @ENV1{16,20,127,44,0} @GLI1{1200,56} C3"
        )

        self.shading = Shading(pyxel.colors)
        self.shading.direction = Vec3(0.4, -1.0, -0.5).normalize()

        self.camera = Camera()
        self.camera.clear_color = 0

        self.enemies = [Enemy(index) for index in range(len(ENEMY_COLORS))]
        for enemy in self.enemies:
            self.add_child(enemy)
        self.locked_enemies = []
        self.lasers = None

        pyxel.run(self.update_game, self.draw_game)

    def on_update(self):
        mouse_offset = pyxel.mouse_y / pyxel.height - 0.5
        eye = Vec3(0, 132 - mouse_offset * 44, 110)
        self.camera.transform = Mat4.look_at(eye, AIM_POINT)

    def on_draw(self):
        # Draw the ground grid
        for i in range(13):
            grid_pos = -77.0 + 154.0 * i / 12
            self.line(Vec3(grid_pos, 0, -77.0), Vec3(grid_pos, 0, 77.0), 5)
            self.line(Vec3(-77.0, 0, grid_pos), Vec3(77.0, 0, grid_pos), 5)

    def update_game(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        self.update()

        drag_started = pyxel.btnp(pyxel.MOUSE_BUTTON_LEFT)
        if self.lasers and self.lasers.parent:
            if not drag_started:
                return

            # A new drag interrupts the volley and begins a new lock-on.
            if self.lasers.age < LASER_DURATION:
                self.lasers.hit()
            self.remove_child(self.lasers)
            self.lasers = None

        if drag_started:
            self.locked_enemies = []
        if pyxel.btn(pyxel.MOUSE_BUTTON_LEFT):
            for enemy in self.enemies:
                if enemy in self.locked_enemies:
                    continue

                point = self.project_to_screen(enemy.transform.pos)
                distance = math.hypot(
                    pyxel.mouse_x - point[0], pyxel.mouse_y - point[1]
                )
                if distance < LOCK_RADIUS:
                    self.locked_enemies.append(enemy)
                    pyxel.play(0, 0)
        elif pyxel.btnr(pyxel.MOUSE_BUTTON_LEFT):
            self.start_fire()

    def draw_game(self):
        self.draw(0, 0, pyxel.width, pyxel.height)

        # Draw lock-on markers
        for enemy in self.locked_enemies:
            x, y = self.project_to_screen(enemy.transform.pos)
            for r, color in [(4, 8), (5, 7), (6, 8)]:
                pyxel.rectb(round(x) - r, round(y) - r, r * 2 + 1, r * 2 + 1, color)

        # Draw the cursor
        pyxel.circb(pyxel.mouse_x, pyxel.mouse_y, 7, 7)
        pyxel.circb(pyxel.mouse_x, pyxel.mouse_y, 9, 7)

        pyxel.text(76, 5, "Drag: Lock   Release: Fire", 7)

    def project_to_screen(self, world_pos):
        camera = self.camera
        local = world_pos.to_local(camera.transform)
        depth = -local.z
        scale = pyxel.height / (2 * depth * math.tan(math.radians(camera.fov) / 2))
        return pyxel.width / 2 + local.x * scale, pyxel.height / 2 - local.y * scale

    def start_fire(self):
        if not self.locked_enemies:
            return

        self.lasers = Lasers(self.locked_enemies, self.camera.transform)
        self.add_child(self.lasers)
        self.locked_enemies = []
        pyxel.play(1, 1)
        pyxel.play(3, 3)


App()
