# Pyxel Cube Design Notes

`pyxel.cube` is the software-rendered 3D extension of Pyxel. This maintainer
document records its design rationale: conventions, renderer and collision
models, API-shape decisions, and deliberately excluded or deferred features.
For usage and exact signatures, see the
[Pyxel Cube User Guide](user-guide-cube.md),
[Pyxel Cube API Reference](api-reference-cube.md), and
[Python API stub](../python/pyxel/cube/__init__.pyi).

---

## 1. Overall Policy

- **Import style**: `from pyxel.cube import ...` and bring in only what's
  used.
- **Independent conventions**: Pyxel Cube is treated as an independent 3D
  subsystem. It aligns with widely used 3D APIs, engines, and math libraries
  (pyrr, PyGLM, three.js, Godot, Unity) rather than copying Pyxel 2D
  conventions whenever the two diverge.
- **Software 3D rendering**: Pyxel Cube is a software 3D renderer designed for
  retro pixel-art games. GPU integration features (flat-array layout,
  shader programs) are intentionally out of scope.
- **Performance target**: 60 fps on Raspberry Pi 4 / 5. Pi Zero 2 is
  best-effort.
- **No persistent mutable gameplay state**: immutable, lazily initialized
  geometry caches aside, the cube module holds no mutable state that affects
  results across calls. Its two thread-local slots are the per-draw context,
  which is emptied on exit, and a collision scratch buffer that keeps only its
  capacity between frames.
- **Implementation locality**: core / bindings / python sources live under
  `cube/` subfolders. Modifications to existing (non-cube) Pyxel code are
  minimized.

---

## 2. Conventions

### 2.1 Coordinates and Units

- **+X right, +Y up, +Z toward viewer**, **right-handed**, forward = `-Z`.
  Aligned with Godot / pyglet / three.js / OpenGL / glTF.
- `Node.draw(x, y, w, h)` maps the camera's line of sight to the center of
  the destination rectangle.
- **Angles in degrees** throughout the Pyxel Cube API.
- **Euler order**: apply the X rotation first about the fixed world X axis, then
  Y about fixed world Y, then Z about fixed world Z. The combined transform is
  `Rz * Ry * Rx`.
- **Pyxel 2D screen is Y-down; Pyxel Cube is Y-up.** The 2D / 3D mismatch
  follows the same pattern as Godot's "2D Y-down + 3D Y-up" and is
  intentional.
- **Per-frame units** for velocity and angular velocity. Pyxel Cube is not
  a physical simulator; for a non-mesh collider, `velocity` is the
  world-space translation applied per update and `angular_velocity` is the
  local rotation (axis × degrees) applied per update. No time-based (m/s,
  rad/s) units appear anywhere in the API.
- **Full-size extents**: `Node.box(size)` uses `abs(size)` as its width, height,
  and depth. `Collider.size` gives the full dimensions of the unrounded core,
  not Bullet-style half-extents; a non-negative rounding radius extends each
  side, so the overall collider extents are
  `abs(size) + 2 * max(radius, 0)` component-wise.

### 2.2 Math Primitives

- `Vec3`, `Mat4`, and `Quat` are immutable. Operations return new
  instances, and the constants (`Vec3.ZERO`, `Mat4.IDENTITY`,
  `Quat.IDENTITY`, ...) are shared singletons.
- Degenerate inputs fall back to the neutral value instead of raising: a
  zero vector normalizes to `Vec3.ZERO`, a singular matrix inverts to
  `Mat4.IDENTITY`, and a zero-length axis or zero quaternion yields the
  identity rotation.
- `Quat` stores `(x, y, z, w)` with `w` as the scalar (three.js / Unity /
  Godot order).
- `Mat4` is indexed by `(row, col)`; the storage layout is an
  implementation detail.
- There is no component-wise `Vec3 * Vec3`; per-axis scaling goes through
  `Mat4.from_scale(other) * v`.

### 2.3 Node Attribute Cascade

- **parent-dominant** (`active`, `visible`): when an ancestor's value is
  `False`, every descendant is treated as `False` regardless of its own
  setting.
- **inherits from ancestor** (`camera`, `shading`): when this node's value
  is `None`, the effective value is the closest non-`None` ancestor's.
  Set once on the scene root and override per subtree as needed.
- **this node only** (`collider`, `tags`): no propagation.

### 2.4 Opt-in Behavior

- `Collider.trigger` and `Collider.rolls` default to `False`, so the
  simplest behavior (solid body, sliding only) takes no arguments.
  Industry engines default to the richer behavior and offer constraint
  flags; Pyxel Cube inverts the polarity to keep the no-argument case
  obvious.
- `mass` must be finite and non-negative. `mass = 0.0` is the
  no-contact-response sentinel. A non-mesh collider with zero mass is a
  kinematic mover: contact response
  never pushes it back, but its configured motion is still integrated. A
  mesh collider is static terrain regardless of mass; collision physics
  ignores its `velocity` and `angular_velocity`.
- There is no scene-level gravity; users add `Vec3(0, -g, 0)` to
  `collider.velocity` in `on_update`, keeping the per-frame integration
  explicit in the Pyxel 2D "one tick at a time" spirit.
- `restitution` and `friction` are dimensionless tuning knobs, not
  calibrated material constants. A contact uses the larger restitution
  and the average friction of the two colliders.

---

## 3. API Shape Decisions

- `Mat4.rot` is a `Quat`, so rotation interpolation reads
  `m1.rot.slerp(m2.rot, t)` and avoids the ambiguity of Euler-angle
  interpolation (Unity's `Transform.rotation`).
- `Mat4.from_axis_angle` exists alongside `Quat.from_axis_angle` so call
  sites avoid the `Mat4.from_quat(Quat.from_axis_angle(...))` chain, and
  `Mat4.compose` takes a `Quat` to pair with `Mat4.rot`.
- `Quat.to_matrix` and `Mat4.from_quat` are two access points for one
  conversion, one per class.
- `to_local` / `to_world` and their `_dir` variants are named methods
  because they read better than `mat.inverse() * v` and the direction-only
  forms have no operator spelling.
- `Mat4.translate` and `Mat4.rotate` act in the matrix's local frame. A
  root Node can apply a world-space shift by left-multiplying
  `Mat4.from_translation`; a child first converts the shift through its
  parent's world transform (§ 6.3).
- `Mat4.scale_by` keeps clear of the `scale` accessor.
- View construction lives on `Mat4.look_at`, not on `Camera`, so camera
  transforms can be animated and interpolated like any other transform.
- `Camera.ortho_size` encodes both "orthographic?" and "what size?";
  `None` restores the perspective projection driven by `fov`.
- `Camera` holds no viewport. The rectangle is a per-`draw` argument, so
  one camera serves split screens and render-to-texture; screen-space
  helpers (`world_to_screen`, `screen_to_ray`) stay deferred for the same
  reason (§ 10).
- `Camera.clear_color` clears the target before each draw with that
  camera; `None` composites over the existing pixels. The depth buffer is
  cleared on every draw regardless.
- `Shading` merges the color lookup table with the scene light direction.
  A separate `Light` class with ambient and intensity knobs adds little
  to the toon-style flat shading the renderer targets, because the table
  already controls brightness per cell.
- The scene root is a plain `Node`: `update`, `draw`, and the spatial
  queries are `Node` methods, so no `Scene` class exists. Apps
  conventionally subclass `Node` (often named `Scene`) for the root and
  set `camera` and `shading` on it.
- `find_by_name` returns a list because names are not unique (several
  "zako" enemies may share one); `find_by_tags` shares the shape.
- `world_transform` is computed on demand by walking up the tree and is
  not cached. Callers that read it repeatedly in a hot path should compute it
  once and reuse the value.
- Spatial queries search the subtree of the node they are called on;
  scene-wide queries go through the root. There is no `ignore`
  parameter: filter list-returning queries, for example
  `[n for n in root.overlap_sphere(pos, r) if n is not self]`. For a ray,
  filter `raycast_all(...)`; discarding a self-hit from nearest-only
  `raycast(...)` cannot recover a hit behind it. The order of overlap results is
  implementation-defined.
- `RaycastHit` carries the industry minimal set (`node`, `point`,
  `normal`, `distance`); triangle indices and barycentric coordinates
  are not useful at this shape vocabulary.
- `Contact`, `RaycastHit`, and `Motion` are engine-built: no public
  constructor and read-only fields.
- `Primitive.cull` is a property of the primitive, not of the draw,
  because two-sidedness is geometric (grass billboards versus solid
  boxes); a shape with mixed regions splits into two primitives combined
  through `Mesh` parts or child nodes. `mode` is likewise part of the
  asset: solid and wireframe views of the same vertices are two
  primitives.
- The vertex-attribute lists of `Primitive` are live views (the
  `Sound.notes` pattern): element and slice assignment write through, and
  the attributes themselves cannot be reassigned.

---

## 4. Rendering Model

- **Immediate mode**: each `draw` walks the subtree parent-first, skips
  subtrees with `visible = False`, and runs every visited node's
  `on_draw`. Nothing is retained between calls, so multi-angle rendering
  is another `draw` with another camera.
- **Per-`on_draw` render state**: `shaded`, `dither`, `depth_test`,
  `depth_write`, and `depth_offset` are set by methods that apply to the
  draws that follow in the same `on_draw` and reset to their defaults at
  every node's `on_draw` entry, so state never leaks between nodes.
- `depth_offset` biases only the depth test and write, as if the draw
  moved along the view direction; negative is toward the camera, the
  `glPolygonOffset` / Direct3D / Unity sign.
- **Shading table**: one row per palette color and four brightness
  levels. Level 2 is the palette color itself, levels 0 and 1 are shades,
  and level 3 is the highlight. Each cell is a `(primary, secondary)`
  pair, rendered as a flat fill when the two match and as a 2×2 checker
  dither when they differ, which lets a ramp express colors the palette
  lacks. A face's level follows from the light direction and its normal.
- `shaded` has no effect on lines, points, outlines, screen-facing
  circles, and text; sprites always render unshaded.
- **Palette substitution** is a special case of the table: setting all
  four levels of a row to `(target, target)` maps one palette index to
  another regardless of brightness, the 3D counterpart of `pyxel.pal`.
- **Table synthesis** (`Shading.build`):
  - Each level targets the source color shifted in HSV value.
  - Candidate colors are ranked by weighted HSV distance, with hue weighted
    eight times more heavily than saturation or value.
  - A dither pair is considered only when its two colors are perceptually
    compatible.
  - The two shade levels are selected together from three patterns: two flat
    colors, a flat plus a dither, or a dither plus a flat.
  - A crossing penalty discourages a chromatic source from dropping to gray,
    while still allowing a gray source to pick a chromatic accent.
  - Shade luminance is compared in linear light so each shade stays visibly
    darker than its source. If no candidate satisfies the constraints, the
    level falls back to a flat pair of the source color.
  - The implementation is in
    [`crates/pyxel-core/src/cube/shading.rs`](../crates/pyxel-core/src/cube/shading.rs),
    with property tests covering monotone ramps and related invariants.
- **Per-face normals only**; smooth shading is not supported. Empty
  `normals` are derived at draw time and never cached back;
  `compute_normals` stores them for static geometry.
  Editing positions does not update stored normals; recompute or clear them
  after changing the geometry.
- `text` draws screen-space glyphs at the projected anchor with the depth
  test applied at the anchor's depth; `sprite`, `circ`, and `circb`
  always face the camera.

---

## 5. Assets and Instancing

- `Mesh` is asset-only: parallel arrays of parts with `parents[i] < i`,
  so world transforms resolve in one forward pass. `Node.from_mesh`
  instantiates a node tree per actor, and many trees share one mesh
  without copying vertex data.
- `Motion` clips target part indices, so a clip applies only to trees
  created from the mesh it was imported with. Playback is
  transform-only: it never deforms vertices, updates collider BVHs, or
  animates materials.
  For each animated part, transform components without channels use the bind
  pose, not the current node transform.
- `Mesh.from_glb` is a compatibility profile for Blockbench low-poly
  exports, not a general glTF loader. Unsupported features whose omission
  still leaves a displayable mesh (skins, morph targets, material animation,
  blend alpha, matrix node transforms, extra vertex attributes) print a
  warning and fall back to the closest displayable mesh; failures that leave
  no usable mesh (external files, invalid buffers, missing positions) raise.
  The supported set is listed in the API reference.
- A mesh used as terrain builds its BVH on the first collision query and
  keeps it until its collision geometry or part transforms change. The cache
  is internal and does not affect equality or `repr`.

---

## 6. Collision and Response Model

### 6.1 Colliders

One `Collider` class holds shape, behavior flags, physical coefficients,
and motion state; splitting it Unity-style into `Collider` and
`Rigidbody` doubles the objects per actor with no clear benefit at this scale. The
shape family follows from `size` and `radius`. Each `size` component whose
absolute value is less than `1e-9` is treated as zero:

- A zero `size` is a sphere with radius `max(radius, 0)`.
- A `size` equivalent to `(0, h, 0)` is a Y-axis capsule whose center
  segment has length `abs(h)` and whose total height is `abs(h) + 2 * max(radius, 0)`.
- Any other `size` is a rounded box whose unrounded core has dimensions
  `abs(size)` and whose radius extends each side.

A `mesh` replaces those shapes with static triangle terrain. Collision
physics ignores `size`, `radius`, `mass`, `rolls`, `velocity`, and
`angular_velocity` while `mesh` is set. Direct edits to the Node or its
ancestors can still reposition the terrain, but continuous collision
detection does not sweep that movement.

Analytic shapes require a rigid effective world transform (translation and
rotation, without scale or shear). A terrain mesh may use a fixed invertible
affine transform. These constraints keep the documented dimensions and
collision normals unambiguous.

### 6.2 Narrow Phase

Pairs are solved in the body frame, so collider rotation is honored and the
world AABB stays a broad-phase construct. Except for the bounded approximation
below, tests use the actual shapes rather than world AABBs. Box-vs-triangle
extends the SAT axes by the rounding radius and may over-report by at most
`r·(√3−1)` in corner regions.

### 6.3 Response

The collision solver never writes `transform` or `velocity`. It resolves each
contact into one `Contact` per side, with `normal` pointing from the other
body toward this one, `depth` already split by the mass ratio, and
additive `delta_velocity` / `delta_angular_velocity` (`delta_rotation` is
reserved and currently the identity). The user applies them in
`on_collide`:

```python
def on_collide(self, other, contact):
    offset = contact.normal * contact.depth
    if self.parent is not None:
        parent_world = self.parent.world_transform
        offset = (
            Vec3.ZERO
            if abs(parent_world.determinant()) < 1e-12
            else offset.to_local_dir(parent_world)
        )
    push = Mat4.from_translation(offset)
    spin = Mat4.from_quat(contact.delta_rotation)
    self.transform = push * self.transform * spin
    self.collider.velocity += contact.delta_velocity
    self.collider.angular_velocity += contact.delta_angular_velocity
```

The decomposition is deliberate:

- **Translation starts as `normal * depth` in world space**, a visible vector
  the user can apply partially (one axis only, or not at all). A child
  converts it to its parent's local coordinates before left-multiplying its
  local transform; the body's own rotation does not redirect the shift.
- **Rotation is a `Quat`** composed through `Mat4.from_quat` and
  right-multiplied so it stays local to the body's origin.
- **Velocities are additive**, so the intent written in `on_update`
  (gravity, thrust, input) survives and only the collision's contribution
  lands on top.

Skipping the response is legitimate (a one-way platform that swallows the
contact when the player drops through); the solver never overwrites the
user's state. Mass share: an immovable side receives `depth = 0` and zero
deltas. Positive masses split the correction in proportion to the other
body's mass, so equal masses split the penetration in half. For a trigger pair,
`point` and `normal` still describe the contact, while `depth` and every
correction delta are zero.

---

## 7. The Update Pipeline

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

3. **Motion integration**: for each Node that owns a non-mesh `Collider`,
   the engine applies the collider's `velocity` and `angular_velocity` to
   the Node's `transform`. Translation is expressed in world space and is
   converted to the parent's local coordinates; rotation is local
   (right-multiply), so spin updates the body's orientation without orbiting
   it around the world origin:

   ```python
   offset = collider.velocity
   if node.parent is not None:
       parent_world = node.parent.world_transform
       offset = (
           Vec3.ZERO
           if abs(parent_world.determinant()) < 1e-12
           else offset.to_local_dir(parent_world)
       )
   t_vel = Mat4.from_translation(offset)
   r_avel = Mat4.from_axis_angle(
       collider.angular_velocity.normalize(),
       collider.angular_velocity.length(),
   )
   node.transform = t_vel * node.transform * r_avel
   ```

   The parent transform must be invertible to represent a world-space
   displacement in the child's local transform. Under a singular parent,
   linear integration and its swept collision motion are skipped; local
   angular integration still applies. A mesh collider's motion fields are
   ignored.

4. **AABB refresh**: each `Collider`'s axis-aligned bounding box is
   recomputed from the current transform.

5. **Broad phase**: candidate pairs are enumerated by AABB overlap with
   an `O(N²)` sweep. Each moving analytic collider's AABB is swept by the linear
   displacement actually integrated that frame (the union of the previous
   and current position boxes), so a fast mover still overlaps everything
   along its path. Mesh colliders carry a lazily built internal BVH that the
   narrow phase queries with the dynamic body's mesh-local AABB.

6. **Narrow phase**: each candidate pair is tested for actual collision.
   Every sphere / capsule / rounded-box pairing (all six combinations)
   is supported, plus each of the three against a static mesh. When the
   overlap test finds no contact, a swept (time-of-impact) fallback
   re-tests the pair along the bodies' relative per-frame motion to detect
   crossings missed at the final positions. It exists for every supported
   pair family, including the three shapes against a static mesh.
   Mesh-vs-mesh is unsupported (both sides are static and need no
   resolution payload) and is silently skipped. The capsule-vs-box
   closest point is solved exactly by splitting the segment at the box's
   slab crossings; the only approximation is the one in § 6.2. The
   result is a stream of `Contact` records with `point`, `normal`, and
   `depth` filled in.

   Swept tests cover only the colliders' integrated linear velocity. Motion
   caused by direct transform edits, ancestor movement, or angular velocity
   is tested at the resulting pose but is not swept between poses.

7. **Response resolution**: the engine computes the mass-share split
   and writes the per-side push-back, rotation, and motion deltas into
   each `Contact`. For trigger colliders, the deltas are zero (the user
   sees the contact geometry but no motion correction).

8. **Notification**: for each pair `(a, b)`, the engine invokes
   `a.on_collide(b, contact_a)` and `b.on_collide(a, contact_b)`. The
   call order across pairs is deterministic (the tree's pre-order),
   and both `Contact` payloads are fixed before either callback. Within
   a pair, `a` runs first, so `b` can observe node-state changes made by
   `a`; those changes do not alter the precomputed `contact_b`.

9. **Deferred destruction**: Nodes whose `destroy()` was called during
   the frame are removed from the tree, and `on_destroy` is invoked on
   each. Removal is deferred to the end of the step so that
   mid-traversal `destroy()` calls do not invalidate the iteration.
   `destroy()` itself only sets the `destroyed` flag on the node and
   propagates it to every descendant; parent / child links survive the
   rest of the frame so traversals stay safe. The pipeline walks the
   tree post-order, fires `on_destroy` (leaf first, root last, matching
   the Unity / Godot convention), then detaches each flagged node.
   `Node.destroyed` exposes the flag read-only so user hooks can
   early-return from `on_update` / `on_collide` after a `destroy()`
   within the same frame. Because `update()` runs before `draw()`, a
   node destroyed in `update()` is already detached when the next
   `draw()` walks the tree, so its `on_draw` does not fire.

The pipeline is single-pass per frame: there is no iterative
constraint solver. The collision model targets games that can accept a
single resolution pass per frame; stable stacks and constraint chains are
out of scope.

---

## 8. Performance Notes

- A typical pixel-art game has tens to a few hundred drawables per
  frame. At that scale, immediate-mode drawing has comparable cost to
  Pyxel 2D's existing per-call overhead.
- Multi-angle rendering calls `draw` again with a different camera; each
  call re-runs the subtree's `on_draw` traversal and rasterizes afresh.
- `Mesh` is loaded once; `Node` trees built from it carry per-instance
  poses without copying mesh data.
- `Vec3` / `Mat4` / `Quat` are immutable and their constants are shared
  singletons; arithmetic returns fresh instances, and the implementation
  keeps allocation cheap for hot-path math.
- The collision pipeline targets ~100 dynamic bodies plus a thousand or
  so static mesh triangles at 60 fps on Raspberry Pi 4 / 5. Heavier
  scenes are out of scope.

---

## 9. Naming Policy

Pyxel Cube's public surface mixes conventions from several 3D engines.
The mix is intentional, not accidental, and follows the rules below.

1. **Math primitives** (`Vec3`, `Mat4`, `Quat`): the industry-common
   minimal set, with Python `snake_case` spelling. Method names align
   with Godot / pyrr / pygame (`length_squared`, `distance_squared_to`,
   `normalize`) rather than Unity's `magnitude` / `sqrMagnitude`.
2. **Form**: Python `snake_case` everywhere; no `is_` prefix on
   booleans (`active`, `visible`, `trigger`, `rolls`); no camelCase.
3. **Physics** (`Collider`, `Contact`): `mass = 0` disables contact
   correction for that collider, while `restitution` / `friction` are the
   coefficient names — combined with Unity-style attribute names
   (`velocity`, `angular_velocity`) for the motion state.
4. **Scene graph** (`Node`): Unity-style direction accessors
   (`forward` / `right` / `up`) on Node, Godot-style multi-membership
   tags (`tags: list[str]`), and Unity-style lookup names
   (`find_by_name`, `find_by_tags`).
5. **Spatial queries** (`Node.raycast` / `Node.overlap_*`): Unity
   Physics conventions, with shared parameter names (`origin`,
   `direction`, `max_distance`, `hit_triggers`, `tags`) across all
   four primitives.
6. **Opt-in flags**: Pyxel Cube's behavior flags (`trigger`, `rolls`)
   default `False`, so the "do the simplest thing" case takes no
   arguments.
7. **Per-frame units**: `velocity` / `angular_velocity` are per-frame,
   not time-based. Applications that need time-based physics can integrate
   a dedicated engine such as Bullet or Rapier.

The mix is documented here, not inside the .pyi, so that readers
coming from a specific engine know which conventions they will
recognize and which they will need to translate.

---

## 10. Open Items

- **Skeletal / blended animation system**: `Motion` samples imported
  transform channels on `Node.from_mesh` trees. Skinning, joint
  constraints, inverse kinematics, and animation blending remain
  deferred until a real game needs them.
- **Camera world ↔ screen helpers**: `Camera.world_to_screen(pos, ...)` /
  `screen_to_ray(...)` would help HUD coordinates and mouse picking.
  Deferred because the viewport is a per-`draw` argument rather than a
  `Camera` attribute (§ 3), and the need has not yet surfaced.
- **Extended topology modes**: `MODE_LINE_STRIP`, `MODE_LINE_LOOP`,
  `MODE_TRIANGLE_STRIP`, `MODE_TRIANGLE_FAN` along OpenGL's primitive
  numbering, for compact ribbon / fan / strip emissions through `prim`.
  Deferred until a real-game use case surfaces.
- **World-scaled text variant**: an optional `text` mode that lays
  glyphs as 3D-space quads transformed by a `Mat4`. Users who need
  3D-text shapes can build them through `prim`. Revisit if a real use
  case surfaces.
- **Lifecycle hook on attach**: `on_attach` / `on_ready` for scene-tree
  insertion was considered and rejected; `__init__` covers the typical
  setup needs and the additional hook adds learning surface for
  marginal benefit. Revisit if real-game patterns need it.
- **Additional interpolation**: `Quat.lerp` and `Mat4.lerp` are not
  provided. Users compose `q.slerp(...)` /
  `Mat4.compose(pos.lerp(...), rot.slerp(...), scale.lerp(...))` at the
  call site. Revisit if a clean use case emerges that the composition
  does not cover.

---

## 11. Decisions Explicitly Ruled Out

These were considered and rejected during design; revisit only with new
evidence.

### 11.1 Math Classes

- **`Vec2` / `Vec4` / `Mat3`** — not exposed publicly. Cube draws use
  `Vec3` for points and `Mat4` for full transforms.
- **Component-wise `Vec3 * Vec3`** — ambiguous in 3D math vocabulary.
  Use `Mat4.from_scale(other) * v` for per-axis scaling.
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
  rotations. In typical Pyxel Cube usage, numerical drift is negligible.
- **Scalar overloads of `Mat4.translate` / `Mat4.scale_by`** —
  `Vec3`-only signatures match three.js / Godot / pyrr / Unity.
- **`Quat.__add__` / `Quat.__sub__` / `Quat.__truediv__`** —
  quaternion linear combinations have limited use; `slerp` covers
  what's needed.
- **`Quat.lerp` / `Mat4.lerp`** — `slerp` is the standard quaternion
  interpolation; matrix interpolation composes from
  `pos.lerp`, `rot.slerp`, and `scale.lerp` at the call site.
- **`Quat.from_x_rotation` / `from_y_rotation` / `from_z_rotation`** —
  subsumed by `Quat.from_axis_angle` and `Quat.from_euler`.

### 11.2 Naming

- **`Vec` / `Mat` / `Quaternion`** — too generic or too long.
- **`Vector3` / `Matrix4`** — standard but verbose; `Vec3` / `Mat4`
  chosen for brevity.
- **`length_sq` / `distance_sq_to`** — short forms dropped in favor of
  `length_squared` / `distance_squared_to`, matching pygame / Godot.
- **`max_len` argument name** — `max_length` instead.
- **`from_rotation` factory name** — `from_euler` makes the Euler-angle
  intent explicit and pairs with `Quat.from_euler`.
- **Past-tense method names (`normalized`, `inverted`)** — Python
  idiom uses present tense (`normalize`, `inverse`).
- **`rotate_axis` / `rotate_arbitrary`** — `Mat4.rotate(axis, deg)` is
  short and unambiguous.
- **`is_trigger` / `lock_rotation` (with `is_` prefix)** — Pyxel Cube
  bools use no `is_` prefix, paralleling `active` / `visible`. The
  flags are `trigger` and `rolls`.

### 11.3 Camera and Lighting

- **`Camera.screen_to_ray(...)` / `Camera.world_to_screen(...)`** —
  the viewport is not held on `Camera` (it is a per-`draw` argument),
  so screen-to-world helpers need the viewport passed in explicitly.
  Deferred until a concrete user need emerges. Until such helpers are
  added, call sites can compose the existing projection and
  inverse-transform operations.
- **`Light` as a separate class** — lighting parameters (`ambient`,
  `direction`, `intensity`) in a `Light` class with a `Node.light`
  cascade and a separate LUT attribute. Collapsed into the single
  `Shading` class holding the color LUT and the scene-wide
  `direction`; `ambient` / `intensity` add little to the toon-style
  flat shading on top of the per-cell brightness control the LUT
  already exposes.

### 11.4 Drawing

- **Retained-mode scene graph with registered per-Node draw
  primitives** — replaced by per-`on_draw` immediate-mode draw commands.
- **A multi-shape aggregate registered into a node** — rejected;
  `Primitive` is reserved for the vertex-data asset.
- **Specialized Node subclasses** (`SpriteNode`, `LineNode`,
  `MeshNode`, `TextNode`, etc.) — one Node class is enough; per-shape
  behavior is in the `Node` draw commands.
- **`Shader` class** — replaced by `Shading` (LUT + direction).
- **`scene.push_matrix` / `scene.pop_matrix`** — draw commands accept
  their own `mat` directly.
- **`int` for the `img` parameter of `sprite` / `plane`** — `Image`
  only.
- **`Tilemap` on `sprite` / `plane`** — tile-grid storage does not
  match a UV-based texture sampler.
- **`(u, v, sw, sh)` source-rectangle form on textured commands** —
  replaced by a 4-vertex `uvs` tuple, which also expresses flips,
  rotations, and trapezoidal mappings.
- **Scalar `(u1, v1, u2, v2, u3, v3, u4, v4)` for UVs** — too many
  positional arguments.
- **`fill` draw op on `Node`** — no clean 3D meaning.
- **Standalone `pal` operation / `Node.pal` draw state** — replaced by
  setting `Shading` rows uniformly (§ 4).
- **Per-call keyword-argument modifiers** (`shaded=`, `dither_alpha=`,
  `depth_test=`, … on every draw command) **and cascading draw-state
  properties** — replaced by per-`on_draw` state-setter methods
  (`shaded` / `dither` / `depth_test` / `depth_write` / `depth_offset`)
  that reset to defaults at each node's `on_draw` entry.
- **A standalone `alpha` modifier name** — the pseudo-alpha control is
  the `dither(alpha)` setter, named to flag that the implementation is
  Bayer dithering rather than true alpha blending.
- **`draw_text` / `draw_image` / `make_*` prefixes** — Pyxel Cube uses
  bare verbs.
- **GPU-oriented features** (flat 16-element `to_list` / `from_list`
  on Mat4, OpenGL handles, shader programs) — Pyxel Cube is
  software-rendered.
- **`col_tex` argument name for asset draws** — `tex` lacks language
  fit without a `Texture` class.
- **`col_image` argument name** — `col_img` is internally consistent.
- **`prim` as the topology attribute name on `Primitive`** — `mode`
  pairs with the `MODE_*` constant prefixes and avoids overloading the
  `Node.prim` command name.
- **`Primitive.MODE_LINES` / `DRAW_LINES` constant prefixes** —
  `MODE_LINES` matches the OpenGL `GL_LINES` style.
- **`FloatBuffer` / `IntBuffer` typed-buffer classes** — `Primitive`
  attributes use private list-like live views with Python sequence
  operations; no public buffer classes are exposed.

### 11.5 Scene Structure and Lookup

- **A distinct `Scene` class** — the scene root is a plain `Node`; the
  frame loop (`update` / `draw`) and the spatial queries are `Node`
  methods, so no separate `Scene` type is exposed.
- **`add_child` returning the child or self for chaining** — tree ops
  return `None`.
- **Module-level functions in `pyxel.cube`** — the namespace stays
  classes-only.
- **`MeshPart` / `MeshNode` as a separate class for parts** — rejected
  in favor of parallel arrays on `Mesh`.
- **Recursive `Mesh` tree** — rejected.
- **`Mesh` holding `image` as an asset attribute** — split into
  `Primitive` (shape) + `Mesh.col_img` (texture).
- **A public per-part image attribute on `Mesh`** — hand-built
  mixed-image models split into multiple `Mesh` instances combined via
  a `Node` hierarchy; GLB imports keep their material slots internally.

### 11.6 Collision and Physics

- **`Collider` / `Rigidbody` split (Unity-style)** — Pyxel Cube
  collapses shape + body + flags + motion state into a single
  `Collider` class. The split doubles the number of objects users have
  to wire up for every interactive Node, with no clear benefit at PS1
  scale.
- **Layer / mask bitfield filtering** — replaced by `tags` (list of
  strings) and post-hit filtering. Bitfield layers add a fixed-budget
  numeric namespace that does not pay off in cube-scale games; tags
  scale naturally and remain readable.
- **`ignore: Node | list[Node]` parameter on spatial queries** —
  `tags`-based filtering plus a one-line post-filter cover the use
  cases without adding a parameter every caller has to thread through.
- **Engine-driven `transform` write-back** — the engine overwriting the
  user's `transform` after collision conflicts with override patterns
  (one-way platforms, user-driven teleport on contact). The engine
  writes the resolution into `Contact` and the user applies it in
  `on_collide` (§ 6.3).
- **User-constructible / mutable `Contact`, `RaycastHit`, and
  `Motion`** — these payloads are not exposed for user construction or
  mutation because the engine creates them; they have no public constructors
  and expose read-only properties.
- **Iterative constraint solver / sequential impulse loop** —
  out of scope. The collision model uses a single resolution pass per frame;
  constraint chains and stable stacks are not targeted.
- **`Collider` body-type enum** (`STATIC` / `KINEMATIC` / `DYNAMIC`)
  — Pyxel Cube uses `mass = 0` for a kinematic collider that receives no
  contact correction, instead of a separate enum. Mesh colliders are the
  static terrain type.
- **`Rigidbody.useGravity`-style flag** — Pyxel Cube has no
  scene-level gravity; users add it to `collider.velocity` in
  `on_update` themselves (§ 2.4).
