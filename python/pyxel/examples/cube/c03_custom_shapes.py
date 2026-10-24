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


class App(Node):
    def __init__(self):
        super().__init__()

        pyxel.init(256, 192, title="Custom Shapes")

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
        self.shading.direction = Vec3(0.4, -1.0, -0.5)

        self.camera = Camera()
        self.camera.clear_color = 0

        self.body = Primitive.sphere(1.0)
        # Keep the original vertices so deformation does not accumulate.
        self.base_positions = list(self.body.positions)
        self.centers = [Vec3.ZERO for color in ENEMY_COLORS]
        self.locked = []
        self.targets = []
        self.flashes = [0] * len(ENEMY_COLORS)

        pyxel.run(self.update_game, self.draw_game)

    def start_fire(self):
        self.age = 1
        self.targets = []

        # Spread the launch directions into a fan.
        right = Vec3.RIGHT.to_world_dir(self.camera.transform)
        center_index = (len(self.locked) - 1) * 0.5
        for index, enemy in enumerate(self.locked):
            lock_side = (index - center_index) / max(center_index, 1.0)
            target_side = (self.centers[enemy] - AIM_POINT).dot(right)
            if abs(target_side) <= 4.4:
                target_side = lock_side or 1.0
            side_sign = 1.0 if target_side >= 0.0 else -1.0
            direction = (self.centers[enemy] - LASER_ORIGIN).normalize()
            fan = side_sign * 1.2 * (0.7 + 0.3 * abs(lock_side))
            self.targets.append((enemy, (direction + right * fan).normalize()))

        # Two vertices across the texture's width, two triangles per segment.
        indices = []
        uvs = []
        for i in range(LASER_POINTS):
            uvs += [0, i / (LASER_POINTS - 1), 1, i / (LASER_POINTS - 1)]
            if i < LASER_POINTS - 1:
                j = i * 2
                indices += [j, j + 1, j + 2, j + 1, j + 3, j + 2]

        self.strip = Primitive(
            Primitive.MODE_TRIANGLES,
            [0.0] * (LASER_POINTS * 6),
            indices,
            uvs=uvs,
            cull=Primitive.CULL_NONE,
        )

        self.locked = []
        pyxel.play(1, 1)
        pyxel.play(3, 3)

    def make_path(self, enemy_center, direction, progress):
        step_length = (enemy_center - LASER_ORIGIN).length() * 1.15 / (LASER_POINTS - 1)
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

        # Bend the end of the path to meet the target.
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

    def on_draw(self):
        # Draw the ground grid.
        for i in range(13):
            grid_pos = -77.0 + 154.0 * i / 12
            self.line(Vec3(grid_pos, 0, -77.0), Vec3(grid_pos, 0, 77.0), 5)
            self.line(Vec3(-77.0, 0, grid_pos), Vec3(77.0, 0, grid_pos), 5)

        time = pyxel.frame_count * 0.06
        for index, color in enumerate(ENEMY_COLORS):
            phase = index * 1.3
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
            flash = self.flashes[index]
            color = 7 if flash and flash % 2 == 0 else color
            self.prim(Mat4.from_translation(self.centers[index]), self.body, color)

        if self.targets:
            self.depth_test(False)
            self.shaded(False)
            camera = self.camera.transform
            right = Vec3.RIGHT.to_world_dir(camera)
            progress = min(1.0, self.age / LASER_DURATION)
            for enemy, direction in self.targets:
                path = self.make_path(self.centers[enemy], direction, progress)
                positions = []
                for i, point in enumerate(path):
                    # Face each segment toward the camera.
                    tangent = path[min(i + 1, len(path) - 1)] - path[max(0, i - 1)]
                    side = tangent.cross(point - camera.pos)
                    side = side.normalize() if side.length() > 1e-6 else right
                    positions.extend(point - side * LASER_HALF_WIDTH)
                    positions.extend(point + side * LASER_HALF_WIDTH)

                self.strip.positions[:] = positions
                self.prim(Mat4.IDENTITY, self.strip, self.laser_image, colkey=0)

    def update_game(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        mouse_offset = pyxel.mouse_y / pyxel.height - 0.5
        eye = Vec3(0, 132 - mouse_offset * 44, 110)
        self.camera.transform = Mat4.look_at(eye, AIM_POINT)

        frame = pyxel.frame_count
        for i in range(len(ENEMY_COLORS)):
            self.flashes[i] = max(0, self.flashes[i] - 1)
            orbit = i * math.tau / len(ENEMY_COLORS) + frame * 0.01
            self.centers[i] = Vec3(
                38.5 * math.cos(orbit),
                27.5 + 5.5 * math.sin(frame * 0.02 + i * 1.3),
                38.5 * math.sin(orbit),
            )

        drag_started = pyxel.btnp(pyxel.MOUSE_BUTTON_LEFT)
        if self.targets:
            self.age += 1
            hit = self.age == LASER_DURATION
            # A new drag interrupts the volley and begins a new lock-on.
            if drag_started and self.age < LASER_DURATION:
                hit = True
            if hit:
                pyxel.play(2, 2)
                for enemy, direction in self.targets:
                    self.flashes[enemy] = 12
            if drag_started or self.age > LASER_DURATION + 8:
                self.targets = []
            else:
                return

        if drag_started:
            self.locked = []
        if pyxel.btn(pyxel.MOUSE_BUTTON_LEFT):
            for index, center in enumerate(self.centers):
                x, y = self.project_to_screen(center)
                distance = math.hypot(pyxel.mouse_x - x, pyxel.mouse_y - y)
                if distance < LOCK_RADIUS and index not in self.locked:
                    self.locked.append(index)
                    pyxel.play(0, 0)
        elif pyxel.btnr(pyxel.MOUSE_BUTTON_LEFT) and self.locked:
            self.start_fire()

    def draw_game(self):
        self.draw(0, 0, pyxel.width, pyxel.height)

        for index in self.locked:
            x, y = self.project_to_screen(self.centers[index])
            for r, color in [(4, 8), (5, 7), (6, 8)]:
                pyxel.rectb(round(x) - r, round(y) - r, r * 2 + 1, r * 2 + 1, color)

        pyxel.circb(pyxel.mouse_x, pyxel.mouse_y, 7, 7)
        pyxel.circb(pyxel.mouse_x, pyxel.mouse_y, 9, 7)

        pyxel.text(76, 5, "Drag: Lock   Release: Fire", 7)

    def project_to_screen(self, world_pos):
        camera = self.camera
        local = world_pos.to_local(camera.transform)
        depth = -local.z
        scale = pyxel.height / (2 * depth * math.tan(math.radians(camera.fov) / 2))
        return pyxel.width / 2 + local.x * scale, pyxel.height / 2 - local.y * scale


App()
