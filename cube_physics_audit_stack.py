import pyxel
from cube_physics_stack import Can, Floor

from pyxel.cube import Camera, Mat4, Scene, Shading, Vec3

pyxel.init(160, 120)
scene = Scene()
scene.clear_color = 1
scene.shading = Shading([pyxel.colors[i] for i in range(16)])
scene.shading.direction = Vec3(0.4, -0.8, 0.2)
scene.add_child(Floor())
cans = [Can(Vec3(0, 0.4 + y * 0.85, 0)) for y in range(4)]
for c in cans:
    scene.add_child(c)
camera = Camera()
camera.transform = Mat4.look_at(Vec3(0, 4, 10), Vec3.ZERO, Vec3.UP)

for _ in range(240):
    scene.update()

ys = sorted(c.transform.pos.y for c in cans)
print(f"settled y values: {[f'{y:.2f}' for y in ys]}")
assert all(ys[i] > ys[i - 1] for i in range(1, len(ys))), "cans collapsed"
assert ys[0] < 0.8, "lowest can floats"
print("stack audit OK")
pyxel.quit()
