from __future__ import annotations

import base64
import json
import struct
import zlib
from pathlib import Path

# Binary container builders


def _pad4(data: bytes, pad: bytes = b" ") -> bytes:
    # glTF chunks are 4-byte aligned: JSON pads with spaces, BIN with zeros.
    return data + pad * ((4 - len(data) % 4) % 4)


def _png(
    width: int,
    height: int,
    color_type: int,
    pixels: list[tuple[int, ...]],
) -> bytes:
    raw = bytearray()
    for y in range(height):
        raw.append(0)  # per-scanline filter byte: 0 = None
        for x in range(width):
            raw.extend(pixels[y * width + x])

    def chunk(kind: bytes, payload: bytes) -> bytes:
        body = kind + payload
        return (
            struct.pack(">I", len(payload))
            + body
            + struct.pack(">I", zlib.crc32(body) & 0xFFFFFFFF)
        )

    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(
            b"IHDR",
            # Fields: width, height, bit depth 8, color type (0 = gray,
            # 2 = RGB, 4 = gray+alpha, 6 = RGBA), then compression /
            # filter / interlace methods, each 0 (the only PNG methods).
            struct.pack(">IIBBBBB", width, height, 8, color_type, 0, 0, 0),
        )
        + chunk(b"IDAT", zlib.compress(bytes(raw)))
        + chunk(b"IEND", b"")
    )


def _write_glb(path: Path, gltf: dict, bin_blob: bytes) -> Path:
    json_chunk = _pad4(json.dumps(gltf, separators=(",", ":")).encode())
    bin_chunk = _pad4(bin_blob, b"\x00")
    total_len = 12 + 8 + len(json_chunk) + 8 + len(bin_chunk)
    # GLB container: 12-byte header (magic, version 2, total length),
    # then length-prefixed JSON and BIN chunks.
    path.write_bytes(
        b"glTF"
        + struct.pack("<II", 2, total_len)
        + struct.pack("<I4s", len(json_chunk), b"JSON")
        + json_chunk
        + struct.pack("<I4s", len(bin_chunk), b"BIN\x00")
        + bin_chunk
    )
    return path


# Fixture writers


def write_blockbench_profile_glb(path: Path, *, smooth_motion: bool = False) -> Path:
    def cuboid(width: float, height: float, depth: float):
        hx = width / 2.0
        hy = height / 2.0
        hz = depth / 2.0
        faces = [
            (
                [(0.0, 0.0, 1.0)] * 4,
                [(-hx, -hy, hz), (hx, -hy, hz), (hx, hy, hz), (-hx, hy, hz)],
            ),
            (
                [(0.0, 0.0, -1.0)] * 4,
                [(hx, -hy, -hz), (-hx, -hy, -hz), (-hx, hy, -hz), (hx, hy, -hz)],
            ),
            (
                [(1.0, 0.0, 0.0)] * 4,
                [(hx, -hy, hz), (hx, -hy, -hz), (hx, hy, -hz), (hx, hy, hz)],
            ),
            (
                [(-1.0, 0.0, 0.0)] * 4,
                [(-hx, -hy, -hz), (-hx, -hy, hz), (-hx, hy, hz), (-hx, hy, -hz)],
            ),
            (
                [(0.0, 1.0, 0.0)] * 4,
                [(-hx, hy, hz), (hx, hy, hz), (hx, hy, -hz), (-hx, hy, -hz)],
            ),
            (
                [(0.0, -1.0, 0.0)] * 4,
                [(-hx, -hy, -hz), (hx, -hy, -hz), (hx, -hy, hz), (-hx, -hy, hz)],
            ),
        ]
        positions: list[float] = []
        normals: list[float] = []
        uvs: list[float] = []
        indices: list[int] = []
        for face_index, (face_normals, face_positions) in enumerate(faces):
            base = face_index * 4
            for position in face_positions:
                positions.extend(position)
            for normal in face_normals:
                normals.extend(normal)
            uvs.extend([0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0])
            indices.extend([base, base + 1, base + 2, base, base + 2, base + 3])
        return (
            struct.pack("<" + "f" * len(positions), *positions),
            struct.pack("<" + "f" * len(normals), *normals),
            struct.pack("<" + "f" * len(uvs), *uvs),
            struct.pack("<" + "H" * len(indices), *indices),
        )

    body_positions, body_normals, body_uvs, body_indices = cuboid(1.2, 1.2, 0.8)
    face_positions, face_normals, face_uvs, face_indices = cuboid(0.55, 0.45, 0.1)
    times = struct.pack("<fff", 0.0, 0.5, 1.0)
    # Encode smooth or linear animation samples
    if smooth_motion:
        translation_interpolation = "CUBICSPLINE"
        translation_accessor_count = 9
        translations = struct.pack(
            "<" + "f" * 27,
            0.5,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.5,
            0.0,
            0.0,
            0.5,
            0.0,
            0.0,
            0.25,
            0.0,
            0.0,
            0.5,
            0.0,
            0.0,
            0.5,
            0.0,
            0.0,
            0.5,
            0.0,
            0.0,
            0.5,
            0.0,
            0.0,
        )
    else:
        translation_interpolation = "LINEAR"
        translation_accessor_count = 3
        translations = struct.pack(
            "<fffffffff",
            0.0,
            0.0,
            0.0,
            0.25,
            0.0,
            0.0,
            0.5,
            0.0,
            0.0,
        )
    sin_22_5 = 0.3826834323650898
    cos_22_5 = 0.9238795325112867
    sin_45 = 0.7071067811865475
    rotations = struct.pack(
        "<ffffffffffff",
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        sin_22_5,
        cos_22_5,
        0.0,
        0.0,
        sin_45,
        sin_45,
    )
    scales = struct.pack(
        "<fffffffff",
        1.0,
        1.0,
        1.0,
        1.0,
        1.5,
        1.0,
        1.0,
        1.0,
        1.0,
    )
    red_png = _png(2, 2, 6, [(255, 0, 0, 255)] * 4)
    yellow_png = _png(2, 2, 6, [(255, 255, 0, 255)] * 4)

    data_chunks = [
        body_positions,
        body_normals,
        body_uvs,
        body_indices,
        face_positions,
        face_normals,
        face_uvs,
        face_indices,
        times,
        translations,
        rotations,
        scales,
        red_png,
        yellow_png,
    ]
    chunks: list[bytes] = []
    offsets: list[int] = []
    cursor = 0
    for data in data_chunks:
        offsets.append(cursor)
        chunks.append(data)
        cursor += len(data)
        pad = (4 - cursor % 4) % 4
        if pad:
            chunks.append(b"\x00" * pad)
            cursor += pad
    bin_blob = b"".join(chunks)

    gltf = {
        "asset": {"version": "2.0", "generator": "Blockbench"},
        "scene": 0,
        "scenes": [{"nodes": [0]}],
        "nodes": [
            {"name": "bb_scene", "children": [1, 2]},
            {"name": "body", "mesh": 0},
            {"name": "face", "mesh": 1, "translation": [0.0, 0.0, 0.55]},
        ],
        "meshes": [
            {
                "name": "body",
                "primitives": [
                    {
                        "attributes": {
                            "POSITION": 0,
                            "NORMAL": 1,
                            "TEXCOORD_0": 2,
                        },
                        "indices": 3,
                        "material": 0,
                        "mode": 4,
                    }
                ],
            },
            {
                "name": "face",
                "primitives": [
                    {
                        "attributes": {
                            "POSITION": 4,
                            "NORMAL": 5,
                            "TEXCOORD_0": 6,
                        },
                        "indices": 7,
                        "material": 1,
                        "mode": 4,
                    }
                ],
            },
        ],
        "materials": [
            {
                "alphaCutoff": 0.05,
                "alphaMode": "MASK",
                "pbrMetallicRoughness": {"baseColorTexture": {"index": 0}},
            },
            {
                "alphaCutoff": 0.05,
                "alphaMode": "MASK",
                "pbrMetallicRoughness": {"baseColorTexture": {"index": 1}},
            },
        ],
        "textures": [{"source": 0}, {"source": 1}],
        "images": [
            {"bufferView": 12, "mimeType": "image/png"},
            {"bufferView": 13, "mimeType": "image/png"},
        ],
        "buffers": [{"byteLength": len(bin_blob)}],
        "bufferViews": [
            {"buffer": 0, "byteOffset": offsets[0], "byteLength": len(body_positions)},
            {"buffer": 0, "byteOffset": offsets[1], "byteLength": len(body_normals)},
            {"buffer": 0, "byteOffset": offsets[2], "byteLength": len(body_uvs)},
            {"buffer": 0, "byteOffset": offsets[3], "byteLength": len(body_indices)},
            {"buffer": 0, "byteOffset": offsets[4], "byteLength": len(face_positions)},
            {"buffer": 0, "byteOffset": offsets[5], "byteLength": len(face_normals)},
            {"buffer": 0, "byteOffset": offsets[6], "byteLength": len(face_uvs)},
            {"buffer": 0, "byteOffset": offsets[7], "byteLength": len(face_indices)},
            {"buffer": 0, "byteOffset": offsets[8], "byteLength": len(times)},
            {"buffer": 0, "byteOffset": offsets[9], "byteLength": len(translations)},
            {"buffer": 0, "byteOffset": offsets[10], "byteLength": len(rotations)},
            {"buffer": 0, "byteOffset": offsets[11], "byteLength": len(scales)},
            {"buffer": 0, "byteOffset": offsets[12], "byteLength": len(red_png)},
            {"buffer": 0, "byteOffset": offsets[13], "byteLength": len(yellow_png)},
        ],
        "accessors": [
            {
                "bufferView": 0,
                "componentType": 5126,
                "count": 24,
                "type": "VEC3",
                "min": [-0.6, -0.6, -0.4],
                "max": [0.6, 0.6, 0.4],
            },
            {"bufferView": 1, "componentType": 5126, "count": 24, "type": "VEC3"},
            {"bufferView": 2, "componentType": 5126, "count": 24, "type": "VEC2"},
            {"bufferView": 3, "componentType": 5123, "count": 36, "type": "SCALAR"},
            {
                "bufferView": 4,
                "componentType": 5126,
                "count": 24,
                "type": "VEC3",
                "min": [-0.275, -0.225, -0.05],
                "max": [0.275, 0.225, 0.05],
            },
            {"bufferView": 5, "componentType": 5126, "count": 24, "type": "VEC3"},
            {"bufferView": 6, "componentType": 5126, "count": 24, "type": "VEC2"},
            {"bufferView": 7, "componentType": 5123, "count": 36, "type": "SCALAR"},
            {
                "bufferView": 8,
                "componentType": 5126,
                "count": 3,
                "type": "SCALAR",
                "min": [0.0],
                "max": [1.0],
            },
            {
                "bufferView": 9,
                "componentType": 5126,
                "count": translation_accessor_count,
                "type": "VEC3",
            },
            {"bufferView": 10, "componentType": 5126, "count": 3, "type": "VEC4"},
            {"bufferView": 11, "componentType": 5126, "count": 3, "type": "VEC3"},
        ],
        "animations": [
            {
                "name": "idle",
                "samplers": [
                    {
                        "input": 8,
                        "output": 9,
                        "interpolation": translation_interpolation,
                    },
                    {"input": 8, "output": 10, "interpolation": "LINEAR"},
                    {"input": 8, "output": 11, "interpolation": "STEP"},
                ],
                "channels": [
                    {"sampler": 0, "target": {"node": 0, "path": "translation"}},
                    {"sampler": 1, "target": {"node": 1, "path": "rotation"}},
                    {"sampler": 2, "target": {"node": 1, "path": "scale"}},
                ],
            }
        ],
    }

    return _write_glb(path, gltf, bin_blob)


def write_single_texture_motion_glb(
    path: Path,
    *,
    texture_pixels: list[tuple[int, int, int, int]] | None = None,
    texture_size: tuple[int, int] = (2, 2),
    texture_count: int = 1,
    material_count: int = 1,
    png_color_type: int = 6,
    primitive_mode: int = 4,
    external_buffer: bool = False,
    external_image: bool = False,
    morph_target: bool = False,
    skin: bool = False,
    matrix_transform: bool = False,
    normal_texture: bool = False,
    material_animation: bool = False,
    base_color_factor: list[float] | None = None,
    tangent_attribute: bool = False,
    alpha_mode: str | None = None,
    alpha_cutoff: float | None = None,
) -> Path:
    positions = struct.pack(
        "<ffffffffffff",
        -0.5,
        -0.5,
        0.0,
        0.5,
        -0.5,
        0.0,
        0.5,
        0.5,
        0.0,
        -0.5,
        0.5,
        0.0,
    )
    uvs = struct.pack("<ffffffff", 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0)
    indices = struct.pack("<HHHHHH", 0, 1, 2, 0, 2, 3)
    tangents = struct.pack("<" + "f" * 16, *([0.0, 0.0, 1.0, 1.0] * 4))
    times = struct.pack("<ff", 0.0, 1.0)
    translations = struct.pack("<ffffff", 0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
    morph_positions = struct.pack("<ffffffffffff", *([0.0] * 12))
    if texture_pixels is None:
        texture_pixels = [
            (0, 0, 0, 255),
            (255, 255, 255, 255),
            (255, 0, 0, 255),
            (0, 255, 0, 255),
        ]
    if len(texture_pixels) != texture_size[0] * texture_size[1]:
        raise ValueError("texture_pixels length must match texture_size")
    if png_color_type == 0:
        pixels = [(p[0],) for p in texture_pixels]
    elif png_color_type == 2:
        pixels = [p[:3] for p in texture_pixels]
    elif png_color_type == 4:
        pixels = [(p[0], p[3]) for p in texture_pixels]
    else:
        pixels = list(texture_pixels)
    png = _png(texture_size[0], texture_size[1], png_color_type, pixels)
    chunks: list[bytes] = []
    offsets: list[int] = []
    cursor = 0
    data_chunks = [positions, uvs, indices, times, translations, morph_positions, png]
    if tangent_attribute:
        data_chunks.append(tangents)
    for data in data_chunks:
        offsets.append(cursor)
        chunks.append(data)
        cursor += len(data)
        pad = (4 - cursor % 4) % 4
        if pad:
            chunks.append(b"\x00" * pad)
            cursor += pad
    bin_blob = b"".join(chunks)
    primitive = {
        "attributes": {"POSITION": 0, "TEXCOORD_0": 1},
        "indices": 2,
        "material": 0,
        "mode": primitive_mode,  # 4 = TRIANGLES
    }
    if morph_target:
        primitive["targets"] = [{"POSITION": 5}]
    if tangent_attribute:
        primitive["attributes"]["TANGENT"] = 6

    node = {"name": "actor", "mesh": 0}
    if skin:
        node["skin"] = 0
    if matrix_transform:
        node["matrix"] = [
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ]

    materials = []
    for _ in range(material_count):
        pbr = {"baseColorTexture": {"index": 0}}
        if base_color_factor is not None:
            pbr["baseColorFactor"] = base_color_factor
        material = {"pbrMetallicRoughness": pbr}
        if alpha_mode is not None:
            material["alphaMode"] = alpha_mode
        if alpha_cutoff is not None:
            material["alphaCutoff"] = alpha_cutoff
        materials.append(material)
    if normal_texture:
        materials[0]["normalTexture"] = {"index": 0}

    animation_target = {"node": 0, "path": "translation"}
    if material_animation:
        animation_target = {
            "extensions": {
                "KHR_animation_pointer": {
                    "pointer": "/materials/0/pbrMetallicRoughness/baseColorFactor"
                }
            }
        }

    gltf = {
        "asset": {"version": "2.0"},
        "scene": 0,
        "scenes": [{"nodes": [0]}],
        "nodes": [node],
        "meshes": [{"primitives": [primitive]}],
        "materials": materials,
        "textures": [{"source": 0} for _ in range(texture_count)],
        "images": [{"bufferView": 6, "mimeType": "image/png"}],
        "buffers": [{"byteLength": len(bin_blob)}],
        "bufferViews": [
            {"buffer": 0, "byteOffset": offsets[0], "byteLength": len(positions)},
            {"buffer": 0, "byteOffset": offsets[1], "byteLength": len(uvs)},
            {"buffer": 0, "byteOffset": offsets[2], "byteLength": len(indices)},
            {"buffer": 0, "byteOffset": offsets[3], "byteLength": len(times)},
            {"buffer": 0, "byteOffset": offsets[4], "byteLength": len(translations)},
            {"buffer": 0, "byteOffset": offsets[5], "byteLength": len(morph_positions)},
            {"buffer": 0, "byteOffset": offsets[6], "byteLength": len(png)},
        ],
        # componentType 5126 = FLOAT, 5123 = UNSIGNED_SHORT.
        "accessors": [
            {
                "bufferView": 0,
                "componentType": 5126,
                "count": 4,
                "type": "VEC3",
                "min": [-0.5, -0.5, 0.0],
                "max": [0.5, 0.5, 0.0],
            },
            {"bufferView": 1, "componentType": 5126, "count": 4, "type": "VEC2"},
            {"bufferView": 2, "componentType": 5123, "count": 6, "type": "SCALAR"},
            {
                "bufferView": 3,
                "componentType": 5126,
                "count": 2,
                "type": "SCALAR",
                "min": [0.0],
                "max": [1.0],
            },
            {"bufferView": 4, "componentType": 5126, "count": 2, "type": "VEC3"},
            {"bufferView": 5, "componentType": 5126, "count": 4, "type": "VEC3"},
        ],
        "animations": [
            {
                "name": "slide",
                "samplers": [{"input": 3, "output": 4, "interpolation": "LINEAR"}],
                "channels": [{"sampler": 0, "target": animation_target}],
            }
        ],
    }
    if tangent_attribute:
        gltf["bufferViews"].append(
            {"buffer": 0, "byteOffset": offsets[7], "byteLength": len(tangents)}
        )
        gltf["accessors"].append(
            {"bufferView": 7, "componentType": 5126, "count": 4, "type": "VEC4"}
        )
    if skin:
        gltf["skins"] = [{"joints": [0]}]
    if external_buffer:
        # The BIN chunk stays in place; the parser rejects the buffer
        # entry itself once it carries a URI instead of the BIN chunk.
        gltf["buffers"] = [
            {
                "uri": "data:application/octet-stream;base64,"
                + base64.b64encode(bin_blob).decode(),
                "byteLength": len(bin_blob),
            }
        ]
    if external_image:
        gltf["images"] = [
            {"uri": "data:image/png;base64," + base64.b64encode(png).decode()}
        ]
    if material_animation:
        gltf["extensionsUsed"] = ["KHR_animation_pointer"]

    return _write_glb(path, gltf, bin_blob)


def write_alpha_texture_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(
        path,
        texture_pixels=[
            (0, 0, 0, 255),
            (255, 255, 255, 255),
            (255, 0, 0, 128),
            (0, 255, 0, 255),
        ],
    )


def write_rgb_texture_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, png_color_type=2)


def write_gray_texture_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, png_color_type=0)


def write_gray_alpha_texture_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, png_color_type=4)


def write_tinted_texture_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(
        path,
        texture_pixels=[
            (255, 255, 255, 255),
            (255, 255, 255, 255),
            (255, 255, 255, 255),
            (255, 255, 255, 255),
        ],
        base_color_factor=[1.0, 0.0, 0.0, 1.0],
    )


def write_two_texture_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, texture_count=2)


def write_two_material_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, material_count=2)


def write_normal_texture_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, normal_texture=True)


def write_matrix_transform_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, matrix_transform=True)


def write_tangent_attribute_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, tangent_attribute=True)


def write_material_animation_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, material_animation=True)


def write_line_mode_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, primitive_mode=1)  # 1 = LINES


def write_external_buffer_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, external_buffer=True)


def write_external_image_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, external_image=True)


def write_morph_target_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, morph_target=True)


def write_skin_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, skin=True)


def write_non_indexed_glb(path: Path, *, vertex_count: int = 6) -> Path:
    # Untextured, unanimated GLB whose primitive has no indices accessor;
    # the default 6 vertices form two triangles read straight from
    # POSITION.
    coords: list[float] = []
    for i in range(vertex_count):
        coords.extend([float(i % 3), float(i // 3), 0.0])
    positions = struct.pack(f"<{len(coords)}f", *coords)
    gltf = {
        "asset": {"version": "2.0"},
        "scene": 0,
        "scenes": [{"nodes": [0]}],
        "nodes": [{"name": "tris", "mesh": 0}],
        "meshes": [
            {"primitives": [{"attributes": {"POSITION": 0}, "material": 0, "mode": 4}]}
        ],
        "materials": [
            {"pbrMetallicRoughness": {"baseColorFactor": [1.0, 1.0, 1.0, 1.0]}}
        ],
        "buffers": [{"byteLength": len(positions)}],
        "bufferViews": [{"buffer": 0, "byteOffset": 0, "byteLength": len(positions)}],
        "accessors": [
            {
                "bufferView": 0,
                "componentType": 5126,
                "count": vertex_count,
                "type": "VEC3",
                "min": [0.0, 0.0, 0.0],
                "max": [2.0, float((vertex_count - 1) // 3), 0.0],
            }
        ],
    }
    return _write_glb(path, gltf, positions)


def write_authored_normals_glb(path: Path) -> Path:
    positions = struct.pack(
        "<fffffffff",
        -0.5,
        -0.5,
        0.0,
        0.5,
        -0.5,
        0.0,
        0.0,
        0.5,
        0.0,
    )
    normals = struct.pack(
        "<fffffffff",
        0.0,
        0.0,
        -1.0,
        0.0,
        0.0,
        -1.0,
        0.0,
        0.0,
        -1.0,
    )
    chunks: list[bytes] = []
    offsets: list[int] = []
    cursor = 0
    for data in (positions, normals):
        offsets.append(cursor)
        chunks.append(data)
        cursor += len(data)
        pad = (4 - cursor % 4) % 4
        if pad:
            chunks.append(b"\x00" * pad)
            cursor += pad
    bin_blob = b"".join(chunks)
    gltf = {
        "asset": {"version": "2.0"},
        "scene": 0,
        "scenes": [{"nodes": [0]}],
        "nodes": [{"name": "tri", "mesh": 0}],
        "meshes": [
            {
                "primitives": [
                    {
                        "attributes": {"POSITION": 0, "NORMAL": 1},
                        "material": 0,
                        "mode": 4,
                    }
                ]
            }
        ],
        "materials": [
            {"pbrMetallicRoughness": {"baseColorFactor": [1.0, 1.0, 1.0, 1.0]}}
        ],
        "buffers": [{"byteLength": len(bin_blob)}],
        "bufferViews": [
            {"buffer": 0, "byteOffset": offsets[0], "byteLength": len(positions)},
            {"buffer": 0, "byteOffset": offsets[1], "byteLength": len(normals)},
        ],
        "accessors": [
            {
                "bufferView": 0,
                "componentType": 5126,
                "count": 3,
                "type": "VEC3",
                "min": [-0.5, -0.5, 0.0],
                "max": [0.5, 0.5, 0.0],
            },
            {"bufferView": 1, "componentType": 5126, "count": 3, "type": "VEC3"},
        ],
    }
    return _write_glb(path, gltf, bin_blob)


def write_two_material_two_texture_glb(
    path: Path, *, textured: bool, right_material: bool = True
) -> Path:
    left_positions = struct.pack(
        "<ffffffffffff",
        -1.2,
        -0.5,
        0.0,
        -0.2,
        -0.5,
        0.0,
        -0.2,
        0.5,
        0.0,
        -1.2,
        0.5,
        0.0,
    )
    right_positions = struct.pack(
        "<ffffffffffff",
        0.2,
        -0.5,
        0.0,
        1.2,
        -0.5,
        0.0,
        1.2,
        0.5,
        0.0,
        0.2,
        0.5,
        0.0,
    )
    uvs = struct.pack("<ffffffff", 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0)
    indices = struct.pack("<HHHHHH", 0, 1, 2, 0, 2, 3)
    red_png = _png(1, 1, 6, [(255, 0, 0, 255)])
    green_png = _png(1, 1, 6, [(0, 255, 0, 255)])

    data_chunks = [left_positions, indices, right_positions, indices]
    if textured:
        data_chunks.extend([uvs, uvs, red_png, green_png])

    chunks: list[bytes] = []
    offsets: list[int] = []
    cursor = 0
    for data in data_chunks:
        offsets.append(cursor)
        chunks.append(data)
        cursor += len(data)
        pad = (4 - cursor % 4) % 4
        if pad:
            chunks.append(b"\x00" * pad)
            cursor += pad
    bin_blob = b"".join(chunks)

    left_attributes = {"POSITION": 0}
    right_attributes = {"POSITION": 2}
    accessors = [
        {
            "bufferView": 0,
            "componentType": 5126,
            "count": 4,
            "type": "VEC3",
            "min": [-1.2, -0.5, 0.0],
            "max": [-0.2, 0.5, 0.0],
        },
        {"bufferView": 1, "componentType": 5123, "count": 6, "type": "SCALAR"},
        {
            "bufferView": 2,
            "componentType": 5126,
            "count": 4,
            "type": "VEC3",
            "min": [0.2, -0.5, 0.0],
            "max": [1.2, 0.5, 0.0],
        },
        {"bufferView": 3, "componentType": 5123, "count": 6, "type": "SCALAR"},
    ]
    buffer_views = [
        {"buffer": 0, "byteOffset": offsets[0], "byteLength": len(left_positions)},
        {"buffer": 0, "byteOffset": offsets[1], "byteLength": len(indices)},
        {"buffer": 0, "byteOffset": offsets[2], "byteLength": len(right_positions)},
        {"buffer": 0, "byteOffset": offsets[3], "byteLength": len(indices)},
    ]
    materials = [{"pbrMetallicRoughness": {"baseColorFactor": [1.0, 0.0, 0.0, 1.0]}}]
    if right_material:
        materials.append(
            {"pbrMetallicRoughness": {"baseColorFactor": [0.0, 1.0, 0.0, 1.0]}}
        )
    images = []
    textures = []

    if textured:
        if not right_material:
            raise ValueError("textured materialless fixture is not supported")
        left_attributes["TEXCOORD_0"] = 4
        right_attributes["TEXCOORD_0"] = 5
        materials = [
            {"pbrMetallicRoughness": {"baseColorTexture": {"index": 0}}},
            {"pbrMetallicRoughness": {"baseColorTexture": {"index": 1}}},
        ]
        textures = [{"source": 0}, {"source": 1}]
        images = [
            {"bufferView": 6, "mimeType": "image/png"},
            {"bufferView": 7, "mimeType": "image/png"},
        ]
        buffer_views.extend(
            [
                {"buffer": 0, "byteOffset": offsets[4], "byteLength": len(uvs)},
                {"buffer": 0, "byteOffset": offsets[5], "byteLength": len(uvs)},
                {"buffer": 0, "byteOffset": offsets[6], "byteLength": len(red_png)},
                {"buffer": 0, "byteOffset": offsets[7], "byteLength": len(green_png)},
            ]
        )
        accessors.extend(
            [
                {"bufferView": 4, "componentType": 5126, "count": 4, "type": "VEC2"},
                {"bufferView": 5, "componentType": 5126, "count": 4, "type": "VEC2"},
            ]
        )

    right_primitive = {
        "attributes": right_attributes,
        "indices": 3,
        "mode": 4,
    }
    if right_material:
        right_primitive["material"] = 1

    gltf = {
        "asset": {"version": "2.0"},
        "scene": 0,
        "scenes": [{"nodes": [0]}],
        "nodes": [{"name": "panels", "mesh": 0}],
        "meshes": [
            {
                "primitives": [
                    {
                        "attributes": left_attributes,
                        "indices": 1,
                        "material": 0,
                        "mode": 4,
                    },
                    right_primitive,
                ]
            }
        ],
        "materials": materials,
        "buffers": [{"byteLength": len(bin_blob)}],
        "bufferViews": buffer_views,
        "accessors": accessors,
    }
    if textured:
        gltf["textures"] = textures
        gltf["images"] = images

    return _write_glb(path, gltf, bin_blob)


def write_materialless_primitive_glb(path: Path) -> Path:
    return write_two_material_two_texture_glb(
        path, textured=False, right_material=False
    )
