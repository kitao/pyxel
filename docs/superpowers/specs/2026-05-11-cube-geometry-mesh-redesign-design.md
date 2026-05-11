# Pyxel Cube Geometry / Mesh Redesign

This document captures the design decisions for restructuring the
geometry / mesh asset layer of Pyxel cube. It supersedes the existing
`cube-design.md` §10 (FloatBuffer / IntBuffer) and §11 (Mesh) and adds
guidance to §12.5 (Node draw commands) for the updated `node.prim` and
`node.mesh` signatures.

---

## 1. Motivation

The current cube asset model has three structural issues:

- **Class inflation**: `FloatBuffer` and `IntBuffer` were introduced as
  typed 1-D containers for fast Rust ↔ Python transfer, but they
  duplicate Python's native `list[float]` / `list[int]` slice / fill /
  resize idioms for cube's typical data sizes (tens to hundreds of
  vertices per mesh).
- **Mesh role confusion**: the existing `Mesh` collapses geometry data
  (positions / indices / normals / uvs) and material data (image /
  colkey) into one flat class. The two layers have different reuse
  patterns (geometry is shared across many models; image / colkey is
  per-model).
- **Missing hierarchical container**: there is no asset-level container
  for "a 3D model with multiple geometric parts and their hierarchy".
  User code that wants to express a character with body / hair / weapon
  parts has to build a `Node` tree manually each time.

This redesign splits the asset model into two clean classes (`Geometry`
+ `Mesh`) and removes `FloatBuffer` / `IntBuffer`. The net effect on
public class count is `-2 + 1 = -1` (FloatBuffer and IntBuffer removed,
Geometry added, existing Mesh redesigned in place).

---

## 2. Scope of Change

### Removed

- `FloatBuffer` class (cube-design.md §10) — replaced by `list[float]`
  direct exposure
- `IntBuffer` class (cube-design.md §10) — replaced by `list[int]`
  direct exposure
- Existing `Mesh` class shape (cube-design.md §11) — replaced by the new
  `Mesh` design below
- `Node.PRIM_POINTS / PRIM_LINES / PRIM_TRIANGLES` constants
  (cube-design.md §12.1) — relocated to `Geometry`

### Added

- `Geometry` class (vertex data + topology + cull mode)
- New `Mesh` class shape (parallel-array hierarchy of geometries +
  shared texture)

### Modified

- `Node.prim` signature: replaces positional `mode` / `positions` /
  `indices` / `normals` / `uvs` / `first` / `count` arguments with a
  single `Geometry` parameter (cube-design.md §12.5)
- `Node.mesh` signature: drops `col` argument; `image` / `colkey` move
  into the new `Mesh` class as `col_img` / `colkey` (cube-design.md
  §12.5)

### Unchanged

- All math classes (`Vec3` / `Mat4` / `Quat`), `Camera`, `Shading`,
  `Contact`, `Collider`, `Scene`
- `Node.BILLBOARD_OFF / ON / FIXED_Y` constants and `Node` lifecycle
- `node.sprite` / `node.plane` and other immediate-mode draw commands
  (`pset`, `line`, `tri`, etc.)

---

## 3. Geometry Class

`Geometry` is a static vertex-data asset. It carries the buffers, the
topology mode, and the back-face cull mode. It is intentionally
shareable across many `Node` instances (and across `Mesh.geometries`
slots).

### 3.1 Public Surface

```python
class Geometry:
    # Topology mode constants (attribute: prim)
    PRIM_POINTS: int = 0
    PRIM_LINES: int = 1
    PRIM_TRIANGLES: int = 2

    # Back-face cull constants (attribute: cull)
    CULL_NONE: int = 0
    CULL_BACK: int = 1
    CULL_FRONT: int = 2

    # Vertex attributes (parallel arrays of floats, three per vertex
    # for positions / normals, two per vertex for uvs)
    positions: list[float]
    normals: list[float] | None
    uvs: list[float] | None

    # Topology
    indices: list[int] | None
    prim: int

    # Back-face cull
    cull: int

    def __init__(
        self,
        positions: list[float] | None = None,
        normals: list[float] | None = None,
        uvs: list[float] | None = None,
        indices: list[int] | None = None,
        prim: int = PRIM_TRIANGLES,
        cull: int = CULL_BACK,
    ) -> None: ...

    def __repr__(self) -> str: ...
    def compute_normals(self, smooth: bool = False) -> None: ...
```

### 3.2 Field Semantics

| Field | Type | Default | Meaning |
|---|---|---|---|
| `positions` | `list[float]` | `[]` | flat `(x, y, z)` triples, `len(positions) % 3 == 0` |
| `normals` | `list[float] \| None` | `None` | flat `(nx, ny, nz)` per vertex; `None` triggers auto-compute on first draw |
| `uvs` | `list[float] \| None` | `None` | flat `(u, v)` per vertex; `None` disables texture sampling |
| `indices` | `list[int] \| None` | `None` | flat indices; `None` draws as a flat list (`prim`-determined chunk size) |
| `prim` | `int` | `PRIM_TRIANGLES` | topology; constrains the chunk size of `indices` (3 / 2 / 1) |
| `cull` | `int` | `CULL_BACK` | back-face cull mode for triangle topology |

### 3.3 Normal Auto-Cache

When `normals` is `None`, the renderer computes per-face flat normals
on the first draw and stores them on the `normals` attribute (cache).
Subsequent draws reuse the cached normals. To force recomputation after
mutating `positions`, set `normals = None` (or call `compute_normals()`
to refresh explicitly).

`compute_normals(smooth: bool = False)`:
- `smooth=False` (default): one normal per face, replicated to each
  vertex of that face (flat shading).
- `smooth=True`: averages adjacent face normals at shared vertices for
  smooth shading. Requires `indices` populated (per-face normals need
  to identify shared vertices). With `indices = None`, falls back to
  flat behavior.

### 3.4 Topology and `prim` / `indices` Interaction

The `prim` value determines how indices (or positions, if `indices` is
`None`) are grouped:

- `PRIM_POINTS`: 1 index per point
- `PRIM_LINES`: 2 indices per line segment
- `PRIM_TRIANGLES`: 3 indices per triangle

A geometry's `prim` is part of the geometry's identity — switching prim
mode at draw time is not supported (different prim modes have different
index-list interpretations). To draw the same vertices as both a solid
mesh and a wireframe, create two separate `Geometry` instances with
different `indices` and `prim` values.

### 3.5 Per-Geometry Cull Mode

`cull` is held on `Geometry` (not per-draw) because cull is a geometric
property: planar grass billboards need `CULL_NONE` (two-sided), solid
boxes need `CULL_BACK` (single-sided), independent of the draw context.
If a shape has mixed cull regions (e.g., character body cull-on +
hair-plane cull-off), split the shape into two `Geometry` instances
and combine them through `Mesh` parts or a `Node` hierarchy.

### 3.6 List vs. Typed Buffer

Earlier drafts used `FloatBuffer` / `IntBuffer` to expose typed 1-D
buffers backed by a contiguous Rust array. The redesign drops both in
favor of native `list[float]` / `list[int]`:

- Python's native list supports slice / append / extend / clear without
  a custom API surface.
- The Rust binding layer copies the list contents into a contiguous
  internal buffer at attribute assignment time. Subsequent draws read
  from the internal buffer (no per-element FFI).
- Slice mutation on the returned list (e.g., `geom.positions[0:9] =
  [...]`) does not propagate to the internal buffer; callers reassign
  the list (`geom.positions = new_list`) to trigger a refresh. Dynamic
  per-frame deformation is an explicit `geom.positions = ...` per
  frame; cube's primary use case is static asset construction at load
  time, where this cost is paid once.

This trades the buffer-protocol / `memoryview` capability of the old
design for a simpler attribute API and a smaller class surface (two
classes removed; one new class added in §3 below). Cube's target scale
(tens to hundreds of vertices per geometry, dozens of geometries per
scene) does not stress the list-conversion path.

---

## 4. Mesh Class

`Mesh` is an asset container for a hierarchical 3D model. It bundles
multiple `Geometry` parts (positions / topology / cull) with a shared
texture or flat color (`col_img`) and the parent-child relationships
between parts (parallel arrays).

### 4.1 Public Surface

```python
class Mesh:
    # Parts (parallel arrays, len(geometries) == len(transforms) == len(parents))
    geometries: list[Geometry | None]
    transforms: list[Mat4]
    parents: list[int]               # parents[i] < i, -1 = root

    # Shared material
    col_img: int | Image             # int = flat color, Image = texture
    colkey: int | None               # transparent key when col_img is Image

    def __init__(
        self,
        geometries: list[Geometry | None] | None = None,
        transforms: list[Mat4] | None = None,
        parents: list[int] | None = None,
        col_img: int | Image = 7,
        colkey: int | None = None,
    ) -> None: ...

    def __repr__(self) -> str: ...
    def descendants(self, i: int) -> list[int]: ...
```

### 4.2 Parallel Arrays

The three lists `geometries`, `transforms`, and `parents` index the
same set of mesh parts:

- `geometries[i]`: `Geometry` for part `i`, or `None` for a pure group
  (transform-only joint / pivot that draws nothing itself but
  influences children).
- `transforms[i]`: local transform of part `i` in its parent's frame.
- `parents[i]`: index of part `i`'s parent in the same arrays, or `-1`
  if part `i` is a root.

### 4.3 Topological Order Constraint

`parents[i] < i` is required for every `i` (a part's parent must come
earlier in the array). This makes world-transform computation a single
forward pass:

```python
world = [None] * len(transforms)
for i in range(len(transforms)):
    if parents[i] == -1:
        world[i] = transforms[i]
    else:
        world[i] = world[parents[i]] * transforms[i]
```

The constructor validates the constraint and raises `ValueError` if it
is violated, plus when the three arrays have mismatched lengths.

### 4.4 `descendants` Helper

`descendants(i: int) -> list[int]` returns all part indices that are
transitive children of part `i` (excluding `i` itself), in topological
order. Used by callers that want to bulk-transform a subtree (e.g.,
"move the weapon and everything attached to it"). Runs in O(N) by a
single forward sweep using the topological-order invariant.

### 4.5 Shared `col_img` / `colkey` Across Parts

`col_img` and `colkey` are held at the `Mesh` level (not per part).
Mixed-texture models (where different parts need different images) are
expressed as separate `Mesh` instances combined through a `Node`
hierarchy. This keeps `Mesh` focused on "one model, one texture
atlas" — the typical structure for pixel-art asset workflows.

### 4.6 Why No Part Names

Earlier drafts included a `names: list[str]` parallel array so callers
could look up parts by name (e.g., `mesh.find("sword")`). The redesign
removes it:

- Cube does not yet have a joint-animation system (cube-design.md §15
  open item); name-based lookup is most useful in that context.
- A typical pixel-art mesh has 1-10 parts, manageable by index
  constants in user code (`SWORD = 2`).
- glTF-import paths can construct an external `dict[str, int]` lookup
  table without burdening the core class.

When the animation pipeline lands, names can be re-added as part of
that work, alongside whatever animation-data structure that pipeline
needs.

---

## 5. Drawing API Updates

### 5.1 `node.prim`

```python
def prim(
    self,
    mat: Mat4,
    geom: Geometry,
    *,
    col_img: int | Image = 7,
    colkey: int | None = None,
    shaded: bool = True,
    dither_alpha: float = 1.0,
    depth_test: bool = True,
    depth_write: bool = True,
    billboard: int = 0,
) -> None: ...
```

The previous signature's `mode`, `positions`, `indices`, `normals`,
`uvs`, `first`, and `count` arguments collapse into the single
`Geometry` parameter. `col` is renamed to `col_img` to match the new
`Mesh.col_img` and to make the `int | Image` shape explicit at the
call site.

### 5.2 `node.mesh`

```python
def mesh(
    self,
    mat: Mat4,
    mesh_asset: Mesh,
    *,
    shaded: bool = True,
    dither_alpha: float = 1.0,
    depth_test: bool = True,
    depth_write: bool = True,
    billboard: int = 0,
) -> None: ...
```

`col`, `image`, and `colkey` move into `Mesh` as `col_img` / `colkey`
(see §4.1) and disappear from the per-call argument list. The renderer
walks the mesh's part list, composes each part's world transform with
`mat`, and draws each non-`None` geometry through the same path as
`node.prim`.

### 5.3 Other Immediate-Mode Commands

The non-asset draw commands (`pset`, `line`, `tri`, `trib`, `circ`,
`circb`, `sphere`, `sphereb`, `rect`, `rectb`, `elli`, `ellib`, `box`,
`boxb`, `text`, `sprite`, `plane`) keep their existing signatures.
Internally, the implementations of `box`, `sphere`, etc. that
previously routed through `prim` now route through `prim` with a
cached internal `Geometry` for each primitive shape.

---

## 6. Naming Conventions

The redesign settles three naming choices that previously varied:

### 6.1 `col` / `img` / `col_img` Role Split

| Role | Argument name | Type | Where it appears |
|---|---|---|---|
| Flat-color-only primitive | `col` | `int` | `pset`, `line`, `tri`, `rect`, `box`, `text`, etc. |
| Texture-only quad | `img` | `Image` | `sprite`, `plane` |
| Asset draw (flat-or-texture union) | `col_img` | `int \| Image` | `prim`, `Mesh.col_img` |

The three argument names are linked by short-form composition:
`col_img` = `col` + `img`. Each layer's responsibility is encoded in
the argument name.

The mismatch between pyxel 2D's `pyxel.blt(img: int | Image, ...)`
(where `int` is a bank number) and cube's `img: Image` (Image instance
only) is intentional — cube callers fetch `pyxel.images[i]` explicitly
so the cube API does not couple to bank-index semantics. This
restriction is documented in cube-design.md §16.3.

### 6.2 `Geometry.PRIM_*` / `Geometry.CULL_*` Constants

`PRIM_POINTS / PRIM_LINES / PRIM_TRIANGLES` and `CULL_NONE / CULL_BACK
/ CULL_FRONT` live as class-level constants on `Geometry`. The `PRIM_`
prefix is the established 3D-graphics short form for "primitive"
(OpenGL `GL_POINTS`-style constants). The `CULL_` prefix disambiguates
the directionality from generic words like `BACK` or `FRONT`.

`Node.PRIM_*` (formerly on `Node` for the old `node.prim(mode, ...)`
signature) are removed; their canonical home is `Geometry` now that
the mode is a geometry property.

### 6.3 Attribute Names `prim` and `cull`

The attribute names on `Geometry` use the lowercase short form
matching the constant prefixes: `geom.prim = Geometry.PRIM_LINES`,
`geom.cull = Geometry.CULL_BACK`. The names are self-consistent within
`Geometry` and align with the cube-wide convention of compact
attribute identifiers.

---

## 7. Open Items Deferred to Future Work

These were identified during the redesign discussion but are left for
later:

- **`Mesh.find(name)` and `names` array**: paired with the animation
  pipeline (cube-design.md §15 open item). When animation lands,
  re-evaluate whether parts need stable string identifiers.
- **`MeshPart` as a separate class**: the parallel-array layout was
  preferred over a tree of `MeshPart` instances to avoid an extra
  public class. If a future use case demonstrates value in
  mutable / extensible per-part state, revisit.
- **`Texture` class as a wrapper around `Image`**: if cube grows
  3D-specific texture state (filter mode, wrap mode, mipmaps), a
  `Texture` class wrapping `Image` may justify renaming `col_img` to
  `col_tex`. As of this design, cube uses `Image` directly and does
  not need a separate `Texture` type.
- **QUADS / TRIANGLE_STRIP / TRIANGLE_FAN topology**: deferred until a
  real-game use case demonstrates value. The current three primitive
  modes (POINTS / LINES / TRIANGLES) cover all immediate needs;
  cube-design.md §15 already lists STRIP / FAN as future additions
  along the OpenGL numbering, and QUADS can join that list.
- **Per-part image override**: today `Mesh.col_img` is single per
  mesh. Mixed-image models split into multiple `Mesh` instances.
  If frequent mixed-image-within-one-asset cases emerge, a per-part
  texture override mechanism can be added.
- **Index-based slice draw on `prim`**: the previous `prim`'s `first /
  count` arguments are dropped. If a future case requires drawing a
  subset of a geometry's indices, add it as a new method or
  per-call argument at that point.

---

## 8. Discussed and Rejected

Decisions ruled out during the design discussion; revisit only with
new evidence.

- **`col_tex` argument name**: would imply a `Texture` class
  (alignment with three.js / Godot / Unity, all of which have a
  separate `Texture` class). Cube uses `Image` directly; the language
  fit of `tex` is weak without the corresponding class. Revisit if a
  `Texture` wrapper is introduced.
- **`col_image` argument name**: short-form `col` + full-form
  `image` mixes naming registers within one compound; the all-short
  `col_img` is internally consistent.
- **`mode` as the topology attribute name**: was considered alongside
  `prim` and `primitive`; `prim` aligns with the `Node.prim(...)`
  draw command and the `PRIM_*` constant prefix without verbosity.
- **`Geometry.MODE_LINES`**: ruled out in favor of `Geometry.PRIM_LINES`
  to match the established 3D-graphics prefix (OpenGL `GL_LINES`
  style) and to reuse the `Node.PRIM_*` symbols that already shipped.
- **`MeshPart` / `MeshNode` as a separate class for parts**: rejected
  in favor of parallel arrays (`geometries` / `transforms` /
  `parents`) to keep the public class count down and avoid nested
  navigation (`mesh.parts[i].transform`) for what is essentially a
  flat array of part data.
- **Recursive `Mesh` (each `Mesh` has `children: list[Mesh]`)**:
  rejected as overly nested for what is functionally a parts array,
  and creates a confusing "root vs child Mesh" image-cascade story.
- **`Mesh` holding `image` as an asset attribute (old `cube-design.md`
  §11)**: rejected per the design discussion's principle that the
  reusable, shareable layer (`Geometry`) holds shape data, while the
  textured-model layer (`Mesh`) holds material data; the user
  guidance is that an asset (Geometry) does not own its texture, but
  a model-level container (Mesh) does.
- **Children list on `parents` parallel arrays**: storing both
  parents and children arrays was rejected as redundant — the
  drawing pass walks parts linearly and never enumerates children,
  and the `descendants(i)` editing helper can derive subtree membership
  from `parents` in O(N).
- **Topological order off (free parent indices)**: rejected because
  it complicates per-frame world-transform computation (recursive or
  multi-pass) without a concrete benefit; the topological-order
  constraint is a fixed-cost validation at construction time.
- **`int | Image` in `node.sprite` / `node.plane`**: rejected. These
  commands' purpose is to draw a textured quad; the UV argument is
  required, and a flat-color quad with required UVs is meaningless.
  Flat-color quads use `node.rect` / `node.box` instead.

---

## 9. Implementation Plan Outline

(Detailed plan to be authored by `superpowers:writing-plans` after
this spec is approved.)

- Update `python/pyxel/cube/__init__.pyi` to the new `Geometry` /
  `Mesh` shapes and the redesigned `node.prim` / `node.mesh` signatures
- Replace `crates/pyxel-core/src/cube/{mesh.rs, float_buffer.rs,
  int_buffer.rs}` and `crates/pyxel-binding/src/cube/{mesh.rs,
  float_buffer.rs, int_buffer.rs}` with new `geometry.rs` and a
  rewritten `mesh.rs` per the parallel-array layout
- Update `crates/pyxel-core/src/cube/{node.rs, draw.rs, raster.rs}`
  for the new `prim` / `mesh` paths (mode + cull come from `Geometry`,
  per-frame world transform composition for `Mesh`)
- Rewrite `python/tests/cube/test_mesh.py`,
  `python/tests/cube/test_float_buffer.py`,
  `python/tests/cube/test_int_buffer.py` against the new shapes (the
  buffer tests fold into geometry / mesh tests once buffers are
  removed)
- Update `docs/cube-design.md` §10 (delete), §11 (replace), §12.1
  (remove `Node.PRIM_*`), §12.5 (revise `node.prim` / `node.mesh`),
  and §16 (move `col_tex` / `col_image` / `mode` / etc. rejections
  into the appropriate sub-sections)
- Add or update example scripts in `python/pyxel/examples/` to use
  the new API (the cube branch does not yet have public cube
  examples; the redesign work is a good moment to seed one)

---

## 10. Verification

- `make lint` / `make lint-wasm` warning-free
- `make test` passes (including the rewritten `python/tests/cube/`
  suite)
- Existing `crates/pyxel-core` MML / image / audio tests remain green
  (the redesign is contained to cube and should not touch them)
- A manual smoke test of `python/pyxel/examples/<cube example>.py` (to
  be authored as part of the implementation) renders correctly with
  the new API
