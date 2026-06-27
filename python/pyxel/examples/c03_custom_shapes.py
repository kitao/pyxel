import math

import pyxel
from pyxel.cube import Camera, Mat4, Node, Primitive, Shading, Vec3

ENEMY_COLORS = [8, 9, 10, 12, 14]
LOCK_RADIUS = 13.0

LASER_ORIGIN = Vec3(0.0, 0.5, 10.0)
LASER_POINTS = 24
LASER_DURATION = 14
LASER_OUTER_WIDTH = 0.22
LASER_CORE_WIDTH = 0.08

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

        self.body = Primitive.sphere(1.0)
        self.base_positions = list(self.body.positions)

    def update_body(self, time):
        phase = self.phase
        positions = []
        for i in range(0, len(self.base_positions), 3):
            x, y, z = self.base_positions[i : i + 3]
            radius = (
                1.0
                + 0.25 * math.sin(3.0 * x + 1.7 * time + phase)
                + 0.22 * math.sin(3.5 * y + 1.3 * time + phase * 1.7)
                + 0.20 * math.sin(4.0 * z + 2.1 * time + phase * 0.6)
            )
            positions += [x * radius, y * radius, z * radius]
        self.body.positions[:] = positions
        self.body.compute_normals()

    def on_update(self):
        frame = pyxel.frame_count
        self.update_body(frame * 0.06)

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
        self.launch_directions = []
        self.fire_frame = 0

        self.outer_beams = [self.make_beam() for _ in enemies]
        self.core_beams = [self.make_beam() for _ in enemies]

    def make_beam(self):
        beam_indices = []
        for point_index in range(LASER_POINTS - 1):
            base_index = point_index * 2
            beam_indices += [
                base_index,
                base_index + 1,
                base_index + 2,
                base_index + 1,
                base_index + 3,
                base_index + 2,
            ]
        return Primitive(
            Primitive.MODE_TRIANGLES,
            [0.0] * (LASER_POINTS * 2 * 3),
            beam_indices,
            cull=Primitive.CULL_NONE,
        )

    def clear_locks(self):
        for enemy in self.enemies:
            enemy.is_locked = False
        self.locked_enemies = []

    def lock_enemy(self, enemy):
        enemy.is_locked = True
        self.locked_enemies.append(enemy)
        pyxel.play(0, 0)

    def camera_axes(self):
        camera_transform = self.effective_camera.transform
        return (
            camera_transform.pos,
            Vec3.RIGHT.to_world_dir(camera_transform),
            Vec3.UP.to_world_dir(camera_transform),
        )

    def project_to_screen(self, world_pos):
        camera_pos, right, up = self.camera_axes()
        forward = Vec3.FORWARD.to_world_dir(self.effective_camera.transform)
        view_vector = world_pos - camera_pos
        depth = view_vector.dot(forward)
        if depth <= 0.0:
            return None

        scale = math.tan(math.radians(self.effective_camera.fov) * 0.5)
        aspect = pyxel.width / pyxel.height
        screen_x = pyxel.width * (
            0.5 + view_vector.dot(right) / (2.0 * depth * aspect * scale)
        )
        screen_y = pyxel.height * (0.5 - view_vector.dot(up) / (2.0 * depth * scale))
        return screen_x, screen_y

    def can_lock(self, enemy):
        screen_pos = self.project_to_screen(enemy.center)
        if screen_pos is None:
            return False

        screen_x, screen_y = screen_pos
        mouse_distance = math.hypot(pyxel.mouse_x - screen_x, pyxel.mouse_y - screen_y)
        return mouse_distance < LOCK_RADIUS

    def start_fire(self):
        if not self.locked_enemies:
            return

        self.firing_enemies = list(self.locked_enemies)
        self.launch_directions = [
            self.make_launch_direction(enemy.center, enemy_index)
            for enemy_index, enemy in enumerate(self.firing_enemies)
        ]

        for enemy in self.firing_enemies:
            enemy.is_locked = False

        self.locked_enemies = []
        self.fire_frame = 1
        pyxel.play(1, 1)

    def on_update(self):
        drag_started = pyxel.btnp(pyxel.MOUSE_BUTTON_LEFT)

        if self.fire_frame:
            self.fire_frame += 1
            if self.fire_frame == LASER_DURATION:
                self.hit_enemies()
            if drag_started or self.fire_frame > LASER_DURATION + 8:
                self.finish_fire()
                if not drag_started:
                    return

            if self.fire_frame:
                laser_progress = min(1.0, self.fire_frame / LASER_DURATION)
                for enemy_index, enemy in enumerate(self.firing_enemies):
                    self.update_beam(
                        self.outer_beams[enemy_index],
                        enemy.center,
                        enemy_index,
                        laser_progress,
                    )
                    self.update_beam(
                        self.core_beams[enemy_index],
                        enemy.center,
                        enemy_index,
                        laser_progress,
                        LASER_CORE_WIDTH,
                    )
                return

        if drag_started:
            self.clear_locks()
        if pyxel.btn(pyxel.MOUSE_BUTTON_LEFT):
            for enemy in self.enemies:
                if not enemy.is_locked and self.can_lock(enemy):
                    self.lock_enemy(enemy)
        elif pyxel.btnr(pyxel.MOUSE_BUTTON_LEFT) and self.locked_enemies:
            self.start_fire()

    def make_launch_direction(self, enemy_center, enemy_index):
        _, right, _ = self.camera_axes()
        enemy_count = len(self.firing_enemies)
        center_index = (enemy_count - 1) * 0.5
        lock_side = (enemy_index - center_index) / max(center_index, 1.0)
        target_side = (enemy_center - AIM_POINT).dot(right)
        if abs(target_side) <= 0.4:
            target_side = lock_side
            if target_side == 0.0:
                target_side = 1.0
        side_sign = 1.0 if target_side >= 0.0 else -1.0
        target_direction = (enemy_center - LASER_ORIGIN).normalize()
        fan_strength = side_sign * 1.2 * (0.7 + 0.3 * abs(lock_side))
        return (target_direction + right * fan_strength).normalize()

    def make_visible_laser_path(self, enemy_center, enemy_index, laser_progress):
        laser_progress = max(0.0, min(1.0, laser_progress))
        to_enemy = enemy_center - LASER_ORIGIN
        direction = self.launch_directions[enemy_index]
        step_length = to_enemy.length() * 1.15 / (LASER_POINTS - 1)
        current_point = LASER_ORIGIN
        full_path = [current_point]
        for point_index in range(1, LASER_POINTS):
            path_ratio = (point_index - 1) / (LASER_POINTS - 2)
            if path_ratio <= 0.3:
                steer_strength = 0.0
            else:
                steer_ratio = (path_ratio - 0.3) / 0.7
                steer_strength = (
                    steer_ratio * steer_ratio * (3.0 - 2.0 * steer_ratio) * 0.35
                )
            direction = (
                direction * (1.0 - steer_strength)
                + (enemy_center - current_point).normalize() * steer_strength
            ).normalize()
            current_point += direction * step_length
            full_path.append(current_point)

        end_correction = enemy_center - full_path[-1]
        for point_index, point in enumerate(full_path):
            progress = point_index / (LASER_POINTS - 1)
            full_path[point_index] = point + end_correction * (progress**2.5)
        full_path[-1] = enemy_center

        visible_path = []
        for point_index in range(LASER_POINTS):
            progress = laser_progress * point_index / (LASER_POINTS - 1)
            segment_index = min(int(progress * (LASER_POINTS - 1)), LASER_POINTS - 2)
            segment_blend = progress * (LASER_POINTS - 1) - segment_index
            visible_path.append(
                full_path[segment_index] * (1.0 - segment_blend)
                + full_path[segment_index + 1] * segment_blend
            )
        return visible_path

    def hit_enemies(self):
        if not self.firing_enemies:
            return
        pyxel.play(2, 2)
        for enemy in self.firing_enemies:
            enemy.flash_timer = 12
            enemy.is_locked = False

    def finish_fire(self):
        if self.fire_frame < LASER_DURATION:
            self.hit_enemies()
        for enemy in self.firing_enemies:
            enemy.is_locked = False
        self.firing_enemies = []
        self.launch_directions = []
        self.fire_frame = 0

    def on_draw(self):
        self.depth_test(False)
        self.shaded(False)

        if self.fire_frame:
            for enemy_index in range(len(self.firing_enemies)):
                self.prim(Mat4.IDENTITY, self.outer_beams[enemy_index], 11)
                self.prim(Mat4.IDENTITY, self.core_beams[enemy_index], 7)

    def update_beam(
        self, beam, enemy_center, enemy_index, laser_progress, width=LASER_OUTER_WIDTH
    ):
        camera_pos, right, _ = self.camera_axes()
        path_points = self.make_visible_laser_path(
            enemy_center, enemy_index, laser_progress
        )
        beam_positions = []
        for point_index, point in enumerate(path_points):
            prev_point = path_points[max(0, point_index - 1)]
            next_point = path_points[min(len(path_points) - 1, point_index + 1)]
            tangent = next_point - prev_point
            side_vector = tangent.cross(point - camera_pos)
            side_vector = (
                side_vector.normalize() * width
                if side_vector.length() > 1e-6
                else right * width
            )
            for vertex in (point - side_vector, point + side_vector):
                beam_positions += [vertex.x, vertex.y, vertex.z]
        beam.positions[:] = beam_positions


class Scene(Node):
    def __init__(self):
        super().__init__()

        self.shading = Shading(pyxel.colors)
        self.shading.direction = Vec3(0.4, -1.0, -0.5).normalize()

        self.camera = Camera()
        self.camera.clear_color = 0

        self.enemies = [Enemy(index) for index in range(len(ENEMY_COLORS))]

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
            screen_pos = self.scene.laser.project_to_screen(enemy.center)
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
