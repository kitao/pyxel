import math

import pyxel
from pyxel.cube import Camera, Mat4, Vec3


# Shared orbit camera helper used by the cube_physics_* samples.
# Left-drag: yaw / pitch around the target.
# Mouse wheel: zoom in / out.
class OrbitCamera:
    def __init__(
        self,
        target: Vec3 = Vec3.ZERO,
        yaw_deg: float = 0.0,
        pitch_deg: float = 30.0,
        radius: float = 18.0,
        fov: float = 60.0,
    ):
        self.target = target
        self.yaw = yaw_deg
        self.pitch = pitch_deg
        self.radius = radius
        self._prev_mx = pyxel.mouse_x
        self._prev_my = pyxel.mouse_y
        self.camera = Camera()
        self.camera.fov = fov
        self._refresh()

    def update(self):
        mx, my = pyxel.mouse_x, pyxel.mouse_y
        dx, dy = mx - self._prev_mx, my - self._prev_my
        self._prev_mx, self._prev_my = mx, my
        if pyxel.btn(pyxel.MOUSE_BUTTON_LEFT):
            self.yaw -= dx * 0.6
            self.pitch = max(-85.0, min(85.0, self.pitch + dy * 0.6))
        wheel = pyxel.mouse_wheel
        if wheel:
            self.radius = max(1.5, self.radius * (0.9 if wheel > 0 else 1.1))
        self._refresh()

    def _refresh(self):
        py = math.radians(self.pitch)
        yw = math.radians(self.yaw)
        cp = math.cos(py)
        eye = Vec3(
            self.target.x + self.radius * cp * math.sin(yw),
            self.target.y + self.radius * math.sin(py),
            self.target.z + self.radius * cp * math.cos(yw),
        )
        self.camera.transform = Mat4.look_at(eye, self.target, Vec3.UP)
