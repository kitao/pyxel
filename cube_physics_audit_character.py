import pyxel
from cube_physics_character import Character, Stage

from pyxel.cube import Camera, Mat4, Scene, Shading, Vec3

pyxel.init(160, 120)
scene = Scene()
scene.clear_color = 1
scene.shading = Shading([pyxel.colors[i] for i in range(16)])
scene.shading.direction = Vec3(0.4, -0.8, 0.2)
scene.add_child(Stage())
char = Character()
scene.add_child(char)
camera = Camera()
camera.transform = Mat4.look_at(Vec3(0, 12, 14), Vec3.ZERO, Vec3.UP)

for _ in range(120):
    scene.update()

resting_y = char.transform.pos.y
print(f"resting_y={resting_y:.3f}")
assert 0.4 < resting_y < 0.9, "character did not settle on the stage"
print("character audit OK")
pyxel.quit()
