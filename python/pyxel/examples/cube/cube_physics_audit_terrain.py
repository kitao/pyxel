import pyxel
from cube_physics_terrain import Ball, Floor

from pyxel.cube import Camera, Mat4, Node, Shading, Vec3

pyxel.init(160, 120)
scene = Node()
scene.shading = Shading([pyxel.colors[i] for i in range(16)])
scene.shading.direction = Vec3(0.4, -0.8, 0.2)
scene.add_child(Floor())
ball = Ball()
scene.add_child(ball)
camera = Camera()
camera.clear_color = 1
scene.camera = camera
camera.transform = Mat4.look_at(Vec3(0, 10, 18), Vec3.ZERO, Vec3.UP)

initial_y = ball.transform.pos.y
for _frame in range(240):
    scene.update()
final_y = ball.transform.pos.y
print(f"initial_y={initial_y:.3f}  final_y={final_y:.3f}")
assert final_y < initial_y - 0.5, "ball did not descend"
print("terrain audit OK")
pyxel.quit()
