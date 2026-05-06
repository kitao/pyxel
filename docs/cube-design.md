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

## 2. Public Classes (10)

| Class | Role |
|---|---|
| `Vec3` | Immutable 3D vector |
| `Mat4` | Immutable 4×4 matrix (transforms; projection lives in `Camera`) |
| `Quat` | Immutable quaternion rotation |
| `Camera` | View information (transform, fov, near, far, optional ortho size) |
| `ColorRamp` | Color LUT (palette × 16 brightness levels) |
| `Light` | Flat-shading parameters (ambient, direction, intensity) |
| `Mesh` | Single-geometry asset (vertices / faces / UVs / col / image); mutable, primitive factories + dynamic build |
| `Model` | Hierarchy template asset (multiple Mesh refs + parent / child + relative transforms + part names); load-only |
| `Node` | Hierarchy instance with transform, immediate-mode draw commands, and lifecycle hooks; references `Mesh` during draw |
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
m.to_local(other)        # Mat4 expressed in other's local space
m.to_world(other)        # Mat4 expressed in world space (other = local origin)
m.to_local_dir(other)    # like to_local but translation-free
m.to_world_dir(other)    # like to_world but translation-free
```

Same semantics as `Vec3.to_local` etc., but operating on a full transform.

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

## 8. ColorRamp

Color LUT shared by the whole scene during a `render` call. The table is a
2D structure: rows are palette colors, columns are 16 brightness levels.

```python
ramp = ColorRamp()                        # default ramp built from current palette
ramp[col, level]                     # int — sampled color at this cell
ramp[col, level] = value             # int — overwrite this cell
ramp.build()                         # rebuild from current pyxel palette
```

- `ColorRamp()` initializes with a default ramp derived from the current Pyxel
  palette via the same algorithm as `build()`. Ready to use without
  further setup.
- `build()` rebuilds the ramp from the current Pyxel palette, used after
  the user changes the palette via `pyxel.colors`. Synchronization is
  manual (no automatic update).
- `__getitem__` / `__setitem__` use `(col, level)` keys, parallel with
  Mat4's `(row, col)` indexing style.

Out of range keys raise `IndexError`. Multi-color dithering is not used
(ramps reduce surface noise in pixel art).

**Dimensions**: row count follows `pyxel.colors` length (Pyxel default
16, but the actual length of the palette at `build()` time); column
count is fixed at 16 brightness levels.

**`build()` algorithm**: for each (col, level), the target RGB is the
col's RGB scaled by `level / 15`; the picked palette index is the one
with the smallest squared Euclidean RGB distance to that target.
Perceptual color spaces (Lab etc.) are intentionally not used — simple
RGB distance is good enough for the small Pyxel palette and avoids
overhead. `__repr__` is provided for debugging.

Bulk get/set (`to_list` / `from_list`), factory variants, and
file load/save are intentionally not provided in the initial API — they
have no equivalent in pyxel main (where similar APIs are deprecated in
favor of slice assignment) and add no clear benefit at cube's scale.

---

## 9. Light

Flat-shading parameters held independently so multiple light setups can be
swapped quickly.

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
through `ColorRamp` to produce the final palette index.

**Flat shading only**: face-constant color, no Gouraud / Phong / per-pixel
lighting. The reasoning is twofold — software rasterization makes per-pixel
lighting expensive, and Pyxel's small palette cannot represent fine
gradients anyway.

`__repr__` is provided for debugging.

---

## 10. Mesh and Model

Two assets that work together for 3D model data:

- **Mesh**: a single geometry data unit (vertices / faces / UVs / col /
  image). Mutable and resizable; suitable for static primitives, manual
  shapes, and dynamic per-frame deformation.
- **Model**: a hierarchy template that bundles multiple `Mesh` references
  with parent / child relations, each part's relative transform, and each
  part's name. Used for multi-part assets (characters, props with movable
  joints).

Their relation mirrors the asset / instance split common to 3D engines
(Unity's Mesh ↔ Prefab, Godot's Mesh ↔ PackedScene, three.js's
BufferGeometry ↔ GLTF Group). `Mesh` data is shared across instances;
`create_node()` only duplicates the lightweight `Node` tree.

### 10.1 Mesh

A single geometry asset. Internally Rust manages a contiguous vertex /
face buffer (cache-friendly, SIMD-friendly). The buffer is mutable and
resizable so users can reshape a mesh per frame for dynamic effects.

#### Construction

```python
m = Mesh()                                              # empty mesh
m = Mesh.from_vertices(vertices, faces, uvs=, col=, image=)
m = Mesh.box(Vec3(1, 1, 1), col=4)                      # typical primitive
m = Mesh.sphere(1.0, segments=16, col=12)
m = Mesh.cylinder(0.5, 2.0, segments=16, col=8)
m = Mesh.plane(2.0, 1.0, col=11)
```

`Mesh` has no `load()`. File-based assets always go through `Model.load`
(§ 10.2) — even single-mesh files come back as a one-part Model. This
keeps the load entry point unified and lets the `cube` file format
remain a single hierarchy-aware format.

#### Per-element access

```python
m.vertex_count                 # int, read-only
m.face_count                   # int, read-only
v = m.get_vertex(i)            # Vec3
m.set_vertex(i, Vec3(...))
uv = m.get_uv(i)               # tuple[float, float]
m.set_uv(i, (u, v))
f = m.get_face(i)              # tuple[int, int, int]
m.set_face(i, (a, b, c))
```

#### Resize

```python
m.resize(vertex_count, face_count)
```

Reallocates the underlying buffer. Overhead-tolerant — same semantics as
`Image.resize`. Use it once in `__init__` for static counts, then mutate
per element. For variable counts (e.g. trail effects) call `resize` only
when the count actually changes; consider sizing for the maximum and
tracking active range in user code.

#### Color and texture

```python
m.col = 7                      # default face color (used when image is None)
m.image = pyxel.images[0]
```

#### Single-Node instantiation

```python
node = mesh.create_node()
```

Creates one `Node` whose `on_draw` automatically draws this mesh at the
node's local origin. For multi-part assets, use `Model` instead.

#### Drawing patterns

| Use case | Pattern |
|---|---|
| Static mesh on an actor | hold one `Mesh` and call `self.mesh(mat, mesh)` in `on_draw`, or use `mesh.create_node()` for a self-drawing Node |
| Dynamic mesh (per-frame deform) | hold one `Mesh`, mutate via `set_vertex` / `set_face`, then `self.mesh(mat, mesh)` |
| Many small line / triangle draws | use `self.line` / `self.tri` in `on_draw` directly (no `Mesh` needed) |

### 10.2 Model

A hierarchy template that references multiple `Mesh` assets. Captures
parent / child relations, each part's relative transform, and each part's
name. `create_node()` instantiates the template into a `Node` tree;
`Mesh` data is shared across all instances.

```python
model = Model.load("character.cube")
char1 = model.create_node()                # Node tree (parts share Mesh data)
scene.add_child(char1)

# Pose each part by name
head = char1.find("head")
head.transform = head.transform.rotate_y(45)

char2 = model.create_node()                # second instance, same template
scene.add_child(char2)
```

`Model` is load-only — `Model.load(filename)` is the sole way to
construct one (no public `__init__`). When users want to assemble a
hierarchy programmatically (procedural characters, runtime-composed
rigs), they build a `Node` tree directly from individual `Mesh`
instances:

```python
class RandomChar(Node):
    def __init__(self):
        super().__init__()
        body = Mesh.box(Vec3(1, 2, 0.5)).create_node()
        head = Mesh.sphere(0.5).create_node()
        head.transform = Mat4.from_translation(Vec3(0, 1.5, 0))
        self.add_child(body)
        body.add_child(head)
```

This keeps the dynamic-hierarchy path inside the `Node` system (already
the engine's hierarchy mechanism) and reserves `Model` for the static
"pre-baked hierarchy from asset file" role.

The supported file format is deferred to implementation (initial
expectation: a project-defined binary capturing vertices, faces, UVs,
texture references, parent / child links, relative transforms, and part
names).

`Model` has no direct draw command — drawing is the `Node` tree's
responsibility once instantiated. This keeps the asset / draw separation
clean and avoids per-frame Node-tree construction in the hot path.

---

## 11. Node

Base class for everything in the scene tree. A `Node` carries a transform,
hierarchy links, draw / collide / lifecycle hooks, and node-local draw
commands. `Scene` (§ 12) is the root `Node`; user-defined actors subclass
`Node` and override the lifecycle hooks.

```python
class Player(Node):
    def __init__(self):
        super().__init__()
        self.body = Mesh.box(Vec3(1, 2, 0.5), col=4)

    def on_update(self):
        self.transform = self.transform.translate(Vec3(0.1, 0, 0))

    def on_draw(self):
        self.mesh(Mat4.IDENTITY, self.body)
```

### 11.1 Attributes

| Attribute | Type | Cascade | Meaning |
|---|---|---|---|
| `name` | `str` | — | identifier for `find()`; `Model.load` populates it from part names |
| `transform` | `Mat4` | composed with parent's transform during draw | local-space transform |
| `active` | `bool` | parent-dominant (False halts subtree update + collision) | enable/disable update + collision |
| `visible` | `bool` | parent-dominant (False halts subtree drawing) | enable/disable draw |
| `collider` | `Collider \| None` | this node only | shape used for collision against other nodes |
| `light` | `Light \| None` | None inherits from the closest non-None ancestor | lighting parameters effective for this subtree |
| `color_ramp` | `ColorRamp \| None` | None inherits from the closest non-None ancestor | color LUT effective for this subtree |
| `parent` (read-only property) | `Node \| None` | — | direct parent in the tree |
| `children` (read-only property) | `tuple[Node, ...]` | — | direct children |
| `camera` (read-only property) | `Camera` | — | active camera; valid only inside `on_draw` |

#### Cascade modes

- **parent-dominant**: when an ancestor's value is False, every descendant
  is treated as False regardless of its own setting. Used for `active` and
  `visible` — a single ancestor flag can suspend an entire branch.
- **inherits-from-ancestor**: when this node's value is `None`, the
  effective value is the closest non-`None` ancestor's value. Used for
  `light` and `color_ramp` — set them once on `Scene` (or any subtree
  root) and override per-subtree as needed.
- **this node only**: no propagation. Used for `collider`.

### 11.2 Tree Operations

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

### 11.3 World Transform

```python
node.world_transform()              # Mat4 — composition of all ancestor transforms
```

Computed on demand by walking up the tree. Cube does not cache the world
transform; users that hit this in a hot path should compute and reuse the
value within a single frame.

### 11.4 Draw State

These methods adjust per-draw state inside `on_draw` and reset to defaults
when `on_draw` returns. They mirror Pyxel 2D's per-frame state but scoped
per node-draw rather than per frame:

```python
self.pal(src_col=None, dst_col=None)    # palette substitution
self.dither(alpha)                      # dither pattern by alpha (0.0-1.0)
self.depth_test(enabled)                # toggle depth test for subsequent draws
self.depth_write(enabled)               # toggle depth write for subsequent draws
```

### 11.5 Immediate-Mode Draw Commands

Inside `on_draw`, the node draws into the current camera and screen.
Coordinates are node-local (the engine composes parent transforms
during draw).

```python
# Vertex-specified
self.pset(pos, col)
self.line(p1, p2, col)
self.tri(p1, p2, p3, col)
self.trib(p1, p2, p3, col)

# Screen-aligned (always face the camera)
self.circ(pos, r, col)
self.circb(pos, r, col)
self.text(pos, s, col, font=None)

# Mat4-positioned (in mat's plane)
self.rect(mat, w, h, col)
self.rectb(mat, w, h, col)
self.elli(mat, w, h, col)
self.ellib(mat, w, h, col)

# Billboard / planar / textured
self.sprite(pos, img, uvs, w, h, colkey=None, angle=0.0)
self.plane(mat, img, uvs, w, h, colkey=None)

# Mesh asset draw
self.mesh(mat, mesh)
```

The two positioning conventions:

- **Vec3-positioned** (`pos`): used by vertex-specified primitives,
  screen-aligned shapes, and billboards.
- **Mat4-positioned** (`mat`): used by primitives with full orientation
  (rectangles, ellipses, planes, mesh assets).

#### Common conventions

- **Center pivot**: every shape is centered at its `pos` / `mat.pos`. No
  top-left pivot anywhere in the cube API.
- **`circ` / `circb`**: always face the camera; radius `r` is in world
  units (perspective shrinks distant circles); border is 1 pixel
  regardless of distance.
- **`line`**: world-positioned, fixed 1-pixel width.
- **`text`**: screen-aligned, distance-independent character size (font's
  native pixel size). Positioned at `pos` projected to screen, centered
  around the text bounding box.
- **`sprite`**: billboard — quad always faces the camera. `angle`
  rotates in screen space (around view-z), in degrees.
- **`plane`**: free-oriented quad. `mat` carries position, rotation, and
  scale; `(w, h)` is the quad's local width and height.
- **`mesh`**: draws the given `Mesh` asset's geometry, transformed by
  `mat` in node-local space. Use `Mat4.IDENTITY` to draw the mesh at the
  node's origin; pass a non-identity `mat` to nudge the mesh relative to
  the node without changing the node's own transform.

#### Texture and UV layout

`sprite` and `plane` take `img: Image | Tilemap` (integer image bank
indices are not accepted; use `pyxel.images[i]` if needed). `int` is
intentionally excluded because cube treats Image and Tilemap as a single
texture concept and the integer index space differs between them in
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

### 11.6 Lifecycle Hooks

Subclasses override these hooks to define behavior. Defaults are no-ops.

```python
def on_update(self): ...                         # called once per scene update
def on_draw(self): ...                           # called once per scene draw
def on_collide(self, other, contact): ...        # called when collision is detected
def on_destroy(self): ...                        # called when destroy() runs
```

- `on_update`: business logic per frame. The driver visits the tree
  pre-order; subtrees with `active = False` are skipped.
- `on_draw`: drawing calls (immediate-mode + `self.mesh`). The driver
  visits subtrees with `visible = True` and runs each node's `on_draw`
  with draw state reset to defaults at entry.
- `on_collide`: invoked once per frame for each colliding pair. `contact`
  is `Contact | None` (None when contact details are not produced by
  the collider type yet). Both nodes in a pair receive the call.
- `on_destroy`: cleanup hook. Called once just before the node leaves
  the tree.

---

## 12. Scene

`Node`-derived root that drives the per-frame update / draw cycle and
owns the screen clear color. The application instantiates one `Scene`
(or several), adds actor `Node` subtrees as children, and calls `update`
and `draw` from Pyxel's update / draw callbacks.

```python
class App:
    def __init__(self):
        pyxel.init(256, 192)
        self.scene = Scene()
        self.scene.clear_color = 0
        self.scene.light = Light()
        self.scene.color_ramp = ColorRamp()
        self.camera = Camera()
        self.scene.add_child(Player())
        pyxel.run(self.update, self.draw)

    def update(self):
        self.scene.update()

    def draw(self):
        self.scene.draw(0, 0, 256, 192, self.camera)
```

### 12.1 Inherited from Node

`Scene` inherits all `Node` attributes and methods (§ 11), so it is
indistinguishable from any other node when assigning lights, ramps,
running lifecycle hooks, or composing transforms. The convention is to
set scene-wide `light` and `color_ramp` on the `Scene` itself; descendants
inherit through the None-fallback rule.

### 12.2 Scene-specific Attributes

| Attribute | Type | Default | Meaning |
|---|---|---|---|
| `clear_color` | `int \| None` | `None` | screen + depth buffer clear color before each draw; `None` skips clear |

`clear_color = None` is the "do not clear" mode (transparent overlay,
multi-pass, or the application is clearing externally). `clear_color =
int` fills the destination region with that color and resets the depth
buffer at the start of each `draw` (the 3D equivalent of `pyxel.cls`).

### 12.3 Driver Methods

```python
scene.update()
scene.draw(x, y, w, h, camera, screen=None)
```

- **`update()`**: traverses the tree pre-order and calls each active
  node's `on_update`. Subtrees rooted at a node with `active = False`
  are skipped entirely. After hooks run, collision detection runs across
  the active colliders and fires `on_collide` on both sides of each
  colliding pair.
- **`draw(x, y, w, h, camera, screen=None)`**: rasterizes the scene
  into the destination rectangle `(x, y, w, h)` using `camera`. The
  driver clears the rectangle with `clear_color` (when set), traverses
  the visible subtree, and runs each node's `on_draw`.
  - `screen=None` (default): target `pyxel.screen`.
  - `screen=Image`: target a custom image (render-to-texture for
    minimap, multi-pass effects, off-screen rendering).

### 12.4 Multi-angle Rendering

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

## 13. Performance Notes

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

## 14. Open Items

- **Mesh file format**: the concrete on-disk format for `Model.load(filename)`.
- **Default ramp generation**: the algorithm `ColorRamp()` and `ColorRamp.build()`
  use to derive a default ramp from the current palette.
- **Joint animation system**: `Node.transform` is the per-frame surface;
  whether a higher-level `Motion` / animation player class is also needed
  is to be decided alongside the first real-game implementation.
- **Camera world ↔ screen helpers**: `Camera.world_to_screen(pos, ...)` /
  `screen_to_world(x, y, depth, ...)` would help HUD coordinates and
  mouse picking. Deferred because viewport size is not held by `Camera`
  in cube (it is a per-`render` argument), and the current need has not
  yet surfaced.
- **Per-command dither**: a `scene.dither(alpha)` command analogous to
  `pyxel.dither(alpha)` would let mid-queue dither state apply to
  subsequent commands. Deferred until the rendering implementation can
  evaluate the visual benefit against the small Pyxel palette.
- **Text positioning variants**: a billboard or planar (Mat4-positioned)
  `text` variant in addition to the screen-aligned default. Deferred
  until concrete environment-label use cases appear.

---

## 15. Decisions Explicitly Ruled Out

These were considered and rejected during design; revisit only with new
evidence.

### 15.1 Math classes

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

### 15.2 Naming

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

### 15.3 Drawing

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
- **`Model` class** (mesh instance node) — superseded by `Node` built from
  `Mesh.create_node()`.
- **`Shader` class** (combined shading + lighting + color tables) —
  replaced by `ColorRamp` (color LUT) + `Light` (parameters). The split lets
  multiple ramps and multiple lights be created and swapped independently.
- **`scene.push_matrix` / `scene.pop_matrix`** — the matrix stack pattern
  was dropped because draw commands accept their own `mat` directly, and
  `model(mat, node)` already covers hierarchical placement.
- **`int` for `img` parameter on `sprite` / `plane`** — `Image | Tilemap`
  only. Pyxel 2D's `pyxel.blt(img: int | Image, ...)` allows a bank index,
  but image bank index space and tilemap bank index space overlap, so a
  bare `int` would be ambiguous in cube. Callers pass `pyxel.images[i]` /
  `pyxel.tilemaps[i]` explicitly.
- **`sprite_tm` / `plane_tm`** — separate Tilemap-textured methods folded
  into the unified `sprite` / `plane` via `img: Image | Tilemap`.
- **`(u, v, sw, sh)` source-rectangle form on textured commands** —
  replaced by a 4-vertex `uvs` tuple. The 4-corner form is the
  software rasterizer's natural input and expresses flips, rotations,
  and trapezoidal mappings without auxiliary parameters.
- **Scalar `(u1, v1, u2, v2, u3, v3, u4, v4)` for UVs** — too many
  positional arguments; the nested-tuple form keeps the call site
  readable.
- **`fill` draw op on `Node`** — out of scope. `fill` has no clean 3D
  meaning (no obvious enclosure to flood). (Per-draw palette substitution
  is provided as `Node.pal` draw state, see § 11.4.)
- **`box` / `box_outline` / `sphere` / `sphere_outline` draw ops on
  `Node`** — covered by `Mesh.box()` / `Mesh.sphere()` plus
  `node.mesh(mat, mesh)`; no need for immediate-mode 3D-solid primitives
  in the initial API.
- **`draw_text` / `draw_image` / `make_*` prefixes** — cube uses bare
  verbs (`node.text`, `node.sprite`, etc.) without prefix on draw
  commands or factories.
- **`make_*` factory prefix** — the library uses `from_*` consistently
  for class-method factories (`Mat4.from_translation`,
  `Quat.from_axis_angle`).
- **GPU-oriented features** (flat 16-element `to_list` / `from_list` on
  Mat4, OpenGL handles, shader programs) — cube is software-rendered;
  GPU integration is intentionally out of scope.

### 15.4 Scene structure

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
