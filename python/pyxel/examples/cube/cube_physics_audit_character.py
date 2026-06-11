import pyxel
from cube_physics_character import Character, Stage

from pyxel.cube import Camera, Collider, Mat4, Node, Shading, Vec3


# Capsule probe: same push-back pattern as Character, but with the
# capsule collider shape from cube-design.md § 11.1 (size=(0, h, 0) +
# radius). It must rest on the stage mesh at half_h + radius, which
# only the capsule-vs-triangle narrow phase produces.
class CapsuleProbe(Node):
    def __init__(self):
        super().__init__()
        self.transform = Mat4.from_translation(Vec3(2, 3, 2))
        self.collider = Collider(size=Vec3(0, 1, 0), radius=0.3, mass=1.0)

    def on_update(self):
        self.collider.velocity += Vec3(0, -0.02, 0)

    def on_collide(self, other, contact):
        push = Mat4.from_translation(contact.normal * contact.depth)
        self.transform = push * self.transform
        self.collider.velocity += contact.delta_velocity


pyxel.init(160, 120)
scene = Node()
scene.shading = Shading([pyxel.colors[i] for i in range(16)])
scene.shading.direction = Vec3(0.4, -0.8, 0.2)
scene.add_child(Stage())
char = Character()
scene.add_child(char)
capsule = CapsuleProbe()
scene.add_child(capsule)
camera = Camera()
camera.clear_color = 1
scene.camera = camera
camera.transform = Mat4.look_at(Vec3(0, 12, 14), Vec3.ZERO, Vec3.UP)

for _ in range(120):
    scene.update()

resting_y = char.transform.pos.y
print(f"resting_y={resting_y:.3f}")
assert 0.4 < resting_y < 0.9, "character did not settle on the stage"
# Capsule center must settle near half_h (0.5) + radius (0.3) = 0.8.
capsule_y = capsule.transform.pos.y
print(f"capsule_resting_y={capsule_y:.3f}")
assert 0.7 < capsule_y < 0.9, "capsule did not settle at half_h + radius"
print("character audit OK")
pyxel.quit()
