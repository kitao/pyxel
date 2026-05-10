from typing import Iterator, overload

from pyxel import Font, Image

# Vec3 class
class Vec3:
    ZERO: Vec3
    ONE: Vec3
    RIGHT: Vec3
    LEFT: Vec3
    UP: Vec3
    DOWN: Vec3
    FORWARD: Vec3
    BACK: Vec3

    x: float
    y: float
    z: float

    def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0) -> None: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...
    def __getitem__(self, key: int) -> float: ...
    def __iter__(self) -> Iterator[float]: ...
    def __len__(self) -> int: ...
    def __add__(self, other: Vec3) -> Vec3: ...
    def __sub__(self, other: Vec3) -> Vec3: ...
    def __mul__(self, scalar: float) -> Vec3: ...
    def __rmul__(self, scalar: float) -> Vec3: ...
    def __truediv__(self, scalar: float) -> Vec3: ...
    def __neg__(self) -> Vec3: ...
    def dot(self, other: Vec3) -> float: ...
    def cross(self, other: Vec3) -> Vec3: ...
    def length(self) -> float: ...
    def length_squared(self) -> float: ...
    def distance_to(self, other: Vec3) -> float: ...
    def distance_squared_to(self, other: Vec3) -> float: ...
    def angle_to(self, other: Vec3) -> float: ...
    def normalize(self) -> Vec3: ...
    def clamp_length(self, max_length: float) -> Vec3: ...
    def min(self, other: Vec3) -> Vec3: ...
    def max(self, other: Vec3) -> Vec3: ...
    def lerp(self, other: Vec3, t: float) -> Vec3: ...
    def slerp(self, other: Vec3, t: float) -> Vec3: ...
    def reflect(self, normal: Vec3) -> Vec3: ...
    def project(self, other: Vec3) -> Vec3: ...
    def to_local(self, mat: Mat4) -> Vec3: ...
    def to_world(self, mat: Mat4) -> Vec3: ...
    def to_local_dir(self, mat: Mat4) -> Vec3: ...
    def to_world_dir(self, mat: Mat4) -> Vec3: ...

# Mat4 class
class Mat4:
    IDENTITY: Mat4

    pos: Vec3
    rot: Vec3
    scale: Vec3

    def __init__(self) -> None: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...
    def __getitem__(self, key: tuple[int, int]) -> float: ...
    @overload
    def __mul__(self, other: Mat4) -> Mat4: ...
    @overload
    def __mul__(self, other: Vec3) -> Vec3: ...
    @staticmethod
    def from_translation(pos: Vec3) -> Mat4: ...
    @staticmethod
    def from_rotation(rot: Vec3) -> Mat4: ...
    @staticmethod
    def from_quat(quat: Quat) -> Mat4: ...
    @staticmethod
    def from_scale(scale: Vec3) -> Mat4: ...
    @staticmethod
    def compose(pos: Vec3, rot: Vec3, scale: Vec3) -> Mat4: ...
    @staticmethod
    def look_at(eye: Vec3, target: Vec3, up: Vec3 = Vec3.UP) -> Mat4: ...
    def translate(self, v: Vec3) -> Mat4: ...
    def rotate(self, axis: Vec3, deg: float) -> Mat4: ...
    def rotate_x(self, deg: float) -> Mat4: ...
    def rotate_y(self, deg: float) -> Mat4: ...
    def rotate_z(self, deg: float) -> Mat4: ...
    def scale_by(self, v: Vec3) -> Mat4: ...
    def inverse(self) -> Mat4: ...
    def transpose(self) -> Mat4: ...
    def determinant(self) -> float: ...
    def to_local(self, mat: Mat4) -> Mat4: ...
    def to_world(self, mat: Mat4) -> Mat4: ...
    def to_local_dir(self, mat: Mat4) -> Mat4: ...
    def to_world_dir(self, mat: Mat4) -> Mat4: ...

# Quat class
class Quat:
    IDENTITY: Quat

    x: float
    y: float
    z: float
    w: float

    def __init__(
        self,
        x: float = 0.0,
        y: float = 0.0,
        z: float = 0.0,
        w: float = 1.0,
    ) -> None: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...
    def __getitem__(self, key: int) -> float: ...
    def __iter__(self) -> Iterator[float]: ...
    def __len__(self) -> int: ...
    @overload
    def __mul__(self, other: Quat) -> Quat: ...
    @overload
    def __mul__(self, other: Vec3) -> Vec3: ...
    def __neg__(self) -> Quat: ...
    @staticmethod
    def from_axis_angle(axis: Vec3, deg: float) -> Quat: ...
    @staticmethod
    def from_euler(rot: Vec3) -> Quat: ...
    @staticmethod
    def from_two_vectors(a: Vec3, b: Vec3) -> Quat: ...
    @staticmethod
    def from_matrix(mat: Mat4) -> Quat: ...
    def conjugate(self) -> Quat: ...
    def inverse(self) -> Quat: ...
    def normalize(self) -> Quat: ...
    def length(self) -> float: ...
    def length_squared(self) -> float: ...
    def dot(self, other: Quat) -> float: ...
    def angle_to(self, other: Quat) -> float: ...
    def to_matrix(self) -> Mat4: ...
    def to_euler(self) -> Vec3: ...
    def to_axis_angle(self) -> tuple[Vec3, float]: ...
    def slerp(self, other: Quat, t: float) -> Quat: ...

# Camera class
class Camera:
    transform: Mat4
    fov: float
    near: float
    far: float
    ortho_size: float | None

    def __init__(self) -> None: ...
    def __repr__(self) -> str: ...

# Shading class — palette × level lookup table plus the scene-wide light
# direction. Each cell is either flat (primary == secondary) or a 50:50
# 2x2 checker between primary and secondary.
class Shading:
    direction: Vec3

    def __init__(self, colors: list[int]) -> None: ...
    def __repr__(self) -> str: ...
    def __getitem__(self, key: tuple[int, int]) -> tuple[int, int]: ...
    def __setitem__(self, key: tuple[int, int], value: tuple[int, int]) -> None: ...
    def build(self, colors: list[int]) -> None: ...

# Contact class — placeholder for collision-pipeline payload (deferred;
# see cube-design.md § 15).
class Contact:
    point: Vec3
    normal: Vec3

    def __init__(self) -> None: ...
    def __repr__(self) -> str: ...

# Collider class — placeholder; shape vocabulary and collision pipeline
# are deferred (cube-design.md § 15). Constructable today so user code
# can stage `node.collider = Collider()` ahead of the implementation.
class Collider:
    def __init__(self) -> None: ...
    def __repr__(self) -> str: ...

# FloatBuffer class
class FloatBuffer:
    @property
    def size(self) -> int: ...
    def __init__(self, source: int | list[float] = 0) -> None: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    @overload
    def __getitem__(self, i: int) -> float: ...
    @overload
    def __getitem__(self, s: slice) -> list[float]: ...
    @overload
    def __setitem__(self, i: int, value: float) -> None: ...
    @overload
    def __setitem__(self, s: slice, values: list[float]) -> None: ...
    @overload
    def __setitem__(self, s: slice, values: FloatBuffer) -> None: ...
    def __iter__(self) -> Iterator[float]: ...
    def __len__(self) -> int: ...
    def fill(self, value: float) -> None: ...
    def resize(self, size: int) -> None: ...

# IntBuffer class
class IntBuffer:
    @property
    def size(self) -> int: ...
    def __init__(self, source: int | list[int] = 0) -> None: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    @overload
    def __getitem__(self, i: int) -> int: ...
    @overload
    def __getitem__(self, s: slice) -> list[int]: ...
    @overload
    def __setitem__(self, i: int, value: int) -> None: ...
    @overload
    def __setitem__(self, s: slice, values: list[int]) -> None: ...
    @overload
    def __setitem__(self, s: slice, values: IntBuffer) -> None: ...
    def __iter__(self) -> Iterator[int]: ...
    def __len__(self) -> int: ...
    def fill(self, value: int) -> None: ...
    def resize(self, size: int) -> None: ...

# Mesh class
class Mesh:
    positions: FloatBuffer  # flat (x,y,z) triples; PRIM_TRIANGLES winding
    indices: (
        IntBuffer | None
    )  # flat triangle indices; None draws as a flat triangle list
    normals: (
        FloatBuffer | None
    )  # flat (nx,ny,nz) per-vertex; None auto-computes from face
    uvs: FloatBuffer | None  # flat (u,v) per-vertex; None disables texture sampling
    image: Image | None  # source texture; None falls back to the mesh draw col
    colkey: int | None  # transparent color when image is set; None disables colkey

    def __init__(
        self,
        positions: FloatBuffer | None = None,
        indices: IntBuffer | None = None,
        normals: FloatBuffer | None = None,
        uvs: FloatBuffer | None = None,
        image: Image | None = None,
        colkey: int | None = None,
    ) -> None: ...
    def __repr__(self) -> str: ...

# Node class
class Node:
    # Primitive mode constants for `prim` (OpenGL-ordered)
    PRIM_POINTS: int
    PRIM_LINES: int
    PRIM_TRIANGLES: int

    # Billboard mode constants for the per-call `billboard` argument
    BILLBOARD_OFF: int
    BILLBOARD_ON: int
    BILLBOARD_FIXED_Y: int

    name: str  # tag for find()
    transform: Mat4
    active: bool  # parent-dominant; False halts update + collision
    visible: bool  # parent-dominant; False halts drawing
    shading: Shading | None  # None inherits from the closest non-None ancestor
    collider: Collider | None  # this node only (collision pipeline deferred)

    @property
    def parent(self) -> Node | None: ...
    @property
    def children(self) -> tuple[Node, ...]: ...
    @property
    def camera(self) -> Camera: ...  # valid only inside on_draw
    def __init__(self) -> None: ...
    def __repr__(self) -> str: ...
    def world_transform(self) -> Mat4: ...
    def find(self, name: str) -> Node | None: ...  # subtree DFS by name
    def add_child(self, node: Node) -> None: ...
    def remove_child(self, node: Node) -> None: ...
    def destroy(self) -> None: ...

    # Immediate-mode draw commands (node-local coordinates).
    # Modifier keyword arguments: shaded, dither_alpha (Bayer-dither
    # pseudo-alpha; 1.0 = opaque), depth_test, depth_write, billboard.
    # Each command exposes only the modifiers that meaningfully apply
    # to it (see cube-design.md § 12.5 for the rules).
    def pset(
        self,
        pos: Vec3,
        col: int,
        *,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
    ) -> None: ...
    def line(
        self,
        p1: Vec3,
        p2: Vec3,
        col: int,
        *,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
        billboard: int = 0,
    ) -> None: ...
    def tri(
        self,
        p1: Vec3,
        p2: Vec3,
        p3: Vec3,
        col: int,
        *,
        shaded: bool = True,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
        billboard: int = 0,
    ) -> None: ...
    def trib(
        self,
        p1: Vec3,
        p2: Vec3,
        p3: Vec3,
        col: int,
        *,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
        billboard: int = 0,
    ) -> None: ...
    def circ(
        self,
        pos: Vec3,
        r: float,
        col: int,
        *,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
    ) -> None: ...
    def circb(
        self,
        pos: Vec3,
        r: float,
        col: int,
        *,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
    ) -> None: ...
    def sphere(
        self,
        pos: Vec3,
        r: float,
        col: int,
        *,
        shaded: bool = True,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
    ) -> None: ...
    def sphereb(
        self,
        pos: Vec3,
        r: float,
        col: int,
        *,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
    ) -> None: ...
    def rect(
        self,
        mat: Mat4,
        w: float,
        h: float,
        col: int,
        *,
        shaded: bool = True,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
        billboard: int = 0,
    ) -> None: ...
    def rectb(
        self,
        mat: Mat4,
        w: float,
        h: float,
        col: int,
        *,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
        billboard: int = 0,
    ) -> None: ...
    def elli(
        self,
        mat: Mat4,
        w: float,
        h: float,
        col: int,
        *,
        shaded: bool = True,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
        billboard: int = 0,
    ) -> None: ...
    def ellib(
        self,
        mat: Mat4,
        w: float,
        h: float,
        col: int,
        *,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
        billboard: int = 0,
    ) -> None: ...
    def box(
        self,
        mat: Mat4,
        size: Vec3,
        col: int,
        *,
        shaded: bool = True,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
        billboard: int = 0,
    ) -> None: ...
    def boxb(
        self,
        mat: Mat4,
        size: Vec3,
        col: int,
        *,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
        billboard: int = 0,
    ) -> None: ...
    def text(
        self,
        pos: Vec3,
        s: str,
        col: int,
        *,
        font: Font | None = None,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
    ) -> None: ...
    def sprite(
        self,
        pos: Vec3,
        img: Image,
        uvs: tuple[
            tuple[float, float],
            tuple[float, float],
            tuple[float, float],
            tuple[float, float],
        ],
        w: float,
        h: float,
        *,
        colkey: int | None = None,
        angle: float = 0.0,
        shaded: bool = False,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
    ) -> None: ...
    def plane(
        self,
        mat: Mat4,
        img: Image,
        uvs: tuple[
            tuple[float, float],
            tuple[float, float],
            tuple[float, float],
            tuple[float, float],
        ],
        w: float,
        h: float,
        *,
        colkey: int | None = None,
        shaded: bool = True,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
        billboard: int = 0,
    ) -> None: ...
    def mesh(
        self,
        mat: Mat4,
        mesh_asset: Mesh,
        *,
        col: int = 7,
        shaded: bool = True,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
        billboard: int = 0,
    ) -> None: ...
    def prim(
        self,
        mat: Mat4,
        mode: int,
        positions: FloatBuffer,
        *,
        indices: IntBuffer | None = None,
        normals: FloatBuffer | None = None,
        uvs: FloatBuffer | None = None,
        first: int = 0,
        count: int | None = None,
        col: int | Image = 7,
        colkey: int | None = None,
        shaded: bool = True,
        dither_alpha: float = 1.0,
        depth_test: bool = True,
        depth_write: bool = True,
        billboard: int = 0,
    ) -> None: ...

    # Lifecycle hooks
    def on_update(self) -> None: ...
    def on_draw(self) -> None: ...
    def on_collide(self, other: Node, contact: Contact | None = None) -> None: ...
    def on_destroy(self) -> None: ...

# Scene class
class Scene(Node):
    clear_color: int | None  # None skips clear; int fills screen + depth buffer

    def __init__(self) -> None: ...
    def __repr__(self) -> str: ...
    def update(self) -> None: ...
    def draw(
        self,
        x: int,
        y: int,
        w: int,
        h: int,
        camera: Camera,
        screen: Image | None = None,
    ) -> None: ...
