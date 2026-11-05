import struct

import pytest
from pyxel.cube import Mesh, Node, Vec3

from ._glb_fixtures import _write_glb

TARGETS = [
    "POSITION",
    "NORMAL",
    "TEXCOORD_0",
    "indices",
    "time",
    "translation",
    "rotation",
    "scale",
]


def layout_document(target="POSITION"):
    positions = struct.pack("<9f", 0, 0, 0, 1, 0, 0, 0, 1, 0)
    document = {
        "asset": {"version": "2.0"},
        "scene": 0,
        "scenes": [{"nodes": [0]}],
        "nodes": [{"mesh": 0}],
        "meshes": [{"primitives": [{"attributes": {"POSITION": 0}}]}],
        "buffers": [{"byteLength": len(positions)}],
        "bufferViews": [{"buffer": 0, "byteLength": len(positions)}],
        "accessors": [
            {
                "bufferView": 0,
                "componentType": 5126,
                "count": 3,
                "type": "VEC3",
                "min": [0, 0, 0],
                "max": [1, 1, 0],
            }
        ],
    }
    data = bytearray(positions)

    def append(values, component_type, dimensions, count):
        index = len(document["accessors"])
        document["bufferViews"].append(
            {"buffer": 0, "byteOffset": len(data), "byteLength": len(values)}
        )
        document["accessors"].append(
            {
                "bufferView": len(document["bufferViews"]) - 1,
                "componentType": component_type,
                "count": count,
                "type": dimensions,
            }
        )
        data.extend(values)
        data.extend(bytes(-len(data) % 4))
        document["buffers"][0]["byteLength"] = len(data)
        return index

    primitive = document["meshes"][0]["primitives"][0]
    if target == "POSITION":
        index = 0
    elif target == "NORMAL":
        index = append(struct.pack("<9f", *([0, 0, 1] * 3)), 5126, "VEC3", 3)
        primitive["attributes"][target] = index
    elif target == "TEXCOORD_0":
        index = append(struct.pack("<6f", 0, 0, 1, 0, 0, 1), 5126, "VEC2", 3)
        primitive["attributes"][target] = index
    elif target == "indices":
        index = append(struct.pack("<3I", 0, 1, 2), 5125, "SCALAR", 3)
        primitive[target] = index
    else:
        input_index = append(struct.pack("<2f", 0, 1), 5126, "SCALAR", 2)
        document["accessors"][input_index].update(min=[0], max=[1])
        rotation = target == "rotation"
        values = [0, 0, 0, 1, 0, 0, 1, 0] if rotation else [0, 0, 0, 1, 2, 3]
        output_index = append(
            struct.pack(f"<{len(values)}f", *values),
            5126,
            "VEC4" if rotation else "VEC3",
            2,
        )
        document["animations"] = [
            {
                "samplers": [{"input": input_index, "output": output_index}],
                "channels": [
                    {
                        "sampler": 0,
                        "target": {
                            "node": 0,
                            "path": "translation" if target == "time" else target,
                        },
                    }
                ],
            }
        ]
        index = input_index if target == "time" else output_index
    return document, bytes(data), index


@pytest.mark.parametrize("target", TARGETS)
def test_from_glb_accepts_each_consumed_accessor_family(tmp_path, target):
    document, data, _ = layout_document(target)
    mesh = Mesh.from_glb(str(_write_glb(tmp_path / "valid.glb", document, data)))
    assert list(mesh.primitives[1].positions) == [0, 0, 0, 1, 0, 0, 0, 1, 0]
    if "animations" in document:
        assert mesh.motions[0].length == 30


@pytest.mark.parametrize("target", TARGETS)
@pytest.mark.parametrize(
    "damage", ["stride", "offset", "count", "empty", "dimensions", "component"]
)
def test_from_glb_rejects_invalid_accessor_layout(tmp_path, target, damage):
    document, data, index = layout_document(target)
    accessor = document["accessors"][index]
    view = document["bufferViews"][accessor["bufferView"]]
    if damage == "stride":
        view["byteStride"] = 4 if accessor["type"] != "SCALAR" else 1
    elif damage == "offset":
        accessor["byteOffset"] = 2**62
    elif damage == "count":
        accessor["count"] = 2**62
    elif damage == "empty":
        accessor["count"] = 0
    elif damage == "dimensions":
        accessor["type"] = "MAT4"
    else:
        accessor["componentType"] = 5125 if target != "indices" else 5126
    filename = _write_glb(tmp_path / "invalid.glb", document, data)

    with pytest.raises(ValueError):
        Mesh.from_glb(str(filename))


def test_from_glb_accepts_interleaved_vertex_layout(tmp_path):
    document, data, _ = layout_document()
    interleaved = b"".join(data[i : i + 12] + b"xxxx" for i in range(0, 36, 12))
    document["buffers"][0]["byteLength"] = len(interleaved)
    document["bufferViews"][0].update(byteLength=len(interleaved), byteStride=16)
    mesh = Mesh.from_glb(
        str(_write_glb(tmp_path / "interleaved.glb", document, interleaved))
    )
    assert list(mesh.primitives[1].positions) == [0, 0, 0, 1, 0, 0, 0, 1, 0]


@pytest.mark.parametrize(
    ("target", "component", "code", "maximum"),
    [
        ("TEXCOORD_0", 5121, "B", 255),
        ("TEXCOORD_0", 5123, "H", 65535),
        ("indices", 5121, "B", 1),
        ("indices", 5123, "H", 1),
        ("indices", 5125, "I", 1),
        ("rotation", 5120, "b", 127),
        ("rotation", 5121, "B", 255),
        ("rotation", 5122, "h", 32767),
        ("rotation", 5123, "H", 65535),
    ],
)
def test_from_glb_keeps_supported_integer_accessors(
    tmp_path, target, component, code, maximum
):
    document, data, index = layout_document(target)
    accessor = document["accessors"][index]
    accessor.update(componentType=component, normalized=target != "indices")
    view = document["bufferViews"][accessor["bufferView"]]
    values = {
        "TEXCOORD_0": [0, 0, maximum, 0, 0, maximum],
        "indices": [0, 1, 2],
        "rotation": [0, 0, 0, maximum, 0, 0, maximum, 0],
    }[target]
    packed = struct.pack(f"<{len(values)}{code}", *values)
    data = data[: view["byteOffset"]] + packed
    view["byteLength"] = len(packed)
    document["buffers"][0]["byteLength"] = len(data)
    mesh = Mesh.from_glb(str(_write_glb(tmp_path / "integer.glb", document, data)))
    if target == "TEXCOORD_0":
        assert list(mesh.primitives[1].uvs) == [0, 0, 1, 0, 0, 1]
    elif target == "indices":
        assert list(mesh.primitives[1].indices) == [0, 1, 2]
    else:
        node = Node.from_mesh(mesh)
        node.apply_motion(mesh.motions[0], 30, loop=False)
        assert node.children[0].transform * Vec3.RIGHT == Vec3.LEFT


@pytest.mark.parametrize("base_view", [False, True])
@pytest.mark.parametrize(
    "damage",
    [
        None,
        "stride",
        "offset",
        "value_offset",
        "count",
        "empty",
        "order",
        "index",
        "component",
        "accessor_count",
    ],
)
def test_from_glb_sparse_layout(tmp_path, base_view, damage):
    document, data, _ = layout_document()
    document["bufferViews"].extend(
        [
            {"buffer": 0, "byteOffset": len(data), "byteLength": 8},
            {"buffer": 0, "byteOffset": len(data) + 8, "byteLength": 24},
        ]
    )
    indices = [1, 2]
    if damage == "order":
        indices = [1, 1]
    elif damage == "index":
        indices = [1, 3]
    data += struct.pack("<2I6f", *indices, 2, 0, 0, 0, 2, 0)
    document["buffers"][0]["byteLength"] = len(data)
    accessor = document["accessors"][0]
    if not base_view:
        accessor.pop("bufferView")
    sparse = {
        "count": 2,
        "indices": {"bufferView": 1, "componentType": 5125},
        "values": {"bufferView": 2},
    }
    accessor["sparse"] = sparse
    if damage == "stride":
        document["bufferViews"][2]["byteStride"] = 4
    elif damage == "offset":
        sparse["indices"]["byteOffset"] = 2**62
    elif damage == "value_offset":
        sparse["values"]["byteOffset"] = 2**62
    elif damage == "count":
        sparse["count"] = 4
    elif damage == "empty":
        sparse["count"] = 0
    elif damage == "component":
        sparse["indices"]["componentType"] = 5126
    elif damage == "accessor_count":
        accessor["count"] = 2**62
    filename = _write_glb(tmp_path / "sparse.glb", document, data)

    if damage:
        with pytest.raises(ValueError):
            Mesh.from_glb(str(filename))
    else:
        mesh = Mesh.from_glb(str(filename))
        assert list(mesh.primitives[1].positions) == [0, 0, 0, 2, 0, 0, 0, 2, 0]


@pytest.mark.parametrize("offset", [100, 2**62, 2**64 - 1])
def test_from_glb_rejects_image_view_outside_buffer(tmp_path, offset):
    document, data, _ = layout_document()
    document["bufferViews"].append(
        {"buffer": 0, "byteOffset": offset, "byteLength": 36}
    )
    document["images"] = [{"bufferView": 1, "mimeType": "image/png"}]
    filename = _write_glb(tmp_path / "image.glb", document, data)
    with pytest.raises(ValueError, match="bufferView 1 is out of bounds"):
        Mesh.from_glb(str(filename))


def test_from_glb_rejects_external_buffer_before_uri_decoding(tmp_path):
    document, data, _ = layout_document()
    document["buffers"][0]["uri"] = "%FF"
    filename = _write_glb(tmp_path / "external.glb", document, data)
    with pytest.raises(ValueError, match="GLB external buffers are not supported"):
        Mesh.from_glb(str(filename))
