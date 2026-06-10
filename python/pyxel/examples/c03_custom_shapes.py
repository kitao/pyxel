import math

import pyxel
from pyxel.cube import Camera, Mat4, Node, PrimData, Shading, Vec3

ENEMY_COUNT = 5
ENEMY_COLORS = [11, 3, 14, 12, 10]
RINGS = 6
SEGS = 10
SIZE = 1.0
DRIFT_R = 3.5
FLOAT_Y = 2.5
GRID_LINES = 12
GRID_HALF = 7.0
FLOOR_COLOR = 5

EYE = Vec3(0.0, 12.0, 10.0)
TARGET = Vec3(0.0, 1.0, 0.0)
EMITTER = Vec3(0.0, 0.5, 7.0)
AIM_W = 9.0
AIM_H = 7.0
LOCK_DIST = 1.6
FIRE_FRAMES = 14
HOLD_FRAMES = 8
FLASH_FRAMES = 6
FAN = 0.35
LAUNCH = 6.0
N_PTS = 12
BEAM_W = 0.25
RETICLE_SIZE = 0.7
MARK_SIZE = 1.0
RETICLE_COLOR = 7
MARK_COLOR = 8
FLASH_COLOR = 7


def build_sphere(rings, segs):
    dirs = []
    for i in range(rings + 1):
        lat = math.pi * i / rings - math.pi / 2
        for j in range(segs + 1):
            lon = 2 * math.pi * j / segs
            dirs.append(
                (
                    math.cos(lat) * math.cos(lon),
                    math.sin(lat),
                    math.cos(lat) * math.sin(lon),
                )
            )
    indices = []
    for i in range(rings):
        for j in range(segs):
            a = i * (segs + 1) + j
            b = a + 1
            c = a + (segs + 1)
            d = c + 1
            indices += [a, b, d, a, d, c]
    normals = []
    for f in range(0, len(indices), 3):
        i0, i1, i2 = indices[f], indices[f + 1], indices[f + 2]
        nx = dirs[i0][0] + dirs[i1][0] + dirs[i2][0]
        ny = dirs[i0][1] + dirs[i1][1] + dirs[i2][1]
        nz = dirs[i0][2] + dirs[i1][2] + dirs[i2][2]
        length = math.sqrt(nx * nx + ny * ny + nz * nz) or 1.0
        normals += [nx / length, ny / length, nz / length]
    return dirs, indices, normals


def wobble_radius(d, t, phase):
    r = 1.0
    r += 0.25 * math.sin(3.0 * d[0] + 1.7 * t + phase)
    r += 0.22 * math.sin(3.5 * d[1] + 1.3 * t + phase * 1.7)
    r += 0.20 * math.sin(4.0 * d[2] + 2.1 * t + phase * 0.6)
    return r


def make_laser_texture():
    tex = pyxel.Image(8, 8)
    tex.set(
        0,
        0,
        [
            "00000000",
            "cccccccc",
            "77777777",
            "77777777",
            "77777777",
            "77777777",
            "cccccccc",
            "00000000",
        ],
    )
    return tex


class Enemy(Node):
    def __init__(self, index):
        super().__init__()
        self.phase = index * 1.3
        self.orbit = index * (2 * math.pi / ENEMY_COUNT)
        self.color = ENEMY_COLORS[index % len(ENEMY_COLORS)]
        self.center = Vec3(0.0, FLOAT_Y, 0.0)
        self.locked = False
        self.flash = 0
        self.dirs, indices, normals = build_sphere(RINGS, SEGS)
        self.body = PrimData(
            PrimData.MODE_TRIANGLES,
            self.wobbled(0.0),
            indices,
            normals=normals,
            cull=PrimData.CULL_NONE,
        )

    def wobbled(self, t):
        pos = []
        for d in self.dirs:
            r = SIZE * wobble_radius(d, t, self.phase)
            pos += [d[0] * r, d[1] * r, d[2] * r]
        return pos

    def on_update(self):
        self.body.positions[:] = self.wobbled(pyxel.frame_count * 0.06)
        cx = DRIFT_R * math.cos(self.orbit + pyxel.frame_count * 0.01)
        cz = DRIFT_R * math.sin(self.orbit + pyxel.frame_count * 0.01)
        cy = FLOAT_Y + 0.5 * math.sin(pyxel.frame_count * 0.02 + self.phase)
        self.center = Vec3(cx, cy, cz)
        self.transform = Mat4.from_translation(self.center)
        if self.flash > 0:
            self.flash -= 1

    def on_draw(self):
        self.prim(
            Mat4.IDENTITY, self.body, FLASH_COLOR if self.flash > 0 else self.color
        )


class Floor(Node):
    def __init__(self):
        super().__init__()
        pos = []
        for k in range(GRID_LINES + 1):
            u = -GRID_HALF + 2 * GRID_HALF * k / GRID_LINES
            pos += [u, 0.0, -GRID_HALF, u, 0.0, GRID_HALF]
            pos += [-GRID_HALF, 0.0, u, GRID_HALF, 0.0, u]
        self.grid = PrimData(
            PrimData.MODE_LINES,
            pos,
            list(range(len(pos) // 3)),
            cull=PrimData.CULL_NONE,
        )

    def on_draw(self):
        self.prim(Mat4.IDENTITY, self.grid, FLOOR_COLOR)


class Weapon(Node):
    def __init__(self, enemies):
        super().__init__()
        self.enemies = enemies
        forward = (TARGET - EYE).normalize()
        self.cam_right = forward.cross(Vec3(0.0, 1.0, 0.0)).normalize()
        self.cam_up = self.cam_right.cross(forward)
        self.reticle = TARGET
        self.locked = []
        self.firing = []
        self.fire_t = 0
        self.laser_tex = make_laser_texture()
        self.beams = [self.make_ribbon() for _ in range(ENEMY_COUNT)]
        self.marker = PrimData(
            PrimData.MODE_LINES,
            [0.0] * 12,
            [0, 1, 1, 2, 2, 3, 3, 0],
            cull=PrimData.CULL_NONE,
        )

    def make_ribbon(self):
        uvs = []
        indices = []
        for k in range(N_PTS):
            u = k / (N_PTS - 1)
            uvs += [u, 0.07, u, 0.93]
        for k in range(N_PTS - 1):
            a = 2 * k
            indices += [a, a + 1, a + 2, a + 1, a + 3, a + 2]
        return PrimData(
            PrimData.MODE_TRIANGLES,
            [0.0] * (2 * N_PTS * 3),
            indices,
            uvs=uvs,
            cull=PrimData.CULL_NONE,
        )

    def clear_locks(self):
        for e in self.enemies:
            e.locked = False
        self.locked = []

    def on_update(self):
        u = (pyxel.mouse_x / pyxel.width - 0.5) * AIM_W
        v = -(pyxel.mouse_y / pyxel.height - 0.5) * AIM_H
        self.reticle = TARGET + self.cam_right * u + self.cam_up * v

        if self.fire_t > 0:
            self.fire_t += 1
            if self.fire_t > FIRE_FRAMES + HOLD_FRAMES:
                for e in self.firing:
                    e.flash = FLASH_FRAMES
                    e.locked = False
                self.firing = []
                self.fire_t = 0
            return

        if pyxel.btnp(pyxel.MOUSE_BUTTON_LEFT):
            self.clear_locks()
        if pyxel.btn(pyxel.MOUSE_BUTTON_LEFT):
            ray = self.reticle - EYE
            for e in self.enemies:
                if not e.locked:
                    w = e.center - EYE
                    if (w - w.project(ray)).length() < LOCK_DIST:
                        e.locked = True
                        self.locked.append(e)
        elif pyxel.btnr(pyxel.MOUSE_BUTTON_LEFT) and self.locked:
            self.firing = self.locked
            self.locked = []
            self.fire_t = 1

    def update_beam(self, beam, target, idx, extent):
        angle = (idx - (len(self.firing) - 1) * 0.5) * FAN
        launch = self.cam_up * math.cos(angle) + self.cam_right * math.sin(angle)
        ctrl = EMITTER + launch * LAUNCH
        pts = []
        for k in range(N_PTS):
            s = extent * k / (N_PTS - 1)
            inv = 1.0 - s
            pts.append(
                EMITTER * (inv * inv) + ctrl * (2.0 * inv * s) + target * (s * s)
            )
        pos = []
        for k in range(N_PTS):
            if k == 0:
                d = pts[1] - pts[0]
            elif k == N_PTS - 1:
                d = pts[k] - pts[k - 1]
            else:
                d = pts[k + 1] - pts[k - 1]
            perp = d.cross(pts[k] - EYE)
            length = perp.length()
            perp = (
                perp * (BEAM_W / length) if length > 1e-6 else self.cam_right * BEAM_W
            )
            left = pts[k] - perp
            right = pts[k] + perp
            pos += [left.x, left.y, left.z, right.x, right.y, right.z]
        beam.positions[:] = pos

    def square(self, center, s):
        a = center + self.cam_right * s + self.cam_up * s
        b = center + self.cam_right * s - self.cam_up * s
        c = center - self.cam_right * s - self.cam_up * s
        d = center - self.cam_right * s + self.cam_up * s
        self.marker.positions[:] = [
            a.x,
            a.y,
            a.z,
            b.x,
            b.y,
            b.z,
            c.x,
            c.y,
            c.z,
            d.x,
            d.y,
            d.z,
        ]

    def on_draw(self):
        self.depth_test(False)
        self.shaded(False)
        self.square(self.reticle, RETICLE_SIZE)
        self.prim(Mat4.IDENTITY, self.marker, RETICLE_COLOR)
        for e in self.enemies:
            if e.locked:
                self.square(e.center, MARK_SIZE)
                self.prim(Mat4.IDENTITY, self.marker, MARK_COLOR)
        if self.fire_t > 0:
            extent = min(1.0, self.fire_t / FIRE_FRAMES)
            for i, e in enumerate(self.firing):
                self.update_beam(self.beams[i], e.center, i, extent)
                self.prim(Mat4.IDENTITY, self.beams[i], self.laser_tex, colkey=0)


class Scene(Node):
    def __init__(self):
        super().__init__()

        self.shading = Shading(pyxel.colors)
        self.shading.direction = Vec3(0.4, -1.0, -0.5).normalize()

        self.camera = Camera()
        self.camera.clear_color = 0
        self.camera.transform = Mat4.look_at(EYE, TARGET)

        self.add_child(Floor())
        enemies = [Enemy(i) for i in range(ENEMY_COUNT)]
        for e in enemies:
            self.add_child(e)
        self.add_child(Weapon(enemies))


class App:
    def __init__(self):
        pyxel.init(256, 192, title="Custom Shapes")
        self.scene = Scene()
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, pyxel.width, pyxel.height)

        pyxel.text(4, 4, "Drag: lock   Release: fire", 7)


App()
