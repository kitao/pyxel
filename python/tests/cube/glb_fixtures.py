from __future__ import annotations

import json
import struct
import zlib
from pathlib import Path


def _pad4(data: bytes, pad: bytes = b" ") -> bytes:
    return data + pad * ((4 - len(data) % 4) % 4)


def _png_rgba(
    width: int,
    height: int,
    pixels: list[tuple[int, int, int, int]],
) -> bytes:
    raw = bytearray()
    for y in range(height):
        raw.append(0)
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
            struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0),
        )
        + chunk(b"IDAT", zlib.compress(bytes(raw)))
        + chunk(b"IEND", b"")
    )


def _write_glb(path: Path, gltf: dict, bin_blob: bytes) -> Path:
    json_chunk = _pad4(json.dumps(gltf, separators=(",", ":")).encode())
    bin_chunk = _pad4(bin_blob, b"\x00")
    total_len = 12 + 8 + len(json_chunk) + 8 + len(bin_chunk)
    path.write_bytes(
        b"glTF"
        + struct.pack("<II", 2, total_len)
        + struct.pack("<I4s", len(json_chunk), b"JSON")
        + json_chunk
        + struct.pack("<I4s", len(bin_chunk), b"BIN\x00")
        + bin_chunk
    )
    return path


def write_single_texture_motion_glb(
    path: Path,
    *,
    texture_pixels: list[tuple[int, int, int, int]] | None = None,
    texture_count: int = 1,
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
    png = _png_rgba(2, 2, texture_pixels)
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
        "mode": 4,
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
        "materials": [{"pbrMetallicRoughness": {"baseColorTexture": {"index": 0}}}],
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


def write_two_texture_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, texture_count=2)


def write_morph_target_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, morph_target=True)


def write_skin_glb(path: Path) -> Path:
    return write_single_texture_motion_glb(path, skin=True)
