# Pyxel Cube API Design

3D extension module for Pyxel: `pyxel.cube`.

This document is the narrative reference for the Pyxel cube API — what was
decided, why, what was ruled out, and what is deferred. Detailed type
signatures live in `python/pyxel/cube/__init__.pyi`.

---

## 1. Overall Policy

- **Import style**: `from pyxel.cube import ...` and bring in only what's
  used.
- **Separate worldview**: Pyxel cube is treated as an independent 3D
  subsystem. It aligns with mainstream 3D math libraries (pygame, pyrr,
  PyGLM, three.js, Godot, Unity) rather than copying Pyxel 2D conventions
  whenever the two diverge.
- **Software 3D rendering**: cube is a software 3D renderer designed for
  retro pixel-art games. GPU integration features (flat-array layout,
  shader programs) are intentionally out of scope.
- **Performance target**: 60 fps on Raspberry Pi 4 / 5. Pi Zero 2 is
  best-effort.
- **No global state**: the cube module must not hold global mutable
  variables.
- **Implementation locality**: core / bindings / python sources live under
  `cube/` subfolders. Modifications to existing (non-cube) Pyxel code are
  minimized.

---

## 2. Public Classes (12)

| Class | Role |
|---|---|
| `Vec3` | Immutable 3D vector |
| `Mat4` | Immutable 4×4 matrix (transforms; projection lives in `Camera`) |
| `Quat` | Immutable quaternion rotation |
| `Camera` | View information (transform, fov, near, far, optional ortho size) |
| `ShadeRamp` | Shading LUT (palette × 8 brightness levels); absorbs per-color palette substitution |
| `Light` | Flat-shading parameters (ambient, direction, intensity) |
| `Contact` | Collision-pipeline payload placeholder (point / normal); pipeline deferred (§ 15) |
| `Collider` | Collision-shape placeholder; pipeline deferred (§ 15) |
| `Geometry` | Static vertex-data asset (positions / normals / uvs / indices / prim mode / cull mode); shareable across Node draws and Mesh parts |
| `Mesh` | Hierarchical 3D model asset (parallel arrays of geometries / transforms / parents) with shared col_img and colkey |
| `Node` | Hierarchy instance with transform, immediate-mode draw commands, and lifecycle hooks; references `Geometry` / `Mesh` during draw |
| `Scene` | `Node`-derived root that drives the update / draw cycle and owns the clear color |

---

## 3. Coordinate System and Conventions

- **+X right, +Y up, +Z toward viewer**, **right-handed**, forward = `-Z`.
- Aligned with Godot / pyglet / three.js / OpenGL / glTF.
- **Origin** for `scene.draw(x, y, w, h, camera)` maps the world origin to
  the center of the destination rectangle.
- **Angles in degrees** throughout the cube API.
- **Euler order**: `XYZ` extrinsic (apply X rotation first about the world
  X axis, then Y about world Y, then Z about world Z; matches three.js /
  Blender / Maya defaults).
- **Pyxel 2D screen is Y-down; Pyxel cube is Y-up.** The 2D / 3D mismatch
  follows the same pattern as Godot's "2D Y-down + 3D Y-up" and is
  intentional.

---

## 4. Vec3

Immutable 3D vector. Arithmetic and transform methods return new instances;
attributes are read-only.

### 4.1 Constants

| Name | Value |
|---|---|
| `Vec3.ZERO` | `(0, 0, 0)` |
| `Vec3.ONE` | `(1, 1, 1)` |
| `Vec3.RIGHT` | `(1, 0, 0)` |
| `Vec3.LEFT` | `(-1, 0, 0)` |
| `Vec3.UP` | `(0, 1, 0)` |
| `Vec3.DOWN` | `(0, -1, 0)` |
| `Vec3.FORWARD` | `(0, 0, -1)` |
| `Vec3.BACK` | `(0, 0, 1)` |

Constants are **shared immutable singletons** (same instance returned every
access). Vec3 is immutable, so accidental mutation is impossible by
construction.

### 4.2 Construction and Sequence Protocol

```python
v = Vec3(x, y, z)        # constructor (each arg defaults to 0.0)
v.x, v.y, v.z            # read-only attributes
v[0], v[1], v[2]         # __getitem__(key: int) -> float
list(v), tuple(v)        # __iter__ over (x, y, z)
len(v)                   # 3
v == other, v != other   # value-wise comparison
hash(v)                  # value-based hash (usable as dict key / set element)
repr(v)                  # debug representation
```

`__getitem__` raises `IndexError` for keys outside `0..2`.

### 4.3 Operators

```python
v + other                # Vec3
v - other                # Vec3
v * scalar               # Vec3 (scalar multiply)
scalar * v               # Vec3 (right-multiply by scalar)
v / scalar               # Vec3
-v                       # Vec3
```

No component-wise `Vec3 * Vec3`. Scaling against another vector goes
through `Mat4.from_scale(other) * v` or explicit per-axis math.

### 4.4 Math

```python
v.dot(other)             # float
v.cross(other)           # Vec3
v.length()               # float
v.length_squared()       # float
v.distance_to(other)     # float
v.distance_squared_to(other)  # float
v.angle_to(other)        # float, degrees
v.normalize()            # Vec3 (zero-length input returns Vec3.ZERO)
v.clamp_length(max_length)    # Vec3 (truncated to max_length if longer)
v.min(other)             # Vec3 (component-wise min)
v.max(other)             # Vec3 (component-wise max)
v.lerp(other, t)         # Vec3 (t outside [0, 1] extrapolates)
v.slerp(other, t)        # Vec3 (spherical, expects unit vectors)
v.reflect(normal)        # Vec3 (reflection across plane normal)
v.project(other)         # Vec3 (projection of self onto other)
```

- `angle_to` returns degrees, consistent with the rest of cube.
- `normalize` of a zero-length vector returns `Vec3.ZERO` (no exception).
- `lerp` does not clamp `t`; values outside `[0, 1]` extrapolate. Callers
  clamp explicitly if needed.
- `slerp` expects unit vectors; behavior on non-unit input is undefined.

### 4.5 Coordinate System Conversions

```python
v.to_local(mat)          # Vec3 transformed into mat's local space (point)
v.to_world(mat)          # Vec3 transformed from mat's local space to world (point)
v.to_local_dir(mat)      # like to_local but ignores translation (direction)
v.to_world_dir(mat)      # like to_world but ignores translation (direction)
```

`mat * v` (Mat4 operator) is the same as `v.to_world(mat)`. The named
methods exist because they read more naturally than `mat.inverse() * v` for
the local case and because the direction-only variants have no clean
operator form.

---

## 5. Mat4

Immutable 4×4 matrix. Mutate methods return new instances. Stored layout
(row-major / column-major) is an implementation detail; the public API
indexes by `(row, col)`.

### 5.1 Constants

| Name | Value |
|---|---|
| `Mat4.IDENTITY` | identity matrix |

Shared immutable singleton, same as Vec3 constants.

### 5.2 Decomposed View

```python
m.pos                    # Vec3, read-only: translation component
m.rot                    # Vec3, read-only: Euler angles (degrees, XYZ extrinsic)
m.scale                  # Vec3, read-only: per-axis scale
```

These are convenience read accessors that decompose the affine part of the
matrix. Behavior on non-affine matrices (e.g. perspective) is
implementation-defined.

### 5.3 Element Access

```python
m[i, j]                  # __getitem__(key: tuple[int, int]) -> float
                         # i = row, j = column (math notation M_{ij})
```

`__getitem__` raises `IndexError` for keys outside `0..3 × 0..3`. Mat4 is
immutable, so there is no `__setitem__`.

### 5.4 Operators

```python
m1 * m2                  # Mat4 (matrix multiply)
m * v                    # Vec3 (transform point, equivalent to v.to_world(m))
m1 == m2, m1 != m2       # value-wise comparison
hash(m)                  # value-based hash (usable as dict key / set element)
repr(m)                  # debug representation
```

### 5.5 Class-Method Factories

All factory names follow the `from_*` convention except for the two
compound builders (`compose`, `look_at`).

```python
Mat4.from_translation(pos)              # pos: Vec3
Mat4.from_rotation(rot)                 # rot: Vec3 (Euler degrees, XYZ extrinsic)
Mat4.from_quat(quat)                    # quat: Quat
Mat4.from_scale(scale)                  # scale: Vec3
Mat4.compose(pos, rot, scale)           # TRS in one call
Mat4.look_at(eye, target, up=Vec3.UP)   # camera-style view matrix
```

`compose` builds `T × R × S` in a single call (equivalent to
`Mat4.from_translation(pos) * Mat4.from_rotation(rot) * Mat4.from_scale(scale)`).

### 5.6 Mutate Methods (return new Mat4)

```python
m.translate(v)           # v: Vec3 — accumulate translation
m.rotate(axis, deg)      # arbitrary-axis rotation
m.rotate_x(deg)          # convenience for X axis
m.rotate_y(deg)
m.rotate_z(deg)
m.scale_by(v)            # v: Vec3 — accumulate per-axis scaling
```

`scale_by` (rather than `scale`) avoids name collision with the `scale`
read accessor. All mutate methods compose onto the current matrix and
return a new Mat4.

### 5.7 Matrix Operations

```python
m.inverse()              # Mat4
m.transpose()            # Mat4
m.determinant()          # float
```

### 5.8 Coordinate System Conversions

```python
m.to_local(mat)          # Mat4 expressed in mat's local space
m.to_world(mat)          # Mat4 expressed in world space (mat = local origin)
m.to_local_dir(mat)      # like to_local but translation-free
m.to_world_dir(mat)      # like to_world but translation-free
```

Same semantics as `Vec3.to_local` etc., but operating on a full transform.
The argument name `mat` mirrors `Vec3.to_local(mat)` so the four methods
read uniformly across both classes.

---

## 6. Quat

Immutable quaternion rotation. Component order is `(x, y, z, w)` with `w`
as the scalar (matches three.js / Unity / Godot ; differs from the
mathematical `(w, x, y, z)` convention).

### 6.1 Constants

| Name | Value |
|---|---|
| `Quat.IDENTITY` | identity rotation `(0, 0, 0, 1)` |

### 6.2 Construction and Sequence Protocol

```python
q = Quat(x, y, z, w)         # defaults: x=y=z=0.0, w=1.0 (identity)
q.x, q.y, q.z, q.w           # read-only attributes
q[0], q[1], q[2], q[3]       # __getitem__(key: int) -> float
list(q), tuple(q)            # __iter__ over (x, y, z, w)
len(q)                       # 4
q == other, q != other       # value-wise comparison
hash(q)                      # value-based hash (usable as dict key / set element)
repr(q)                      # debug representation
```

### 6.3 Operators

```python
q1 * q2                  # Quat (rotation composition)
q * v                    # Vec3 (rotate vector)
-q                       # Quat (negate; same rotation, different sign)
```

No `+`, `-`, or `/` — quaternion linear combinations are seldom meaningful
in game use, and slerp covers interpolation.

### 6.4 Class-Method Factories

```python
Quat.from_axis_angle(axis, deg)      # axis: Vec3, deg: float
Quat.from_euler(rot)                 # rot: Vec3 (degrees, XYZ extrinsic)
Quat.from_two_vectors(a, b)          # rotation that takes a to b
Quat.from_matrix(mat)                # rotation extracted from mat: Mat4
```

`from_two_vectors(a, b)` returns the shortest-arc rotation that maps `a` to
`b`; both are normalized internally if not already.

### 6.5 Unary Operations

```python
q.conjugate()            # Quat
q.inverse()              # Quat
q.normalize()            # Quat (zero-length input returns Quat.IDENTITY)
q.length()               # float
q.length_squared()       # float
```

### 6.6 Binary Operations

```python
q.dot(other)             # float
q.angle_to(other)        # float, degrees
```

### 6.7 Conversions

```python
q.to_matrix()            # Mat4 (rotation-only)
q.to_euler()             # Vec3 (degrees, XYZ extrinsic)
q.to_axis_angle()        # tuple[Vec3, float] — (axis, degrees)
```

`Quat.to_matrix()` and `Mat4.from_quat(q)` are intentionally provided as
two access points for the same conversion (one per class), to let either
class be the entry point.

### 6.8 Interpolation

```python
q.slerp(other, t)        # Quat (spherical linear interpolation)
```

`t` outside `[0, 1]` extrapolates; callers clamp explicitly if needed.

---

## 7. Camera

View information held independently from the scene so multiple cameras can
be swapped quickly (e.g. multi-angle rendering).

```python
camera = Camera()
camera.transform = Mat4.look_at(eye, target)
camera.fov = 60.0
```

| Attribute | Type | Default | Meaning |
|---|---|---|---|
| `transform` | `Mat4` | `Mat4.IDENTITY` | camera world transform |
| `fov` | `float` | `60.0` | vertical field of view in degrees (perspective) |
| `near` | `float` | `0.1` | near plane distance |
| `far` | `float` | `1000.0` | far plane distance |
| `ortho_size` | `float \| None` | `None` | `None` → perspective; value → orthographic with that vertical size |

The single `ortho_size: float | None` attribute encodes both "is
orthographic?" and "what size?" in one place. Setting `ortho_size = N`
switches the camera to orthographic projection with vertical world size
`N`; setting `ortho_size = None` restores perspective using `fov`.

`Camera()` produces a camera ready to use with the listed defaults.
`__repr__` is provided for debugging visibility; `__eq__` / `copy()` are
not provided (default identity equality applies). View-control helpers
like `look_at` are intentionally not on `Camera` — building the transform
through `Mat4` (e.g. `camera.transform = Mat4.look_at(...)`) keeps
animation and interpolation flexible.

---

## 8. ShadeRamp

Shading LUT shared by the whole scene during a `draw` call. The table is a
2D structure: rows are palette colors, columns are 8 brightness levels.
ShadeRamp also absorbs per-color palette substitution (replacing the
classic `pal` operation): set every level of a row to the same target
color and that source color is replaced uniformly across the subtree
that resolves to this ramp.

```python
ramp = ShadeRamp()                   # default ramp built from current palette
ramp[col, level]                     # tuple[int, int, int] — (primary, secondary, ratio)
ramp[col, level] = value             # tuple[int, int, int] — overwrite this cell
ramp.build()                         # rebuild from current pyxel palette
```

Each cell is a `(primary, secondary, ratio)` triple. `primary` and
`secondary` are palette indices; `ratio` is `0..16`, the count of cells
in the 4×4 Bayer dither pattern that pick `secondary` over `primary`.
The default-built ramp emits only `0` (flat fill) or `8` (50:50 dither);
`__setitem__` accepts any `0..16` for user-customized cells.

- `ShadeRamp()` initializes with a default ramp derived from the current
  Pyxel palette via the same algorithm as `build()`. Ready to use without
  further setup.
- `build()` rebuilds the ramp from the current Pyxel palette, used after
  the user changes the palette via `pyxel.colors`. Synchronization is
  manual (no automatic update).
- `__getitem__` / `__setitem__` use `(col, level)` keys, parallel with
  Mat4's `(row, col)` indexing style.

Out of range keys raise `IndexError`. The 4×4 Bayer dither lets cube
approximate brightness levels that no single palette color matches; `ratio
== 0` cells render as a flat `primary` fill so dithering is opt-in per
cell.

**Dimensions**: row count follows `pyxel.colors` length (Pyxel default
16, but the actual length of the palette at `build()` time); column
count is fixed at 8 brightness levels.

**`build()` algorithm**: for each col, an internal *chain* is constructed
by walking same-hue neighbors in OKLab perceptual space, starting from
the col's own color. The chain extends up to 3 steps darker (shade) and
2 steps brighter (highlight); each step requires an OKLab L delta of at
least 0.10 against its predecessor and stays within the same hue family
(chromatic↔achromatic crossings are refused). The 8 ramp levels are then
filled by case analysis on chain length:

- `lv 3, 4, 5` are always the base flat plateau.
- `lv 7` is the chain top flat (= the highlight color); when the chain
  top is achromatic, lv 7 falls back to a dither between hl1 and the top
  so the brightest cell isn't a pure white/gray.
- `lv 6` is `(base, hl1)` dither when only one highlight step exists, or
  `hl1` flat when two exist.
- `lv 0` is the chain-bottom flat (= the darkest shade reached).
- `lv 2` is `(base, sh1)` dither when only one shade step exists, or
  `sh1` flat otherwise.
- `lv 1` fills the gap between lv 0 and lv 2 — sh1 flat when shade=1,
  `(sh1, sh2)` dither when shade=2, sh2 flat when shade=3.

By construction, each dither pair lies on adjacent chain entries and the
two sides of every dither lv are flats matching the dither's two colors,
so the ramp transitions smoothly without spurious colors leaking across
boundaries. The chain-step floor (L_STEP_MIN = 0.10) keeps ramp depth
independent of palette richness — packing more colors into the same
hue family no longer collapses contrast. The full design rationale is
captured in `docs/superpowers/specs/2026-05-09-cube-shade-ramp-design.md`.

`__repr__` is provided for debugging.

**Palette substitution use case**: setting
`ramp[src_col, level] = (dst_col, dst_col, 0)` for every `level` makes
`src_col` always render as `dst_col` regardless of the per-face
brightness — the equivalent of Pyxel 2D's `pyxel.pal(src_col, dst_col)`.
(`ratio = 0` collapses the cell to a flat `primary` fill; `secondary`
is irrelevant in that case, so any palette index works there.) This
unifies palette substitution and shading under one structure (one shared
LUT instead of separate `pal` state plus a shading table — see § 16.3
for the rationale).

Bulk get/set (`to_list` / `from_list`), factory variants, and
file load/save are intentionally not provided in the initial API — they
have no equivalent in pyxel main (where similar APIs are deprecated in
favor of slice assignment) and add no clear benefit at cube's scale.

---

## 9. Light

Flat-shading parameters typically held scene-wide (Scene seeds a
default at construction; see § 13.1). Multiple Light instances can be
swapped on `Scene.light` for global changes, or set per Node to
override lighting within a subtree (§ 12.4).

```python
light = Light()
light.ambient = 0.0
light.direction = Vec3(0, -1, 0)
light.intensity = 1.0
```

| Attribute | Type | Default | Meaning |
|---|---|---|---|
| `ambient` | `float` | `0.0` | base brightness offset added before lighting |
| `direction` | `Vec3` | `Vec3.DOWN` | parallel light direction (the direction the light travels) |
| `intensity` | `float` | `1.0` | scale factor applied to directional contribution |

Output brightness for a face is computed as
`ambient + max(0, dot(face_normal, -direction)) * intensity`, then mapped
through `ShadeRamp` to produce the final palette index.

**Flat shading only**: face-constant color, no Gouraud / Phong / per-pixel
lighting. The reasoning is twofold — software rasterization makes per-pixel
lighting expensive, and Pyxel's small palette cannot represent fine
gradients anyway.

`__repr__` is provided for debugging.

---

## 10. Buffers

Static vertex-data asset. `Geometry` carries the vertex attributes
(positions / normals / uvs), the topology (indices, prim mode), and
the back-face cull mode. It is shareable across many `Node` draws and
across `Mesh` parts.

### 10.1 Class-level Constants

| Name | Value | Attribute |
|---|---|---|
| `Geometry.PRIM_POINTS` | `0` | `prim` |
| `Geometry.PRIM_LINES` | `1` | `prim` |
| `Geometry.PRIM_TRIANGLES` | `2` | `prim` |
| `Geometry.CULL_NONE` | `0` | `cull` |
| `Geometry.CULL_BACK` | `1` | `cull` |
| `Geometry.CULL_FRONT` | `2` | `cull` |

Mode values follow OpenGL ordering. Cull values use a small `CULL_` enum
because raw `BACK` / `FRONT` would be ambiguous against other directional
constants.

### 10.2 Attributes

| Field | Type | Default | Meaning |
|---|---|---|---|
| `positions` | `list[float]` | `[]` | flat (x, y, z) triples; `len(positions) % 3 == 0` |
| `normals` | `list[float] \| None` | `None` | flat (nx, ny, nz) per vertex; `None` triggers per-face auto-compute on first draw |
| `uvs` | `list[float] \| None` | `None` | flat (u, v) per vertex; `None` disables texture sampling |
| `indices` | `list[int] \| None` | `None` | flat indices; `None` draws as a flat list whose chunk size follows `prim` |
| `prim` | `int` | `PRIM_TRIANGLES` | topology (see § 10.1) |
| `cull` | `int` | `CULL_BACK` | back-face cull mode |

### 10.3 Construction and Mutation

```python
geom = Geometry(
    positions=[0, 1, 0,  -1, -1, 0,  1, -1, 0],
    indices=[0, 1, 2],
    uvs=[0.5, 0,  0, 1,  1, 1],
    prim=Geometry.PRIM_TRIANGLES,
    cull=Geometry.CULL_NONE,
)
geom.positions = [...]                  # reassign whole buffer
geom.compute_normals(smooth=False)      # explicit per-face / smooth normals
```

`__init__` is all-optional. Python native `list[float]` / `list[int]`
are used directly — the redesign drops the earlier `FloatBuffer` /
`IntBuffer` typed-buffer classes in favor of the standard list
container. The binding layer copies the list into a contiguous internal
buffer at attribute assignment, so subsequent draws read from the
internal buffer without per-element FFI.

Slice mutation on the returned list (`geom.positions[0:9] = [...]`)
does **not** propagate to the internal buffer; reassign the whole list
to refresh (`geom.positions = new_list`). Dynamic per-frame deformation
is expressed as a per-frame reassignment; cube's primary use case is
static asset construction at load time.

### 10.4 Normal Auto-Cache

When `normals` is `None`, the renderer computes per-face flat normals
on the first draw and stores them on the attribute. Subsequent draws
reuse the cached normals. To force recomputation after mutating
`positions`, set `geom.normals = None` (the next draw recomputes), or
call `compute_normals()` to refresh explicitly.

`compute_normals(smooth: bool = False)`:

- `smooth=False` (default): one normal per face, replicated to each
  vertex of that face (flat shading).
- `smooth=True`: averages adjacent face normals at shared vertices
  (smooth shading). Requires `indices` populated; with `indices = None`
  the per-face flat behavior is used.

### 10.5 Topology and `prim` / `indices` Interaction

`prim` determines how indices (or positions, if `indices` is `None`)
are grouped:

- `PRIM_POINTS`: 1 index per point.
- `PRIM_LINES`: 2 indices per line segment.
- `PRIM_TRIANGLES`: 3 indices per triangle.

A geometry's `prim` is part of the asset's identity — switching the
mode at draw time is not supported, because different prim modes have
different index-list interpretations. To draw the same vertices as both
a solid mesh and a wireframe, create two separate `Geometry` instances
with different `indices` and `prim` values.

### 10.6 Per-Geometry Cull Mode

`cull` is held on `Geometry` (not per draw) because it is a geometric
property: planar grass billboards need `CULL_NONE` (two-sided), solid
boxes need `CULL_BACK` (single-sided), independent of the draw
context. If a shape has mixed cull regions (e.g., character body
cull-on + hair-plane cull-off), split the shape into two `Geometry`
instances and combine them through `Mesh` parts or a `Node` hierarchy.

---

## 11. Mesh

A hierarchical 3D model asset. `Mesh` bundles multiple `Geometry`
parts (positions / topology / cull) with a shared texture or flat
color (`col_img`) and parent-child relationships between parts (held
as parallel arrays). Drawing routes through `Node.mesh(mat, mesh)`,
which composes per-part world transforms and emits each part through
the same internal path as `Node.prim`.

### 11.1 Members

| Field | Type | Meaning |
|---|---|---|
| `geometries` | `list[Geometry \| None]` | part i's `Geometry`, or `None` for a pure group (transform-only, no draw) |
| `transforms` | `list[Mat4]` | part i's local transform in its parent's frame |
| `parents` | `list[int]` | part i's parent index; `-1` marks a root; `parents[i] < i` always |
| `col_img` | `int \| Image` | flat color (when `int`) or shared texture (when `Image`) for all parts |
| `colkey` | `int \| None` | transparent color when `col_img` is `Image` |

### 11.2 Parallel Arrays

The three lists `geometries`, `transforms`, and `parents` are parallel:
all three index the same set of mesh parts and must have the same
length. The constructor validates the length match and raises
`ValueError` on mismatch.

### 11.3 Topological Order Constraint

`parents[i] < i` is required for every `i`. The constructor enforces
the constraint, raising `ValueError` if violated. This makes world
transform computation a single forward pass:

```python
world = [None] * len(transforms)
for i in range(len(transforms)):
    if parents[i] == -1:
        world[i] = transforms[i]
    else:
        world[i] = world[parents[i]] * transforms[i]
```

### 11.4 Construction

```python
character = Mesh(
    geometries=[geom_body, geom_hair, geom_sword, None],
    transforms=[
        Mat4.IDENTITY,
        Mat4.from_translation(Vec3(0, 1, 0)),
        Mat4.from_rotation(Vec3(0, 0, 90)),
        Mat4.from_translation(Vec3(0.5, 0, 0)),
    ],
    parents=[-1, 0, 3, 0],
    col_img=pyxel.images[0],
    colkey=0,
)
```

`__init__` is all-optional. Parts can be added by reassigning the three
arrays (each reassignment revalidates). `Mesh` has no factory methods;
build the asset by populating the arrays directly. For load-from-file
workflows (e.g., glTF import), the importer assembles the arrays in
topological order and hands them to the constructor.

### 11.5 `descendants` Helper

```python
indices: list[int] = mesh.descendants(i)
```

Returns all part indices that are transitive children of part `i`
(excluding `i` itself), in topological order. Used by callers that
want to bulk-transform a subtree (e.g., move the weapon and everything
attached). Runs in O(N) via a single forward sweep using the
topological-order invariant.

### 11.6 Shared `col_img` and `colkey`

`col_img` is shared across every part of the mesh. Mixed-texture models
(different parts needing different images) are expressed as separate
`Mesh` instances combined through a `Node` hierarchy. This keeps `Mesh`
focused on "one model, one texture atlas" — the typical structure for
pixel-art asset workflows.

### 11.7 Drawing patterns

| Use case | Pattern |
|---|---|
| Reusable mesh on an actor | hold one `Mesh` and call `self.mesh(mat, mesh_asset)` in `on_draw` |
| Same model, different transform per instance | one `Mesh` referenced from many `Node` instances |
| Dynamic mesh (per-frame deform) | reassign `geom.positions` per frame for the part being deformed |
| Many small line / triangle draws | use `self.line` / `self.tri` in `on_draw` directly (no `Mesh` needed) |
| Custom raw draw | construct a `Geometry` directly and call `self.prim(mat, geom)` |

`Mesh` is asset-only — drawing routes through `Node.mesh` (or `Node.prim`
for a single `Geometry` without the hierarchical container).

---

## 12. Node

Base class for everything in the scene tree. A `Node` carries a transform,
hierarchy links, draw / collide / lifecycle hooks, and node-local draw
commands. `Scene` (§ 13) is the root `Node`; user-defined actors subclass
`Node` and override the lifecycle hooks.

```python
class Player(Node):
    def on_update(self):
        self.transform = self.transform.translate(Vec3(0.1, 0, 0))

    def on_draw(self):
        self.box(Mat4.IDENTITY, Vec3(1, 2, 0.5), 4)
```

### 12.1 Attributes

| Attribute | Type | Cascade | Meaning |
|---|---|---|---|
| `name` | `str` | — | identifier for `find()` |
| `transform` | `Mat4` | composed with parent's transform during draw | local-space transform |
| `active` | `bool` | parent-dominant (False halts subtree update + collision) | enable/disable update + collision |
| `visible` | `bool` | parent-dominant (False halts subtree drawing) | enable/disable draw |
| `light` | `Light \| None` | None inherits from the closest non-None ancestor | lighting parameters effective for this subtree |
| `shade_ramp` | `ShadeRamp \| None` | None inherits from the closest non-None ancestor | shading LUT effective for this subtree |
| `collider` | `Collider \| None` | this node only | collision shape (placeholder; pipeline deferred — § 15) |
| `parent` (read-only property) | `Node \| None` | — | direct parent in the tree |
| `children` (read-only property) | `tuple[Node, ...]` | — | direct children |
| `camera` (read-only property) | `Camera` | — | active camera; valid only inside `on_draw` |

Per-draw modifiers (`shaded`, `dither_alpha`, `depth_test`,
`depth_write`, `billboard`) are not Node properties — they are passed
as keyword arguments to each draw command (§ 12.5). Only "scene-wide
data shared across many draws" (`light`, `shade_ramp`) is held on the
Node tree.

#### Cascade modes

- **parent-dominant**: when an ancestor's value is False, every descendant
  is treated as False regardless of its own setting. Used for `active` and
  `visible` — a single ancestor flag can suspend an entire branch.
- **inherits-from-ancestor**: when this node's value is `None`, the
  effective value is the closest non-`None` ancestor's value. Used for
  `light` and `shade_ramp` — set them once on `Scene` (Scene seeds
  defaults at construction) and override per-subtree as needed.
- **this node only**: no propagation. Used for `collider`.

#### Class-level constants

`Node` exposes integer constants for billboard modes (used by per-call
`billboard` arguments):

| Constant | Value | Meaning |
|---|---|---|
| `Node.BILLBOARD_OFF` | 0 | no billboard adjustment (Node `transform` used as-is) |
| `Node.BILLBOARD_ON` | 1 | full camera-facing billboard (rotation overridden each draw) |
| `Node.BILLBOARD_FIXED_Y` | 2 | Y axis fixed to world up; X / Z rotation follows the camera |

Billboard mode values follow Godot's `BillboardMode` concept with
shortened names for brevity: `OFF` corresponds to Godot's `DISABLED`,
`ON` to `ENABLED`, and `FIXED_Y` matches Godot's `FIXED_Y` directly.

The primitive mode constants (`PRIM_POINTS` / `PRIM_LINES` /
`PRIM_TRIANGLES`) live on `Geometry` instead (see § 10.1); the mode is
a property of the asset, not the draw caller. Future additions
(`PRIM_LINE_STRIP`, `PRIM_LINE_LOOP`, `PRIM_TRIANGLE_STRIP`,
`PRIM_TRIANGLE_FAN`) will keep the OpenGL numbering established on
`Geometry`.

### 12.2 Tree Operations

```python
parent.add_child(child)             # also unlinks child from any prior parent
parent.remove_child(child)
descendant = root.find(name)        # subtree DFS, returns first match (or None)
node.destroy()                      # detach and remove from the tree
```

`add_child` implicitly removes the child from its previous parent. `find`
performs a depth-first pre-order search starting at `self` (matching this
node's own `name` first), returning the first match or `None`. `destroy`
removes the node from its parent and triggers `on_destroy`.

### 12.3 World Transform

```python
node.world_transform()              # Mat4 — composition of all ancestor transforms
```

Computed on demand by walking up the tree. Cube does not cache the world
transform; users that hit this in a hot path should compute and reuse the
value within a single frame.

### 12.4 Lighting Cascade

`light` and `shade_ramp` are Node properties (§ 12.1) that follow the
inherits-from-ancestor cascade. `Scene` seeds defaults at construction
(see § 13.1), so descendants always resolve to a usable lighting
setup. The convention is to set them once on `Scene` for scene-wide
control, then override per-subtree where a region needs different
lighting:

```python
# scene-wide default
scene.light.direction = Vec3(0.3, -0.7, 0.4)
scene.shade_ramp.build()

# subtree override: a dim cave area
cave = Node()
cave.light = Light()
cave.light.ambient = 0.05
cave.light.intensity = 0.2
scene.add_child(cave)
```

Other draw-related controls (`shaded`, `dither_alpha`, `depth_test`,
`depth_write`, `billboard`) are not Node properties — they are passed
as keyword arguments to each draw command (§ 12.5). The line is drawn
between "scene-wide data shared by many draws" (`light`, `shade_ramp`)
and "per-call decisions that can vary between adjacent draws within
the same `on_draw`". See § 16.3 for the rationale.

### 12.5 Immediate-Mode Draw Commands

Inside `on_draw`, the node draws into the current camera and screen.
Coordinates are node-local (the engine composes parent transforms
during draw). Every draw command is a one-liner that fully specifies
its style through positional + keyword-only arguments — there is no
"current draw state" the user has to track.

```python
# Vec3 vertex-list primitives
self.pset(pos, col, ...)
self.line(p1, p2, col, ...)
self.tri(p1, p2, p3, col, ...)
self.trib(p1, p2, p3, col, ...)

# Screen-aligned 1-point shapes (always face camera)
self.circ(pos, r, col, ...)
self.circb(pos, r, col, ...)

# 3D solids (Vec3-positioned, symmetric)
self.sphere(pos, r, col, ...)
self.sphereb(pos, r, col, ...)

# Mat4-positioned plane shapes
self.rect(mat, w, h, col, ...)
self.rectb(mat, w, h, col, ...)
self.elli(mat, w, h, col, ...)
self.ellib(mat, w, h, col, ...)

# Mat4-positioned solids
self.box(mat, size, col, ...)
self.boxb(mat, size, col, ...)

# Text (Vec3-positioned, screen-space billboard glyphs)
self.text(pos, s, col, font=None, ...)

# Image quads
self.sprite(pos, img, uvs, w, h, ...)        # always camera-facing
self.plane(mat, img, uvs, w, h, ...)         # free orientation

# Mesh asset draw (hierarchical, parallel-array based; see § 11)
self.mesh(mat, mesh_asset, ...)

# Generic primitive draw (low-level, takes a Geometry; see § 10)
# The Geometry carries its own positions / normals / uvs / indices /
# prim mode / cull mode.
self.prim(mat, geom, col_img=7, colkey=None, ...)
```

#### Positioning conventions

- **Vec3-positioned** (`pos`, `p1`, `p2`, `p3`): used by vertex-specified
  primitives, screen-aligned shapes, billboards, `text` (screen-space
  glyphs anchored at the projected point), and `sphere` (symmetric: a
  single point + radius is sufficient).
- **Mat4-positioned** (`mat`): used by primitives that need full
  orientation — plane shapes, 3D solids with a directional axis,
  `plane`, `mesh`, and `prim`.

#### Common keyword-only arguments

Every draw command takes keyword-only modifier arguments. Only those
that meaningfully apply per command are exposed (rules below):

| Argument | Type | Default | Applies to | Meaning |
|---|---|---|---|---|
| `col_img` | `int \| Image` | `7` | `prim` only | flat color when `int`, texture when `Image` (Mesh's own `col_img` is consulted by `mesh`) |
| `colkey` | `int \| None` | `None` | image / mesh / prim commands | transparent palette index for textures |
| `angle` | `float` | `0.0` | `sprite` only | screen-space rotation in degrees |
| `font` | `Font \| None` | `None` | `text` only | overrides default font |
| `shaded` | `bool` | varies | filled-face commands | apply directional + ambient lighting through `ShadeRamp` |
| `dither_alpha` | `float` | `1.0` | all draw commands | Bayer-dither pseudo-alpha (`1.0` opaque, `0.0` fully transparent); same value space as `pyxel.dither(alpha)` |
| `depth_test` | `bool` | `True` | all draw commands | enable depth-buffer comparison |
| `depth_write` | `bool` | `True` | all draw commands | enable depth-buffer writes |
| `billboard` | `int` | `BILLBOARD_OFF` | most Mat4 / multi-Vec3 commands | rotate `transform` to face the camera (see § 12.1 constants) |

Rules for which modifier appears on which command:

- **`shaded`** is omitted from commands that have no surface normal —
  lines, points, outlines (`pset`, `line`, `trib`, `rectb`, `ellib`,
  `boxb`, `sphereb`), screen-aligned circles (`circ`, `circb`), and
  `text`. `prim` carries `shaded` for use with a `Geometry` whose
  `prim` is `PRIM_TRIANGLES`; with `PRIM_LINES` / `PRIM_POINTS` the
  value is silently ignored.
- **`billboard`** is omitted where it has no visible effect — single
  points (`pset`), screen-aligned circles (`circ`, `circb`), `text`
  (always screen-space billboard), and symmetric solids (`sphere`,
  `sphereb`). `sprite` is always camera-facing (no `billboard` argument;
  the function pins `BILLBOARD_ON` internally).

Default values for `shaded` follow "what looks natural with no
arguments":

- **Outlines / lines / points / screen-aligned shapes**: `shaded` not
  exposed (treated as unshaded internally).
- **Filled 2D / 3D solids** (`tri`, `rect`, `elli`, `box`, `sphere`,
  `plane`, `mesh`, `prim` on a `PRIM_TRIANGLES` Geometry): `shaded=True`.
- **`sprite`**: `shaded=False` (decoration / particle use cases are
  the majority; set `shaded=True` to blend a sprite into the scene's
  ambient + directional lighting).

#### Shape conventions

- **Center pivot**: every shape is centered at its `pos` / `mat.pos`. No
  top-left pivot anywhere in the cube API.
- **`circ` / `circb`**: always face the camera; radius `r` is in world
  units (perspective shrinks distant circles); border is 1 pixel
  regardless of distance.
- **`line`**: world-positioned, fixed 1-pixel width.
- **`sphere` / `sphereb`**: 12-vertex / 20-triangle icosahedron scaled by
  `r`. Internally backed by a cached static vertex buffer and routed
  through `prim`. `sphere` is filled, `sphereb` is wireframe
  (icosahedron edges).
- **`box` / `boxb`**: `size` is `Vec3(width, height, depth)`. Internally
  backed by a cached static unit-cube buffer and routed through `prim`.
  `box` is filled, `boxb` is wireframe edges.
- **`text`**: 3D anchor + screen-space glyphs. `pos` is projected to
  screen, then characters render in 2D pixels at the font's native size
  (no perspective scaling on the glyphs themselves). Always
  camera-facing; ancestor rotation / scale do not affect glyph layout.
  Centered around the text bounding box at the projected point.
  `depth_test` / `depth_write` still apply at `pos`'s screen-z.
- **`sprite`**: billboard quad always facing the camera. `angle`
  rotates the quad in screen space (around view-z), in degrees.
  Internally: `billboard` is pinned to `BILLBOARD_ON`. With
  `shaded=True` the sprite's normal is taken as `-view_dir` (matches
  Godot `Sprite3D.shaded`).
- **`plane`**: free-oriented quad. `mat` carries position, rotation,
  and scale; `(w, h)` is the quad's local width and height.
- **`mesh`**: draws the given `Mesh` asset's geometry, transformed by
  `mat` in node-local space. Use `Mat4.IDENTITY` to draw the mesh at
  the mesh root at the node's origin; pass a non-identity `mat` to
  nudge the whole mesh relative to the node without changing the
  node's own transform. The mesh's parts each carry their own local
  transform, prim mode, and cull mode; the renderer composes them in
  topological order. `col_img` and `colkey` live on the `Mesh`
  (`mesh.col_img`, `mesh.colkey`) and are not exposed as per-call
  arguments — the asset is the single source of truth for its
  material.
- **`prim`**: low-level entry that all higher-level commands route
  through. Takes a single `Geometry` argument carrying the vertex
  data (positions / normals / uvs / indices), the prim mode, and the
  cull mode. `col_img` accepts `int | Image` (integer = flat color,
  Image = textured triangles); `colkey` is the transparent palette
  index when `col_img` is an Image.

#### Texture and UV layout

`sprite` and `plane` take `img: Image` (integer image bank indices are
not accepted; use `pyxel.images[i]` if needed). `Tilemap` is excluded:
its tile-grid storage and tile-number indirection do not fit a UV-based
texture sampler, and the integer index space differs from `Image` in
Pyxel 2D.

`uvs` is a 4-vertex UV tuple in row-major order:

```python
uvs: tuple[
    tuple[float, float],   # vertex 0 — top-left
    tuple[float, float],   # vertex 1 — top-right
    tuple[float, float],   # vertex 2 — bottom-left
    tuple[float, float],   # vertex 3 — bottom-right
]
```

Passing `((0, 0), (1, 0), (0, 1), (1, 1))` (when w/h match the source
image size) reproduces the source image in its natural orientation. The
4-corner form is the software rasterizer's natural input and lets the
caller express flips, 90° rotations, and arbitrary trapezoidal mapping
in one parameter without a separate `flip_x` / `flip_y` / `angle90` API.

### 12.6 Lifecycle Hooks

Subclasses override these hooks to define behavior. Defaults are no-ops.

```python
def on_update(self): ...                         # called once per scene update
def on_draw(self): ...                           # called once per scene draw
def on_collide(self, other, contact=None): ...   # invoked by the (deferred) collision pipeline
def on_destroy(self): ...                        # called when destroy() runs
```

- `on_update`: business logic per frame. The driver visits the tree
  pre-order; subtrees with `active = False` are skipped.
- `on_draw`: drawing calls (immediate-mode + `self.mesh`). The driver
  visits subtrees with `visible = True` and runs each node's `on_draw`
  with draw state reset to defaults at entry.
- `on_collide`: signature is exposed today so user subclasses can stage
  collision-response code, but the cube runtime does **not** invoke it
  yet — the collision pipeline (§ 15) is deferred. `contact` will be a
  `Contact` once the pipeline lands; today it is always `None`.
- `on_destroy`: cleanup hook. Called once just before the node leaves
  the tree.

---

## 13. Scene

`Node`-derived root that drives the per-frame update / draw cycle and
owns the screen clear color. The application instantiates one `Scene`
(or several), adds actor `Node` subtrees as children, and calls `update`
and `draw` from Pyxel's update / draw callbacks.

```python
class App:
    def __init__(self):
        pyxel.init(256, 192)
        # Scene seeds light + shade_ramp with defaults; override for taste.
        self.scene = Scene()
        self.scene.clear_color = 0
        self.camera = Camera()
        self.scene.add_child(Player())
        pyxel.run(self.update, self.draw)

    def update(self):
        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, 256, 192, self.camera)
```

### 13.1 Inherited from Node

`Scene` inherits all `Node` attributes and methods (§ 12), so it is
indistinguishable from any other node when assigning lights, ramps,
running lifecycle hooks, or composing transforms. Scene's `__init__`
seeds `light` and `shade_ramp` with non-`None` defaults so descendants
always resolve a usable lighting setup through the
inherits-from-ancestor cascade (no `effective_*` lookup ever returns
`None` to the renderer). The convention is to override these on the
`Scene` itself for scene-wide changes; descendants inherit through the
`None`-fallback rule.

### 13.2 Scene-specific Attributes

| Attribute | Type | Default | Meaning |
|---|---|---|---|
| `clear_color` | `int \| None` | `None` | screen + depth buffer clear color before each draw; `None` skips clear |

`clear_color = None` is the "do not clear" mode (transparent overlay,
multi-pass, or the application is clearing externally). `clear_color =
int` fills the destination region with that color and resets the depth
buffer at the start of each `draw` (the 3D equivalent of `pyxel.cls`).

### 13.3 Driver Methods

```python
scene.update()
scene.draw(x, y, w, h, camera, screen=None)
```

- **`update()`**: traverses the tree pre-order and calls each active
  node's `on_update`. Subtrees rooted at a node with `active = False`
  are skipped entirely. (The cube runtime does not yet drive collision
  detection or the `on_collide` hook — that pipeline is deferred to
  § 15; the `Collider` / `Contact` classes exist today as placeholders.)
- **`draw(x, y, w, h, camera, screen=None)`**: rasterizes the scene
  into the destination rectangle `(x, y, w, h)` using `camera`. The
  driver clears the rectangle with `clear_color` (when set), traverses
  the visible subtree, and runs each node's `on_draw`.
  - `screen=None` (default): target `pyxel.screen`.
  - `screen=Image`: target a custom image (render-to-texture for
    minimap, multi-pass effects, off-screen rendering).

### 13.4 Multi-angle Rendering

Build the scene tree once, then call `draw` as many times per frame as
needed with different cameras / rectangles / target screens. The same
scene state drives every `draw` call within a frame; the only differences
are the camera, the destination rectangle, and the target screen.
Examples: minimap with a top-down camera, picture-in-picture rear-view,
render-to-texture for reuse as a `plane` texture.

```python
def draw(self):
    self.scene.draw(0, 0, 256, 192, self.main_camera)
    self.scene.draw(0, 144, 64, 48, self.minimap_camera)
```

---

## 14. Performance Notes

- A typical pixel-art game has tens to a few hundred drawables per frame.
  At that scale, immediate-mode command queueing has comparable cost to
  Pyxel 2D's existing per-call overhead.
- Multi-angle rendering (re-running `render` with a different camera) is
  cheap in the sense that no scene-graph traversal is repeated — only the
  rasterization stage runs again per call.
- `Mesh` is loaded once; `Node` trees built from it carry per-instance
  poses without copying mesh data.
- `Vec3` / `Mat4` / `Quat` are immutable; their constants are shared
  singletons. Arithmetic methods return fresh instances. The implementation
  is expected to keep allocation cheap for hot-path math operations.

---

## 15. Open Items

- **Collider system**: empty `Collider` / `Contact` classes,
  `Node.collider` slot (this-node-only), and `on_collide(other,
  contact)` lifecycle hook are exposed today as placeholders so user
  code can stage setups. Shape vocabulary (sphere / box / mesh),
  contact-payload fields beyond `point` / `normal`, broad-phase
  queries, and the `Scene.update`-driven dispatch are deferred until
  the first real-game collision use case surfaces.
- **Default ramp generation**: the algorithm `ShadeRamp()` and `ShadeRamp.build()`
  use to derive a default ramp from the current palette.
- **Joint animation system**: `Node.transform` is the per-frame surface;
  whether a higher-level `Motion` / animation player class is also needed
  is to be decided alongside the first real-game implementation.
- **Camera world ↔ screen helpers**: `Camera.world_to_screen(pos, ...)` /
  `screen_to_world(x, y, depth, ...)` would help HUD coordinates and
  mouse picking. Deferred because viewport size is not held by `Camera`
  in cube (it is a per-`render` argument), and the current need has not
  yet surfaced.
- **Extended PRIM modes**: `PRIM_LINE_STRIP`, `PRIM_LINE_LOOP`,
  `PRIM_TRIANGLE_STRIP`, `PRIM_TRIANGLE_FAN` along OpenGL's primitive
  numbering, for compact ribbon / fan / strip emissions through `prim`.
  Defer until a real-game use case surfaces.
- **World-scaled text variant**: an optional `text` mode that lays
  glyphs as 3D-space quads transformed by `mat` (so rotation / scale
  affect glyph layout). The current `text(pos, ...)` is screen-space
  glyphs at the projected point; users that need 3D-text shapes can
  build them through `prim()`. Revisit if a real use case surfaces.
- **Parent reference**: `Node.parent` returns `None` today as a
  placeholder. A real parent reference would create a Python ref cycle
  (`parent → children → parent`) that the cyclic GC cannot break
  without weakref plumbing; populating it is deferred until a real-game
  use case surfaces. Tree traversal via `children` and explicit user
  references cover the current needs.

---

## 16. Decisions Explicitly Ruled Out

These were considered and rejected during design; revisit only with new
evidence.

### 16.1 Math classes

- **`Vec2` / `Vec4` / `Mat3`** — not exposed publicly. Cube draws use
  `Vec3` for points and `Mat4` for full transforms; lower-dimensional
  variants would multiply the API surface without clear benefit.
- **Component-wise `Vec3 * Vec3`** — out, ambiguous in 3D math vocabulary.
  Use `Mat4.from_scale(other) * v` for per-axis scaling.
- **Swizzle properties** (`v.xy`, `v.yzx`, etc.) — not a Pyxel cube
  convention; numpy / pyrr-style swizzle is left to user code.
- **Mutable Vec3 / Mat4 / Quat** — cube standardizes on immutable. Mutable
  versions were ruled out because immutable types eliminate aliasing
  bugs and cleanly support shared singleton constants.
- **`Mat4.perspective` / `Mat4.orthographic` / `Mat4.frustum` factories**
  — projection lives in `Camera`, not in `Mat4`.
- **`Mat4.is_identity()` / `Quat.is_normalized()` predicate methods** —
  `mat == Mat4.IDENTITY` and explicit length checks cover these.
- **`Vec3.direction_to(other)`** — Godot-only; `(b - a).normalize()` is
  short enough not to warrant a dedicated method.
- **`Vec3.bounce(normal)` / `Vec3.slide(normal)`** — Godot-only.
- **`Vec3.refract(normal, eta)`** — optical refraction; not needed for the
  cube software renderer's lighting model.
- **`Mat4.orthonormal()`** — drift correction for accumulated rotations.
  In cube's typical usage (a few transform compositions per frame),
  numerical drift is negligible. Add later if profiling shows a need.
- **`Mat4.translate` / `Mat4.scale_by` scalar overloads** — `(x, y, z)`
  scalar overload was dropped in favor of `Vec3` only, to align with the
  mainstream 3D library convention (three.js / Godot / pyrr / Unity all
  take a single Vec3).
- **`Quat.__add__` / `Quat.__sub__` / `Quat.__truediv__`** — quaternion
  linear combinations have limited use; `slerp` covers what's needed.
- **`Quat.lerp`** — `slerp` is the standard quaternion interpolation.
- **`Quat.from_x_rotation` / `from_y_rotation` / `from_z_rotation`** —
  axis-specific factories are subsumed by `Quat.from_axis_angle` and
  `Quat.from_euler`.
- **`Mat4.compose(pos, quat, scale)` quat overload** — cube standardizes
  on Euler `Vec3` for rotation arguments. Use `Mat4.from_quat(q) * ...` for
  quaternion-driven composition.

### 16.2 Naming

- **`Vec` / `Mat` / `Quaternion`** — too generic or too long.
- **`Vector3` / `Matrix4`** — standard but verbose; `Vec3` / `Mat4` chosen
  for brevity, matching pyrr / three.js / PyGLM.
- **`length_sq` / `distance_sq_to`** — short forms dropped in favor of
  `length_squared` / `distance_squared_to`, to match pygame / Godot
  Python-style naming.
- **`max_len` argument name** — replaced with `max_length` to keep the
  cube API uniform on the `length` term.
- **`(sx, sy, sz)` argument names on scale** — irrelevant after dropping
  the scalar overload; the single `Vec3` argument carries the meaning.
- **Past-tense method names (`normalized`, `inverted`)** — Python idiom
  uses present tense (`normalize`, `inverse`) regardless of immutability.
- **`rotate_axis` / `rotate_arbitrary`** — `Mat4.rotate(axis, deg)` is
  short and unambiguous.

### 16.3 Drawing

- **Retained-mode scene graph with per-Node draw callbacks** — replaced
  by Scene's immediate-mode queue. The retained model overcomplicated
  per-frame variation (different draws per camera angle, conditional
  shapes) and required extra abstractions (`Primitive`, draw-state
  cascade). The current Mesh / Node split keeps "scene graph" purely as
  an animation tool, while `Scene` handles "what to draw this frame".
- **`Primitive` class** (multi-shape aggregate registered into a node).
- **Specialized Node subclasses** (`SpriteNode`, `LineNode`, `MeshNode`,
  `TextNode`, etc.). One Node class is enough; per-shape behavior is in
  Scene's draw commands.
- **`Shader` class** (combined shading + lighting + color tables) —
  replaced by `ShadeRamp` (shading LUT, also absorbing palette
  substitution) + `Light` (parameters). The split lets multiple ramps
  and multiple lights be created and swapped independently.
- **`scene.push_matrix` / `scene.pop_matrix`** — the matrix stack pattern
  was dropped because draw commands accept their own `mat` directly, and
  the Node tree's transform composition already covers hierarchical
  placement.
- **`int` for `img` parameter on `sprite` / `plane`** — `Image` only.
  Pyxel 2D's `pyxel.blt(img: int | Image, ...)` allows a bank index, but
  cube callers pass `pyxel.images[i]` explicitly to avoid coupling the
  cube API to bank-index semantics.
- **`Tilemap` on `sprite` / `plane`** — Tilemap's tile-grid storage and
  tile-number indirection do not match a UV-based texture sampler;
  selection happens at draw-call granularity rather than per-pixel.
  Use `Image` directly.
- **`(u, v, sw, sh)` source-rectangle form on textured commands** —
  replaced by a 4-vertex `uvs` tuple. The 4-corner form is the
  software rasterizer's natural input and expresses flips, rotations,
  and trapezoidal mappings without auxiliary parameters.
- **Scalar `(u1, v1, u2, v2, u3, v3, u4, v4)` for UVs** — too many
  positional arguments; the nested-tuple form keeps the call site
  readable.
- **`fill` draw op on `Node`** — out of scope. `fill` has no clean 3D
  meaning (no obvious enclosure to flood). (Palette substitution is
  absorbed into `ShadeRamp` § 8 instead of being a draw op.)
- **Standalone `pal` operation / `Node.pal` draw state** — replaced by
  setting `ShadeRamp` rows uniformly (a row whose every level maps to
  the same target color is equivalent to `pyxel.pal(src, dst)`). One LUT
  (`ShadeRamp`) covers both shading and palette substitution; a
  separate `pal` table would duplicate state and inflate per-Node memory
  when the palette grows.
- **`Node.depth_test(...)` / `Node.depth_write(...)` / `Node.dither(...)`
  draw-state methods** — and also the later
  `Node.shaded` / `Node.dither` / `Node.depth_test` / `Node.depth_write`
  / `Node.billboard` *properties* with inherits-from-ancestor cascade
  — both rejected. State-machine methods burden the renderer with
  current-state tracking; cascading properties make it awkward to mix
  lit and unlit draws within a single `on_draw` (a common pattern: a
  shaded mesh actor + unshaded sprite decorations + dither-faded
  smoke trails). The settled answer is **per-call keyword arguments**
  on every draw command (§ 12.5). "Node = Actor" stays clean (Node
  carries lifecycle, hierarchy, transform — not draw-style state).
  Subtree-wide effects are still expressible via local helper
  closures inside the `on_draw`.
- **`alpha` argument named bare `alpha`** — rejected in favor of
  `dither_alpha` to flag that the implementation is Bayer dithering,
  not continuous alpha blending. Value space matches
  `pyxel.dither(alpha)`: `1.0` opaque, `0.0` transparent.
- **`Node.text(pos, ...)` screen-aligned form** — replaced by
  `Node.text(mat, ...)` Mat4 form. 3D-spaced text (signs, NPC labels)
  needs world-scale sizing; HUD text remains Pyxel 2D's
  `pyxel.text(x, y, ...)` job, called after `scene.draw()`.
- **`Node.billboard` cascade property** — rejected for the same reason
  as the other cascade-based modifiers. Replaced by per-call
  `billboard` keyword argument on Mat4-based and multi-Vec3 commands.
  Constants `BILLBOARD_OFF` / `BILLBOARD_ON` / `BILLBOARD_FIXED_Y`
  mirror Godot's `BillboardMode`. `sprite` is hard-coded to
  `BILLBOARD_ON` (the function name communicates "always face camera");
  `pset` / `circ` / `circb` / `sphere` / `sphereb` omit the argument
  because billboard has no visible effect on those shapes.
- **`draw_text` / `draw_image` / `make_*` prefixes** — cube uses bare
  verbs (`node.text`, `node.sprite`, etc.) without prefix on draw
  commands or factories.
- **`make_*` factory prefix** — the library uses `from_*` consistently
  for class-method factories (`Mat4.from_translation`,
  `Quat.from_axis_angle`).
- **GPU-oriented features** (flat 16-element `to_list` / `from_list` on
  Mat4, OpenGL handles, shader programs) — cube is software-rendered;
  GPU integration is intentionally out of scope.
- **`col_tex` argument name for asset draws** — would imply a `Texture`
  class. The 3D industry's `tex` convention (three.js / Godot / Unity)
  aligns with a separate Texture class on each engine. Cube uses
  `pyxel.Image` directly with no `Texture` wrapper, so `tex` lacks the
  language fit; revisit if a Texture class is introduced.
- **`col_image` argument name** — short-form `col` + full-form `image`
  mixes naming registers within one compound; the all-short `col_img`
  is internally consistent.
- **`mode` as the topology attribute name on `Geometry`** — generic
  ("mode of what?") and clashes with `cull` as a separate "mode" axis.
  `prim` is chosen for parallelism with `Node.prim(...)` and the
  `PRIM_*` constant prefix.
- **`Geometry.MODE_LINES` / `Geometry.DRAW_LINES` constant prefixes** —
  ruled out in favor of `Geometry.PRIM_LINES`. `MODE_` ties weakly
  back to the attribute name and `DRAW_` reads above the topology
  layer; `PRIM_` matches the 3D-graphics convention (OpenGL `GL_LINES`
  style) and reuses the symbols that already shipped under `Node.PRIM_*`
  in the previous draft.
- **`FloatBuffer` / `IntBuffer` typed-buffer classes** — replaced by
  native Python `list[float]` / `list[int]`. The earlier design's
  buffer-protocol / `memoryview` integration is rarely needed at
  cube's scale, and the two extra classes inflated the public surface
  for a small subset of consumers. The binding layer copies lists into
  contiguous internal buffers at attribute assignment, so the
  rendering hot path is unaffected.

### 16.4 Scene structure

- **Scene as a Node subclass** — the previous design made `Scene` extend
  `Node` and used per-Node draw callbacks. Replaced by `Scene` as an
  immediate-mode command queue independent of the Mesh-driven Node tree.
- **`add_child` returning the child or self for chaining** — tree ops
  return `None`; chaining is not part of the cube convention.
- **`scene.find` / `node.find_by_name` / path queries** — application
  code keeps its own references; built-in lookup adds API surface
  without clear benefit at cube's scale.
- **Module-level functions in `pyxel.cube`** — the namespace stays
  classes-only for clarity.
- **`MeshPart` / `MeshNode` as a separate class for parts** — rejected
  in favor of parallel arrays (`geometries` / `transforms` / `parents`)
  on `Mesh`. Keeps the public class count down and avoids nested
  navigation (`mesh.parts[i].transform`) for what is essentially a
  flat array of per-part data.
- **Recursive `Mesh` tree** (each `Mesh` has `children: list[Mesh]`) —
  rejected as overly nested for what is functionally a parts array,
  and creates a confusing "root vs child Mesh" image-cascade story.
- **`names: list[str]` on `Mesh`** — deferred until a joint-animation
  system lands (§ 15 open item). At cube's scale, integer indices and
  application-side constants cover the immediate need.
- **`Mesh` holding `image` as an asset attribute** — the prior design
  bundled `positions / indices / image / colkey` in a single `Mesh`.
  The redesign splits geometry data (`Geometry`) from textured-model
  data (`Mesh`), so the reusable shape layer never owns texture; the
  textured-model layer (Mesh) owns its texture via `col_img`.
- **Per-part `image` override inside `Mesh`** — rejected in the initial
  design; mixed-image models split into multiple `Mesh` instances
  combined via a `Node` hierarchy. Revisit if frequent
  mixed-image-within-one-asset cases emerge.
