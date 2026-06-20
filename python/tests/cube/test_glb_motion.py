import inspect
from pathlib import Path

import pytest

from pyxel import Image
from pyxel.cube import Mesh, Motion, Node, Vec3

from .glb_fixtures import (
    write_alpha_texture_glb,
    write_morph_target_glb,
    write_single_texture_motion_glb,
    write_skin_glb,
    write_two_texture_glb,
)


EXAMPLES_DIR = Path(__file__).parents[2] / "pyxel" / "examples"


def _vec3(values, index):
    base = index * 3
    return tuple(values[base : base + 3])


def _sub(a, b):
    return tuple(a[i] - b[i] for i in range(3))


def _cross(a, b):
    return (
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    )


def _dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def _center(a, b, c):
    return tuple((a[i] + b[i] + c[i]) / 3.0 for i in range(3))


def test_from_glb_loads_single_texture_mesh(tmp_path):
    path = write_single_texture_motion_glb(tmp_path / "actor.glb")
    mesh = Mesh.from_glb(str(path), colkey=0)

    assert len(mesh.primitives) == 2
    assert mesh.primitives[0] is None
    assert mesh.primitives[1] is not None
    assert mesh.names == ["actor", "actor_primitive_0"]
    assert isinstance(mesh.col_img, Image)
    assert mesh.colkey == 0


def test_bundled_actor_cube_is_closed_and_outward_wound():
    path = EXAMPLES_DIR / "assets" / "cube_actor.glb"
    mesh = Mesh.from_glb(str(path), colkey=0)
    primitives = [primitive for primitive in mesh.primitives if primitive is not None]

    assert primitives
    for primitive in primitives:
        positions = list(primitive.positions)
        indices = list(primitive.indices)
        coord_edges = {}

        for offset in range(0, len(indices), 3):
            ia, ib, ic = indices[offset : offset + 3]
            a = _vec3(positions, ia)
            b = _vec3(positions, ib)
            c = _vec3(positions, ic)
            normal = _cross(_sub(b, a), _sub(c, a))

            assert _dot(normal, _center(a, b, c)) > 0.0
            for edge in ((a, b), (b, c), (c, a)):
                key = tuple(sorted(edge))
                coord_edges[key] = coord_edges.get(key, 0) + 1

        assert set(coord_edges.values()) == {2}


def test_from_glb_loads_motion(tmp_path):
    path = write_single_texture_motion_glb(tmp_path / "actor.glb")
    mesh = Mesh.from_glb(str(path), fps=30.0)

    assert len(mesh.motions) == 1
    assert isinstance(mesh.motions[0], Motion)
    assert mesh.motions[0].name == "slide"
    assert mesh.motions[0].length == 30.0


def test_from_glb_rejects_alpha_texture(tmp_path):
    path = write_alpha_texture_glb(tmp_path / "alpha.glb")

    with pytest.raises(ValueError, match="alpha"):
        Mesh.from_glb(str(path))


def test_from_glb_rejects_multiple_textures(tmp_path):
    path = write_two_texture_glb(tmp_path / "two_textures.glb")

    with pytest.raises(ValueError, match="multiple textures"):
        Mesh.from_glb(str(path))


def test_from_glb_rejects_morph_targets(tmp_path):
    path = write_morph_target_glb(tmp_path / "morph.glb")

    with pytest.raises(ValueError, match="morph targets"):
        Mesh.from_glb(str(path))


def test_from_glb_rejects_skins(tmp_path):
    path = write_skin_glb(tmp_path / "skin.glb")

    with pytest.raises(ValueError, match="skins"):
        Mesh.from_glb(str(path))


def test_apply_motion_updates_imported_node_tree(tmp_path):
    path = write_single_texture_motion_glb(tmp_path / "actor.glb")
    mesh = Mesh.from_glb(str(path), fps=30.0)
    root = Node.from_mesh(mesh)

    root.apply_motion(mesh.motions[0], 15.0)

    assert root.transform.pos == Vec3(0.5, 0.0, 0.0)


def test_play_motion_advances_during_update(tmp_path):
    path = write_single_texture_motion_glb(tmp_path / "actor.glb")
    mesh = Mesh.from_glb(str(path), fps=30.0)
    root = Node.from_mesh(mesh)

    root.play_motion(mesh.motions[0], start_frame=0.0)
    root.update()

    assert root.transform.pos.x > 0.0


def test_stop_motion_leaves_current_pose(tmp_path):
    path = write_single_texture_motion_glb(tmp_path / "actor.glb")
    mesh = Mesh.from_glb(str(path), fps=30.0)
    root = Node.from_mesh(mesh)

    root.apply_motion(mesh.motions[0], 10.0)
    assert root.transform.pos == Vec3(1.0 / 3.0, 0.0, 0.0)

    root.stop_motion()
    before = root.transform
    root.update()

    assert root.transform == before


def test_apply_motion_rejects_unrelated_node_tree(tmp_path):
    path = write_single_texture_motion_glb(tmp_path / "actor.glb")
    mesh = Mesh.from_glb(str(path), fps=30.0)

    with pytest.raises(ValueError, match="Node.from_mesh"):
        Node().apply_motion(mesh.motions[0], 0.0)


def test_motion_api_signatures():
    assert "colkey" in str(inspect.signature(Mesh.from_glb))
    assert "fps" in str(inspect.signature(Mesh.from_glb))
    assert "loop=True" in str(inspect.signature(Node.apply_motion))
    assert "start_frame" in str(inspect.signature(Node.play_motion))
