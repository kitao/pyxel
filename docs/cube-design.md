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
| `Camera` | View information (transform, fov, near, far, optional ortho size) plus the clear color |
| `Shading` | Color lookup table (palette × levels) plus scene-wide light direction |
| `Primitive` | Static vertex-data asset (positions / normals / uvs / indices / mode / cull); shareable across Node draws and Mesh parts |
| `Mesh` | Hierarchical 3D model asset (parallel arrays of primitives / transforms / parents) with shared col_img, colkey, GLB import, and imported motion clips |
| `Motion` | Imported transform animation clip attached to a `Mesh` |
| `Collider` | Unified collider holding shape + behavior flags + physical coefficients + motion state |
| `Contact` | Collision-pipeline payload (contact geometry + engine-resolved motion deltas) |
| `RaycastHit` | Result payload returned by `Node.raycast` / `Node.raycast_all` |
| `Node` | Scene-tree node: transform, hierarchy, immediate-mode draw commands, motion playback, lifecycle hooks, collision callbacks, and — on the root — the update / draw cycle and spatial queries |

---

## 3. Coordinate System and Conventions

- **+X right, +Y up, +Z toward viewer**, **right-handed**, forward = `-Z`.
- Aligned with Godot / pyglet / three.js / OpenGL / glTF.
- **Origin** for `node.draw(x, y, w, h)` maps the world origin to
  the center of the destination rectangle.
- **Angles in degrees** throughout the cube API.
- **Euler order**: `XYZ` extrinsic (apply X rotation first about the world
  X axis, then Y about world Y, then Z about world Z; matches three.js /
  Blender / Maya defaults).
- **Pyxel 2D screen is Y-down; Pyxel cube is Y-up.** The 2D / 3D mismatch
  follows the same pattern as Godot's "2D Y-down + 3D Y-up" and is
  intentional.
- **Per-frame units** for velocity and angular velocity. Pyxel cube is not
  a physical simulator; `velocity` is the translation applied per frame
  and `angular_velocity` is the rotation (axis × degrees) applied per
  frame. No time-based (m/s, rad/s) units anywhere in the cube API.

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

Constants are **shared immutable singletons**. Vec3 is immutable, so
accidental mutation is impossible by construction.

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

- `angle_to` returns degrees.
- `normalize` of a zero-length vector returns `Vec3.ZERO` (no exception).
- `lerp` does not clamp `t`; values outside `[0, 1]` extrapolate.
- `slerp` expects unit vectors; behavior on non-unit input is undefined.

### 4.5 Coordinate System Conversions

```python
v.to_local(mat)          # Vec3 transformed into mat's local space (point)
v.to_world(mat)          # Vec3 transformed from mat's local space to world (point)
v.to_local_dir(mat)      # like to_local but ignores translation (direction)
v.to_world_dir(mat)      # like to_world but ignores translation (direction)
```

`mat * v` (Mat4 operator) is the same as `v.to_world(mat)`. The named
methods exist because they read more naturally than `mat.inverse() * v`
for the local case and because the direction-only variants have no clean
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

### 5.2 Decomposed View

```python
m.pos                    # Vec3, read-only: translation component
m.rot                    # Quat, read-only: rotation component
m.scale                  # Vec3, read-only: per-axis scale
```

`m.rot` is exposed as `Quat` (not Euler `Vec3`) so that rotation
interpolation reads naturally as `m1.rot.slerp(m2.rot, t)`. This aligns
with Unity's `Transform.rotation` and avoids the gimbal-lock and
ambiguity issues of Euler-angle interpolation.

Behavior on non-affine matrices (e.g. perspective) is
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
hash(m)                  # value-based hash
repr(m)                  # debug representation
```

### 5.5 Class-Method Factories

```python
Mat4.from_translation(pos)              # pos: Vec3
Mat4.from_euler(euler)                  # euler: Vec3 (degrees, XYZ extrinsic)
Mat4.from_quat(rot)                     # rot: Quat
Mat4.from_scale(scale)                  # scale: Vec3
Mat4.from_axis_angle(axis, deg)         # axis: Vec3, deg: float
Mat4.compose(pos, rot, scale)           # pos: Vec3, rot: Quat, scale: Vec3
Mat4.look_at(eye, target, up=Vec3.UP)   # camera-style view matrix
```

`from_euler` (renamed from the earlier `from_rotation`) makes the
"Euler-angle" intent explicit and pairs symmetrically with
`Quat.from_euler`.

`from_axis_angle(axis, deg)` is provided directly on Mat4 to parallel
`Quat.from_axis_angle` and avoid forcing the `Mat4.from_quat(Quat.from_axis_angle(...))`
chain at every call site. A zero-length axis returns `Mat4.IDENTITY`
(no exception), matching `Quat.from_axis_angle`.

`compose(pos, rot, scale)` takes `rot: Quat` (paralleling `m.rot: Quat`).
The `T × R × S` composition equivalently spells
`Mat4.from_translation(pos) * Mat4.from_quat(rot) * Mat4.from_scale(scale)`.

`look_at(eye, target, up)` is right-handed (camera looks toward `target`,
forward = `-Z`). When `up` is parallel or anti-parallel to the forward
direction (e.g. straight-up / straight-down camera with `up=Vec3.UP`),
the function picks a fallback up axis so the resulting basis stays
non-degenerate.

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
read accessor.

### 5.7 Matrix Operations

```python
m.inverse()              # Mat4 (singular input returns Mat4.IDENTITY)
m.transpose()            # Mat4
m.determinant()          # float
```

`inverse` of a near-singular matrix (`|determinant| < 1e-12`) returns
`Mat4.IDENTITY` (no exception), matching the silent-fallback policy
used by `Vec3.normalize` and `Quat.inverse`.

### 5.8 Coordinate System Conversions

```python
m.to_local(mat)          # Mat4 expressed in mat's local space
m.to_world(mat)          # Mat4 expressed in world space (mat = local origin)
m.to_local_dir(mat)      # like to_local but translation-free
m.to_world_dir(mat)      # like to_world but translation-free
```

---

## 6. Quat

Immutable quaternion rotation. Component order is `(x, y, z, w)` with `w`
as the scalar (matches three.js / Unity / Godot; differs from the
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
hash(q)                      # value-based hash
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
Quat.from_axis_angle(axis, deg)              # axis: Vec3, deg: float
Quat.from_euler(rot)                         # rot: Vec3 (degrees, XYZ extrinsic)
Quat.from_two_vectors(a, b)                  # rotation that takes a to b
Quat.from_matrix(mat)                        # rotation extracted from mat: Mat4
Quat.from_direction(forward, up=Vec3.UP)     # rotation that looks toward forward
```

- `from_two_vectors(a, b)` returns the shortest-arc rotation that maps
  `a` to `b`; both are normalized internally.
- `from_direction(forward, up)` produces a rotation whose forward axis
  matches `forward`, with `up` resolving roll. This parallels Unity's
  `Quaternion.LookRotation` and is the common AI-facing /
  camera-tracking primitive.

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
two access points for the same conversion (one per class).

### 6.8 Interpolation

```python
q.slerp(other, t)        # Quat (spherical linear interpolation)
```

`t` outside `[0, 1]` extrapolates; callers clamp explicitly if needed.

---

## 7. Camera

View information held independently from the node tree. A camera is
assigned to a node through `Node.camera` (a cascading attribute, § 14.1)
and can be swapped between draws for multi-angle rendering.

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
| `clear_color` | `int \| None` | `None` | screen clear color applied before each `draw` that uses this camera; `None` skips the screen clear |

The single `ortho_size: float | None` attribute encodes both "is
orthographic?" and "what size?" in one place. Setting `ortho_size = N`
switches the camera to orthographic projection with vertical world size
`N`; setting `ortho_size = None` restores perspective using `fov`.

`clear_color` is the color the target screen is filled with at the start
of each `draw` that uses this camera; `None` draws over the existing
contents without clearing (useful when compositing over a 2D background
or layering several `draw` passes). The depth buffer is independent of
this setting — it is cleared at the start of every `draw` regardless.

View-control helpers like `look_at` are intentionally not on `Camera` —
building the transform through `Mat4` (e.g.
`camera.transform = Mat4.look_at(...)`) keeps animation and interpolation
flexible.

The Camera does not hold viewport information (width / height); the
viewport is a per-`draw` argument, so multiple viewports can share
the same Camera (split-screen, picture-in-picture, render-to-texture).

---

## 8. Shading

Color lookup table shared by the whole scene during a `draw` call, plus a
scene-wide directional-light vector. The lookup table is a 2D structure:
rows are palette colors, columns are 4 brightness levels.

```python
shading = Shading(colors)             # build initial LUT from a palette
shading.direction = Vec3(0, -1, 0)    # scene-wide light direction
shading[col, level]                   # tuple[int, int] — (primary, secondary)
shading[col, level] = (p, s)          # overwrite this cell
shading.build(colors)                 # rebuild the LUT from a palette
```

Each cell is a `(primary, secondary)` pair of palette indices. When the
two match, the cell renders as a flat fill of `primary`. When they
differ, the rasterizer emits a 50:50 2×2 checker dither between
`primary` and `secondary` — the eye blends the pair into a single
perceived mid-color at the 2×2-pixel scale, which lets the table express
brightness gradients that no single palette color matches exactly.

### 8.1 Attributes

| Attribute | Type | Meaning |
|---|---|---|
| `direction` | `Vec3` | scene-wide directional-light vector (the direction the light travels) |

The table itself is accessed through `__getitem__` / `__setitem__` with
`(col, level)` keys.

### 8.2 Construction and Rebuild

- `Shading(colors)` builds the initial LUT from the given palette
  (`colors: list[int]`, 24-bit RGB values). Ready to use without further
  setup.
- `Shading.build(colors)` rebuilds the LUT from the given palette. Used
  after the user changes the palette via `pyxel.colors`. Synchronization
  is manual (no automatic update).

### 8.3 Levels

The column count is fixed at 4 brightness levels:

- `lv 0` (darkest shade)
- `lv 1` (shade)
- `lv 2` (base — equals the palette color)
- `lv 3` (highlight)

The renderer maps a face's computed brightness (from light direction
dotted with the face normal, plus the base palette color) into one of
the 4 levels.

### 8.4 Build Algorithm (informative)

`Shading(colors)` and `Shading.build(colors)` synthesize one row per
palette entry by picking the best-matching dither pairs across the
palette:

- The target color for each level is the source color with its HSV value
  shifted up (highlight) or down (shade).
- For each target, the algorithm searches the palette for the closest
  candidate using a weighted HSV distance. Hue gets the heaviest weight
  (8×) so the ramp stays within the source's hue band; saturation and
  value are weighted at 1× each.
- Dither pairs are admitted only when the two constituents are
  perceptually compatible (their HSV distance stays below a single
  threshold) — otherwise the dither flickers instead of blending.
- The shade levels (lv 0 / lv 1) are picked as a coupled pair from
  three connectivity patterns (two flats; flat + dither; dither + flat)
  and the pattern with the lowest total distance wins.
- An asymmetric crossing penalty discourages chromatic-to-achromatic
  drops (a hue source picking a gray candidate) while allowing the
  reverse (gray source picking a chromatic accent).
- Per-step luminance monotonicity is enforced via sRGB-relative luminance
  so the ramp's perceived brightness order matches HSV value order.

When the palette has no usable candidates for a level (e.g. a saturated
hue with no darker neighbors), that level falls back to a flat of the
source color, so the ramp degrades gracefully on sparse palettes.

The full algorithm lives in `crates/pyxel-core/src/cube/shading.rs` and
is exercised by snapshot tests there.

### 8.5 Indexing

```python
shading[col, level]          # tuple[int, int] (primary, secondary)
shading[col, level] = pair   # overwrite the cell with a (primary, secondary) tuple
```

Out-of-range keys raise `IndexError`. Setting `primary == secondary`
collapses the cell to a flat fill (no dither).

### 8.6 Palette Substitution

Setting every level of a row to the same `(target, target)` pair makes
the source palette index render as the target index regardless of
brightness — the 3D equivalent of Pyxel 2D's `pyxel.pal(src, dst)`.
Palette substitution and shading thus share a single LUT rather than
needing two parallel tables.

---

## 9. Primitive

Static vertex-data asset. `Primitive` carries the vertex attributes
(positions / normals / uvs), the topology (indices, prim mode), and
the back-face cull mode. It is shareable across many `Node` draws and
across `Mesh` parts.

### 9.1 Class-level Constants

| Name | Value | Attribute |
|---|---|---|
| `Primitive.MODE_POINTS` | `0` | `mode` |
| `Primitive.MODE_LINES` | `1` | `mode` |
| `Primitive.MODE_TRIANGLES` | `2` | `mode` |
| `Primitive.CULL_NONE` | `0` | `cull` |
| `Primitive.CULL_BACK` | `1` | `cull` |
| `Primitive.CULL_FRONT` | `2` | `cull` |

Mode values follow OpenGL ordering. Cull values use a small `CULL_` enum
because raw `BACK` / `FRONT` would be ambiguous against other directional
constants.

### 9.2 Attributes

| Field | Type | Default | Meaning |
|---|---|---|---|
| `positions` | `list[float]` | required | flat (x, y, z) triples; `len(positions) % 3 == 0` |
| `normals` | `list[float]` | `[]` | flat (nx, ny, nz) per face (one triple per triangle); empty computes per-face normals on the fly at draw time (see § 9.4) |
| `uvs` | `list[float]` | `[]` | flat (u, v) per vertex; empty disables texture sampling |
| `indices` | `list[int]` | required | flat indices; empty consumes positions sequentially in chunks that follow `mode` |
| `mode` | `int` | required | topology (see § 9.1) |
| `cull` | `int` | `CULL_BACK` | back-face cull mode |

### 9.3 Construction and Mutation

```python
primitive = Primitive(
    Primitive.MODE_TRIANGLES,
    [0, 1, 0,  -1, -1, 0,  1, -1, 0],
    [0, 1, 2],
    uvs=[0.5, 0,  0, 1,  1, 1],
    cull=Primitive.CULL_NONE,
)
primitive.positions[0:3] = [0, 2, 0]    # in-place mutation (live view)
primitive.compute_normals()             # explicit per-face flat normals
```

`mode`, `positions`, and `indices` are required positional arguments;
`normals`, `uvs`, and `cull` are optional keywords. Python native
`list[float]` / `list[int]` are used directly.

The vertex-attribute lists are live views backed by the internal
buffer (the `Sound.notes` pattern): index and slice mutation write
through to the buffer, and replacing the whole content is spelled
`primitive.positions[:] = [...]`. The attributes themselves cannot be
reassigned.

### 9.4 Normal Computation

When `normals` is empty and the draw is shaded, the renderer computes
per-face flat normals on the fly for that draw (one `(nx, ny, nz)` per
triangle, from each triangle's world-space vertices). This recomputes
every draw — it is **not** cached back onto the attribute. For static
geometry redrawn each frame, call `compute_normals()` once to compute
and store the per-face normals on `primitive.normals`, so later draws
reuse the stored set instead of recomputing. Mutating `positions` does
not invalidate stored normals; clear them (`primitive.normals[:] = []`)
to return to on-the-fly computation, or call `compute_normals()` again
to refresh.

Smooth shading (averaging adjacent face normals at shared vertices) is
not supported; cube targets flat-shaded retro look and per-face normals
match the rasterizer's input layout.

### 9.5 Topology and `mode` / `indices` Interaction

`mode` determines how indices (or positions, when `indices` is empty)
are grouped:

- `MODE_POINTS`: 1 index per point.
- `MODE_LINES`: 2 indices per line segment.
- `MODE_TRIANGLES`: 3 indices per triangle.

A `Primitive`'s `mode` is part of the asset's identity — switching the
mode at draw time is not supported. To draw the same vertices as both
a solid mesh and a wireframe, create two separate `Primitive` instances.

### 9.6 Per-Primitive Cull Mode

`cull` is held on `Primitive` (not per draw) because it is a geometric
property: planar grass billboards need `CULL_NONE` (two-sided), solid
boxes need `CULL_BACK` (single-sided), independent of the draw
context. If a shape has mixed cull regions, split it into two `Primitive`
instances and combine them through `Mesh` parts or a `Node` hierarchy.

---

## 10. Mesh

A hierarchical 3D model asset. `Mesh` bundles multiple `Primitive`
parts (positions / topology / cull) with a shared texture or flat
color (`col_img`) and parent-child relationships between parts (held
as parallel arrays). `Node.from_mesh(mesh)` creates a `Node` tree from
the asset; each generated drawable node emits its part through the same
internal path as `Node.prim`.

### 10.1 Members

| Field | Type | Meaning |
|---|---|---|
| `primitives` | `list[Primitive \| None]` | part i's `Primitive`, or `None` for a pure group (transform-only, no draw) |
| `transforms` | `list[Mat4]` | part i's local transform in its parent's frame |
| `parents` | `list[int]` | part i's parent index; `-1` marks a root; `parents[i] < i` always |
| `names` | `list[str]` | part i's node name; imported model node names live here |
| `motions` | `list[Motion]` | transform animation clips imported with the mesh |
| `col_img` | `int \| Image` | flat color (when `int`) or shared texture (when `Image`) for all parts |
| `colkey` | `int \| None` | transparent color when `col_img` is `Image` |

### 10.2 Parallel Arrays

The four lists `primitives`, `transforms`, `parents`, and `names` are
parallel: all four index the same set of mesh parts and must have the same
length. The constructor validates the length match and raises
`ValueError` on mismatch.

### 10.3 Topological Order Constraint

`parents[i] < i` is required for every `i`. The constructor enforces
the constraint. This makes world transform computation a single forward
pass:

```python
world = [None] * len(transforms)
for i in range(len(transforms)):
    if parents[i] == -1:
        world[i] = transforms[i]
    else:
        world[i] = world[parents[i]] * transforms[i]
```

### 10.4 Construction

```python
character = Mesh(
    primitives=[prim_body, prim_hair, prim_sword, None],
    transforms=[
        Mat4.IDENTITY,
        Mat4.from_translation(Vec3(0, 1, 0)),
        Mat4.from_euler(Vec3(0, 0, 90)),
        Mat4.from_translation(Vec3(0.5, 0, 0)),
    ],
    parents=[-1, 0, 3, 0],
    names=["body", "hair", "sword", "hand"],
    col_img=pyxel.images[0],
    colkey=0,
)

character_node = Node.from_mesh(character)
```

`__init__` is all-optional. Parts can be added by reassigning the three
arrays. For load-from-file workflows (e.g., glTF import), the importer
assembles the arrays in topological order and hands them to the
constructor.

### 10.5 GLB Import

```python
mesh = Mesh.from_glb("actor.glb", colkey=0, fps=30.0)
actor = Node.from_mesh(mesh)
```

`Mesh.from_glb` loads binary glTF (`.glb`) files and converts the default
scene into the same part arrays used by the regular constructor. The first
implementation is intentionally narrow and predictable:

- Only embedded binary buffers and embedded images are supported.
- At most one image, one texture, and one material are accepted. The texture
  becomes the mesh-wide `col_img`; untextured files use the default flat
  color path.
- Texture pixels are quantized to the current Pyxel palette. Any pixel whose
  alpha is not 255 is rejected; transparent texels are represented by the
  caller-provided `colkey` value, not by alpha conversion.
- Transform animation channels for translation, rotation, and scale are
  imported into `mesh.motions`. The `fps` argument converts glTF seconds into
  Pyxel frame numbers.
- Skins, morph targets, material animation, external files, multiple
  textures, and non-triangle mesh primitives are rejected rather than guessed.

This keeps the Blockbench export path explicit: author the palette-indexed
texture normally, reserve one palette color for transparency, then pass that
palette index as `colkey` when loading the GLB.

### 10.6 Motion Clips

```python
motion = mesh.motions[0]
actor.apply_motion(motion, frame=0)
actor.play_motion(motion, loop=True, speed=1.0)
actor.stop_motion()
```

`Motion` values are immutable clips owned by the `Mesh` they were imported
with. They target mesh part indices, so a motion can be applied only to a
`Node` tree created from the same `Mesh` by `Node.from_mesh(mesh)`.

`motion.name` is the glTF animation name and `motion.length` is the clip
length in Pyxel frames. Missing channels leave the corresponding transform
component at the mesh bind pose.

### 10.7 `descendants` Helper

```python
indices: list[int] = mesh.descendants(i)
```

Returns all part indices that are transitive children of part `i`
(excluding `i` itself), in topological order. Runs in O(N) via a single
forward sweep using the topological-order invariant.

### 10.8 Shared `col_img` and `colkey`

`col_img` is shared across every part of the mesh. Mixed-texture models
split into separate `Mesh` instances combined through a `Node` hierarchy.

Internally, `Mesh` also carries a private collision BVH cache. It is
built on the first collision query that touches the mesh and is reused
thereafter; the cache is not exposed through the public surface and
does not affect equality or repr.

### 10.9 Drawing patterns

| Use case | Pattern |
|---|---|
| Reusable mesh on an actor | hold one `Mesh` and create one actor tree with `Node.from_mesh(mesh_asset)` |
| Same model, different transform per instance | one `Mesh` referenced from many generated `Node` trees |
| Dynamic mesh (per-frame deform) | mutate the deformed part's `primitive.positions[:]` per frame |
| Many small line / triangle draws | use `self.line` / `self.tri` in `on_draw` directly |
| Custom raw draw | construct a `Primitive` directly and call `self.prim(mat, primitive)` |

`Mesh` is asset-only. It is instantiated into a per-actor `Node` tree;
`Node.prim` remains the explicit low-level draw path for a single
`Primitive` without the hierarchical container.

---

## 11. Collider

Unified collider class that covers spheres, capsules, rounded boxes, and
static triangle-mesh terrain in a single shape representation. Also
carries the per-collider behavior flags, physical coefficients, and
motion state. Attached to a `Node` through `Node.collider`.

### 11.1 Shape

Three shape inputs combine to form the rounded-box family:

| Field | Type | Default | Meaning |
|---|---|---|---|
| `size` | `Vec3` | `Vec3.ZERO` | full size of the inner box (width, height, depth) |
| `radius` | `float` | `0.0` | corner / surface rounding radius (= sphere radius when size is zero) |
| `mesh` | `Mesh \| None` | `None` | static mesh collider; when set, `size` / `radius` are ignored |

- `size=Vec3.ZERO, radius=r` → sphere of radius `r`.
- `size=Vec3(0, h, 0), radius=r` → capsule of total height `h + 2r` and radius `r`.
- `size=Vec3(w, h, d), radius=r` → rounded box of inner size `(w, h, d)` and corner radius `r`.
- `mesh=Mesh(...)` → static triangle mesh terrain. Dynamic / animated mesh
  colliders are not supported (the implementation builds an internal
  AABB-tree BVH on the Mesh asset at the first collision query and
  caches it). When `mesh` is set, the collider is treated as
  `mass = 0.0` (immovable) regardless of the value the user passed, and
  collision resolution routes the full correction to the other side.

The `size` value follows the **full-size** convention (Unity / Godot 4
style), not the Bullet `halfExtents` convention. The same `Vec3(w, h, d)`
that produces a visible box via `Node.box(mat, size, ...)` produces the
collision shape of the same dimensions via `Collider(size=...)`. The
narrow phase honors the collider's rotation exactly — shapes are solved
in the body's local frame, not as world-axis-aligned boxes — and the
only rounding-radius approximation is in the mesh triangle test, whose
corner regions over-report by at most `r·(√3−1)` (§ 16 step 5).

### 11.2 Behavior Flags (opt-in)

| Field | Type | Default | Meaning |
|---|---|---|---|
| `trigger` | `bool` | `False` | `True` = notify only (no push-back). `False` = solid body |
| `rolls` | `bool` | `False` | `True` = collisions generate angular velocity. `False` = sliding only |

Both flags default `False`, reflecting Pyxel cube's opt-in posture: the
simplest default behavior (solid + no rotation) is automatic, and the
extra capabilities are enabled per-collider. Industry engines typically
default to the richer behavior and offer constraint flags; Pyxel cube
inverts the polarity to keep the no-argument case obvious.

### 11.3 Physical Coefficients

| Field | Type | Default | Meaning |
|---|---|---|---|
| `mass` | `float` | `1.0` | mass for collision share allocation. `0.0` = immovable (Bullet-style sentinel) |
| `restitution` | `float` | `0.0` | bounce coefficient (0 = absorb, 1 = elastic) |
| `friction` | `float` | `0.5` | tangential damping coefficient (0 = ice, 1 = strong drag) |

`mass = 0.0` marks the collider as **immovable**: no push-back is applied
to it, no matter what hits it. Static walls, terrain, and conveyor-style
moving platforms all use `mass = 0.0`. A platform with `mass = 0.0` plus
a non-zero `velocity` becomes a kinematic mover (engine never pushes it
back, but its velocity still propagates per frame).

Coefficients are dimensionless tuning knobs, not physically calibrated
constants. Pyxel cube is not a simulator; `restitution = 0.6` should be
read as "a moderate bounce" rather than as a measured material property.

### 11.4 Motion State

| Field | Type | Default | Meaning |
|---|---|---|---|
| `velocity` | `Vec3` | `Vec3.ZERO` | translation applied per frame |
| `angular_velocity` | `Vec3` | `Vec3.ZERO` | rotation as axis × per-frame degrees |

Both quantities are **per-frame**, not time-based. `velocity.length()`
is the distance traveled in one frame; `angular_velocity.length()` is
the rotation in degrees applied in one frame around the axis
`angular_velocity.normalize()`.

The engine reads `collider.velocity` and `collider.angular_velocity`
during the `update` pipeline and translates / rotates the owning Node's
transform accordingly (see § 16). The user sets these in `on_update` to
express movement intent and re-assigns them in `on_collide` to apply
the engine-resolved post-collision values.

### 11.5 Usage Sketch

```python
class Ball(Node):
    def __init__(self, pos):
        super().__init__()
        self.transform = Mat4.from_translation(pos)
        self.collider = Collider(
            radius=0.3, mass=1.0, rolls=True,
            restitution=0.6, friction=0.2,
        )

    def on_update(self):
        self.collider.velocity += Vec3(0, -0.02, 0)   # gravity per frame

    def on_collide(self, other, contact):
        push = Mat4.from_translation(contact.normal * contact.depth)
        spin = Mat4.from_quat(contact.delta_rotation)
        self.transform = push * self.transform * spin
        self.collider.velocity += contact.delta_velocity
        self.collider.angular_velocity += contact.delta_angular_velocity
```

---

## 12. Contact

Collision payload passed to `Node.on_collide(other, contact)`. Holds the
contact geometry (point, normal, depth) plus engine-resolved motion
deltas that the user applies to push the colliding body back into a
non-penetrating state.

| Field | Type | Meaning |
|---|---|---|
| `point` | `Vec3` | world-space contact point |
| `normal` | `Vec3` | world-space contact normal (points from `other` toward `self`) |
| `depth` | `float` | self-side push-back distance (already adjusted for the mass-share split) |
| `delta_rotation` | `Quat` | rotation correction to apply to `self.transform` (compose via `Mat4.from_quat`) |
| `delta_velocity` | `Vec3` | additive velocity correction from the collision |
| `delta_angular_velocity` | `Vec3` | additive angular velocity correction from the collision |

The current engine resolves rotational response through
`delta_angular_velocity` (the spin integrates into the transform on the
next frame, § 16). `delta_rotation` is reserved for a future immediate
orientation correction and is presently always identity, so applying it
(below) is a harmless no-op until that response is implemented.

### 12.1 Application Pattern

```python
def on_collide(self, other, contact):
    # 1) Position push-back. contact.normal is world-space, so compose
    # the world translation by left-multiplying with from_translation
    # rather than calling self.transform.translate (which is local-
    # frame, § 5.6, and would bend the push vector through the body's
    # own basis once it starts rotating).
    push = Mat4.from_translation(contact.normal * contact.depth)
    spin = Mat4.from_quat(contact.delta_rotation)
    self.transform = push * self.transform * spin
    # 2) Velocity / angular velocity updates: additive.
    self.collider.velocity += contact.delta_velocity
    self.collider.angular_velocity += contact.delta_angular_velocity
```

The decomposition is deliberate:

- **Translation is `normal * depth`**, not hidden inside a matrix. The
  user sees and writes the push-back vector directly, which keeps the
  intent legible and allows partial-response patterns (apply only along
  one axis, skip the response under specific conditions, etc.). It is
  applied as a left-multiplied `Mat4.from_translation`, i.e. a
  world-space shift; the local-frame `Mat4.translate` would produce a
  different motion for a body whose orientation drifts from identity.
- **Rotation is `delta_rotation: Quat`**, composed into a matrix with
  `Mat4.from_quat` and right-multiplied onto `self.transform`. Rotation
  has no natural "scalar × axis" form analogous to `normal * depth`, so
  the engine emits the resolved rotation as a quaternion (which also
  interpolates cleanly). Right-multiplication keeps the rotation local
  to the body's origin.
- **Velocity / angular velocity are additive**. Adding the engine-resolved
  deltas to the current values composes naturally with whatever the user
  set in `on_update` (gravity, thrust, controller input). The user's
  pre-collision motion is preserved; only the collision's contribution
  is applied on top.

### 12.2 Skipping the Response

A user may legitimately ignore the engine-resolved values (e.g., a
one-way platform that swallows the collision when the player presses
"drop down"). In that case `on_collide` simply does not apply the
deltas; the engine never overwrites `transform`, `velocity`, or
`angular_velocity` on the user's behalf.

### 12.3 Mass Share

`depth` and the `delta_*` fields are pre-divided by the engine according
to the colliders' mass ratio. A 1 kg ball hitting an immovable wall sees
`depth` equal to the full penetration depth; a 1 kg ball hitting another
1 kg ball sees `depth` equal to half the penetration on each side.

`mass = 0.0` short-circuits the share computation: the immovable side
receives `depth = 0` and zero deltas; the movable side absorbs the full
correction.

---

## 13. RaycastHit

Result payload returned by `Node.raycast` and `Node.raycast_all`.

| Field | Type | Meaning |
|---|---|---|
| `node` | `Node` | the Node owning the hit collider |
| `point` | `Vec3` | world-space hit point |
| `normal` | `Vec3` | world-space surface normal at the hit point |
| `distance` | `float` | distance from the ray origin to the hit point |

The four-field shape matches the industry minimal set (Unity
`RaycastHit`, Godot `intersect_ray` result, PhysX `PxRaycastHit`,
Bullet ray callback). Larger payloads (triangle index, barycentric
coordinates, lightmap UV) are not exposed; they are not useful at the
PS1-scale shape vocabulary cube supports.

---

## 14. Node

Base class for everything in the scene tree. A `Node` carries a transform,
hierarchy links, tags, draw / collide / lifecycle hooks, and node-local
draw commands. The scene root (§ 15) is itself a `Node`; user-defined
actors subclass `Node` and override the lifecycle hooks.

```python
class Player(Node):
    def __init__(self, pos):
        super().__init__()
        self.transform = Mat4.from_translation(pos)
        self.tags = ["player"]
        self.collider = Collider(radius=0.4, mass=1.0)

    def on_update(self):
        if pyxel.btn(pyxel.KEY_W):
            self.collider.velocity = self.forward * 0.1

    def on_draw(self):
        self.box(Mat4.IDENTITY, Vec3(1, 2, 0.5), 4)
```

### 14.1 Attributes

| Attribute | Type | Cascade | Meaning |
|---|---|---|---|
| `name` | `str` | — | debug identifier (not enforced unique) |
| `transform` | `Mat4` | composed with parent's transform during draw | local-space transform |
| `active` | `bool` | parent-dominant (False halts subtree update + collision) | enable/disable update + collision |
| `visible` | `bool` | parent-dominant (False halts subtree drawing) | enable/disable draw |
| `camera` | `Camera \| None` | None inherits from the closest non-None ancestor | active camera for this subtree's draws |
| `shading` | `Shading \| None` | None inherits from the closest non-None ancestor | shading LUT effective for this subtree |
| `collider` | `Collider \| None` | this node only | collision shape and behavior (§ 11) |
| `tags` | `list[str]` | this node only | multi-tag membership (Godot groups style); used by `find_by_tags` and spatial-query `tags` filters |

#### Read-only properties

| Property | Type | Meaning |
|---|---|---|
| `parent` | `Node \| None` | direct parent in the tree |
| `children` | `tuple[Node, ...]` | direct children |
| `destroyed` | `bool` | `True` after `destroy()` until end-of-frame detachment |
| `forward` | `Vec3` | normalized forward-axis vector derived from `transform` |
| `right` | `Vec3` | normalized right-axis vector derived from `transform` |
| `up` | `Vec3` | normalized up-axis vector derived from `transform` |
| `effective_camera` | `Camera \| None` | cascade-resolved active camera (nearest non-`None` `camera` up the tree); read inside `on_draw` |
| `world_transform` | `Mat4` | composition of all ancestor transforms (§ 14.4) |

`forward` / `right` / `up` read directly from the (local) transform's
basis columns (no per-call Mat4 decomposition); they are normalized so
that they remain unit vectors even when the transform carries
non-uniform scale. Spatial queries (§ 15.4) are `Node` methods that
search the node's own subtree, so an actor reaches scene-wide queries by
holding a reference to the scene root and calling, for example,
`root.raycast(...)`.

#### Cascade modes

- **parent-dominant**: when an ancestor's value is False, every descendant
  is treated as False regardless of its own setting. Used for `active`
  and `visible`.
- **inherits-from-ancestor**: when this node's value is `None`, the
  effective value is the closest non-`None` ancestor's value. Used for
  `shading` and `camera`. Set once on the scene root and override
  per-subtree as needed.
- **this node only**: no propagation. Used for `collider` and `tags`.

### 14.2 Tree Operations

```python
parent.add_child(child)             # also unlinks child from any prior parent
parent.remove_child(child)
node.destroy()                      # detach and remove from the tree
```

`add_child` implicitly removes the child from its previous parent.
`destroy` removes the node from its parent and triggers `on_destroy`.
Destruction is deferred to the end of the current `update` step
to avoid mid-traversal mutation hazards.

### 14.3 Lookup

```python
node.find_by_name(name)             # list[Node] — subtree DFS, all matches
node.find_by_tags(tags)             # list[Node] — subtree DFS, matches any of `tags`
```

Both methods perform a depth-first pre-order traversal starting at
`self`, returning every match (or `[]` if none). `find_by_tags` takes a
list of strings; a Node matches if it carries any of the listed tags.

The list-returning shape is symmetric across both lookups: even
`find_by_name` returns a list, because Pyxel cube does not enforce name
uniqueness (e.g. "zako" enemies spawn under the same name). Callers
that want a single result write `result[0]` or `result[0] if result
else None`.

### 14.4 World Transform

```python
node.world_transform                # Mat4 property — composition of all ancestor transforms
```

Computed on demand by walking up the tree. Cube does not cache the world
transform; users that hit this in a hot path compute and reuse the value
within a single frame.

### 14.5 Immediate-Mode Draw Commands

Inside `on_draw`, the node draws into the effective camera (§ 14.1) and
the target screen. Coordinates are node-local (the engine composes
parent transforms during draw). A draw command takes its geometry, its
color or texture, and the few options intrinsic to it (`col_img`,
`colkey`, `angle`, `font`). Cross-cutting render state (shading, dither,
depth) is not passed per call; it is set through state-setter methods
that reset to their defaults at the start of every node's `on_draw`
(see below).

```python
# Vec3 vertex-list primitives
self.pset(pos, col)
self.line(p1, p2, col)
self.tri(p1, p2, p3, col)
self.trib(p1, p2, p3, col)

# Screen-aligned 1-point shapes (always face the camera)
self.circ(pos, r, col)
self.circb(pos, r, col)

# Mat4-positioned plane shapes
self.rect(mat, w, h, col)
self.rectb(mat, w, h, col)
self.elli(mat, w, h, col)
self.ellib(mat, w, h, col)

# 3D solids (box is Mat4-positioned, sphere is Vec3-positioned)
self.box(mat, size, col_img=7, *, colkey=None)
self.boxb(mat, size, col)
self.sphere(pos, r, col_img=7, *, colkey=None)
self.sphereb(pos, r, col)

# Image quads
self.sprite(pos, img, uvs, w, h, *, colkey=None, angle=0.0)   # always camera-facing
self.plane(mat, img, uvs, w, h, *, colkey=None)               # free orientation

# Generic primitive draw (low-level, takes a Primitive; see § 9)
self.prim(mat, primitive, col_img=7, *, colkey=None)

# Text (Vec3 anchor, screen-space glyphs; always camera-facing)
self.text(pos, s, col, *, font=None)
```

#### Positioning conventions

- **Vec3-positioned** (`pos`, `p1`, `p2`, `p3`): used by vertex-specified
  primitives, screen-aligned shapes, `sprite`, `text`, and `sphere`.
- **Mat4-positioned** (`mat`): used by primitives that need full
  orientation — plane shapes, 3D solids with a directional axis,
  `plane` and `prim`.

#### Per-`on_draw` render state

Cross-cutting render state is set with state-setter methods rather than
per-call arguments. A setter takes effect for the draws that follow it
in the same `on_draw`, and **every node's `on_draw` begins with the
state reset to its defaults**, so one node cannot leak render state into
another (the tree is walked parent-first; see § 16).

```python
self.shaded(on)             # bool       — directional shading through the effective Shading
self.dither(alpha)          # float 0..1 — Bayer-dither pseudo-alpha
self.depth_test(on)         # bool       — depth-buffer comparison
self.depth_write(on)        # bool       — depth-buffer writes
self.depth_offset(offset)   # float      — world-unit depth bias
```

| Setter | Type | Default | Effect |
|---|---|---|---|
| `shaded` | `bool` | `True` | apply directional shading through the effective `Shading` |
| `dither` | `float` (0..1) | `1.0` | Bayer-dither pseudo-alpha (`1.0` opaque, `0.0` fully transparent) |
| `depth_test` | `bool` | `True` | enable depth-buffer comparison |
| `depth_write` | `bool` | `True` | enable depth-buffer writes |
| `depth_offset` | `float` | `0.0` | world-unit depth bias; negative toward the camera, positive away |

- `shaded` has no effect on commands without a surface normal — lines,
  points, outlines, screen-aligned circles, and `text` — and `sprite`
  is drawn unshaded regardless (decoration / particle use is the
  majority).

`depth_offset` biases only the depth test / write — as if the draw moved
`offset` world units along the camera's view direction — without changing
its screen position or size (it is a depth bias, not a translation).
Negative draws toward the camera so it wins against coplanar or
overlapping geometry; positive pushes it away. Use it to layer overlays
(labels, decals, outlines on a surface) without z-fighting. The sign
matches the depth-bias convention of `glPolygonOffset` / Direct3D / Unity
(negative = toward the camera).

The remaining options are per-call arguments, intrinsic to specific
commands:

| Argument | Type | Default | Commands | Meaning |
|---|---|---|---|---|
| `col_img` | `int \| Image` | `7` | `box`, `sphere`, `prim` | flat color when `int`, texture when `Image` |
| `colkey` | `int \| None` | `None` | `box` / `sphere` / `plane` / `sprite` / `prim` | transparent palette index for textures |
| `angle` | `float` | `0.0` | `sprite` only | screen-space rotation in degrees |
| `font` | `Font \| None` | `None` | `text` only | overrides default font |

#### Shape conventions

- **Center pivot**: every shape is centered at its `pos` / `mat.pos`.
- **`circ` / `circb`**: always face the camera; radius `r` is in world
  units; border is 1 pixel.
- **`line`**: world-positioned, fixed 1-pixel width.
- **`sphere` / `sphereb`**: level-1 subdivided icosahedron (42 vertices
  / 80 triangles) scaled by `r`. Internally backed by a cached static
  buffer and routed through `prim`.
- **`box` / `boxb`**: `size` is `Vec3(width, height, depth)`. Internally
  backed by a cached static unit-cube buffer.
- **`text`**: 3D anchor + screen-space glyphs. `pos` is projected to
  screen, then characters render in 2D pixels at the font's native size.
  Always camera-facing. `depth_test` / `depth_write` apply at `pos`'s
  screen-z.
- **`sprite`**: billboard quad always facing the camera. `angle`
  rotates the quad in screen space, in degrees.
- **`plane`**: free-oriented quad. `mat` carries position, rotation,
  and scale; `(w, h)` is the quad's local width and height.
- **`prim`**: low-level entry that most higher-level commands route
  through (`circ` / `circb` / `text` take dedicated screen-space paths).
  Takes a single `Primitive` argument. `col_img` accepts `int | Image`
  (integer = flat color, Image = textured triangles); `colkey` is the
  transparent palette index when `col_img` is an Image.

#### Texture and UV layout

`sprite` and `plane` take `img: Image` (integer image bank indices are
not accepted; use `pyxel.images[i]` if needed). `Tilemap` is excluded.

`uvs` is a 4-vertex UV tuple in row-major order:

```python
uvs: tuple[
    tuple[float, float],   # vertex 0 — top-left
    tuple[float, float],   # vertex 1 — top-right
    tuple[float, float],   # vertex 2 — bottom-left
    tuple[float, float],   # vertex 3 — bottom-right
]
```

The 4-corner form is the software rasterizer's natural input and lets
the caller express flips, 90° rotations, and arbitrary trapezoidal
mapping in one parameter.

### 14.6 Motion Playback

```python
root = Node.from_mesh(mesh)
root.apply_motion(mesh.motions[0], frame=12)
root.play_motion(mesh.motions[0], loop=True, speed=1.0, start_frame=0.0)
root.stop_motion()
```

`apply_motion` samples a clip immediately and writes the sampled local
transforms into the matching generated mesh-part nodes. `play_motion`
stores a playback cursor on the node; every `update()` advances that cursor
after user `on_update` hooks and before collider motion integration. The
stored player belongs to the subtree root it was called on, and
`stop_motion` clears it.

Motion playback is intentionally transform-only. It does not deform vertex
buffers, update mesh-collider BVHs, or apply material animation.

### 14.7 Lifecycle Hooks

Subclasses override these hooks to define behavior. Defaults are no-ops.

```python
def on_update(self): ...                       # called once per scene update
def on_draw(self): ...                         # called once per scene draw
def on_collide(self, other, contact): ...      # invoked when this Node collides with `other`
def on_destroy(self): ...                      # called when destroy() runs
```

- `on_update`: business logic per frame. The driver visits the tree
  pre-order; subtrees with `active = False` are skipped. The typical
  pattern is to write the user's movement intent into
  `self.collider.velocity` and `self.collider.angular_velocity`; the
  engine then integrates them into `self.transform` between
  `on_update` and collision detection (see § 16).
- `on_draw`: drawing calls (immediate-mode + `self.mesh`). The driver
  visits subtrees with `visible = True` and runs each node's `on_draw`
  with draw state reset to defaults at entry.
- `on_collide`: invoked once per contact for each side (`a` then `b`).
  `other` is the colliding Node; `contact` is the engine-resolved
  payload (§ 12). The user typically applies push-back and motion
  deltas in this hook.
- `on_destroy`: cleanup hook. Called once just before the node leaves
  the tree.

---

## 15. Scene Root and Frame Loop

There is no dedicated `Scene` class — the root of a scene tree is an
ordinary `Node`. `Node` itself carries the per-frame `update` / `draw`
cycle and the spatial-query methods (raycast, overlap); the clear color
lives on `Camera`. The application builds a root node, gives it a
`camera` and a `shading`, adds actor `Node` subtrees as children, and
calls `update` and `draw` from Pyxel's update / draw callbacks.

The convention is to subclass `Node` for the root (often named `Scene`)
so scene-wide setup lives in its `__init__`:

```python
class Scene(Node):
    def __init__(self):
        super().__init__()
        self.camera = Camera()
        self.camera.clear_color = 0
        self.shading = Shading(pyxel.colors.to_list())
        self.add_child(Player(Vec3(0, 0, 0)))


class App:
    def __init__(self):
        pyxel.init(256, 192)
        self.scene = Scene()
        pyxel.run(self.update, self.draw)

    def update(self):
        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, 256, 192)
```

### 15.1 The Root Node

The root node is a `Node` like any other (§ 14); nothing distinguishes
it structurally. `camera` and `shading` are cascading attributes, so the
convention is to set them once on the root for a scene-wide view and
lighting, and override them per-subtree where a region needs a different
camera or shading.

### 15.2 Clear Color

The screen clear color is a `Camera` attribute (`clear_color`, § 7), not
a root-node attribute — each camera clears with its own color (or skips
the screen clear when `None`); the depth buffer is cleared every `draw`
regardless. The root node has no scene-specific attributes of its own
beyond the `Node` surface.

### 15.3 Driver Methods

```python
node.update()
node.draw(x, y, w, h, target=None)
```

- **`update()`**: one frame of the game loop over this node's subtree.
  Steps detailed in § 16.
- **`draw(x, y, w, h, target=None)`**: rasterizes this node's subtree
  into the destination rectangle `(x, y, w, h)` using the node's
  effective camera (§ 14.1). Raises if no `camera` is set on the node
  or any ancestor.
  - `target=None` (default): draw to `pyxel.screen`.
  - `target=Image`: draw to a custom image (render-to-texture for
    minimap, multi-pass effects, off-screen rendering).

### 15.4 Spatial Queries

Pyxel cube exposes four spatial-query primitives on `Node`, mirroring
Unity Physics's `Raycast`, `RaycastAll`, `OverlapSphere`, and
`OverlapBox`. Each operates against the colliders in the subtree rooted
at the node it is called on — call it on the scene root to query the
whole scene.

```python
node.raycast(
    origin, direction,
    max_distance=None,
    hit_triggers=False,
    tags=None,
) -> RaycastHit | None

node.raycast_all(
    origin, direction,
    max_distance=None,
    hit_triggers=False,
    tags=None,
) -> list[RaycastHit]

node.overlap_sphere(
    center, radius,
    hit_triggers=False,
    tags=None,
) -> list[Node]

node.overlap_box(
    mat, size,
    hit_triggers=False,
    tags=None,
) -> list[Node]
```

#### Common parameters

- `hit_triggers: bool`: when `False` (default), trigger colliders
  (`collider.trigger == True`) are skipped. When `True`, triggers are
  included in the result. This is independent of `tags` filtering;
  the trigger flag is a property of the collider's physical role
  (solid body vs notify-only), not its game-logic category.
- `tags: list[str] | None`: when `None`, every Node is eligible. When
  set, a Node matches only if at least one of its `tags` appears in the
  filter list (OR semantics, same as `Node.find_by_tags`).

#### `raycast` vs `raycast_all`

- `raycast(...)` returns the **nearest** hit along the ray (or `None`).
- `raycast_all(...)` returns **all** hits along the ray, sorted by
  distance ascending. Use for piercing projectiles, "all enemies the
  laser passes through", etc.

#### `overlap_sphere` / `overlap_box`

Return every Node whose collider overlaps the query volume.

- `overlap_sphere(center, radius, ...)`: spherical query.
- `overlap_box(mat, size, ...)`: oriented-box query. `mat` positions
  and rotates the box; `size` is the full extent (same convention as
  `Collider.size` and `Node.box`).

The result list ordering is implementation-defined; do not rely on a
specific traversal order.

### 15.5 Self-exclusion

The spatial-query API does not include an `ignore` parameter. Spatial
queries follow the same silent-fallback policy as the math primitives:
a query whose origin / probe volume overlaps the caller's own collider
returns a hit on that collider rather than silently dropping it.
Excluding self is a one-line post-filter at the call site (`root` is the
scene-root node the actor holds a reference to):

```python
# Skip self in the overlap list.
nearby = [n for n in root.overlap_sphere(self.transform.pos, 3.0) if n is not self]
# Skip self in raycast hits (origin inside the caller's collider would
# otherwise produce a zero-distance self-hit).
hits = [h for h in root.raycast_all(origin, direction) if h.node is not self]
```

The earlier `ignore: Node | list[Node] | None` parameter was considered
but rejected; `tags`-based filtering covers most of its use cases, and
the post-filter idiom above covers the rest with one line.

### 15.6 Multi-angle Rendering

Build the scene tree once, then call `draw` as many times per frame as
needed into different rectangles / target screens. The camera comes from
the node's `camera` attribute, so swap it between calls (or assign
different cameras to different subtrees). The same scene state drives
every `draw` call within a frame.

```python
def draw(self):
    self.scene.camera = self.main_camera
    self.scene.draw(0, 0, 256, 192)
    self.scene.camera = self.minimap_camera
    self.scene.draw(0, 144, 64, 48)
```

---

## 16. The update Pipeline

The per-frame `Node.update()` runs the following pipeline over the
node's subtree. Each phase operates on the entire tree before the next
phase begins.

1. **User update**: depth-first pre-order traversal of the tree,
   calling each active Node's `on_update`. Subtrees rooted at a node
   with `active = False` are skipped entirely. Users typically set
   `collider.velocity` / `collider.angular_velocity` here to express
   movement intent.

2. **Node motion playback**: active `play_motion` cursors sample their
   `Motion` clips and write local transforms to the generated mesh-part
   nodes in the subtree. This happens after user code has had a chance to
   start / stop / replace playback in `on_update`, and before collider
   velocities are integrated.

3. **Motion integration**: for each Node that owns a `Collider`, the
   engine applies the collider's `velocity` and `angular_velocity` to
   the Node's `transform`. Translation is world-space (left-multiply)
   so the body moves along world axes regardless of its current
   orientation; rotation is local (right-multiply) so spin updates the
   body's orientation without orbiting it around the world origin:

   ```python
   t_vel = Mat4.from_translation(collider.velocity)
   r_avel = Mat4.from_axis_angle(
       collider.angular_velocity.normalize(),
       collider.angular_velocity.length(),
   )
   node.transform = t_vel * node.transform * r_avel
   ```

4. **AABB refresh**: each `Collider`'s axis-aligned bounding box is
   recomputed from the current transform.

5. **Broad phase**: candidate pairs are enumerated by AABB overlap.
   The structure is implementation-defined; the v1 implementation uses
   an `O(N²)` AABB-overlap sweep, which is in budget at PS1 scale
   (~100 movable bodies). Mesh colliders carry a lazily-built internal
   BVH that the narrow phase queries with the dynamic body's mesh-local
   AABB.

6. **Narrow phase**: each candidate pair is tested for actual collision.
   Shapes are classified per § 11.1 (sphere / capsule / rounded box /
   mesh); every sphere / capsule / rounded-box pairing (all six
   combinations) is supported, plus each of the three against a static
   mesh. Pairs are solved shape-exactly in the box side's body frame
   (or on capsule segments), so collider rotation is honored — the
   world AABB stays a broad-phase-only construct. Mesh-vs-mesh is
   unsupported (both sides are static and need no resolution payload)
   and is silently skipped. Two bounded approximations remain: the
   rounding radius enters the box-vs-triangle SAT as a uniform axis
   extension (exact on faces and edges, over-reporting by at most
   `r·(√3−1)` in corner regions), and the capsule-vs-box closest point
   uses alternating projection between the two convex sets (converges
   well within its fixed iteration budget). The result is a stream
   of `Contact` records with `point`, `normal`, and `depth` filled in.

7. **Response resolution**: the engine computes the mass-share split
   and writes the per-side push-back, rotation, and motion deltas into
   each `Contact`. For trigger colliders, the deltas are zero (the user
   sees the contact geometry but no motion correction).

8. **Notification**: for each pair `(a, b)`, the engine invokes
   `a.on_collide(b, contact_a)` and `b.on_collide(a, contact_b)`. The
   call order across pairs is deterministic (the tree's pre-order),
   but applies independently to `a` and `b` — neither sees the other's
   updates within the same pair.

9. **Deferred destruction**: Nodes whose `destroy()` was called during
   the frame are removed from the tree, and `on_destroy` is invoked on
   each. Removal is deferred to the end of the step so that
   mid-traversal `destroy()` calls do not invalidate the iteration.
   `destroy()` itself only sets a `_destroyed` flag on the node and
   propagates it to every descendant; parent / child links survive the
   rest of the frame so traversals stay safe. The pipeline walks the
   tree post-order, fires `on_destroy` (leaf first, root last, matching
   the Unity / Godot convention), then detaches each flagged node.
   `Node.destroyed: bool` exposes the flag read-only so user hooks can
   early-return from `on_update` / `on_collide` after a `destroy()`
   within the same frame. Because `update()` runs before `draw()`, a
   node destroyed in `update()` is already detached when the next
   `draw()` walks the tree, so its `on_draw` does not fire.

The pipeline is single-pass per frame: there is no iterative
constraint solver. PS1-scale games (a hundred or so dynamic bodies)
need at most a single resolution pass per frame; more stable stacks
or constraint chains are out of scope.

---

## 17. Performance Notes

- A typical pixel-art game has tens to a few hundred drawables per
  frame. At that scale, immediate-mode drawing has comparable cost to
  Pyxel 2D's existing per-call overhead.
- Multi-angle rendering calls `draw` again with a different camera; each
  call re-runs the subtree's `on_draw` traversal and rasterizes afresh
  (cube is immediate-mode, with no retained draw-command cache between
  calls). At PS1 scale a few views per frame stay in budget.
- `Mesh` is loaded once; `Node` trees built from it carry per-instance
  poses without copying mesh data.
- `Vec3` / `Mat4` / `Quat` are immutable; their constants are shared
  singletons. Arithmetic methods return fresh instances. The
  implementation is expected to keep allocation cheap for hot-path math
  operations.
- The collision pipeline targets ~100 dynamic bodies plus a thousand or
  so static mesh triangles at 60 fps on Raspberry Pi 4 / 5. Heavier
  scenes are out of scope.

---

## 18. Naming Policy

Pyxel cube's public surface mixes conventions from several 3D engines.
The mix is intentional, not accidental, and follows the rules below.

1. **Math primitives** (`Vec3`, `Mat4`, `Quat`): the industry-common
   minimal set, with Python `snake_case` spelling. Method names align
   with Godot / pyrr / pygame (`length_squared`, `distance_squared_to`,
   `normalize`) rather than Unity's `magnitude` / `sqrMagnitude`.
2. **Form**: Python `snake_case` everywhere; no `is_` prefix on
   booleans (`active`, `visible`, `trigger`, `rolls`); no camelCase.
3. **Physics** (`Collider`, `Contact`): Bullet conventions — `mass = 0`
   as the immovable sentinel, `restitution` / `friction` as the
   coefficient names — combined with Unity-style attribute names
   (`velocity`, `angular_velocity`) for the motion state.
4. **Scene graph** (`Node`): Unity-style direction accessors
   (`forward` / `right` / `up`) on Node, Godot-style multi-membership
   tags (`tags: list[str]`), and Unity-style lookup names
   (`find_by_name`, `find_by_tags`).
5. **Spatial queries** (`Scene.raycast` / `Scene.overlap_*`): Unity
   Physics conventions, with shared parameter names (`origin`,
   `direction`, `max_distance`, `hit_triggers`, `tags`) across all
   four primitives.
6. **Opt-in flags**: Pyxel cube's behavior flags (`trigger`, `rolls`)
   default `False`, so the "do the simplest thing" case takes no
   arguments. Industry engines typically default to the richer
   behavior; cube inverts the polarity to keep the no-argument case
   obvious.
7. **Per-frame units**: `velocity` / `angular_velocity` are per-frame,
   not time-based. This is the Pyxel-2D-style "one tick at a time"
   integration; users that want a physical simulator integrate Bullet
   or Rapier themselves.

The mix is documented here, not inside the .pyi, so that readers
coming from a specific engine know which conventions they will
recognize and which they will need to translate.

---

## 19. Open Items

- **Skeletal / blended animation system**: `Motion` currently samples
  imported transform channels on `Node.from_mesh` trees. Skinning,
  joint constraints, inverse kinematics, and animation blending remain
  deferred until a real game needs them.
- **Camera world ↔ screen helpers**: `Camera.world_to_screen(pos, ...)` /
  `screen_to_ray(...)` would help HUD coordinates and mouse picking.
  Deferred because viewport size is not held by `Camera` in cube (it
  is a per-`draw` argument), and the current need has not yet
  surfaced.
- **Extended topology modes**: `MODE_LINE_STRIP`, `MODE_LINE_LOOP`,
  `MODE_TRIANGLE_STRIP`, `MODE_TRIANGLE_FAN` along OpenGL's primitive
  numbering, for compact ribbon / fan / strip emissions through `prim`.
  Defer until a real-game use case surfaces.
- **World-scaled text variant**: an optional `text` mode that lays
  glyphs as 3D-space quads transformed by `mat` (so rotation / scale
  affect glyph layout). The current `text(pos, ...)` is screen-space
  glyphs at the projected point; users that need 3D-text shapes can
  build them through `prim()`. Revisit if a real use case surfaces.
- **Lifecycle hook on attach**: `on_attach` / `on_ready` for scene-tree
  insertion was considered but rejected; `__init__` covers the typical
  setup needs and the additional hook adds learning surface for
  marginal benefit. Revisit if real-game patterns need it.
- **Mat4 / Quat additional interpolation**: `Quat.lerp` and `Mat4.lerp`
  are not provided. Users who need them compose
  `q.slerp(...)` / `Mat4.compose(pos.lerp(...), rot.slerp(...), scale.lerp(...))`
  at the call site. Revisit if a clean use case emerges that the
  composition does not cover.

---

## 20. Decisions Explicitly Ruled Out

These were considered and rejected during design; revisit only with new
evidence.

### 20.1 Math classes

- **`Vec2` / `Vec4` / `Mat3`** — not exposed publicly. Cube draws use
  `Vec3` for points and `Mat4` for full transforms.
- **Component-wise `Vec3 * Vec3`** — out, ambiguous in 3D math
  vocabulary. Use `Mat4.from_scale(other) * v` for per-axis scaling.
- **Swizzle properties** (`v.xy`, `v.yzx`, etc.) — numpy / pyrr-style
  swizzle is left to user code.
- **Mutable `Vec3` / `Mat4` / `Quat`** — cube standardizes on immutable.
- **`Mat4.perspective` / `Mat4.orthographic` / `Mat4.frustum`** —
  projection lives in `Camera`, not in `Mat4`.
- **`Mat4.is_identity()` / `Quat.is_normalized()`** — `mat ==
  Mat4.IDENTITY` and explicit length checks cover these.
- **`Vec3.direction_to(other)`** — Godot-only; `(b - a).normalize()`
  is short enough not to warrant a dedicated method.
- **`Vec3.bounce(normal)` / `Vec3.slide(normal)`** — Godot-only.
- **`Vec3.refract(normal, eta)`** — optical refraction; not needed for
  the cube software renderer's lighting model.
- **`Mat4.orthonormal()`** — drift correction for accumulated
  rotations. In cube's typical usage, numerical drift is negligible.
- **`Mat4.translate` / `Mat4.scale_by` scalar overloads** — replaced
  by `Vec3`-only signatures to match three.js / Godot / pyrr / Unity.
- **`Quat.__add__` / `Quat.__sub__` / `Quat.__truediv__`** —
  quaternion linear combinations have limited use; `slerp` covers
  what's needed.
- **`Quat.lerp` / `Mat4.lerp`** — `slerp` is the standard quaternion
  interpolation; matrix interpolation composes from
  `pos.lerp`, `rot.slerp`, and `scale.lerp` at the call site.
- **`Quat.from_x_rotation` / `from_y_rotation` / `from_z_rotation`** —
  subsumed by `Quat.from_axis_angle` and `Quat.from_euler`.

### 20.2 Naming

- **`Vec` / `Mat` / `Quaternion`** — too generic or too long.
- **`Vector3` / `Matrix4`** — standard but verbose; `Vec3` / `Mat4`
  chosen for brevity.
- **`length_sq` / `distance_sq_to`** — short forms dropped in favor of
  `length_squared` / `distance_squared_to`, matching pygame / Godot.
- **`max_len` argument name** — replaced with `max_length`.
- **`(sx, sy, sz)` argument names on scale** — irrelevant after
  dropping the scalar overload.
- **Past-tense method names (`normalized`, `inverted`)** — Python
  idiom uses present tense (`normalize`, `inverse`).
- **`rotate_axis` / `rotate_arbitrary`** — `Mat4.rotate(axis, deg)` is
  short and unambiguous.
- **`is_trigger` / `lock_rotation` (with `is_` prefix)** — Pyxel cube
  bools use no `is_` prefix, paralleling `active` / `visible`. The
  flags are `trigger` and `rolls`.

### 20.3 Camera and lighting

- **`Camera.screen_to_ray(...)` / `Camera.world_to_screen(...)`** —
  the viewport is not held on `Camera` (it is a per-`draw` argument),
  so screen-to-world helpers need the viewport passed in explicitly.
  Deferred until the user need surfaces; meanwhile the math is
  straightforward to implement at the call site for the rare picking
  cases.
- **`Light` as a separate class** — earlier drafts split lighting
  parameters (`ambient`, `direction`, `intensity`) into a `Light`
  class with `Node.light` cascade and `Node.shade_ramp` for the LUT.
  Collapsed into a single `Shading` class holding both the color LUT
  and the scene-wide `direction`. `ambient` / `intensity` did not
  survive the collapse — the toon-style flat-shading model the
  renderer targets gains very little from ambient and intensity knobs
  on top of the per-cell brightness control already exposed through
  the LUT.

### 20.4 Drawing

- **Retained-mode scene graph with registered per-Node draw
  primitives** — replaced by per-`on_draw` immediate-mode draw commands.
- **`Primitive` class** (multi-shape aggregate registered into a node).
- **Specialized Node subclasses** (`SpriteNode`, `LineNode`,
  `MeshNode`, `TextNode`, etc.). One Node class is enough; per-shape
  behavior is in the `Node` draw commands.
- **`Shader` class** — replaced by `Shading` (LUT + direction).
- **`scene.push_matrix` / `scene.pop_matrix`** — draw commands accept
  their own `mat` directly.
- **`int` for `img` parameter on `sprite` / `plane`** — `Image` only.
- **`Tilemap` on `sprite` / `plane`** — tile-grid storage does not
  match a UV-based texture sampler.
- **`(u, v, sw, sh)` source-rectangle form on textured commands** —
  replaced by a 4-vertex `uvs` tuple.
- **Scalar `(u1, v1, u2, v2, u3, v3, u4, v4)` for UVs** — too many
  positional arguments.
- **`fill` draw op on `Node`** — no clean 3D meaning.
- **Standalone `pal` operation / `Node.pal` draw state** — replaced by
  setting `Shading` rows uniformly.
- **Per-call keyword-argument modifiers** (`shaded=`, `dither_alpha=`,
  `depth_test=`, … on every draw command) **and cascading draw-state
  properties** — replaced by per-`on_draw` state-setter methods
  (`shaded` / `dither` / `depth_test` / `depth_write` / `depth_offset`)
  that reset to defaults at each node's `on_draw` entry.
- **A standalone `alpha` modifier name** — the pseudo-alpha control is
  the `dither(alpha)` setter, named to flag that the implementation is
  Bayer dithering rather than true alpha blending.
- **`draw_text` / `draw_image` / `make_*` prefixes** — cube uses bare
  verbs.
- **GPU-oriented features** (flat 16-element `to_list` / `from_list`
  on Mat4, OpenGL handles, shader programs) — cube is
  software-rendered.
- **`col_tex` argument name for asset draws** — `tex` lacks language
  fit without a `Texture` class.
- **`col_image` argument name** — `col_img` is internally consistent.
- **`prim` as the topology attribute name on `Primitive`** — `mode`
  pairs with the `MODE_*` constant prefixes and avoids overloading the
  `Node.prim` command name.
- **`Primitive.MODE_LINES` / `DRAW_LINES` constant prefixes** —
  `MODE_LINES` matches the OpenGL `GL_LINES` style.
- **`FloatBuffer` / `IntBuffer` typed-buffer classes** — replaced by
  native Python `list[float]` / `list[int]`.

### 20.5 Scene structure and lookup

- **A distinct `Scene` class** — the scene root is a plain `Node`; the
  frame loop (`update` / `draw`) and the spatial queries are `Node`
  methods, so no separate `Scene` type is exposed. Apps conventionally
  subclass `Node` (often named `Scene`) for the root.
- **`add_child` returning the child or self for chaining** — tree ops
  return `None`.
- **Module-level functions in `pyxel.cube`** — the namespace stays
  classes-only.
- **`MeshPart` / `MeshNode` as a separate class for parts** — rejected
  in favor of parallel arrays on `Mesh`.
- **Recursive `Mesh` tree** — rejected.
- **`Mesh` holding `image` as an asset attribute (pre-redesign)** —
  split into `Primitive` (shape) + `Mesh.col_img` (texture).
- **Per-part `image` override inside `Mesh`** — mixed-image models
  split into multiple `Mesh` instances combined via a `Node`
  hierarchy.

### 20.6 Collision and physics

- **`Collider` / `Rigidbody` split (Unity-style)** — Pyxel cube
  collapses shape + body + flags + motion state into a single
  `Collider` class. The split was rejected because it doubles the
  number of objects users have to wire up for every interactive Node,
  with no clear benefit at PS1 scale.
- **Layer / mask bitfield filtering** — replaced by `tags` (list of
  strings) and post-hit filtering. Bitfield layers add a fixed-budget
  numeric namespace that does not pay off in cube-scale games; tags
  scale naturally and remain readable.
- **`ignore: Node | list[Node]` parameter on spatial queries** —
  considered and rejected; `tags`-based filtering plus a one-line
  post-filter (`[n for n in ... if n is not self]`) cover the use
  cases without adding a parameter every caller has to thread through.
- **`screen_to_ray` on `Camera`** — see § 20.3.
- **Engine-driven `transform` write-back** — earlier drafts had the
  engine overwrite the user's `transform` after collision. Rejected
  because it conflicts with override patterns (one-way platforms,
  user-driven teleport on contact). The engine now writes the
  resolution into `contact.delta_rotation` / `delta_velocity` /
  `delta_angular_velocity`; the user applies them in `on_collide`.
- **Iterative constraint solver / sequential impulse loop** —
  out of scope. PS1-scale games run a single resolution pass per
  frame; constraint chains and stable stacks are not targeted.
- **`Collider` body-type enum** (`STATIC` / `KINEMATIC` / `DYNAMIC`)
  — Pyxel cube uses Bullet-style `mass = 0` as the immovable sentinel
  instead of a separate enum, keeping the field count down.
- **`Rigidbody.useGravity`-style flag** — Pyxel cube has no
  scene-level gravity; users add `Vec3(0, -g, 0)` to
  `collider.velocity` in `on_update` themselves. Keeps the per-frame
  integration explicit and aligns with the "one tick at a time"
  Pyxel-2D mindset.
- **`Vec3.direction_to(other)`** — see § 20.1; the spatial-query API
  does not gain from a shorthand.
