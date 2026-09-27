import struct

import pytest
from pyxel.cube import Collider, Mat4, Mesh, Node, Primitive, Vec3

from ._glb_fixtures import _write_glb


@pytest.fixture
def animated_mesh(tmp_path):
    # Disjoint clips: one moves the authored root, the other its limb.
    # The ornament is intentionally never animated.
    data = struct.pack("<8f", 0, 1, 3, 0, 0, 5, 0, 0)
    document = {
        "asset": {"version": "2.0"},
        "buffers": [{"byteLength": len(data)}],
        "bufferViews": [
            {"buffer": 0, "byteOffset": 0, "byteLength": 8},
            {"buffer": 0, "byteOffset": 8, "byteLength": 24},
        ],
        "accessors": [
            {
                "bufferView": 0,
                "componentType": 5126,
                "count": 2,
                "type": "SCALAR",
                "min": [0],
                "max": [1],
            },
            {"bufferView": 1, "componentType": 5126, "count": 2, "type": "VEC3"},
        ],
        "nodes": [
            {"name": "actor", "translation": [3, 0, 0], "children": [1, 2]},
            {"name": "limb", "translation": [0, 2, 0]},
            {"name": "ornament", "translation": [0, 0, 4]},
        ],
        "scenes": [{"nodes": [0]}],
        "scene": 0,
        "animations": [
            {
                "name": name,
                "samplers": [{"input": 0, "output": 1}],
                "channels": [
                    {"sampler": 0, "target": {"node": index, "path": "translation"}}
                ],
            }
            for name, index in [("root_move", 0), ("limb_move", 1)]
        ]
        + [{"name": "empty", "samplers": [], "channels": []}],
    }
    return Mesh.from_glb(str(_write_glb(tmp_path / "actor.glb", document, data)))


@pytest.mark.parametrize("roots", [0, 1, 2])
def test_from_mesh_always_separates_placement_from_authored_roots(roots):
    mesh = Mesh(
        [None] * roots,
        [Mat4.from_translation(Vec3(3, 0, 0))] * roots,
        [-1] * roots,
        [f"part{i}" for i in range(roots)],
    )
    model = Node.from_mesh(mesh)
    assert model.name == ""
    assert model.transform == Mat4.IDENTITY
    assert len(model.children) == roots

    model.transform = Mat4.from_translation(Vec3(100, 0, 0))
    for child in model.children:
        assert child.parent is model
        assert child.transform.pos == Vec3(3, 0, 0)
        assert child.world_transform.pos == Vec3(103, 0, 0)


def test_placement_root_aligns_render_parts_and_mesh_collider():
    mesh = Mesh(
        [Primitive.box(Vec3(2, 2, 2))],
        [Mat4.from_translation(Vec3(3, 0, 0))],
        [-1],
    )
    model = Node.from_mesh(mesh)
    model.transform = Mat4.from_translation(Vec3(100, 0, 0))
    model.collider = Collider(mesh=mesh)
    hit = model.raycast(Vec3(103, 5, 0), Vec3.DOWN)
    assert hit.node is model
    assert hit.point == Vec3(103, 1, 0)
    assert model.children[0].world_transform.pos == Vec3(103, 0, 0)


def test_motion_preserves_placement_and_nested_instance(animated_mesh):
    outer = Node.from_mesh(animated_mesh)
    inner = Node.from_mesh(animated_mesh)
    outer.transform = Mat4.from_translation(Vec3(100, 0, 0))
    inner.transform = Mat4.from_translation(Vec3(0, 99, 0))
    outer.add_child(inner)

    inner.apply_motion(animated_mesh.motions[1], 15)
    limb = inner.find_by_name("limb")[0]
    inner_pose = limb.transform

    outer.apply_motion(animated_mesh.motions[0], 15)
    assert outer.transform.pos == Vec3(100, 0, 0)
    assert outer.children[0].transform.pos == Vec3(4, 0, 0)
    assert inner.transform.pos == Vec3(0, 99, 0)
    assert inner.children[0].transform.pos == Vec3(3, 0, 0)
    assert limb.transform == inner_pose

    outer.play_motion(animated_mesh.motions[1], start_frame=15)
    assert inner.children[0].transform.pos == Vec3(3, 0, 0)
    assert limb.transform == inner_pose


def test_play_resets_union_to_authored_pose_but_not_static_parts(animated_mesh):
    # Asset edits do not redefine the clips' authored baseline.
    transforms = animated_mesh.transforms
    transforms[0] = Mat4.from_scale(Vec3(2, 2, 2))
    animated_mesh.transforms = transforms
    model = Node.from_mesh(animated_mesh)
    actor = model.find_by_name("actor")[0]
    limb = model.find_by_name("limb")[0]
    ornament = model.find_by_name("ornament")[0]
    ornament.transform = Mat4.from_translation(Vec3(0, 0, 99))
    attachment = Node()
    attachment.transform = Mat4.from_translation(Vec3(0, 10, 0))
    actor.add_child(attachment)

    model.apply_motion(animated_mesh.motions[0], 30, loop=False)
    assert actor.transform.pos == Vec3(5, 0, 0)

    model.apply_motion(animated_mesh.motions[1], 30, loop=False)
    assert actor.transform.pos == Vec3(5, 0, 0)  # Sampling stays selective.

    model.play_motion(animated_mesh.motions[1], start_frame=15)
    assert actor.transform == Mat4.from_translation(Vec3(3, 0, 0))
    assert limb.transform.pos == Vec3(4, 0, 0)
    assert ornament.transform.pos == Vec3(0, 0, 99)
    assert attachment.transform.pos == Vec3(0, 10, 0)
    assert model.transform == Mat4.IDENTITY

    actor.transform = Mat4.from_translation(Vec3(12, 0, 0))
    model.update()
    assert actor.transform.pos == Vec3(12, 0, 0)  # No per-frame reset.

    model.stop_motion()
    pose = limb.transform
    model.update()
    assert limb.transform == pose


def test_subpart_reset_and_reparenting_obey_current_instance_scope(animated_mesh):
    model = Node.from_mesh(animated_mesh)
    actor = model.find_by_name("actor")[0]
    limb = model.find_by_name("limb")[0]
    actor.transform = Mat4.from_translation(Vec3(99, 0, 0))
    limb.play_motion(animated_mesh.motions[1], start_frame=15)
    assert actor.transform.pos == Vec3(99, 0, 0)
    assert limb.transform.pos == Vec3(4, 0, 0)

    limb.stop_motion()
    actor.remove_child(limb)
    model.play_motion(animated_mesh.motions[1], start_frame=0)
    assert limb.transform.pos == Vec3(4, 0, 0)

    model.add_child(limb)
    model.play_motion(animated_mesh.motions[0], start_frame=0)
    assert limb.transform.pos == Vec3(0, 2, 0)


def test_empty_clip_restores_animation_owned_parts(animated_mesh):
    model = Node.from_mesh(animated_mesh)
    model.apply_motion(animated_mesh.motions[0], 30, loop=False)
    model.play_motion(animated_mesh.motions[2])
    assert model.children[0].transform.pos == Vec3(3, 0, 0)
