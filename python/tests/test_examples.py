import subprocess
import sys

import pytest
import pyxel
from _capture import (  # type: ignore[reportMissingImports]
    EXAMPLE_REFS_DIR,
    EXAMPLES_DIR,
    collect_plan_results,
    run_example_subprocess,
    run_flip_example_subprocess,
)

CAPTURE_PLANS = {
    # pyxel.run()-based examples
    "01_hello_pyxel": [{"frame": 8}],
    "04_sound_api": [{"frame": 1}],
    "06_click_game": [
        {"frame": 1},
        {"frame": 10, "mouse": (110, 146), "press": [pyxel.MOUSE_BUTTON_LEFT]},
    ],
    "07_snake": [{"frame": 1}],
    "08_triangle_api": [{"frame": 1}, {"frame": 200}],
    "09_shooter": [
        {"frame": 1, "press": [pyxel.KEY_RETURN]},
        {"frame": 120},
    ],
    "12_perlin_noise": [{"frame": 1}, {"frame": 40}],
    "14_synthesizer": [{"frame": 1}],
    # Asset-loading examples
    "02_jump_game": [{"frame": 10}],
    "10_platformer": [
        {"frame": 1},
        *[
            {"frame": i, "press": [pyxel.KEY_RIGHT, pyxel.KEY_SPACE], "capture": False}
            for i in range(2, 80, 2)
        ],
        {"frame": 80},
    ],
    "11_offscreen": [{"frame": 1}, {"frame": 121}],
    "15_tiled_map_file": [{"frame": 1}],
    "16_transform": [{"frame": 1}, {"frame": 45}],
    "18_audio_playback": [
        {"frame": 1},
        {"frame": 3, "press": [pyxel.KEY_RETURN]},
    ],
    "19_perspective": [
        {"frame": 1},
        {"frame": 20, "press": [pyxel.KEY_RIGHT, pyxel.KEY_W]},
    ],
    # Cube examples
    "c01_hello_cube": [{"frame": 8}],
    "c02_basic_shapes": [
        {"frame": 1},
        {"frame": 30, "press": [pyxel.KEY_SPACE]},
    ],
    "c03_custom_shapes": [
        {"frame": 1},
        {
            "frame": 20,
            "mouse": (135, 115),
            "press": [pyxel.MOUSE_BUTTON_LEFT],
            "capture": False,
        },
        {"frame": 32},
        {"frame": 40},
        {"frame": 48},
    ],
    "c04_mesh_and_motion": [
        {"frame": 1},
        {"frame": 45, "press": [pyxel.KEY_2]},
        {"frame": 91, "press": [pyxel.KEY_RIGHT]},
        {"frame": 92, "press": [pyxel.KEY_3, pyxel.KEY_S]},
        {"frame": 123, "press": [pyxel.KEY_1]},
        {"frame": 124, "press": [pyxel.KEY_S]},
    ],
    "c05_3d_collision": [
        {"frame": 1},
        {"frame": 55, "press": [pyxel.KEY_UP], "capture": False},
        {"frame": 56, "press": [pyxel.KEY_SPACE], "capture": False},
        {"frame": 68, "capture": False},
        {"frame": 69, "press": [pyxel.KEY_SPACE], "capture": False},
        {"frame": 70},
        {"frame": 100, "press": [pyxel.KEY_S]},
    ],
    "c06_3d_physics": [
        {"frame": 1, "mouse": (160, 120), "fps": 30},
        {"frame": 18, "press": [pyxel.KEY_RIGHT], "capture": False},
        {"frame": 21, "press": [pyxel.KEY_UP], "capture": False},
        {
            "frame": 22,
            "mouse": (144, 114),
            "press": [pyxel.KEY_SPACE],
            "capture": False,
        },
        {"frame": 100},
        {"frame": 300},
        {"frame": 301, "press": [pyxel.KEY_R], "capture": False},
        {"frame": 380, "press": [pyxel.KEY_LEFT], "capture": False},
        {"frame": 529, "capture": False},
        {
            "frame": 530,
            "mouse": (153, 99),
            "press": [pyxel.MOUSE_BUTTON_LEFT],
            "capture": False,
        },
        {"frame": 600},
        {"frame": 750},
        {"frame": 1100},
    ],
    # Static-screen and launcher captures
    "05_color_palette": [{"frame": 0}],
    "13_custom_font": [{"frame": 0}],
    "17_app_launcher": [{"frame": 1}],
    # pyxel.run() with SPACE held for clipping
    "03_draw_api": [
        {"frame": 1},
        {"frame": 155, "press": [pyxel.KEY_SPACE]},
    ],
    # while+flip() loop
    "99_flip_animation": [{"frame": 1}, {"frame": 30}],
}

FLIP_EXAMPLES = {"99_flip_animation"}


class TestExamples:
    def test_custom_shapes_laser_texture_and_lifecycle(self):
        code = """
import os
import runpy
import sys
from pathlib import Path

import pyxel
from pyxel.cube import Mat4, Vec3

init = pyxel.init


def init_headless(*args, **kwargs):
    init(*args, **{**kwargs, "headless": True, "fps": 1_000_000})
    os.chdir(Path(sys.argv[1]).parent)


pyxel.init = init_headless
callbacks = {}
pyxel.run = lambda update, draw: callbacks.update(update=update)
namespace = runpy.run_path(sys.argv[1])
app = callbacks["update"].__self__
app.update_game()
# Hide only slime geometry, retaining the actual unified on_draw.
app.base_positions = [0.0] * len(app.base_positions)

app.locked = [0]
app.start_fire()
app.age = 10

# Texture changes must affect both the width and the length of the ribbon,
# without changing its path or creating separate geometry for each color.
texture = pyxel.Image(16, 16)
texture.rect(0, 0, 8, 8, 3)
texture.rect(8, 0, 8, 8, 9)
texture.rect(0, 8, 8, 8, 14)
texture.rect(8, 8, 8, 8, 15)
plain = pyxel.Image(16, 16)
plain.rect(0, 0, 16, 16, 7)
for eye in (Vec3(0, 132, 110), Vec3(110, 100, 80), Vec3(-90, 200, 110)):
    app.camera.transform = Mat4.look_at(eye, namespace["AIM_POINT"])
    app.laser_image = plain
    app.draw(0, 0, 256, 192)
    positions = list(app.strip.positions)
    app.laser_image = texture
    app.draw(0, 0, 256, 192)
    colors = {pyxel.pget(x, y) for y in range(192) for x in range(256)}
    assert {3, 9, 14, 15} <= colors
    assert list(app.strip.positions) == positions

    # Color zero is transparent so a flame texture can have an irregular edge.
    texture.rect(0, 0, 16, 16, 0)
    app.draw(0, 0, 256, 192)
    hidden = [pyxel.pget(x, y) for y in range(192) for x in range(256)]
    saved_targets = app.targets
    app.targets = []
    app.draw(0, 0, 256, 192)
    assert hidden == [pyxel.pget(x, y) for y in range(192) for x in range(256)]
    app.targets = saved_targets
    texture.rect(0, 0, 8, 8, 3)
    texture.rect(8, 0, 8, 8, 9)
    texture.rect(0, 8, 8, 8, 14)
    texture.rect(8, 8, 8, 8, 15)

# Each volley hits once; interruption removes it immediately, expiry later.
app.targets = []
plain.rect(0, 0, 16, 16, 11)
app.laser_image = plain
calls = []
pyxel.play = lambda channel, sound: calls.append((channel, sound))
for interruption in (3, 13, 14, 17, 22, 23, 30):
    pyxel.set_btn(pyxel.MOUSE_BUTTON_LEFT, False)
    pyxel.flip()
    app.locked = list(range(len(app.centers)))
    app.start_fire()
    app.flashes = [0] * len(app.centers)
    calls.clear()
    for age in range(2, 25):
        pyxel.set_btn(pyxel.MOUSE_BUTTON_LEFT, age == interruption)
        app.update_game()
        hit_age = min(14, interruption)
        assert calls.count((2, 2)) == int(age >= hit_age)
        expected_flash = max(0, 12 - (age - hit_age)) if age >= hit_age else 0
        assert app.flashes == [expected_flash] * len(app.centers), (age, app.flashes)
        if age >= min(23, interruption):
            app.draw_game()
            colors = {pyxel.pget(x, y) for y in range(192) for x in range(256)}
            assert 11 not in colors
        pyxel.flip()
"""
        result = subprocess.run(
            [
                sys.executable,
                "-c",
                code,
                str(EXAMPLES_DIR / "cube" / "c03_custom_shapes.py"),
            ],
            capture_output=True,
            text=True,
            timeout=15,
            check=False,
        )
        assert result.returncode == 0, result.stderr

    def test_collision_example_movement_and_camera(self):
        code = """
import os
import runpy
import sys
from pathlib import Path

import pyxel
from pyxel.cube import Camera, Collider, Mat4, Mesh, Node, Quat, Vec3

init = pyxel.init


def init_headless(*args, **kwargs):
    init(*args, **{**kwargs, "headless": True, "fps": 1_000_000})
    os.chdir(Path(sys.argv[1]).parent)


pyxel.init = init_headless
callbacks = {}
pyxel.run = lambda update, draw: callbacks.update(update=update)
namespace = runpy.run_path(sys.argv[1])
app = callbacks["update"].__self__
pyxel.init = lambda *args, **kwargs: None
player = app.player


def step(frames=1, jump=False):
    pyxel.set_btn(pyxel.KEY_SPACE, jump)
    for _ in range(frames):
        app.update_game()
        pyxel.flip()


# Turning or raising the camera must not tilt the coins' vertical spin axes.
coins = app.coins
camera = app.camera.transform
rotations = []
for eye in [Vec3(0, 20, 120), Vec3(120, 80, 30), Vec3(-80, 180, -30)]:
    app.camera.transform = Mat4.look_at(eye, Vec3.ZERO)
    for coin in coins:
        app.on_update()
        matrix = coin.children[0].world_transform
        up = Vec3(matrix[0, 1], matrix[1, 1], matrix[2, 1])
        assert (up - Vec3.UP).length() < 0.00001
    rotations.append(coins[0].children[0].transform.rot)
    pyxel.flip()
assert rotations[0].angle_to(rotations[-1]) > 1
app.camera.transform = camera


# A line through the coin's interior must be hidden at every rotation.
# The front face must occlude the interior line from every viewpoint.
class Axis(Node):
    def on_draw(self):
        self.line(Vec3(0, -2, 0), Vec3(0, 2, 0), 7)


coin_scene = Node()
coin_scene.camera = Camera()
coin_actor = Node.from_mesh(Mesh.from_glb("assets/garden_coin.glb"))
coin_scene.add_child(coin_actor)
axis = Axis()
coin_scene.add_child(axis)
for eye in (Vec3(0, 0, 30), Vec3(0, 20, 30), Vec3(12, 30, 30)):
    coin_scene.camera.transform = Mat4.look_at(eye, Vec3.ZERO)
    for angle in range(0, 360, 15):
        coin_actor.transform = Mat4.IDENTITY.rotate_y(angle)
        axis.visible = False
        pyxel.cls(6)
        coin_scene.draw(0, 0, 64, 64)
        before = [pyxel.pget(x, y) for y in range(64) for x in range(64)]
        assert any(color != 6 for color in before)
        axis.visible = True
        pyxel.cls(6)
        coin_scene.draw(0, 0, 64, 64)
        after = [pyxel.pget(x, y) for y in range(64) for x in range(64)]
        assert before == after, (eye, angle)

step(2)
start_y = player.transform.pos.y
wings = player.actor.find_by_name("LeftWing") + player.actor.find_by_name("RightWing")
rest_wings = [wing.transform.rot for wing in wings]
step(jump=True)
takeoff_wings = [wing.transform.rot for wing in wings]
takeoff_phase = player.motion_frame
step(3)
early_flap_speed = (player.motion_frame - takeoff_phase) / 3
for wing, takeoff in zip(wings, takeoff_wings):
    assert wing.transform.rot.angle_to(takeoff) > 45
early_phase = player.motion_frame
step(9)
late_flap_speed = (player.motion_frame - early_phase) / 9
assert 1 <= late_flap_speed < early_flap_speed <= 1.8
first_y = player.transform.pos.y
step(jump=True)
for wing, takeoff in zip(wings, takeoff_wings):
    assert wing.transform.rot.angle_to(takeoff) < 0.1
step(12)
assert player.transform.pos.y > first_y + 40
assert player.transform.pos.y > start_y + 90
before = player.collider.velocity.y
step(jump=True)
assert player.collider.velocity.y < before  # A third press cannot jump again.
step(90)
assert player.on_floor and player.jumps == 0
for wing, rest in zip(wings, rest_wings):
    assert wing.transform.rot.angle_to(rest) < 0.1
step(jump=True)
assert player.collider.velocity.y > 0
step(90)

platform = next(
    child for child in app.children if isinstance(child, namespace["MovingPlatform"])
)
player.transform = Mat4.from_translation(platform.transform.pos + Vec3(0, 8, 0))
player.collider.velocity = Vec3.ZERO
step()
boarding_offset = player.transform.pos - platform.transform.pos
for _ in range(260):  # Covers a full cycle, including descending motion.
    step()
    offset = player.transform.pos - platform.transform.pos
    assert (offset - boarding_offset).length() < 0.1, offset
step(jump=True)
step(8)
assert player.platform is None
assert player.transform.pos.y > platform.transform.pos.y + 30

# Both an uninterrupted wall and the seam between terrain blocks stay still.
for z in (0, 24):
    app = type(app)()
    player = app.player
    app.yaw = 0
    app.camera_offset = None
    app.update_camera()
    player.transform = Mat4.from_translation(Vec3(24, 8, z))
    pyxel.set_btn(pyxel.KEY_RIGHT, True)
    step(60)
    for _ in range(60):
        step()
        assert (player.transform.pos - Vec3(40, 8, z)).length() < 0.001
        assert player.on_floor
    pyxel.set_btn(pyxel.KEY_RIGHT, False)
    step()

# Walking down both joined ramps keeps continuous ground contact.
app = type(app)()
player = app.player
app.yaw = 0
app.camera_offset = None
app.update_camera()
player.transform = Mat4.from_translation(Vec3(-72, 58, -15))
step(30)
pyxel.set_btn(pyxel.KEY_DOWN, True)
for _ in range(42):
    step()
    assert player.on_floor
    assert player.jumps == 0
assert player.transform.pos.z > 75
assert abs(player.transform.pos.y - 8) < 0.001
step(jump=True)
assert not player.on_floor and player.collider.velocity.y > 0
pyxel.set_btn(pyxel.KEY_DOWN, False)

# A fast landing must stop on the terrace top, even after crossing its plane.
app = type(app)()
player = app.player
player.transform = Mat4.from_translation(Vec3(-60, 145.15, -183))
player.collider.velocity = Vec3(0, -8.8, 0)
step(30)
assert (player.transform.pos - Vec3(-60, 144, -183)).length() < 0.001
assert player.on_floor

# Landing beside a wall must stop above the floor, including terrain seams.
app = type(app)()
player = app.player
for position, velocity, landing_y in [
    ((-102.0, 56.06572, -43.696327), (0.000292, -8.05517, 2.0), 56.0),
    ((-102.0, 57.829544, -54.829929), (1.205075, -10.0, 1.596181), 56.0),
    ((-24.732048, 32.1604, 21.819878), (-1.404791, -10.0, 1.423573), 32.0),
    ((-115.326622, 9.056183, 80.570984), (1.671067, -10.0, 1.098878), 8.0),
    ((-3.384918, 56.99638, -40.179802), (-1.999966, -9.034148, 0.011697), 56.0),
    ((-40.0, 32.751106, -5.127094), (-1.997862, -8.310695, -0.092464), 32.0),
    ((-40.0, 32.725746, -14.326479), (-1.418789, -10.0, 1.409624), 32.0),
    ((-108.308121, 8.997179, 63.045258), (-1.246615, -10.0, -1.563954), 8.0),
    ((-5.4078255, 56.25569, -38.889828), (1.9806976, -9.224616, 0.2771951), 56.0),
    ((-103.38514, 8.016506, 62.859848), (-1.7139558, -9.492782, 1.0307064), 8.0),
]:
    player.reset()
    player.on_floor = False
    player.transform = Mat4.from_translation(Vec3(*position))
    player.collider.velocity = Vec3(*velocity)
    player.move = Vec3(velocity[0], 0, velocity[2])
    step()
    assert player.transform.pos.y >= landing_y - 0.001, player.transform.pos
    assert player.on_floor

# The chimney is solid from above and from the side, like the rest of the house.
app = type(app)()
player = app.player
player.transform = Mat4.from_translation(Vec3(-68, 130, -64))
step(30)
assert player.on_floor
assert abs(player.transform.pos.y - 105) < 0.001
player.transform = Mat4.from_translation(Vec3(-56, 89, -64))
step()
assert player.transform.pos.x >= -55

# Respawning restores the view immediately, without sweeping across the map.
app = type(app)()
player = app.player
step(2)
initial_camera = app.camera.transform
player.coins = 3
player.transform = Mat4.from_translation(Vec3(-112, -120, -90))
step()
assert player.transform.pos.y < -120
for yaw in (0, 120, 240):
    player.transform = Mat4.from_translation(Vec3(-112, -161, -90))
    app.yaw = yaw
    app.update_camera()
    step()
    assert (player.transform.pos - player.start).length() < 0.001
    assert player.on_floor and player.jumps == 0
    assert player.coins == 3
    for _ in range(20):
        step()
        camera = app.camera.transform
        assert (camera.pos - initial_camera.pos).length() < 0.001
        assert camera.rot.angle_to(initial_camera.rot) < 0.1

# Holding right turns the camera behind that heading, without curving the walk.
app = type(app)()
player = app.player
app.yaw = 0
app.camera_offset = None
app.update_camera()
pyxel.set_btn(pyxel.KEY_RIGHT, True)
for _ in range(150):
    # Reuse a clear patch so terrain boundaries do not end the camera check.
    player.transform = Mat4.from_translation(Vec3(-20, 8, 160))
    player.collider.velocity = Vec3.ZERO
    step()
    assert abs(player.transform.pos.z - 160) < 0.001
    assert abs(player.transform.pos.x + 18) < 0.001
assert abs(app.yaw + 90) < 0.1
pyxel.set_btn(pyxel.KEY_RIGHT, False)
angle = app.yaw
step(40)
assert app.yaw == angle

# Unadvertised A/D keys must not turn the camera.
for key in (pyxel.KEY_A, pyxel.KEY_D):
    pyxel.set_btn(key, True)
    step(10)
    assert app.yaw == angle
    pyxel.set_btn(key, False)

# A short tap turns the model smoothly. Releasing movement keeps the view.
app = type(app)()
player = app.player
app.yaw = 0
app.camera_offset = None
app.update_camera()
pyxel.set_btn(pyxel.KEY_RIGHT, True)
step()
facing_right = Quat.from_euler(Vec3(0, 90, 0))
assert abs(player.transform.pos.x + 18) < 0.001
assert 1 < player.rotation.angle_to(facing_right) < 89
step(3)
pyxel.set_btn(pyxel.KEY_RIGHT, False)
angle = app.yaw
step(40)
assert app.yaw == angle
assert player.rotation.angle_to(facing_right) < 0.1

# Capture the heading once at takeoff. Turning in midair and the second jump
# must not change that target, while the camera completes its smooth turn.
previous = app.camera.transform.rot
step(jump=True)
angle = app.yaw
assert abs((angle - player.heading - 180 + 180) % 360 - 180) < 0.001
assert 0.1 < app.camera.transform.rot.angle_to(previous) < 10
for key in (pyxel.KEY_LEFT, pyxel.KEY_UP, pyxel.KEY_DOWN):
    pyxel.set_btn(key, True)
    step(2)
    pyxel.set_btn(key, False)
    assert app.yaw == angle
step(jump=True)
assert app.yaw == angle
step(70)

# Crossing the angle wrap takes the short two-degree path.
player.rotation = Quat.from_euler(Vec3(0, 179, 0))
player.heading = -179
player.animate(False)
target_rotation = Quat.from_euler(Vec3(0, -179, 0))
assert 0.1 < player.rotation.angle_to(target_rotation) < 2

# Approach a stationary wall continuously: move the camera forward, then up,
# without jumping views or moving inside the character.
app = type(app)()
scene = app
player = scene.player
for child in list(scene.children):
    if child is not player:
        scene.remove_child(child)
scene.yaw = 0
scene.camera_offset = None
scene.update_camera()
wall = Node()
wall.collider = Collider(size=Vec3(100, 200, 4), mass=0)
wall.tags = {"ground"}
wall.transform = Mat4.from_translation(Vec3(-20, 80, 300))
scene.add_child(wall)
previous = scene.camera.transform.rot
for z in range(160, 283, 2):
    focus = Vec3(-20, 8, z)
    player.transform = Mat4.from_translation(focus)
    scene.update_camera()
    eye = scene.camera.transform.pos
    assert eye.z < 298
    offset = eye - focus
    assert offset.length() >= 48
    assert scene.raycast(focus, offset, offset.length(), tags={"ground"}) is None
    assert scene.camera.transform.rot.angle_to(previous) < 10
    previous = scene.camera.transform.rot

for _ in range(40):
    scene.update_camera()
scene.remove_child(wall)
blocked_eye = scene.camera.transform.pos
clear_eye = focus + Vec3(0, 63, 100)
scene.update_camera()
eye = scene.camera.transform.pos
assert (eye - blocked_eye).length() > 0
assert (eye - clear_eye).length() > 1
assert (eye - clear_eye).length() < (blocked_eye - clear_eye).length()
for _ in range(80):
    scene.update_camera()
assert (scene.camera.transform.pos - clear_eye).length() < 0.01

# Complete an orbit under leaves and beside buildings and cliffs. Brief
# occlusion during a turn must not become a jump, a stuck view, or a close-up.
for pos in [(-165, 104, -67), (-85, 96, -57), (-56, 104, -112), (-106, 56, -78)]:
    app = type(app)()
    scene = app
    focus = Vec3(*pos)
    scene.player.transform = Mat4.from_translation(focus)
    scene.target = focus
    scene.camera_offset = None
    previous = None
    occluded_frames = 0
    for angle in range(360):
        scene.yaw = angle
        scene.update_camera()
        eye = scene.camera.transform.pos
        offset = eye - focus
        assert offset.length() >= 48
        if previous is not None:
            assert scene.camera.transform.rot.angle_to(previous) < 10
        previous = scene.camera.transform.rot
        if scene.raycast(focus, offset, offset.length(), tags={"ground"}):
            occluded_frames += 1
            assert occluded_frames < 15
        else:
            occluded_frames = 0
        hit = scene.raycast(eye, -offset, offset.length(), tags={"ground"})
        assert hit is None or hit.normal.dot(-offset) <= 0
    for _ in range(60):
        scene.update_camera()
    offset = scene.camera.transform.pos - focus
    expected = Vec3(pyxel.sin(359), 0.63, pyxel.cos(359)).normalize()
    assert (offset.normalize() - expected).length() < 0.005
    assert scene.raycast(focus, offset, offset.length(), tags={"ground"}) is None

# The camera must turn gradually when walking under the low tree at the ramp.
app = type(app)()
scene = app
player = scene.player
player.transform = Mat4.from_translation(Vec3(-72, 58, -15))
scene.target = player.transform.pos
scene.camera_offset = None
scene.yaw = 330
scene.update_camera()
player.move_input = (1, 0)
player.move = Vec3(0, 0, 1)
pyxel.set_btn(pyxel.KEY_RIGHT, True)
previous = scene.camera.transform.rot
for _ in range(90):
    step()
    offset = scene.camera.transform.pos - player.transform.pos
    assert offset.length() >= 48
    assert scene.camera.transform.rot.angle_to(previous) < 10
    previous = scene.camera.transform.rot
pyxel.set_btn(pyxel.KEY_RIGHT, False)
step(60)
offset = scene.camera.transform.pos - player.transform.pos
assert (
    scene.raycast(player.transform.pos, offset, offset.length(), tags={"ground"})
    is None
)

# When touching the underside of an island, the camera ray must start in free
# space rather than on the ceiling. Moving beneath it must not flip the view.
app = type(app)()
scene = app
scene.player.transform = Mat4.from_translation(Vec3(-76, -24, -70))
scene.target = scene.player.transform.pos
scene.camera_offset = None
scene.yaw = 150
for _ in range(40):
    scene.update_camera()
previous = scene.camera.transform.rot
for i in range(17):
    focus = Vec3(-76 - i * 2, -24, -70 + i * 0.4)
    scene.player.transform = Mat4.from_translation(focus)
    scene.update_camera()
    offset = scene.camera.transform.pos - focus
    assert offset.length() >= 48
    assert scene.raycast(focus, offset, offset.length(), tags={"ground"}) is None
    assert scene.camera.transform.rot.angle_to(previous) < 10
    previous = scene.camera.transform.rot

# Collecting another coin after reaching the goal must not replace the fanfare.
app = type(app)()
scene = app
coins = scene.coins
calls = []
play = pyxel.play


def record_play(channel, sound, *args, **kwargs):
    calls.append((channel, sound))
    return play(channel, sound, *args, **kwargs)


pyxel.play = record_play
scene.goal.transform = Mat4.from_translation(scene.player.transform.pos)
coins[0].transform = Mat4.from_translation(scene.player.transform.pos)
step(2)
coins[1].transform = Mat4.from_translation(scene.player.transform.pos)
step(5)
coin_channels = [channel for channel, sound in calls if sound == 1]
goal_channels = [channel for channel, sound in calls if sound == 2]
assert len(coin_channels) == 2
assert len(goal_channels) == 1
assert goal_channels[0] not in coin_channels
assert pyxel.play_pos(goal_channels[0]) is not None
pyxel.set_btn(pyxel.KEY_R, True)
old_player = app.player
step()
assert app.player is old_player and app.player.coins == 2
assert not app.goal.active
assert pyxel.play_pos(goal_channels[0]) is not None
pyxel.set_btn(pyxel.KEY_R, False)
pyxel.play = play
"""

        result = subprocess.run(
            [
                sys.executable,
                "-c",
                code,
                str(EXAMPLES_DIR / "cube" / "c05_3d_collision.py"),
            ],
            capture_output=True,
            text=True,
            timeout=10,
            check=False,
        )
        assert result.returncode == 0, result.stderr

    def test_physics_example_demonstrations_and_controls(self, tmp_path):
        code = """
import math
import runpy
import sys

import pyxel
from pyxel.cube import Vec3

init = pyxel.init
pyxel.init = lambda *args, **kwargs: init(
    *args, **{**kwargs, "headless": True, "fps": 30}
)
callbacks = {}
pyxel.run = lambda update, draw: callbacks.update(update=update, draw=draw)
namespace = runpy.run_path(sys.argv[1])
app = callbacks["update"].__self__
body_type = namespace["Body"]


def step(frames=1, key=None):
    for button in (pyxel.KEY_SPACE, pyxel.KEY_R, pyxel.MOUSE_BUTTON_LEFT):
        pyxel.set_btn(button, button == key)
    for _ in range(frames):
        app.update()
        pyxel.flip()


def bodies():
    return [b for b in app.scene.children if isinstance(b, body_type) and b.collider.mass > 0]


def aim(point):
    local = app.scene.camera.transform.inverse() * point
    scale = 2 * math.tan(math.radians(app.scene.camera.fov / 2)) / pyxel.height
    pyxel.set_mouse_pos(round(pyxel.width / 2 - local.x / local.z / scale),
                        round(pyxel.height / 2 + local.y / local.z / scale))


# Static terrain remains at multiple heights; moving arms turn in two planes.
initial_bodies = bodies()
initial_poses = [b.transform for b in initial_bodies]
paddles = [b for b in app.scene.children if b.collider.mass == 0
           and b.collider.angular_velocity != Vec3.ZERO]
assert any(abs((b.transform.rot * b.collider.angular_velocity).y) > 1 for b in paddles)
assert any(abs((b.transform.rot * b.collider.angular_velocity).z) > 1 for b in paddles)
paddle_start = [b.transform for b in paddles]
assert any(b.collider.size == Vec3.ZERO and b.collider.radius > 0 for b in initial_bodies)
assert any(b.collider.size.x > 0 and b.collider.radius == 0 for b in initial_bodies)
assert any(b.collider.size.x == 0 and b.collider.size.y > 0 for b in initial_bodies)
assert all(isinstance(b.paint, pyxel.Image) for b in initial_bodies)
initial = [b.transform.pos for b in initial_bodies]
camera_start = app.scene.camera.transform.pos
step(1800)
assert all((b.transform.pos - p).length() < 2 for b, p in zip(initial_bodies, initial))
assert any(b.transform != p for b, p in zip(paddles, paddle_start))
assert app.scene.camera.transform.pos == camera_start
assert not app.scene.shots

# Repeated resets must not make fresh stacks collide with the previous ones.
for _ in range(3):
    step(key=pyxel.KEY_R)
    standing = bodies()
    step(120)
    assert len(standing) == len(initial_bodies)
    assert all((b.transform.pos - p.pos).length() < 2
               for b, p in zip(standing, initial_poses))

# The arrows choose the firing viewpoint; the pointer chooses its target.
pyxel.set_btn(pyxel.KEY_RIGHT, True)
pyxel.set_btn(pyxel.KEY_UP, True)
step(20)
pyxel.set_btn(pyxel.KEY_RIGHT, False)
pyxel.set_btn(pyxel.KEY_UP, False)
assert (app.scene.camera.transform.pos - camera_start).length() > 200
assert app.scene.camera.transform.pos.y > camera_start.y
assert app.scene.angle == 75 and app.scene.pitch == 50

# Aimed fire removes a tower support and drops all its upper floors.
step(key=pyxel.KEY_R)
standing = bodies()
floors = [b for b in standing if b.collider.size == Vec3(180, 15, 160)
          and b.transform.pos.x == 290]
initial = [b.transform.pos for b in floors]
app.scene.angle = 115
app.scene.pitch = 20
app.scene.move_camera()
aim(Vec3(240, 412.5, -130))
step(key=pyxel.MOUSE_BUTTON_LEFT)
step(600)
assert all((b.transform.pos - p).length() > 20 for b, p in zip(floors, initial))
assert all(b in app.scene.children for b in standing)

# Aimed fire dislodges the loose bridge and the capsule it carries.
step(key=pyxel.KEY_R)
bridge = next(b for b in bodies() if b.collider.size == Vec3(280, 20, 90))
roller = next(b for b in bodies() if b.collider.radius == 35)
app.scene.angle = 270
app.scene.pitch = 15
app.scene.move_camera()
aim(Vec3(-450, 570, 160))
for _ in range(3):
    step(key=pyxel.MOUSE_BUTTON_LEFT)
    step(19)
step(600)
assert (bridge.world_transform * Vec3(-140, 0, 0)).y < 60
assert roller.transform.pos.y < 100

# One crossbeam hit collapses the open stack and sends debris into the vertical arms.
step(key=pyxel.KEY_R)
standing = bodies()
initial = [b.transform.pos for b in standing]
beams = [b for b in standing if b.collider.size == Vec3(64, 14, 180)]
beam_heights = [b.transform.pos.y for b in beams]
arm_contacts = set()
first_contacts = {}
on_collide = body_type.on_collide

def record_contact(self, other, contact):
    if self in standing and other.collider.mass == 0:
        if other.collider.angular_velocity != Vec3.ZERO:
            arm_contacts.add(other)
            if abs((other.transform.rot * other.collider.angular_velocity).z) > 1:
                first_contacts.setdefault(self, pyxel.frame_count)
    on_collide(self, other, contact)

body_type.on_collide = record_contact
app.scene.angle = 90
app.scene.pitch = 15
app.scene.move_camera()
aim(Vec3(280, 561, 195))
step(key=pyxel.MOUSE_BUTTON_LEFT)
step(800)
body_type.on_collide = on_collide
assert any(abs((b.transform.rot * b.collider.angular_velocity).z) > 1
           for b in arm_contacts)
# Losing half the pillar height establishes collapse, even onto a lower deck.
assert all(b.transform.pos.y < y - 32 for b, y in zip(beams, beam_heights))
assert any(p.y > 500 and b.transform.pos.y < 200
           for b, p in zip(standing, initial))
assert all(b in app.scene.children for b in standing)
assert all(math.isfinite(v) for b in bodies() for v in b.transform.pos)
assert app.scene.shot_count == 1
app.scene.shoot()
app.scene.shoot()
for index, shot in enumerate(app.scene.shots):
    if index == 0:
        assert shot.collider.size == Vec3.ZERO and shot.collider.radius > 0
    elif index == 1:
        assert shot.collider.size.x == 0 and shot.collider.size.y > 0 and shot.collider.radius > 0
    else:
        assert shot.collider.size.x > 0 and shot.collider.radius == 0

# One wall shot should send a burst of original debris onto the central rotor.
for angle, pitch, target in [(270, 15, Vec3(-235, 625, -25)),
                             (70, 15, Vec3(200, 565, 145))]:
    step(key=pyxel.KEY_R)
    standing, first_contacts = bodies(), {}
    body_type.on_collide = record_contact
    app.scene.angle, app.scene.pitch = angle, pitch
    app.scene.move_camera()
    aim(target)
    step(key=pyxel.MOUSE_BUTTON_LEFT)
    step(800)
    body_type.on_collide = on_collide
    assert app.scene.shot_count == 1
    times = sorted(first_contacts.values())
    assert any(sum(t <= other < t + 60 for other in times) >= 8 for t in times)

# An earlier projectile at the muzzle must not become the next shot's target.
step(key=pyxel.KEY_R)
pyxel.set_mouse_pos(200, 175)
app.scene.shoot()
first_velocity = app.scene.shots[-1].collider.velocity
app.scene.shoot()
assert app.scene.shots[-1].collider.velocity == first_velocity

# The wheel remains unbound; Space fires like a click without moving the camera.
step(key=pyxel.KEY_R)
camera = app.scene.camera.transform
pyxel.set_btnv(pyxel.MOUSE_WHEEL_Y, 3)
step(31)
pyxel.set_btnv(pyxel.MOUSE_WHEEL_Y, 0)
assert app.scene.camera.transform == camera
assert app.scene.shot_count == 0
step(31, key=pyxel.KEY_SPACE)
assert app.scene.camera.transform == camera
assert app.scene.shot_count == 4

# Holding fire repeats; the projectile limit bounds scene growth and reset restores it.
scene = app.scene
step(key=pyxel.KEY_R)
assert app.scene is scene and len(bodies()) == len(initial_bodies)
assert not app.scene.shots and app.scene.shot_count == 0
assert app.scene.camera.transform == camera
pyxel.set_mouse_pos(pyxel.width // 2, pyxel.height // 2)
step(31, key=pyxel.MOUSE_BUTTON_LEFT)
assert len(app.scene.shots) == 4
step(150, key=pyxel.MOUSE_BUTTON_LEFT)
assert len(app.scene.shots) == 16
assert len(bodies()) <= len(initial_bodies) + 16

# Reset restores only loose objects; the camera and rotating paddles keep their pose.
camera = scene.camera.transform
paddle_poses = [b.transform for b in paddles]
scene.reset()
assert not scene.shots and scene.shot_count == 0
restored = bodies()
assert len(restored) == len(initial_bodies)
assert scene.camera.transform == camera
assert [b.transform for b in paddles] == paddle_poses
for body, transform in zip(restored, initial_poses):
    assert body.transform == transform
    assert body.collider.velocity == body.collider.angular_velocity == Vec3.ZERO
step()
assert bodies() == restored
callbacks["draw"]()
"""

        result = subprocess.run(
            [
                sys.executable,
                "-c",
                code,
                str(EXAMPLES_DIR / "cube" / "c06_3d_physics.py"),
            ],
            cwd=tmp_path,
            capture_output=True,
            text=True,
            timeout=300,
            check=False,
        )
        assert result.returncode == 0, result.stderr

    def test_shooter_consumes_each_collision_once(self):
        code = """
import runpy
import sys

import pyxel

init = pyxel.init
pyxel.init = lambda *args, **kwargs: init(*args, **kwargs, headless=True)
pyxel.run = lambda update, draw: None

namespace = runpy.run_path(sys.argv[1])
pyxel.flip()
app = namespace["App"].__new__(namespace["App"])

# A destroyed enemy cannot score twice or kill the overlapping player.
# A consumed bullet cannot destroy a second overlapping enemy.
for enemy_count, bullet_count, player_x in [(1, 2, 30), (2, 1, 90)]:
    for name in ("enemies", "bullets", "blasts"):
        namespace[name].clear()
    app.player = namespace["Player"](player_x, 30)
    app.score = 0
    app.scene = namespace["SCENE_PLAY"]
    for _ in range(enemy_count):
        namespace["Enemy"](30, 30)
    for _ in range(bullet_count):
        namespace["Bullet"](30, 30)

    app.update_play_scene()

    assert app.score == 10, app.score
    assert app.scene == namespace["SCENE_PLAY"], app.scene
    assert len(namespace["enemies"]) == enemy_count - 1
    assert len(namespace["bullets"]) == bullet_count - 1
    assert len(namespace["blasts"]) == 1
"""

        result = subprocess.run(
            [sys.executable, "-c", code, str(EXAMPLES_DIR / "09_shooter.py")],
            capture_output=True,
            text=True,
            timeout=10,
            check=False,
        )
        assert result.returncode == 0, result.stderr

    def test_wavetable_strokes_start_at_click_and_fill_dragged_columns(self):
        code = """
import runpy
import sys

import pyxel

init = pyxel.init
pyxel.init = lambda *args, **kwargs: init(*args, **kwargs, headless=True)
pyxel.run = lambda update, draw: None

namespace = runpy.run_path(sys.argv[1])
editor = namespace["WavetableEditor"](8, 8, 0, "Test")
wave = pyxel.tones[0].wavetable
wave[:] = [8] * 32


def frame(col, row, pressed=None):
    pyxel.set_mouse_pos(editor.x + 1 + col * 5, editor.y + 8 + (15 - row) * 3)
    if pressed is not None:
        pyxel.set_btn(pyxel.MOUSE_BUTTON_LEFT, pressed)
    editor.update()
    pyxel.flip()


frame(2, 8, False)
frame(20, 15, True)
expected = [8] * 20 + [15] + [8] * 11
assert list(wave) == expected, list(wave)

frame(24, 1)
expected[20:25] = [1] * 5
assert list(wave) == expected, list(wave)

frame(7, 9, False)
assert list(wave) == expected, list(wave)

frame(5, 0, True)
expected[5] = 0
assert list(wave) == expected, list(wave)
"""

        result = subprocess.run(
            [sys.executable, "-c", code, str(EXAMPLES_DIR / "14_synthesizer.py")],
            capture_output=True,
            text=True,
            timeout=10,
            check=False,
        )
        assert result.returncode == 0, result.stderr

    def test_numbered_examples_have_capture_plans(self):
        planned = set(CAPTURE_PLANS)
        examples = {
            script.stem
            for script in EXAMPLES_DIR.glob("*.py")
            if not script.name.startswith("__")
        }
        examples.update(script.stem for script in (EXAMPLES_DIR / "cube").glob("*.py"))
        assert planned == examples

    @pytest.mark.parametrize(
        "name", list(CAPTURE_PLANS.keys()), ids=list(CAPTURE_PLANS.keys())
    )
    def test_example(self, name, tmp_path, compare_screenshots):
        script_dir = EXAMPLES_DIR / "cube" if name.startswith("c") else EXAMPLES_DIR
        script = script_dir / f"{name}.py"
        assert script.exists(), f"Example not found: {script}"

        plan = CAPTURE_PLANS[name]
        if name in FLIP_EXAMPLES:
            run_flip_example_subprocess(script, plan, tmp_path)
        else:
            run_example_subprocess(script, plan, tmp_path)

        results = collect_plan_results(plan, tmp_path)
        compare_screenshots(name, results, EXAMPLE_REFS_DIR)
