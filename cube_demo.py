import math
import os

import pyxel
from pyxel.cube import (
    Camera,
    Geometry,
    Mat4,
    Mesh,
    Node,
    Quat,
    Scene,
    Shading,
    Vec3,
)

# Unit icosahedron table (12 vertices on |v|=1, 20 outward triangles).
# Mirrors `ICOSA_POSITIONS` / `ICOSA_TRI_INDICES` in pyxel-core/src/cube/draw.rs.
_ICOSA_O = 0.5257311
_ICOSA_T = 0.8506508
_UNIT_ICOSA_VERTICES = [
    -_ICOSA_O,
    _ICOSA_T,
    0.0,
    _ICOSA_O,
    _ICOSA_T,
    0.0,
    -_ICOSA_O,
    -_ICOSA_T,
    0.0,
    _ICOSA_O,
    -_ICOSA_T,
    0.0,
    0.0,
    -_ICOSA_O,
    _ICOSA_T,
    0.0,
    _ICOSA_O,
    _ICOSA_T,
    0.0,
    -_ICOSA_O,
    -_ICOSA_T,
    0.0,
    _ICOSA_O,
    -_ICOSA_T,
    _ICOSA_T,
    0.0,
    -_ICOSA_O,
    _ICOSA_T,
    0.0,
    _ICOSA_O,
    -_ICOSA_T,
    0.0,
    -_ICOSA_O,
    -_ICOSA_T,
    0.0,
    _ICOSA_O,
]
_ICOSA_TRI_INDICES = [
    0,
    11,
    5,
    0,
    5,
    1,
    0,
    1,
    7,
    0,
    7,
    10,
    0,
    10,
    11,
    1,
    5,
    9,
    5,
    11,
    4,
    11,
    10,
    2,
    10,
    7,
    6,
    7,
    1,
    8,
    3,
    9,
    4,
    3,
    4,
    2,
    3,
    2,
    6,
    3,
    6,
    8,
    3,
    8,
    9,
    4,
    9,
    5,
    2,
    4,
    11,
    6,
    2,
    10,
    8,
    6,
    7,
    9,
    8,
    1,
]
# Box vertices: 8 corners of a unit cube centered at origin (the Mesh
# variant scales by `size`). Indices match the cube/draw.rs winding.
_UNIT_BOX_VERTICES = [
    -0.5,
    -0.5,
    -0.5,
    0.5,
    -0.5,
    -0.5,
    0.5,
    0.5,
    -0.5,
    -0.5,
    0.5,
    -0.5,
    -0.5,
    -0.5,
    0.5,
    0.5,
    -0.5,
    0.5,
    0.5,
    0.5,
    0.5,
    -0.5,
    0.5,
    0.5,
]
_BOX_TRI_INDICES = [
    0,
    2,
    1,
    0,
    3,
    2,  # -Z face
    4,
    5,
    6,
    4,
    6,
    7,  # +Z face
    0,
    1,
    5,
    0,
    5,
    4,  # -Y face
    3,
    6,
    2,
    3,
    7,
    6,  # +Y face
    0,
    4,
    7,
    0,
    7,
    3,  # -X face
    1,
    2,
    6,
    1,
    6,
    5,  # +X face
]


# Top: 12 small 2D-style primitives in a 4x3 grid.
LAYOUT_2D = [
    ("pset", -9.0, 7.5),
    ("line", -3.0, 7.5),
    ("tri", 3.0, 7.5),
    ("trib", 9.0, 7.5),
    ("circ", -9.0, 4.0),
    ("circb", -3.0, 4.0),
    ("rect", 3.0, 4.0),
    ("rectb", 9.0, 4.0),
    ("elli", -9.0, 0.5),
    ("ellib", -3.0, 0.5),
    ("sprite", 3.0, 0.5),
    ("plane", 9.0, 0.5),
]
# Bottom: 3 large 3D mesh primitives — colored box, textured box, sphere.
LAYOUT_3D = [
    ("mesh-box", -7.0, -5.5),
    ("mesh-tex-box", 0.0, -5.5),
    ("mesh-sphere", 7.0, -5.5),
]

_CAT_UVS = (
    (0.0, 0.0),
    (16.0 / 256.0, 0.0),
    (0.0, 16.0 / 256.0),
    (16.0 / 256.0, 16.0 / 256.0),
)


def _load_texture():
    asset = os.path.join(
        os.path.dirname(pyxel.__file__),
        "examples",
        "assets",
        "cat_16x16.png",
    )
    pyxel.images[0].load(0, 0, asset)


def _make_box_mesh(size, color):
    # Scale the unit cube; the Mesh wraps a single Geometry carrying the
    # flat color through col_img.
    pos = [v * size for v in _UNIT_BOX_VERTICES]
    geom = Geometry(positions=pos, indices=_BOX_TRI_INDICES)
    return Mesh(
        geometries=[geom],
        transforms=[Mat4.IDENTITY],
        parents=[-1],
        col_img=color,
    )


def _make_sphere_mesh(radius, color):
    # Level-1 subdivided icosahedron (42 vertices / 80 triangles). Mirrors
    # `unit_icosa_lv1_*` in pyxel-core/src/cube/draw.rs so the mesh-asset
    # path matches the immediate-mode `node.sphere()` look.
    edges = [
        (0, 1),
        (0, 5),
        (0, 7),
        (0, 10),
        (0, 11),
        (1, 5),
        (1, 7),
        (1, 8),
        (1, 9),
        (2, 3),
        (2, 4),
        (2, 6),
        (2, 10),
        (2, 11),
        (3, 4),
        (3, 6),
        (3, 8),
        (3, 9),
        (4, 5),
        (4, 9),
        (4, 11),
        (5, 9),
        (5, 11),
        (6, 7),
        (6, 8),
        (6, 10),
        (7, 8),
        (7, 10),
        (8, 9),
        (10, 11),
    ]
    edge_index = {pair: 12 + i for i, pair in enumerate(edges)}
    pos: list[float] = list(_UNIT_ICOSA_VERTICES)
    for a, b in edges:
        mx = (pos[a * 3] + pos[b * 3]) * 0.5
        my = (pos[a * 3 + 1] + pos[b * 3 + 1]) * 0.5
        mz = (pos[a * 3 + 2] + pos[b * 3 + 2]) * 0.5
        inv_len = 1.0 / math.sqrt(mx * mx + my * my + mz * mz)
        pos.extend([mx * inv_len, my * inv_len, mz * inv_len])

    def midpoint(a: int, b: int) -> int:
        return edge_index[(a, b) if a < b else (b, a)]

    tri_indices: list[int] = []
    for i in range(0, len(_ICOSA_TRI_INDICES), 3):
        a = _ICOSA_TRI_INDICES[i]
        b = _ICOSA_TRI_INDICES[i + 1]
        c = _ICOSA_TRI_INDICES[i + 2]
        m_ab = midpoint(a, b)
        m_bc = midpoint(b, c)
        m_ca = midpoint(c, a)
        tri_indices.extend(
            [a, m_ab, m_ca, b, m_bc, m_ab, c, m_ca, m_bc, m_ab, m_bc, m_ca]
        )

    scaled = [v * radius for v in pos]
    geom = Geometry(positions=scaled, indices=tri_indices)
    return Mesh(
        geometries=[geom],
        transforms=[Mat4.IDENTITY],
        parents=[-1],
        col_img=color,
    )


def _make_textured_box(size):
    # Per-face quads (4 verts each, no shared verts) so per-face UVs map
    # onto a single 16x16 texel region of the source image.
    h = size * 0.5
    face_defs = [
        [(-h, -h, -h), (h, -h, -h), (-h, h, -h), (h, h, -h)],
        [(h, -h, h), (-h, -h, h), (h, h, h), (-h, h, h)],
        [(-h, -h, h), (-h, -h, -h), (-h, h, h), (-h, h, -h)],
        [(h, -h, -h), (h, -h, h), (h, h, -h), (h, h, h)],
        [(-h, -h, h), (h, -h, h), (-h, -h, -h), (h, -h, -h)],
        [(-h, h, -h), (h, h, -h), (-h, h, h), (h, h, h)],
    ]
    u_max = 16.0 / 256.0
    pos_list = []
    uv_list = []
    idx_list = []
    for face_index, face in enumerate(face_defs):
        base = face_index * 4
        for v in face:
            pos_list.extend(v)
        uv_list.extend([0.0, 0.0, u_max, 0.0, 0.0, u_max, u_max, u_max])
        # Wind each face CCW so the surface normal points outward (matches
        # the colored-box mesh and gives correct Lambert shading).
        idx_list.extend([base, base + 2, base + 1, base + 1, base + 2, base + 3])
    geom = Geometry(positions=pos_list, indices=idx_list, uvs=uv_list)
    return Mesh(
        geometries=[geom],
        transforms=[Mat4.IDENTITY],
        parents=[-1],
        col_img=pyxel.images[0],
    )


class Showcase(Node):
    def __init__(self):
        super().__init__()
        self.box_mesh = _make_box_mesh(3.6, 8)
        self.tex_box = _make_textured_box(4.0)
        self.sphere_mesh = _make_sphere_mesh(2.2, 11)
        self.frame = 0

    def spin_deg(self) -> float:
        return self.frame * 1.5

    def on_draw(self):
        spin = self.spin_deg()
        for name, x, y in LAYOUT_2D:
            wobble = 12.0 * math.sin(math.radians(spin) + (x + y) * 0.3)
            mat = Mat4.compose(
                Vec3(x, y, 0),
                Quat.from_euler(Vec3(0, 0, wobble)),
                Vec3.ONE,
            )
            self._draw_2d(name, x, y, mat, spin)
        for name, x, y in LAYOUT_3D:
            spin_mat = Mat4.compose(
                Vec3(x, y, 0),
                Quat.from_euler(
                    Vec3(spin * 1.5 + 30.0, spin * 1.2 + 45.0, spin * 0.8)
                ),
                Vec3.ONE,
            )
            if name == "mesh-box":
                self.mesh(spin_mat, self.box_mesh)
            elif name == "mesh-tex-box":
                self.mesh(spin_mat, self.tex_box)
            elif name == "mesh-sphere":
                self.mesh(spin_mat, self.sphere_mesh)

    def _draw_2d(self, name, x, y, mat, spin):
        if name == "pset":
            self.pset(Vec3(x, y, 0), 7)
        elif name == "line":
            self.line(
                Vec3(x - 1.8, y - 1.2, 0),
                Vec3(x + 1.8, y + 1.2, 0),
                10,
            )
        elif name == "tri":
            self.tri(
                Vec3(x - 1.8, y - 1.2, 0),
                Vec3(x + 1.8, y - 1.2, 0),
                Vec3(x, y + 1.5, 0),
                11,
            )
        elif name == "trib":
            self.trib(
                Vec3(x - 1.8, y - 1.2, 0),
                Vec3(x + 1.8, y - 1.2, 0),
                Vec3(x, y + 1.5, 0),
                12,
            )
        elif name == "circ":
            self.circ(Vec3(x, y, 0), 1.4, 8)
        elif name == "circb":
            self.circb(Vec3(x, y, 0), 1.4, 13)
        elif name == "rect":
            self.rect(mat, 3.0, 2.0, 14)
        elif name == "rectb":
            self.rectb(mat, 3.0, 2.0, 15)
        elif name == "elli":
            self.elli(mat, 3.0, 2.0, 9)
        elif name == "ellib":
            self.ellib(mat, 3.0, 2.0, 6)
        elif name == "sprite":
            self.sprite(
                Vec3(x, y, 0),
                pyxel.images[0],
                _CAT_UVS,
                3.0,
                3.0,
                colkey=0,
                angle=spin,
            )
        elif name == "plane":
            self.plane(mat, pyxel.images[0], _CAT_UVS, 3.0, 3.0, colkey=0)


def _palette() -> list[int]:
    return [pyxel.colors[i] for i in range(16)]


class App:
    def __init__(self):
        pyxel.init(256, 192, title="Pyxel Cube Showcase")
        _load_texture()
        self.scene = Scene()
        self.scene.clear_color = 0
        self.shading = Shading(_palette())
        # Light travels from upper-left-front to lower-right-back: +X
        # (right), -Y (down), -Z (away from viewer).
        self.shading.direction = Vec3(0.4, -0.8, -0.4)
        self.scene.shading = self.shading
        self.actor = Showcase()
        self.scene.add_child(self.actor)
        self.camera = Camera()
        self.camera.fov = 60.0
        self.camera.transform = Mat4.look_at(Vec3(0, 0, 22), Vec3.ZERO, Vec3.UP)
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q) or pyxel.btnp(pyxel.KEY_ESCAPE):
            pyxel.quit()
        self.actor.frame += 1
        self.actor.transform = Mat4.from_euler(
            Vec3(0, self.actor.spin_deg() * 0.5, 0)
        )
        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, 256, 192, self.camera)


App()
