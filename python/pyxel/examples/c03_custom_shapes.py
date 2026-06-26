import math

import pyxel
from pyxel.cube import Camera, Mat4, Node, Primitive, Shading, Vec3

ENEMY_COLORS = [8, 9, 10, 12, 14]
ENEMY_RINGS = 6
ENEMY_SEGMENTS = 10

LASER_ORIGIN = Vec3(0.0, 0.5, 10.0)
LASER_POINTS = 24
LASER_DURATION = 14
LASER_WIDTH = 0.22
LASER_CORE_WIDTH = LASER_WIDTH * 0.35

CAMERA_EYE = Vec3(0.0, 12.0, 10.0)
AIM_POINT = Vec3(0.0, 1.0, 0.0)


class Enemy(Node):
    def __init__(self, index):
        super().__init__()
        self.phase = index * 1.3
        self.orbit = index * math.tau / len(ENEMY_COLORS)
        self.color = ENEMY_COLORS[index]
        self.center = Vec3.ZERO
        self.is_locked = False
        self.flash_timer = 0

        self.body_directions = []
        for i in range(ENEMY_RINGS + 1):
            lat = math.pi * i / ENEMY_RINGS - math.pi / 2
            for j in range(ENEMY_SEGMENTS + 1):
                lon = math.tau * j / ENEMY_SEGMENTS
                self.body_directions.append(
                    (
                        math.cos(lat) * math.cos(lon),
                        math.sin(lat),
                        math.cos(lat) * math.sin(lon),
                    )
                )

        indices = []
        for i in range(ENEMY_RINGS):
            for j in range(ENEMY_SEGMENTS):
                base = i * (ENEMY_SEGMENTS + 1) + j
                next_base = base + ENEMY_SEGMENTS + 1
                indices += [
                    base,
                    next_base + 1,
                    base + 1,
                    base,
                    next_base,
                    next_base + 1,
                ]

        self.body = Primitive(
            Primitive.MODE_TRIANGLES,
            self.make_body_positions(0.0),
            indices,
            cull=Primitive.CULL_NONE,
        )

    def make_body_positions(self, time):
        phase = self.phase
        positions = []
        for direction in self.body_directions:
            radius = (
                1.0
                + 0.25 * math.sin(3.0 * direction[0] + 1.7 * time + phase)
                + 0.22 * math.sin(3.5 * direction[1] + 1.3 * time + phase * 1.7)
                + 0.20 * math.sin(4.0 * direction[2] + 2.1 * time + phase * 0.6)
            )
            x, y, z = direction
            positions += [x * radius, y * radius, z * radius]
        return positions

    def trigger_hit(self):
        self.flash_timer = 12

    def on_update(self):
        frame = pyxel.frame_count
        self.body.positions[:] = self.make_body_positions(frame * 0.06)

        self.center = Vec3(
            3.5 * math.cos(self.orbit + frame * 0.01),
            2.5 + 0.5 * math.sin(frame * 0.02 + self.phase),
            3.5 * math.sin(self.orbit + frame * 0.01),
        )
        self.transform = Mat4.from_translation(self.center)

        self.flash_timer = max(0, self.flash_timer - 1)

    def on_draw(self):
        color = 7 if self.flash_timer % 2 == 0 and self.flash_timer else self.color
        self.prim(Mat4.IDENTITY, self.body, color)


class Laser(Node):
    def __init__(self, enemies):
        super().__init__()
        self.enemies = enemies
        self.locked_enemies = []
        self.firing_enemies = []
        self.fan_offsets = []
        self.fire_frame = 0
        self.hit_done = False

        self.outer_prims = [self.make_prim() for _ in enemies]
        self.core_prims = [self.make_prim() for _ in enemies]

    def make_prim(self):
        indices = []
        for i in range(LASER_POINTS - 1):
            base = i * 2
            indices += [base, base + 1, base + 2, base + 1, base + 3, base + 2]
        return Primitive(
            Primitive.MODE_TRIANGLES,
            [0.0] * (LASER_POINTS * 2 * 3),
            indices,
            cull=Primitive.CULL_NONE,
        )

    def clear_locks(self):
        for enemy in self.enemies:
            enemy.is_locked = False
        self.locked_enemies = []

    def lock_enemy(self, enemy):
        if enemy.is_locked:
            return
        enemy.is_locked = True
        self.locked_enemies.append(enemy)
        pyxel.play(0, 0)

    def camera_axes(self):
        mat = self.effective_camera.transform
        return mat.pos, Vec3.RIGHT.to_world_dir(mat), Vec3.UP.to_world_dir(mat)

    def project(self, center):
        eye, right, up = self.camera_axes()
        forward = Vec3.FORWARD.to_world_dir(self.effective_camera.transform)
        to_enemy = center - eye
        depth = to_enemy.dot(forward)
        if depth <= 0.0:
            return None

        scale = math.tan(math.radians(self.effective_camera.fov) * 0.5)
        aspect = pyxel.width / pyxel.height
        x = pyxel.width * (0.5 + to_enemy.dot(right) / (2.0 * depth * aspect * scale))
        y = pyxel.height * (0.5 - to_enemy.dot(up) / (2.0 * depth * scale))
        return x, y

    def can_lock(self, enemy):
        pos = self.project(enemy.center)
        if pos is None:
            return False

        x, y = pos
        return math.hypot(pyxel.mouse_x - x, pyxel.mouse_y - y) < 13.0

    def start_fire(self):
        if not self.locked_enemies:
            return

        self.firing_enemies = self.locked_enemies
        self.fan_offsets = [
            self.fan_offset(enemy.center, i)
            for i, enemy in enumerate(self.firing_enemies)
        ]

        for enemy in self.firing_enemies:
            enemy.is_locked = False

        self.locked_enemies = []
        self.fire_frame = 1
        self.hit_done = False
        pyxel.play(1, 1)

    def on_update(self):
        drag_started = pyxel.btnp(pyxel.MOUSE_BUTTON_LEFT)
        update_locks = True

        if self.fire_frame:
            self.fire_frame += 1
            if self.fire_frame >= LASER_DURATION:
                self.hit_enemies()
            if self.fire_frame > LASER_DURATION + 8:
                self.finish_fire()
                update_locks = drag_started
            elif not drag_started:
                update_locks = False
            else:
                self.finish_fire()

        if update_locks:
            if drag_started:
                self.clear_locks()
            if pyxel.btn(pyxel.MOUSE_BUTTON_LEFT):
                for enemy in self.enemies:
                    if not enemy.is_locked and self.can_lock(enemy):
                        self.lock_enemy(enemy)
            elif pyxel.btnr(pyxel.MOUSE_BUTTON_LEFT) and self.locked_enemies:
                self.start_fire()

        if self.fire_frame:
            laser_extent = min(1.0, self.fire_frame / LASER_DURATION)
            for i, enemy in enumerate(self.firing_enemies):
                self.update_prim(self.outer_prims[i], enemy.center, i, laser_extent)
                self.update_prim(
                    self.core_prims[i], enemy.center, i, laser_extent, LASER_CORE_WIDTH
                )

    def fan_offset(self, enemy_center, index):
        if index < len(self.fan_offsets):
            return self.fan_offsets[index]

        _, right, _ = self.camera_axes()
        count = len(self.firing_enemies) or 1
        spread = index - (count - 1) * 0.5
        spread_unit = spread / max((count - 1) * 0.5, 1.0)
        side = (enemy_center - AIM_POINT).dot(right)
        if abs(side) <= 0.4:
            side = spread_unit or 1.0
        side_sign = 1.0 if side >= 0.0 else -1.0
        return right * (side_sign * 1.2 * (0.7 + 0.3 * abs(spread_unit)))

    def make_path(self, enemy_center, index, extent):
        extent = max(0.0, min(1.0, extent))
        to_enemy = enemy_center - LASER_ORIGIN
        direction = self.fan_offset(enemy_center, index)
        direction = (direction + to_enemy.normalize()).normalize()
        speed = to_enemy.length() * 1.15 / (LASER_POINTS - 1)
        point = LASER_ORIGIN
        full_path = [point]
        for i in range(1, LASER_POINTS):
            path_t = (i - 1) / (LASER_POINTS - 2)
            if path_t <= 0.3:
                steer = 0.0
            else:
                steer_t = (path_t - 0.3) / 0.7
                steer = steer_t * steer_t * (3.0 - 2.0 * steer_t) * 0.35
            direction = (
                direction * (1.0 - steer) + (enemy_center - point).normalize() * steer
            ).normalize()
            point += direction * speed
            full_path.append(point)

        correction = enemy_center - full_path[-1]
        for i, point in enumerate(full_path):
            progress = i / (LASER_POINTS - 1)
            full_path[i] = point + correction * (progress**2.5)
        full_path[-1] = enemy_center

        points = []
        for i in range(LASER_POINTS):
            progress = extent * i / (LASER_POINTS - 1)
            segment = min(int(progress * (LASER_POINTS - 1)), LASER_POINTS - 2)
            blend = progress * (LASER_POINTS - 1) - segment
            points.append(
                full_path[segment] * (1.0 - blend) + full_path[segment + 1] * blend
            )
        return points

    def hit_enemies(self):
        if not self.firing_enemies or self.hit_done:
            return
        self.hit_done = True
        pyxel.play(2, 2)
        for enemy in self.firing_enemies:
            enemy.trigger_hit()
            enemy.is_locked = False

    def finish_fire(self):
        if self.firing_enemies and not self.hit_done:
            self.hit_enemies()
        for enemy in self.firing_enemies:
            enemy.is_locked = False
        self.firing_enemies = []
        self.fan_offsets = []
        self.fire_frame = 0
        self.hit_done = False

    def on_draw(self):
        self.depth_test(False)
        self.shaded(False)

        if self.fire_frame:
            for i in range(len(self.firing_enemies)):
                self.prim(Mat4.IDENTITY, self.outer_prims[i], 11)
                self.prim(Mat4.IDENTITY, self.core_prims[i], 7)

    def update_prim(self, prim, enemy_center, index, extent, width=LASER_WIDTH):
        eye, right, _ = self.camera_axes()
        points = self.make_path(enemy_center, index, extent)
        positions = []
        for i, point in enumerate(points):
            if i == 0:
                tangent = points[1] - point
            elif i == len(points) - 1:
                tangent = point - points[i - 1]
            else:
                tangent = points[i + 1] - points[i - 1]

            side = tangent.cross(point - eye)
            side = side.normalize() * width if side.length() > 1e-6 else right * width
            for vertex in (point - side, point + side):
                positions += [vertex.x, vertex.y, vertex.z]
        prim.positions[:] = positions


class Scene(Node):
    def __init__(self):
        super().__init__()

        self.shading = Shading(pyxel.colors)
        self.shading.direction = Vec3(0.4, -1.0, -0.5).normalize()

        self.camera = Camera()
        self.camera.clear_color = 0

        self.enemies = [Enemy(i) for i in range(len(ENEMY_COLORS))]

        for enemy in self.enemies:
            self.add_child(enemy)

        self.laser = Laser(self.enemies)
        self.add_child(self.laser)

    def on_update(self):
        mouse_offset = pyxel.mouse_y / pyxel.height - 0.5
        eye = Vec3(CAMERA_EYE.x, CAMERA_EYE.y - mouse_offset * 4.0, CAMERA_EYE.z)
        self.camera.transform = Mat4.look_at(eye, AIM_POINT)

    def on_draw(self):
        for i in range(13):
            grid_pos = -7.0 + 14.0 * i / 12
            self.line(Vec3(grid_pos, 0, -7.0), Vec3(grid_pos, 0, 7.0), 5)
            self.line(Vec3(-7.0, 0, grid_pos), Vec3(7.0, 0, grid_pos), 5)


class App:
    def __init__(self):
        pyxel.init(256, 192, title="Custom Shapes")
        pyxel.mouse(False)

        pyxel.sounds[0].set("c4g4", "t", "46", "nn", 5)
        pyxel.sounds[1].set("c3g3c4e4g4", "s", "76543", "nnnff", 4)
        pyxel.sounds[2].set("c2g1c1", "n", "765", "fff", 5)

        self.scene = Scene()

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, pyxel.width, pyxel.height)

        for enemy in self.scene.enemies:
            if not enemy.is_locked:
                continue
            screen_pos = self.scene.laser.project(enemy.center)
            if screen_pos is None:
                continue
            screen_x, screen_y = round(screen_pos[0]), round(screen_pos[1])
            pyxel.rectb(screen_x - 4, screen_y - 4, 9, 9, 8)
            pyxel.rectb(screen_x - 5, screen_y - 5, 11, 11, 7)
            pyxel.rectb(screen_x - 6, screen_y - 6, 13, 13, 8)

        pyxel.circb(pyxel.mouse_x, pyxel.mouse_y, 7, 7)
        pyxel.circb(pyxel.mouse_x, pyxel.mouse_y, 9, 7)

        pyxel.text(76, 5, "Drag: Lock   Release: Fire", 7)


App()
