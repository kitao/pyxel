import inspect
from pathlib import Path

import pytest

import pyxel
from pyxel import Image
from pyxel.cube import Camera, Mat4, Mesh, Motion, Node, Vec3

from .glb_fixtures import (
    write_blockbench_profile_glb,
    write_alpha_texture_glb,
    write_authored_normals_glb,
    write_external_buffer_glb,
    write_external_image_glb,
    write_gray_alpha_texture_glb,
    write_gray_texture_glb,
    write_line_mode_glb,
    write_materialless_primitive_glb,
    write_material_animation_glb,
    write_matrix_transform_glb,
    write_morph_target_glb,
    write_normal_texture_glb,
    write_non_indexed_glb,
    write_rgb_texture_glb,
    write_single_texture_motion_glb,
    write_tinted_texture_glb,
    write_skin_glb,
    write_tangent_attribute_glb,
    write_two_material_glb,
    write_two_material_two_texture_glb,
    write_two_texture_glb,
)

EXAMPLES_DIR = Path(__file__).parents[2] / "pyxel" / "examples"


# Geometry helpers


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


def _render_mesh_colors(mesh):
    pyxel.cls(0)
    scene = Node()
    scene.camera = Camera()
    scene.camera.clear_color = 0
    scene.camera.transform = Mat4.look_at(Vec3(0, 0, 4), Vec3.ZERO, Vec3.UP)
    scene.add_child(Node.from_mesh(mesh))
    scene.draw(0, 0, pyxel.width, pyxel.height)
    return {pyxel.pget(x, y) for y in range(pyxel.height) for x in range(pyxel.width)}


# Tests


def test_from_glb_loads_single_texture_mesh(tmp_path):
    path = write_single_texture_motion_glb(tmp_path / "actor.glb")
    mesh = Mesh.from_glb(str(path), colkey=0)

    assert len(mesh.primitives) == 2
    assert mesh.primitives[0] is None
    assert mesh.primitives[1] is not None
    assert mesh.names == ["actor", "actor_primitive_0"]
    assert isinstance(mesh.col_img, Image)
    assert mesh.colkey == 0


def test_from_glb_loads_blockbench_profile_textured_model(tmp_path):
    path = write_blockbench_profile_glb(tmp_path / "blockbench.glb")
    mesh = Mesh.from_glb(str(path), colkey=0, fps=20.0)

    assert mesh.names == [
        "bb_scene",
        "body",
        "body_primitive_0",
        "face",
        "face_primitive_0",
    ]
    assert len(mesh.primitives) == 5
    assert mesh.primitives[0] is None
    assert mesh.primitives[1] is None
    assert mesh.primitives[2] is not None
    assert mesh.primitives[3] is None
    assert mesh.primitives[4] is not None
    assert isinstance(mesh.col_img, Image)

    colors = _render_mesh_colors(mesh)

    assert 8 in colors
    assert 10 in colors


def test_from_glb_loads_blockbench_profile_motion(tmp_path):
    path = write_blockbench_profile_glb(tmp_path / "blockbench_motion.glb")
    mesh = Mesh.from_glb(str(path), colkey=0, fps=20.0)
    root = Node.from_mesh(mesh)

    assert len(mesh.motions) == 1
    assert mesh.motions[0].name == "idle"
    assert mesh.motions[0].length == 20.0

    root.apply_motion(mesh.motions[0], 10.0)

    body = root.children[0]
    assert root.transform.pos == Vec3(0.25, 0.0, 0.0)
    assert body.transform.scale == Vec3(1.0, 1.5, 1.0)
    assert body.transform.rot.to_euler().z == pytest.approx(45.0)


def test_from_glb_loads_blockbench_profile_smooth_motion(tmp_path):
    path = write_blockbench_profile_glb(
        tmp_path / "blockbench_smooth.glb", smooth_motion=True
    )
    mesh = Mesh.from_glb(str(path), colkey=0, fps=20.0)
    root = Node.from_mesh(mesh)

    root.apply_motion(mesh.motions[0], 5.0)

    assert root.transform.pos.x == pytest.approx(0.125)


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


def test_from_glb_converts_rgb_texture(tmp_path):
    path = write_rgb_texture_glb(tmp_path / "rgb.glb")
    mesh = Mesh.from_glb(str(path), colkey=0)

    assert isinstance(mesh.col_img, Image)


def test_from_glb_converts_gray_texture(tmp_path):
    path = write_gray_texture_glb(tmp_path / "gray.glb")
    mesh = Mesh.from_glb(str(path), colkey=0)

    assert isinstance(mesh.col_img, Image)


def test_from_glb_converts_gray_alpha_texture(tmp_path):
    path = write_gray_alpha_texture_glb(tmp_path / "gray_alpha.glb")
    mesh = Mesh.from_glb(str(path), colkey=0)

    assert isinstance(mesh.col_img, Image)


def test_from_glb_loads_non_indexed_primitive(tmp_path):
    path = write_non_indexed_glb(tmp_path / "non_indexed.glb")
    mesh = Mesh.from_glb(str(path))
    primitive = next(p for p in mesh.primitives if p is not None)

    # Two triangles read straight from POSITION; no indices accessor.
    assert len(primitive.positions) == 18
    assert len(primitive.indices) == 0


def test_from_glb_uses_authored_normals_when_present(tmp_path):
    path = write_authored_normals_glb(tmp_path / "authored_normals.glb")
    mesh = Mesh.from_glb(str(path))
    primitive = next(p for p in mesh.primitives if p is not None)

    assert tuple(primitive.normals) == pytest.approx((0.0, 0.0, -1.0))


def test_from_glb_loads_motion(tmp_path):
    path = write_single_texture_motion_glb(tmp_path / "actor.glb")
    mesh = Mesh.from_glb(str(path), fps=30.0)

    assert len(mesh.motions) == 1
    assert isinstance(mesh.motions[0], Motion)
    assert mesh.motions[0].name == "slide"
    assert mesh.motions[0].length == 30.0


def test_from_glb_ignores_alpha_texture_without_colkey(tmp_path):
    path = write_alpha_texture_glb(tmp_path / "alpha.glb")
    mesh = Mesh.from_glb(str(path))

    assert isinstance(mesh.col_img, Image)


def test_from_glb_converts_mask_alpha_texture_pixels_to_auto_colkey(tmp_path):
    path = write_single_texture_motion_glb(
        tmp_path / "mask_auto_colkey.glb",
        texture_pixels=[
            (255, 255, 255, 255),
            (255, 0, 0, 255),
            (0, 255, 0, 0),
            (0, 0, 255, 255),
        ],
        alpha_mode="MASK",
    )
    mesh = Mesh.from_glb(str(path))

    assert isinstance(mesh.col_img, Image)
    assert mesh.col_img.pget(0, 1) == 0
    assert mesh.colkey == 0


def test_from_glb_converts_mask_alpha_texture_pixels_to_requested_colkey(tmp_path):
    path = write_single_texture_motion_glb(
        tmp_path / "mask_requested_colkey.glb",
        texture_pixels=[
            (255, 255, 255, 255),
            (255, 0, 0, 255),
            (0, 255, 0, 0),
            (0, 0, 255, 255),
        ],
        alpha_mode="MASK",
    )
    mesh = Mesh.from_glb(str(path), colkey=2)

    assert isinstance(mesh.col_img, Image)
    assert mesh.col_img.pget(0, 1) == 2
    assert mesh.colkey == 2


def test_from_glb_uses_mask_alpha_cutoff(tmp_path):
    path = write_single_texture_motion_glb(
        tmp_path / "mask_alpha_cutoff.glb",
        texture_pixels=[
            (255, 255, 255, 255),
            (255, 0, 0, 255),
            (0, 255, 0, 20),
            (0, 0, 255, 0),
        ],
        alpha_mode="MASK",
        alpha_cutoff=0.05,
    )
    mesh = Mesh.from_glb(str(path))

    assert isinstance(mesh.col_img, Image)
    assert mesh.col_img.pget(0, 1) != mesh.colkey
    assert mesh.col_img.pget(1, 1) == mesh.colkey


def test_from_glb_rejects_mask_colkey_collision(tmp_path):
    path = write_single_texture_motion_glb(
        tmp_path / "mask_colkey_collision.glb",
        texture_pixels=[
            (255, 255, 255, 255),
            (255, 0, 0, 255),
            (0, 255, 0, 0),
            (0, 0, 255, 255),
        ],
        alpha_mode="MASK",
    )

    with pytest.raises(ValueError, match="colkey"):
        Mesh.from_glb(str(path), colkey=8)


def test_from_glb_rejects_mask_auto_colkey_when_palette_is_full(tmp_path):
    path = write_single_texture_motion_glb(
        tmp_path / "mask_full_palette.glb",
        texture_pixels=[
            (
                (color >> 16) & 0xFF,
                (color >> 8) & 0xFF,
                color & 0xFF,
                255,
            )
            for color in pyxel.DEFAULT_COLORS
        ]
        + [(0, 0, 0, 0)],
        texture_size=(17, 1),
        alpha_mode="MASK",
    )

    with pytest.raises(ValueError, match="unused colkey"):
        Mesh.from_glb(str(path))


def test_from_glb_loads_multiple_flat_materials(tmp_path):
    path = write_two_material_two_texture_glb(
        tmp_path / "flat_materials.glb", textured=False
    )
    mesh = Mesh.from_glb(str(path))

    colors = _render_mesh_colors(mesh)

    assert 8 in colors
    assert 3 in colors


def test_from_glb_loads_multiple_base_color_textures(tmp_path):
    path = write_two_material_two_texture_glb(
        tmp_path / "two_textures.glb", textured=True
    )
    mesh = Mesh.from_glb(str(path), colkey=0)

    colors = _render_mesh_colors(mesh)

    assert 8 in colors
    assert 3 in colors


def test_from_glb_applies_textured_material_factor(tmp_path):
    path = write_tinted_texture_glb(tmp_path / "tinted_texture.glb")
    mesh = Mesh.from_glb(str(path), colkey=0)

    assert isinstance(mesh.col_img, Image)
    assert {mesh.col_img.pget(x, y) for y in range(2) for x in range(2)} == {8}


def test_from_glb_rejects_materialless_primitive(tmp_path):
    path = write_materialless_primitive_glb(tmp_path / "materialless.glb")

    with pytest.raises(ValueError, match="material"):
        Mesh.from_glb(str(path), colkey=0)


def test_from_glb_loads_unused_extra_material_and_texture(tmp_path):
    path = write_two_texture_glb(tmp_path / "two_textures.glb")
    mesh = Mesh.from_glb(str(path), colkey=0)

    assert isinstance(mesh.col_img, Image)


def test_from_glb_loads_unused_extra_material(tmp_path):
    path = write_two_material_glb(tmp_path / "two_materials.glb")
    mesh = Mesh.from_glb(str(path), colkey=0)

    assert isinstance(mesh.col_img, Image)


def test_from_glb_rejects_unsupported_texture_usage(tmp_path):
    path = write_normal_texture_glb(tmp_path / "normal_texture.glb")

    with pytest.raises(ValueError, match="unsupported texture usage"):
        Mesh.from_glb(str(path))


def test_from_glb_rejects_material_animation(tmp_path):
    path = write_material_animation_glb(tmp_path / "material_animation.glb")

    with pytest.raises(ValueError, match="animation pointer"):
        Mesh.from_glb(str(path))


def test_from_glb_rejects_external_buffer(tmp_path):
    path = write_external_buffer_glb(tmp_path / "external_buffer.glb")

    with pytest.raises(ValueError, match="external buffers"):
        Mesh.from_glb(str(path))


def test_from_glb_rejects_external_image(tmp_path):
    path = write_external_image_glb(tmp_path / "external_image.glb")

    # The gltf import of a GLB slice rejects any image URI itself, so
    # the failure surfaces as an import error rather than the parser's
    # own external-image message.
    with pytest.raises(ValueError, match="external reference"):
        Mesh.from_glb(str(path))


def test_from_glb_rejects_non_triangle_mode(tmp_path):
    path = write_line_mode_glb(tmp_path / "lines.glb")

    with pytest.raises(ValueError, match="only triangle primitives"):
        Mesh.from_glb(str(path))


def test_from_glb_rejects_non_indexed_vertex_count_not_multiple_of_3(tmp_path):
    path = write_non_indexed_glb(tmp_path / "ragged.glb", vertex_count=4)

    with pytest.raises(ValueError, match="multiple of 3"):
        Mesh.from_glb(str(path))


def test_from_glb_rejects_morph_targets(tmp_path):
    path = write_morph_target_glb(tmp_path / "morph.glb")

    with pytest.raises(ValueError, match="morph targets"):
        Mesh.from_glb(str(path))


def test_from_glb_rejects_skins(tmp_path):
    path = write_skin_glb(tmp_path / "skin.glb")

    with pytest.raises(ValueError, match="skins"):
        Mesh.from_glb(str(path))


def test_from_glb_rejects_matrix_node_transforms(tmp_path):
    path = write_matrix_transform_glb(tmp_path / "matrix_transform.glb")

    with pytest.raises(ValueError, match="matrix node transforms"):
        Mesh.from_glb(str(path))


def test_from_glb_rejects_non_blockbench_vertex_attributes(tmp_path):
    path = write_tangent_attribute_glb(tmp_path / "tangent_attribute.glb")

    with pytest.raises(ValueError, match="unsupported vertex attribute"):
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

    # One update advances the playhead by the default speed (1.0), so
    # the linear 0-to-1 slide over 30 frames sits at x = 1 / 30.
    assert root.transform.pos == Vec3(1.0 / 30.0, 0.0, 0.0)


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
