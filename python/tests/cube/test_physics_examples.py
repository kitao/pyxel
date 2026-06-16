import sys
from pathlib import Path

import pyxel

from pyxel.cube import Camera, Collider, Mat4, Node, Shading, Vec3

EXAMPLES_CUBE_DIR = Path(__file__).parents[2] / "pyxel" / "examples" / "cube"
sys.path.insert(0, str(EXAMPLES_CUBE_DIR))

from cube_physics_character import Character, Stage  # noqa: E402
from cube_physics_shoot import Bullet, Target  # noqa: E402
from cube_physics_stack import CAN_LAYOUT, Bullet as StackBullet, Can, Floor  # noqa: E402
from cube_physics_terrain import Ball, Floor as TerrainFloor  # noqa: E402


def _set_scene_defaults(scene: Node) -> None:
    scene.shading = Shading([pyxel.colors[i] for i in range(16)])
    scene.shading.direction = Vec3(0.4, -0.8, 0.2)
    camera = Camera()
    camera.clear_color = 1
    camera.transform = Mat4.look_at(Vec3(0, 10, 18), Vec3.ZERO, Vec3.UP)
    scene.camera = camera


class _CapsuleProbe(Node):
    def __init__(self):
        super().__init__()
        self.transform = Mat4.from_translation(Vec3(2, 3, 2))
        self.collider = Collider(size=Vec3(0, 1, 0), radius=0.3, mass=1.0)

    def on_update(self):
        self.collider.velocity += Vec3(0, -0.02, 0)

    def on_collide(self, other, contact):
        del other
        push = Mat4.from_translation(contact.normal * contact.depth)
        self.transform = push * self.transform
        self.collider.velocity += contact.delta_velocity


def test_character_and_capsule_settle_on_stage_mesh():
    scene = Node()
    _set_scene_defaults(scene)
    scene.add_child(Stage())
    char = Character()
    scene.add_child(char)
    capsule = _CapsuleProbe()
    scene.add_child(capsule)

    for _ in range(120):
        scene.update()

    assert 0.4 < char.transform.pos.y < 0.9
    assert 0.7 < capsule.transform.pos.y < 0.9


def test_shoot_example_collision_destroys_target_and_bullet():
    scene = Node()
    _set_scene_defaults(scene)
    target = Target(Vec3(0, 0, 0))
    scene.add_child(target)
    bullet = Bullet(Vec3(0, 0, 8), Vec3(0, 0, -0.5))
    scene.add_child(bullet)

    for _ in range(60):
        scene.update()
        if target.destroyed and bullet.destroyed:
            break

    assert target.destroyed
    assert bullet.destroyed


def test_stack_example_settles_then_reacts_to_bullet():
    scene = Node()
    _set_scene_defaults(scene)
    scene.add_child(Floor())
    cans = [Can(pos) for pos in CAN_LAYOUT]
    for can in cans:
        scene.add_child(can)

    for _ in range(240):
        scene.update()

    drifts = [
        max(
            abs(can.transform.pos.x - start.x),
            abs(can.transform.pos.y - start.y),
            abs(can.transform.pos.z - start.z),
        )
        for can, start in zip(cans, CAN_LAYOUT)
    ]
    assert max(drifts) < 0.25

    scene.add_child(StackBullet(Vec3(0, 1.0, 8), Vec3(0, 0, -0.4)))
    for _ in range(120):
        scene.update()

    moved = sum(
        1
        for can, start in zip(cans, CAN_LAYOUT)
        if max(abs(can.transform.pos.x - start.x), abs(can.transform.pos.z - start.z))
        > 0.5
    )
    assert moved >= 1


def test_terrain_example_ball_rolls_down_slope():
    scene = Node()
    _set_scene_defaults(scene)
    scene.add_child(TerrainFloor())
    ball = Ball()
    scene.add_child(ball)

    initial_y = ball.transform.pos.y
    for _ in range(240):
        scene.update()

    assert ball.transform.pos.y < initial_y - 0.5
