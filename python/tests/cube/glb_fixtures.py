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


def write_single_texture_motion_glb(
    path: Path,
    *,
    texture_pixels: list[tuple[int, int, int, int]] | None = None,
    texture_count: int = 1,
    material_count: int = 1,
    png_color_type: int = 6,
    primitive_mode: int = 4,
    external_buffer: bool = False,
    external_image: bool = False,
    morph_target: bool = False,
    skin: bool = False,
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
    if png_color_type == 0:
        pixels = [(p[0],) for p in texture_pixels]
    elif png_color_type == 2:
        pixels = [p[:3] for p in texture_pixels]
    elif png_color_type == 4:
        pixels = [(p[0], p[3]) for p in texture_pixels]
    else:
        pixels = list(texture_pixels)
    png = _png(2, 2, png_color_type, pixels)
    chunks: list[bytes] = []
    offsets: list[int] = []
    cursor = 0
    for data in (positions, uvs, indices, times, translations, morph_positions, png):
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

    node = {"name": "actor", "mesh": 0}
    if skin:
        node["skin"] = 0

    gltf = {
        "asset": {"version": "2.0"},
        "scene": 0,
        "scenes": [{"nodes": [0]}],
        "nodes": [node],
        "meshes": [{"primitives": [primitive]}],
        "materials": [
            {"pbrMetallicRoughness": {"baseColorTexture": {"index": 0}}}
            for _ in range(material_count)
        ],
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
                "channels": [
                    {"sampler": 0, "target": {"node": 0, "path": "translation"}}
                ],
            }
        ],
    }
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


def write_two_texture_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, texture_count=2)


def write_two_material_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, material_count=2)


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
        "meshes": [{"primitives": [{"attributes": {"POSITION": 0}, "mode": 4}]}],
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
