import runpy
from pathlib import Path

import pyxel


EXAMPLE_PATH = (
    Path(__file__).parent.parent / "pyxel" / "examples" / "c03_custom_shapes.py"
)


class FakeSound:
    def __init__(self):
        self.set_args = None

    def set(self, *args, **kwargs):
        self.set_args = (args, kwargs)


def load_example(monkeypatch):
    sounds = [FakeSound() for _ in range(8)]
    plays = []
    monkeypatch.setattr(pyxel, "init", lambda *args, **kwargs: None)
    monkeypatch.setattr(pyxel, "run", lambda update, draw: None)
    monkeypatch.setattr(pyxel, "sounds", sounds)
    monkeypatch.setattr(
        pyxel, "play", lambda ch, snd, **kwargs: plays.append((ch, snd))
    )
    ns = runpy.run_path(str(EXAMPLE_PATH), run_name="__test__")
    ns["TEST_SOUNDS"] = sounds
    ns["TEST_PLAYS"] = plays
    return ns


def direct_points(ns, target, count):
    emitter = ns["LASER_ORIGIN"]
    points = []
    for i in range(count):
        t = i / (count - 1)
        points.append(emitter * (1.0 - t) + target * t)
    return points


def make_weapon(ns, slimes=None, eye=None):
    scene = ns["Node"]()
    scene.camera = ns["Camera"]()
    scene.camera.transform = ns["Mat4"].look_at(
        eye or ns["CAMERA_EYE"], ns["AIM_CENTER"]
    )
    weapon = ns["Weapon"](slimes or [])
    scene.add_child(weapon)
    return weapon


def make_slime(ns, index=0):
    scene = ns["Node"]()
    scene.camera = ns["Camera"]()
    scene.camera.transform = ns["Mat4"].look_at(ns["CAMERA_EYE"], ns["AIM_CENTER"])
    slime = ns["Slime"](index)
    scene.add_child(slime)
    return slime


def project_to_screen(ns, weapon, point):
    camera = weapon.effective_camera
    mat = camera.transform
    eye, right, up = weapon.camera_axes()
    forward = ns["Vec3"].FORWARD.to_world_dir(mat)
    rel = point - eye
    depth = rel.dot(forward)
    aspect = pyxel.width / pyxel.height
    scale = 1.0 / ns["math"].tan(ns["math"].radians(camera.fov) * 0.5)
    ndc_x = (rel.dot(right) * scale / aspect) / depth
    ndc_y = (rel.dot(up) * scale) / depth
    return (
        (ndc_x + 1.0) * 0.5 * pyxel.width,
        (1.0 - (ndc_y + 1.0) * 0.5) * pyxel.height,
    )


def test_laser_path_kicks_outward_and_turns_back_to_target(monkeypatch):
    ns = load_example(monkeypatch)
    weapon = make_weapon(ns)
    target = ns["Vec3"](2.5, 2.3, 0.0)
    weapon.firing_slimes = [object(), object()]

    points = weapon.make_laser_path(target, 0, 1.0)
    direct = direct_points(ns, target, len(points))
    deviations = [(point - base).length() for point, base in zip(points, direct)]

    assert len(points) == 24
    assert points[0] == ns["LASER_ORIGIN"]
    assert points[-1].distance_to(target) < 1e-5
    assert max(deviations[2:-2]) > 2.5

    _, right, up = weapon.camera_axes()
    side = [(point - ns["AIM_CENTER"]).dot(right) for point in points]
    target_side = (target - ns["AIM_CENTER"]).dot(right)
    assert max(side) > target_side + 1.2

    left_target = ns["Vec3"](-2.5, 2.3, 0.0)
    left_points = weapon.make_laser_path(left_target, 1, 1.0)
    left_side = [(point - ns["AIM_CENTER"]).dot(right) for point in left_points]
    left_target_side = (left_target - ns["AIM_CENTER"]).dot(right)
    assert min(left_side) < left_target_side - 1.2

    segment_angles = []
    for i in range(1, len(points) - 1):
        a = (points[i] - points[i - 1]).normalize()
        b = (points[i + 1] - points[i]).normalize()
        segment_angles.append(a.angle_to(b))
    assert max(segment_angles) > 8.0
    assert max(segment_angles) < 28.0


def test_laser_path_fans_sideways_before_converging(monkeypatch):
    ns = load_example(monkeypatch)
    weapon = make_weapon(ns)
    target = ns["Vec3"](0.0, 2.3, 0.0)
    weapon.firing_slimes = [object(), object()]

    left_points = weapon.make_laser_path(target, 0, 1.0)
    right_points = weapon.make_laser_path(target, 1, 1.0)

    _, right, up = weapon.camera_axes()
    left_side = [(point - ns["AIM_CENTER"]).dot(right) for point in left_points]
    right_side = [(point - ns["AIM_CENTER"]).dot(right) for point in right_points]
    left_peak = min(range(len(left_side)), key=left_side.__getitem__)
    right_peak = max(range(len(right_side)), key=right_side.__getitem__)

    assert left_side[0] == 0.0
    assert right_side[0] == 0.0
    assert abs(left_side[-1]) < 1e-5
    assert abs(right_side[-1]) < 1e-5
    assert left_side[left_peak] < -2.5
    assert right_side[right_peak] > 2.5
    assert 2 <= left_peak <= len(left_side) - 4
    assert 2 <= right_peak <= len(right_side) - 4
    assert max(left_side) <= 0.2
    assert min(right_side) >= -0.2

    target_height = (target - ns["AIM_CENTER"]).dot(up)
    for points in (left_points, right_points):
        height = [(point - ns["AIM_CENTER"]).dot(up) for point in points]
        assert max(height) < target_height + 1.5


def test_single_laser_launch_side_does_not_flip_near_center(monkeypatch):
    ns = load_example(monkeypatch)
    weapon = make_weapon(ns)
    _, right, _ = weapon.camera_axes()
    weapon.firing_slimes = [object()]

    left_target = ns["AIM_CENTER"] - right * 0.02
    right_target = ns["AIM_CENTER"] + right * 0.02
    left_points = weapon.make_laser_path(left_target, 0, 1.0)
    right_points = weapon.make_laser_path(right_target, 0, 1.0)

    assert (right_points[2] - right_points[0]).dot(right) > 0.0
    assert (left_points[2] - left_points[0]).dot(right) > 0.0


def test_fired_laser_keeps_launch_side_while_target_moves(monkeypatch):
    ns = load_example(monkeypatch)
    slime = ns["Slime"](0)
    weapon = make_weapon(ns, [slime])
    _, right, _ = weapon.camera_axes()

    slime.center = ns["AIM_CENTER"] + right * 0.6
    weapon.lock_slime(slime)
    weapon.start_fire()
    start_side = weapon.laser_fan_offset(slime.center, 0).dot(right)

    slime.center = ns["AIM_CENTER"] - right * 0.6
    moved_side = weapon.laser_fan_offset(slime.center, 0).dot(right)

    assert start_side > 0.0
    assert moved_side > 0.0


def test_laser_path_starts_straight_then_homes_later(monkeypatch):
    ns = load_example(monkeypatch)
    weapon = make_weapon(ns)
    target = ns["Vec3"](0.0, 2.3, 0.0)
    weapon.firing_slimes = [object(), object()]

    points = weapon.make_laser_path(target, 0, 1.0)
    segment_angles = []
    target_angles = []
    for i in range(1, len(points) - 1):
        before = (points[i] - points[i - 1]).normalize()
        after = (points[i + 1] - points[i]).normalize()
        segment_angles.append(before.angle_to(after))
        target_angles.append(after.angle_to((target - points[i]).normalize()))

    assert max(segment_angles[:8]) < 3.0
    assert max(segment_angles[10:20]) > max(segment_angles[:8]) * 3.0
    assert max(segment_angles) < 32.0
    assert min(target_angles[16:]) < max(target_angles[6:10]) - 40.0


def test_laser_path_launches_forward_while_fanning(monkeypatch):
    ns = load_example(monkeypatch)
    weapon = make_weapon(ns)
    target = ns["Vec3"](0.0, 2.3, 0.0)
    weapon.firing_slimes = [object(), object()]

    points = weapon.make_laser_path(target, 0, 1.0)
    forward = (target - ns["LASER_ORIGIN"]).normalize()
    _, right, _ = weapon.camera_axes()
    side = [(point - ns["LASER_ORIGIN"]).dot(right) for point in points]
    progress = [(point - ns["LASER_ORIGIN"]).dot(forward) for point in points]

    assert progress[5] > abs(side[5]) * 0.8
    assert progress[8] > 1.5
    assert abs(side[8]) > 1.5


def test_laser_emitter_is_below_subjective_view(monkeypatch):
    ns = load_example(monkeypatch)
    weapon = make_weapon(ns)

    _, _, up = weapon.camera_axes()
    emitter_height = (ns["LASER_ORIGIN"] - ns["AIM_CENTER"]).dot(up)

    assert emitter_height < -7.0


def test_laser_uses_thick_polyline_vertices(monkeypatch):
    ns = load_example(monkeypatch)
    weapon = make_weapon(ns)
    target = ns["Vec3"](0.0, 2.3, 0.0)

    beam = weapon.beams[0]
    weapon.update_beam_prim(beam, target, 0, 1.0)
    points = list(zip(*[iter(beam.positions)] * 3))

    assert len(points) == 48
    assert ns["PrimData"].MODE_TRIANGLES == beam.mode
    for i in range(0, len(points), 2):
        left = ns["Vec3"](*points[i])
        right = ns["Vec3"](*points[i + 1])
        assert left.distance_to(right) >= 0.22 * 1.5


def test_laser_width_faces_effective_camera(monkeypatch):
    ns = load_example(monkeypatch)
    eye = ns["Vec3"](4.0, 11.0, 9.0)
    weapon = make_weapon(ns, eye=eye)
    target = ns["Vec3"](0.0, 2.3, 0.0)

    centerline = weapon.make_laser_path(target, 0, 1.0)
    beam = weapon.beams[0]
    weapon.update_beam_prim(beam, target, 0, 1.0)
    vertices = list(zip(*[iter(beam.positions)] * 3))
    i = 12

    left = ns["Vec3"](*vertices[i * 2])
    right = ns["Vec3"](*vertices[i * 2 + 1])
    side = (right - left).normalize()
    tangent = (centerline[i + 1] - centerline[i - 1]).normalize()
    to_eye = (eye - centerline[i]).normalize()

    assert abs(side.dot(tangent)) < 0.01
    assert abs(side.dot(to_eye)) < 0.01
    assert abs(side.dot((ns["CAMERA_EYE"] - centerline[i]).normalize())) > 0.1


def test_mouse_position_is_converted_to_lock_ray(monkeypatch):
    ns = load_example(monkeypatch)
    eye = ns["Vec3"](4.0, 12.0, 10.0)
    weapon = make_weapon(ns, eye=eye)
    monkeypatch.setattr(pyxel, "width", 256, raising=False)
    monkeypatch.setattr(pyxel, "height", 192, raising=False)

    for mouse_x, mouse_y in ((128, 96), (143, 141), (48, 40)):
        monkeypatch.setattr(pyxel, "mouse_x", mouse_x, raising=False)
        monkeypatch.setattr(pyxel, "mouse_y", mouse_y, raising=False)

        ray_eye, ray = weapon.update_aim_ray()
        screen_x, screen_y = project_to_screen(ns, weapon, weapon.reticle)

        assert ray_eye == eye
        assert abs(ray.length() - 1.0) < 1e-5
        assert ray.angle_to(weapon.reticle - eye) < 0.05
        assert abs(screen_x - mouse_x) < 1e-4
        assert abs(screen_y - mouse_y) < 1e-4


def test_app_leaves_lock_cursor_to_cube_scene(monkeypatch):
    ns = load_example(monkeypatch)
    app = ns["App"]()
    scene_draws = []
    circles = []

    class FakeScene:
        def draw(self, *args):
            scene_draws.append(args)

    app.scene = FakeScene()
    monkeypatch.setattr(pyxel, "mouse_x", 42, raising=False)
    monkeypatch.setattr(pyxel, "mouse_y", 91, raising=False)
    monkeypatch.setattr(pyxel, "circb", lambda *args: circles.append(args))
    monkeypatch.setattr(pyxel, "text", lambda *args: None)

    app.draw()

    assert scene_draws == [(0, 0, pyxel.width, pyxel.height)]
    assert circles == []


def test_lock_cursor_is_cube_circle_and_marker_uses_frame_color(monkeypatch):
    ns = load_example(monkeypatch)
    slime = ns["Slime"](0)
    weapon = make_weapon(ns, [slime])
    slime.is_locked = True
    circles = []
    lines = []
    monkeypatch.setattr(pyxel, "frame_count", 0, raising=False)

    weapon.circb = lambda *args: circles.append(args)
    weapon.line = lambda p, q, color: lines.append((p, q, color))

    weapon.on_draw()

    assert len(circles) == 1
    center, radius, color = circles[0]
    assert center == weapon.reticle
    assert radius == 0.7
    assert color == 7

    lengths = sorted(p.distance_to(q) for p, q, _ in lines)
    assert len(lengths) == 4
    assert all(abs(length - 1.1) < 1e-5 for length in lengths)
    assert {color for _, _, color in lines} == {8}

    lines.clear()
    monkeypatch.setattr(pyxel, "frame_count", 1, raising=False)
    weapon.on_draw()

    assert {color for _, _, color in lines} == {14}


def test_lock_requires_cursor_to_overlap_slime(monkeypatch):
    ns = load_example(monkeypatch)
    weapon = make_weapon(ns)
    monkeypatch.setattr(pyxel, "width", 256, raising=False)
    monkeypatch.setattr(pyxel, "height", 192, raising=False)
    monkeypatch.setattr(pyxel, "mouse_x", 128, raising=False)
    monkeypatch.setattr(pyxel, "mouse_y", 96, raising=False)
    eye, ray = weapon.update_aim_ray()
    _, right, _ = weapon.camera_axes()

    near = ns["Slime"](0)
    near.center = ns["AIM_CENTER"] + right * 0.4
    edge = ns["Slime"](1)
    edge.center = ns["AIM_CENTER"] + right * 1.1
    away = ns["Slime"](2)
    away.center = ns["AIM_CENTER"] + right * 3.0

    assert weapon.can_lock(near, eye, ray)
    assert weapon.can_lock(edge, eye, ray)
    assert not weapon.can_lock(away, eye, ray)


def test_lock_is_forgiving_on_both_sides_of_slime(monkeypatch):
    ns = load_example(monkeypatch)
    slime = ns["Slime"](0)
    weapon = make_weapon(ns, [slime])
    slime.center = ns["AIM_CENTER"]
    _, right, _ = weapon.camera_axes()

    for side in (-1, 1):
        edge_x, edge_y = project_to_screen(
            ns, weapon, ns["AIM_CENTER"] + right * side * 1.7
        )
        monkeypatch.setattr(pyxel, "mouse_x", edge_x, raising=False)
        monkeypatch.setattr(pyxel, "mouse_y", edge_y, raising=False)
        eye, ray = weapon.update_aim_ray()
        assert weapon.can_lock(slime, eye, ray)

        overlap_x, overlap_y = project_to_screen(
            ns, weapon, ns["AIM_CENTER"] + right * side * 2.3
        )
        monkeypatch.setattr(pyxel, "mouse_x", overlap_x, raising=False)
        monkeypatch.setattr(pyxel, "mouse_y", overlap_y, raising=False)
        eye, ray = weapon.update_aim_ray()
        assert weapon.can_lock(slime, eye, ray)

        outside_x, outside_y = project_to_screen(
            ns, weapon, ns["AIM_CENTER"] + right * side * 2.8
        )
        monkeypatch.setattr(pyxel, "mouse_x", outside_x, raising=False)
        monkeypatch.setattr(pyxel, "mouse_y", outside_y, raising=False)
        eye, ray = weapon.update_aim_ray()
        assert not weapon.can_lock(slime, eye, ray)


def test_each_slime_can_lock_when_cursor_points_near_center(monkeypatch):
    ns = load_example(monkeypatch)
    slimes = [ns["Slime"](i) for i in range(ns["SLIME_COUNT"])]
    weapon = make_weapon(ns, slimes)
    monkeypatch.setattr(pyxel, "width", 256, raising=False)
    monkeypatch.setattr(pyxel, "height", 192, raising=False)
    monkeypatch.setattr(pyxel, "frame_count", 167, raising=False)
    for slime in slimes:
        slime.on_update()

    for slime in slimes:
        mouse_x, mouse_y = project_to_screen(ns, weapon, slime.center)
        monkeypatch.setattr(pyxel, "mouse_x", round(mouse_x), raising=False)
        monkeypatch.setattr(pyxel, "mouse_y", round(mouse_y), raising=False)
        eye, ray = weapon.update_aim_ray()
        assert weapon.can_lock(slime, eye, ray)


def test_back_slime_locks_when_cursor_circle_overlaps_body(monkeypatch):
    ns = load_example(monkeypatch)
    slimes = [ns["Slime"](i) for i in range(ns["SLIME_COUNT"])]
    weapon = make_weapon(ns, slimes)
    monkeypatch.setattr(pyxel, "width", 256, raising=False)
    monkeypatch.setattr(pyxel, "height", 192, raising=False)
    monkeypatch.setattr(pyxel, "frame_count", 40, raising=False)
    for slime in slimes:
        slime.on_update()

    slime = slimes[3]
    points = []
    for i in range(0, len(slime.body.positions), 3):
        local = ns["Vec3"](*slime.body.positions[i : i + 3])
        points.append(project_to_screen(ns, weapon, slime.center + local))

    mouse_x = round(max(point[0] for point in points) + 2)
    mouse_y = round(min(point[1] for point in points) - 2)
    monkeypatch.setattr(pyxel, "mouse_x", mouse_x, raising=False)
    monkeypatch.setattr(pyxel, "mouse_y", mouse_y, raising=False)
    eye, ray = weapon.update_aim_ray()

    assert slime.color == 12
    assert weapon.can_lock(slime, eye, ray)


def test_example_stays_compact_for_sample():
    assert len(EXAMPLE_PATH.read_text().splitlines()) <= 386


def test_only_user_tweak_constants_are_exposed(monkeypatch):
    ns = load_example(monkeypatch)
    constants = {
        name for name in ns if name.isupper() and not name.startswith(("__", "TEST_"))
    }

    assert constants == {
        "SLIME_COLORS",
        "SLIME_COUNT",
        "CAMERA_EYE",
        "AIM_CENTER",
        "LASER_ORIGIN",
        "FIRE_DURATION",
        "LASER_FAN_WIDTH",
        "HOMING_STRENGTH",
        "BEAM_WIDTH",
        "LASER_COLOR",
        "LASER_CORE_COLOR",
    }


def test_names_are_aligned_to_slime(monkeypatch):
    ns = load_example(monkeypatch)

    assert "Slime" in ns
    assert "SLIME_COUNT" in ns
    assert "SLIME_COLORS" in ns
    assert "Enemy" not in ns
    assert "ENEMY_COUNT" not in ns
    assert "ENEMY_COLORS" not in ns
    assert "SND_LOCK" not in ns
    assert "SND_FIRE" not in ns
    assert "SND_HIT" not in ns


def test_helper_names_describe_sample_roles(monkeypatch):
    ns = load_example(monkeypatch)
    slime = ns["Slime"](0)
    weapon = ns["Weapon"]([])

    for name in (
        "make_body_positions",
        "make_beam_prim",
        "camera_axes",
        "update_aim_ray",
        "laser_fan_offset",
        "make_laser_path",
        "update_beam_prim",
    ):
        assert hasattr(slime, name) or hasattr(weapon, name)

    for name in (
        "wobbled_positions",
        "make_beam",
        "make_beam_mesh",
        "camera_frame",
        "mouse_ray",
        "screen_pos",
        "slime_screen_radius",
        "fan_offset",
        "sample_laser_path",
        "update_beam",
        "update_beam_mesh",
    ):
        assert not hasattr(slime, name)
        assert not hasattr(weapon, name)


def test_finishing_fire_uses_simple_target_flash(monkeypatch):
    ns = load_example(monkeypatch)
    slime = ns["Slime"](0)
    weapon = ns["Weapon"]([slime])
    weapon.firing_slimes = [slime]
    weapon.fire_frame = 8

    weapon.finish_fire()

    assert slime.flash_timer == 12
    assert not hasattr(slime, "impact")
    assert not hasattr(slime, "impact_t")
    assert slime.is_locked is False
    assert weapon.firing_slimes == []
    assert weapon.fire_frame == 0


def test_laser_hits_when_beam_reaches_slime(monkeypatch):
    ns = load_example(monkeypatch)
    slime = ns["Slime"](0)
    weapon = make_weapon(ns, [slime])
    plays = ns["TEST_PLAYS"]

    monkeypatch.setattr(pyxel, "width", 256, raising=False)
    monkeypatch.setattr(pyxel, "height", 192, raising=False)
    monkeypatch.setattr(pyxel, "mouse_x", 128, raising=False)
    monkeypatch.setattr(pyxel, "mouse_y", 96, raising=False)

    weapon.lock_slime(slime)
    weapon.start_fire()
    plays.clear()

    for _ in range(ns["FIRE_DURATION"] - 1):
        weapon.on_update()

    assert weapon.fire_frame == ns["FIRE_DURATION"]
    assert slime.flash_timer == 12
    assert plays == [(2, 2)]
    assert weapon.firing_slimes == [slime]

    weapon.finish_fire()

    assert plays == [(2, 2)]
    assert weapon.firing_slimes == []
    assert weapon.fire_frame == 0


def test_sounds_are_configured_for_laser_feedback(monkeypatch):
    ns = load_example(monkeypatch)
    sounds = ns["TEST_SOUNDS"]

    lock_sound = sounds[0].set_args
    fire_sound = sounds[1].set_args
    hit_sound = sounds[2].set_args

    assert lock_sound is not None
    assert fire_sound is not None
    assert hit_sound is not None
    assert lock_sound != fire_sound
    assert fire_sound != hit_sound


def test_laser_colors_are_raystorm_like_and_not_slime_green(monkeypatch):
    ns = load_example(monkeypatch)
    slime = ns["Slime"](0)
    weapon = make_weapon(ns, [slime])
    colors = []

    weapon.firing_slimes = [slime]
    weapon.fire_frame = 1
    weapon.prim = lambda transform, data, color: colors.append(color)

    weapon.on_draw()

    assert ns["LASER_COLOR"] == 11
    assert ns["LASER_CORE_COLOR"] == 7
    assert ns["LASER_COLOR"] not in ns["SLIME_COLORS"]
    assert colors[-2:] == [ns["LASER_COLOR"], ns["LASER_CORE_COLOR"]]


def test_hit_flash_finishes_quickly(monkeypatch):
    ns = load_example(monkeypatch)
    slime = ns["Slime"](0)

    slime.trigger_hit()
    for _ in range(12):
        slime.on_update()

    assert slime.flash_timer == 0


def test_hit_flash_alternates_white_and_slime_color(monkeypatch):
    ns = load_example(monkeypatch)
    slime = ns["Slime"](0)
    colors = []

    slime.prim = lambda transform, data, color: colors.append(color)
    slime.trigger_hit()
    for _ in range(4):
        slime.on_draw()
        slime.on_update()

    assert colors == [7, slime.color, 7, slime.color]
    assert not hasattr(slime, "draw_burst")


def test_weapon_plays_sounds_when_locking_firing_and_hitting(monkeypatch):
    ns = load_example(monkeypatch)
    slime = ns["Slime"](0)
    weapon = make_weapon(ns, [slime])
    plays = ns["TEST_PLAYS"]

    weapon.lock_slime(slime)

    assert slime.is_locked is True
    assert weapon.locked_slimes == [slime]
    assert plays[-1] == (0, 0)

    weapon.start_fire()

    assert weapon.locked_slimes == []
    assert weapon.firing_slimes == [slime]
    assert weapon.fire_frame == 1
    assert plays[-1] == (1, 1)

    weapon.finish_fire()

    assert slime.flash_timer == 12
    assert plays[-1] == (2, 2)
