import pyxel
from cube_physics_shoot import Bullet, Target

from pyxel.cube import Camera, Mat4, Node, Shading, Vec3

pyxel.init(160, 120)
scene = Node()
scene.shading = Shading([pyxel.colors[i] for i in range(16)])
scene.shading.direction = Vec3(0.4, -0.8, 0.2)
target = Target(Vec3(0, 0, 0))
scene.add_child(target)
bullet = Bullet(Vec3(0, 0, 8), Vec3(0, 0, -0.5))
scene.add_child(bullet)
camera = Camera()
camera.clear_color = 1
scene.camera = camera
camera.transform = Mat4.look_at(Vec3(0, 2, 8), Vec3.ZERO, Vec3.UP)

target_destroyed_at = -1
bullet_destroyed_at = -1
for frame in range(60):
    scene.update()
    if target.destroyed and target_destroyed_at == -1:
        target_destroyed_at = frame
    if bullet.destroyed and bullet_destroyed_at == -1:
        bullet_destroyed_at = frame
    if target.destroyed and bullet.destroyed:
        break

print(
    f"target_destroyed_at={target_destroyed_at}  bullet_destroyed_at={bullet_destroyed_at}"
)
assert target.destroyed and bullet.destroyed, "target / bullet not destroyed"
print("shoot audit OK")
pyxel.quit()
