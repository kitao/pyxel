import pyxel
from cube_physics_stack import CAN_LAYOUT, Bullet, Can, Floor

from pyxel.cube import Camera, Mat4, Node, Shading, Vec3

pyxel.init(160, 120)
scene = Node()
scene.shading = Shading([pyxel.colors[i] for i in range(16)])
scene.shading.direction = Vec3(0.4, -0.8, 0.2)
scene.add_child(Floor())
cans = [Can(pos) for pos in CAN_LAYOUT]
for c in cans:
    scene.add_child(c)
camera = Camera()
camera.clear_color = 1
scene.camera = camera
camera.transform = Mat4.look_at(Vec3(0, 4, 10), Vec3.ZERO, Vec3.UP)

# Phase 1: the pyramid must settle near its build positions (the
# single-pass resolver leaves ~0.1 sink; lateral drift means collapse).
for _ in range(240):
    scene.update()
drifts = []
for can, start in zip(cans, CAN_LAYOUT):
    p = can.transform.pos
    drifts.append(max(abs(p.x - start.x), abs(p.y - start.y), abs(p.z - start.z)))
print(f"settle drifts: {[f'{d:.2f}' for d in drifts]}")
assert max(drifts) < 0.25, "pyramid did not hold its formation"

# Phase 2: a bullet through the middle must knock cans away laterally.
scene.add_child(Bullet(Vec3(0, 1.0, 8), Vec3(0, 0, -0.4)))
for _ in range(120):
    scene.update()
moved = sum(
    1
    for can, start in zip(cans, CAN_LAYOUT)
    if max(abs(can.transform.pos.x - start.x), abs(can.transform.pos.z - start.z)) > 0.5
)
print(f"cans knocked away: {moved}")
assert moved >= 1, "bullet did not knock any can"
print("stack audit OK")
pyxel.quit()
