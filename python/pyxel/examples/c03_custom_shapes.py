import math

import pyxel
from pyxel.cube import Camera, Mat4, Node, Primitive, Shading, Vec3

SLIME_COLORS = [8, 9, 10, 12, 14]
CAMERA_EYE = Vec3(0.0, 12.0, 10.0)
AIM_CENTER = Vec3(0.0, 1.0, 0.0)
LASER_ORIGIN = Vec3(0.0, 0.5, 10.0)
FIRE_DURATION = 14
LASER_WIDTH = 0.22


# Build a Primitive body once, then move only its vertex positions each frame.
def build_sphere(rings, segments):
    directions = []
    for i in range(rings + 1):
        lat = math.pi * i / rings - math.pi / 2
        for j in range(segments + 1):
            lon = math.tau * j / segments
            directions.append(
                (
                    math.cos(lat) * math.cos(lon),
                    math.sin(lat),
                    math.cos(lat) * math.sin(lon),
                )
            )

    indices = []
    for i in range(rings):
        for j in range(segments):
            a = i * (segments + 1) + j
            c = a + segments + 1
            indices += [a, a + 1, c + 1, a, c + 1, c]

    normals = []
    for i in range(0, len(indices), 3):
        d0 = directions[indices[i]]
        d1 = directions[indices[i + 1]]
        d2 = directions[indices[i + 2]]
        nx = d0[0] + d1[0] + d2[0]
        ny = d0[1] + d1[1] + d2[1]
        nz = d0[2] + d1[2] + d2[2]
        length = math.sqrt(nx * nx + ny * ny + nz * nz) or 1.0
        normals += [nx / length, ny / length, nz / length]
    return directions, indices, normals


def wobble_radius(direction, time, phase):
    return (
        1.0
        + 0.25 * math.sin(3.0 * direction[0] + 1.7 * time + phase)
        + 0.22 * math.sin(3.5 * direction[1] + 1.3 * time + phase * 1.7)
        + 0.20 * math.sin(4.0 * direction[2] + 2.1 * time + phase * 0.6)
    )


def smoothstep(edge0, edge1, x):
    if x <= edge0:
        return 0.0
    if x >= edge1:
        return 1.0
    x = (x - edge0) / (edge1 - edge0)
    return x * x * (3.0 - 2.0 * x)


def init_sounds():
    pyxel.sounds[0].set("c4g4", "t", "46", "nn", 5)
    pyxel.sounds[1].set("c3g3c4e4g4", "s", "76543", "nnnff", 4)
    pyxel.sounds[2].set("c2g1c1", "n", "765", "fff", 5)


class Slime(Node):
    def __init__(self, index):
        super().__init__()
        self.phase = index * 1.3
        self.orbit = index * math.tau / len(SLIME_COLORS)
        self.color = SLIME_COLORS[index]
        self.center = Vec3.ZERO
        self.is_locked = False
        self.flash_timer = 0

        self.sphere_dirs, indices, normals = build_sphere(6, 10)
        self.body = Primitive(
            Primitive.MODE_TRIANGLES,
            self.make_body_positions(0.0),
            indices,
            normals=normals,
            cull=Primitive.CULL_NONE,
        )

    def make_body_positions(self, t):
        positions = []
        for direction in self.sphere_dirs:
            radius = wobble_radius(direction, t, self.phase)
            x, y, z = direction
            positions += [x * radius, y * radius, z * radius]
        return positions

    def trigger_hit(self):
        self.flash_timer = 12

    def on_update(self):
        t = pyxel.frame_count
        self.body.positions[:] = self.make_body_positions(t * 0.06)
        self.center = Vec3(
            3.5 * math.cos(self.orbit + t * 0.01),
            2.5 + 0.5 * math.sin(t * 0.02 + self.phase),
            3.5 * math.sin(self.orbit + t * 0.01),
        )
        self.transform = Mat4.from_translation(self.center)
        self.flash_timer = max(0, self.flash_timer - 1)

    def on_draw(self):
        color = 7 if self.flash_timer % 2 == 0 and self.flash_timer else self.color
        self.prim(Mat4.IDENTITY, self.body, color)


class Floor(Node):
    def on_draw(self):
        for i in range(13):
            u = -7.0 + 14.0 * i / 12
            self.line(Vec3(u, 0, -7.0), Vec3(u, 0, 7.0), 5)
            self.line(Vec3(-7.0, 0, u), Vec3(7.0, 0, u), 5)


class Weapon(Node):
    def __init__(self, slimes):
        super().__init__()
        self.slimes = slimes
        self.locked_slimes = []
        self.firing_slimes = []
        self.fan_offsets = []
        self.fire_frame = 0
        self.hit_done = False

        self.laser_prims = [self.make_laser_prim() for _ in slimes]

    def make_laser_prim(self):
        point_count = 24
        indices = []
        for i in range(point_count - 1):
            a = i * 2
            indices += [a, a + 1, a + 2, a + 1, a + 3, a + 2]
        return Primitive(
            Primitive.MODE_TRIANGLES,
            [0.0] * (point_count * 2 * 3),
            indices,
            cull=Primitive.CULL_NONE,
        )

    def clear_locks(self):
        for slime in self.slimes:
            slime.is_locked = False
        self.locked_slimes = []

    def lock_slime(self, slime):
        if slime.is_locked:
            return
        slime.is_locked = True
        self.locked_slimes.append(slime)
        pyxel.play(0, 0)

    def camera_axes(self):
        mat = self.effective_camera.transform
        return mat.pos, Vec3.RIGHT.to_world_dir(mat), Vec3.UP.to_world_dir(mat)

    def can_lock(self, slime):
        eye, right, up = self.camera_axes()
        forward = Vec3.FORWARD.to_world_dir(self.effective_camera.transform)
        to_slime = slime.center - eye
        depth = to_slime.dot(forward)
        if depth <= 0.0:
            return False

        scale = math.tan(math.radians(self.effective_camera.fov) * 0.5)
        aspect = pyxel.width / pyxel.height
        x = pyxel.width * (
            0.5 + to_slime.dot(right) / (2.0 * depth * aspect * scale)
        )
        y = pyxel.height * (0.5 - to_slime.dot(up) / (2.0 * depth * scale))
        return math.hypot(pyxel.mouse_x - x, pyxel.mouse_y - y) < 13.0

    def start_fire(self):
        if not self.locked_slimes:
            return

        self.firing_slimes = self.locked_slimes
        self.fan_offsets = [
            self.laser_fan_offset(s.center, i) for i, s in enumerate(self.firing_slimes)
        ]
        self.locked_slimes = []
        self.fire_frame = 1
        self.hit_done = False
        pyxel.play(1, 1)

    def on_update(self):
        drag_started = pyxel.btnp(pyxel.MOUSE_BUTTON_LEFT)

        if self.fire_frame:
            self.fire_frame += 1
            if self.fire_frame >= FIRE_DURATION:
                self.hit_slimes()
            if self.fire_frame > FIRE_DURATION + 8:
                self.finish_fire()
            if not drag_started:
                return
            self.finish_fire()

        if drag_started:
            self.clear_locks()
        if pyxel.btn(pyxel.MOUSE_BUTTON_LEFT):
            for slime in self.slimes:
                if not slime.is_locked and self.can_lock(slime):
                    self.lock_slime(slime)
        elif pyxel.btnr(pyxel.MOUSE_BUTTON_LEFT) and self.locked_slimes:
            self.start_fire()

    def laser_fan_offset(self, slime_center, index):
        if index < len(self.fan_offsets):
            return self.fan_offsets[index]

        _, right, _ = self.camera_axes()
        count = len(self.firing_slimes) or 1
        spread = index - (count - 1) * 0.5
        spread_unit = spread / max((count - 1) * 0.5, 1.0)
        side = (slime_center - AIM_CENTER).dot(right)
        if abs(side) <= 0.4:
            side = spread_unit or 1.0
        side_sign = 1.0 if side >= 0.0 else -1.0
        return right * (side_sign * 1.2 * (0.7 + 0.3 * abs(spread_unit)))

    def make_laser_path(self, slime_center, index, extent):
        point_count = 24
        extent = max(0.0, min(1.0, extent))
        to_slime = slime_center - LASER_ORIGIN
        direction = self.laser_fan_offset(slime_center, index)
        direction = (direction + to_slime.normalize()).normalize()
        speed = to_slime.length() * 1.15 / (point_count - 1)
        point = LASER_ORIGIN
        full_path = [point]
        for i in range(1, point_count):
            t = (i - 1) / (point_count - 2)
            steer = smoothstep(0.3, 1.0, t) * 0.35
            direction = (
                direction * (1.0 - steer) + (slime_center - point).normalize() * steer
            ).normalize()
            point += direction * speed
            full_path.append(point)

        correction = slime_center - full_path[-1]
        for i, point in enumerate(full_path):
            amount = i / (point_count - 1)
            full_path[i] = point + correction * (amount**2.5)
        full_path[-1] = slime_center

        points = []
        for i in range(point_count):
            amount = extent * i / (point_count - 1)
            segment = min(int(amount * (point_count - 1)), point_count - 2)
            blend = amount * (point_count - 1) - segment
            points.append(
                full_path[segment] * (1.0 - blend) + full_path[segment + 1] * blend
            )
        return points

    def hit_slimes(self):
        if not self.firing_slimes or self.hit_done:
            return
        self.hit_done = True
        pyxel.play(2, 2)
        for slime in self.firing_slimes:
            slime.trigger_hit()
            slime.is_locked = False

    def finish_fire(self):
        if self.firing_slimes and not self.hit_done:
            self.hit_slimes()
        for slime in self.firing_slimes:
            slime.is_locked = False
        self.firing_slimes = []
        self.fan_offsets = []
        self.fire_frame = 0
        self.hit_done = False

    def on_draw(self):
        self.depth_test(False)
        self.shaded(False)

        for slime in self.slimes:
            if slime.is_locked:
                self.draw_marker(slime.center)
        if self.fire_frame:
            extent = min(1.0, self.fire_frame / FIRE_DURATION)
            for i, slime in enumerate(self.firing_slimes):
                self.update_laser_prim(self.laser_prims[i], slime.center, i, extent)
                self.prim(Mat4.IDENTITY, self.laser_prims[i], 11)
                self.update_laser_prim(
                    self.laser_prims[i], slime.center, i, extent, LASER_WIDTH * 0.35
                )
                self.prim(Mat4.IDENTITY, self.laser_prims[i], 7)

        pyxel.circb(pyxel.mouse_x, pyxel.mouse_y, 9, 7)

    def draw_marker(self, center):
        _, right, up = self.camera_axes()
        color = (8, 14)[pyxel.frame_count % 2]
        size = 0.55
        corners = [
            center + right * size + up * size,
            center + right * size - up * size,
            center - right * size - up * size,
            center - right * size + up * size,
        ]
        for p, q in zip(corners, corners[1:] + corners[:1]):
            self.line(p, q, color)

    def update_laser_prim(self, laser_prim, slime_center, index, extent, width=LASER_WIDTH):
        eye, right, _ = self.camera_axes()
        points = self.make_laser_path(slime_center, index, extent)
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
            for p in (point - side, point + side):
                positions += [p.x, p.y, p.z]
        laser_prim.positions[:] = positions


class Scene(Node):
    def __init__(self):
        super().__init__()

        self.shading = Shading(pyxel.colors)
        self.shading.direction = Vec3(0.4, -1.0, -0.5).normalize()

        self.camera = Camera()
        self.camera.clear_color = 0
        self.camera.transform = Mat4.look_at(CAMERA_EYE, AIM_CENTER)

        self.add_child(Floor())
        slimes = [Slime(i) for i in range(len(SLIME_COLORS))]
        for slime in slimes:
            self.add_child(slime)
        self.add_child(Weapon(slimes))


class App:
    def __init__(self):
        pyxel.init(256, 192, title="Custom Shapes")
        pyxel.mouse(False)

        init_sounds()
        self.scene = Scene()

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, pyxel.width, pyxel.height)

        pyxel.text(76, 5, "Drag: Lock   Release: Fire", 6)


App()
