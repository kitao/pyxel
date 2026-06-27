<!-- This file is generated from web/api-reference/cube/api-reference.json. -->

# Pyxel Cube API Reference

*This document was auto-generated from the [Pyxel Cube API Reference](https://kitao.github.io/pyxel/web/api-reference/cube/) web page, which also offers multilingual support.*

## Vec3

### `Vec3(x=0.0, y=0.0, z=0.0)` — class

A 3D vector with x, y, and z components. Values are immutable; arithmetic and transform methods return a new Vec3.

**Parameters:**

- `x` (*float*) — X component. Defaults to 0.0.
- `y` (*float*) — Y component. Defaults to 0.0.
- `z` (*float*) — Z component. Defaults to 0.0.

**Example:**

```python
from pyxel.cube import Vec3
v = Vec3(1.0, 2.0, 3.0)
w = (v + Vec3.UP) * 2.0
```

**Note:** Import it from the pyxel.cube module. Supports +, -, * and / with a scalar, unary -, ==, len(), indexing, and iteration.

### `x` — variable

The x component of the vector.

- **Type:** `float`

### `y` — variable

The y component of the vector.

- **Type:** `float`

### `z` — variable

The z component of the vector.

- **Type:** `float`

### `Vec3.ZERO` — constant

Vector with all components 0.

- **Type:** `Vec3`

### `Vec3.ONE` — constant

Vector with all components 1.

- **Type:** `Vec3`

### `Vec3.RIGHT` — constant

Unit vector pointing right (1, 0, 0).

- **Type:** `Vec3`

### `Vec3.LEFT` — constant

Unit vector pointing left (-1, 0, 0).

- **Type:** `Vec3`

### `Vec3.UP` — constant

Unit vector pointing up (0, 1, 0).

- **Type:** `Vec3`

### `Vec3.DOWN` — constant

Unit vector pointing down (0, -1, 0).

- **Type:** `Vec3`

### `Vec3.FORWARD` — constant

Unit vector pointing forward (0, 0, -1). Pyxel Cube uses a right-handed system where forward is -Z.

- **Type:** `Vec3`

### `Vec3.BACK` — constant

Unit vector pointing back (0, 0, 1).

- **Type:** `Vec3`

### `dot(other)` — function

Return the dot product with another vector.

**Parameters:**

- `other` (*Vec3*) — The other vector.

**Returns:** `float` — The dot product.

### `cross(other)` — function

Return the cross product with another vector.

**Parameters:**

- `other` (*Vec3*) — The other vector.

**Returns:** `Vec3` — The cross product vector.

### `length()` — function

Return the length (magnitude) of the vector.

**Returns:** `float` — The length of the vector.

### `length_squared()` — function

Return the squared length. Cheaper than length() for comparisons.

**Returns:** `float` — The squared length.

### `distance_to(other)` — function

Return the distance to another vector.

**Parameters:**

- `other` (*Vec3*) — The other vector.

**Returns:** `float` — The distance.

### `distance_squared_to(other)` — function

Return the squared distance to another vector.

**Parameters:**

- `other` (*Vec3*) — The other vector.

**Returns:** `float` — The squared distance.

### `angle_to(other)` — function

Return the angle to another vector in degrees.

**Parameters:**

- `other` (*Vec3*) — The other vector.

**Returns:** `float` — The angle in degrees (0-180).

### `normalize()` — function

Return a unit vector pointing in the same direction. A zero vector returns Vec3.ZERO.

**Returns:** `Vec3` — The normalized vector.

### `clamp_length(max_length)` — function

Return a copy whose length is clamped to max_length.

**Parameters:**

- `max_length` (*float*) — Maximum allowed length.

**Returns:** `Vec3` — The clamped vector.

### `min(other)` — function

Return the component-wise minimum with another vector.

**Parameters:**

- `other` (*Vec3*) — The other vector.

**Returns:** `Vec3` — The component-wise minimum.

### `max(other)` — function

Return the component-wise maximum with another vector.

**Parameters:**

- `other` (*Vec3*) — The other vector.

**Returns:** `Vec3` — The component-wise maximum.

### `lerp(other, t)` — function

Linearly interpolate toward another vector.

**Parameters:**

- `other` (*Vec3*) — The target vector.
- `t` (*float*) — Interpolation factor from 0.0 to 1.0.

**Returns:** `Vec3` — The interpolated vector.

### `slerp(other, t)` — function

Spherically interpolate toward another vector. Both vectors should be unit length.

**Parameters:**

- `other` (*Vec3*) — The target unit vector.
- `t` (*float*) — Interpolation factor from 0.0 to 1.0.

**Returns:** `Vec3` — The interpolated vector.

### `reflect(normal)` — function

Return the reflection of the vector against a surface with the given unit normal.

**Parameters:**

- `normal` (*Vec3*) — The unit surface normal.

**Returns:** `Vec3` — The reflected vector.

### `project(other)` — function

Return the projection of the vector onto another vector.

**Parameters:**

- `other` (*Vec3*) — The vector to project onto.

**Returns:** `Vec3` — The projected vector.

### `to_local(mat)` — function *(Advanced)*

Convert a world-space position into mat's local space.

**Parameters:**

- `mat` (*Mat4*) — The reference transform.

**Returns:** `Vec3` — The position in local space.

### `to_world(mat)` — function *(Advanced)*

Convert a local-space position into world space through mat.

**Parameters:**

- `mat` (*Mat4*) — The reference transform.

**Returns:** `Vec3` — The position in world space.

### `to_local_dir(mat)` — function *(Advanced)*

Convert a world-space direction into mat's local space, ignoring translation.

**Parameters:**

- `mat` (*Mat4*) — The reference transform.

**Returns:** `Vec3` — The direction in local space.

### `to_world_dir(mat)` — function *(Advanced)*

Convert a local-space direction into world space, ignoring translation.

**Parameters:**

- `mat` (*Mat4*) — The reference transform.

**Returns:** `Vec3` — The direction in world space.

## Mat4

### `Mat4()` — class

A 4x4 transform matrix. The constructor creates an identity matrix. Values are immutable; transform methods return a new Mat4.

**Example:**

```python
from pyxel.cube import Mat4, Quat, Vec3
node.transform = Mat4.from_translation(Vec3(0, 1, 0))
spin = node.transform * Mat4.from_axis_angle(Vec3.UP, 90)
```

**Note:** Supports * with another Mat4 or a Vec3, indexing with [row, col], and ==. Used for Node.transform and the placement argument of draw commands.

### `Mat4.IDENTITY` — constant

The identity matrix.

- **Type:** `Mat4`

### `pos` — variable

The translation part of the matrix.

- **Type:** `Vec3`

### `rot` — variable

The rotation part of the matrix as a Quat. Assumes a translation-rotation-scale composition.

- **Type:** `Quat`

### `scale` — variable

The scale part of the matrix.

- **Type:** `Vec3`

### `Mat4.from_translation(pos)` — class

Create a translation matrix.

**Parameters:**

- `pos` (*Vec3*) — The translation.

**Returns:** `Mat4`

### `Mat4.from_euler(euler)` — class

Create a rotation matrix from Euler angles in degrees, applied around the world axes in X, Y, Z order.

**Parameters:**

- `euler` (*Vec3*) — Rotation around each axis in degrees.

**Returns:** `Mat4`

### `Mat4.from_quat(rot)` — class

Create a rotation matrix from a Quat.

**Parameters:**

- `rot` (*Quat*) — The rotation.

**Returns:** `Mat4`

### `Mat4.from_scale(scale)` — class

Create a scale matrix.

**Parameters:**

- `scale` (*Vec3*) — The per-axis scale.

**Returns:** `Mat4`

### `Mat4.from_axis_angle(axis, deg)` — class

Create a rotation matrix around an axis.

**Parameters:**

- `axis` (*Vec3*) — The rotation axis.
- `deg` (*float*) — The rotation angle in degrees.

**Returns:** `Mat4`

### `Mat4.compose(pos, rot, scale)` — class

Create a matrix combining translation, rotation, and scale, applied in scale, rotation, translation order.

**Parameters:**

- `pos` (*Vec3*) — The translation.
- `rot` (*Quat*) — The rotation.
- `scale` (*Vec3*) — The per-axis scale.

**Returns:** `Mat4`

**Example:**

```python
mat = Mat4.compose(Vec3(0, 1, 0), Quat.from_euler(Vec3(0, 45, 0)), Vec3.ONE)
```

### `Mat4.look_at(eye, target, up=Vec3.UP)` — class

Create a transform located at eye and facing target. Typically assigned to a camera's transform.

**Parameters:**

- `eye` (*Vec3*) — The viewpoint position.
- `target` (*Vec3*) — The point to look at.
- `up` (*Vec3*) — The up direction. Defaults to Vec3.UP.

**Returns:** `Mat4`

**Example:**

```python
camera.transform = Mat4.look_at(Vec3(0, 3, 8), Vec3.ZERO)
```

### `translate(v)` — function

Return a new matrix moved by v along the matrix's local axes.

**Parameters:**

- `v` (*Vec3*) — The local-space translation.

**Returns:** `Mat4`

### `rotate(axis, deg)` — function

Return a new matrix rotated around a local axis.

**Parameters:**

- `axis` (*Vec3*) — The rotation axis.
- `deg` (*float*) — The rotation angle in degrees.

**Returns:** `Mat4`

### `rotate_x(deg)` — function

Return a new matrix rotated around the local X axis.

**Parameters:**

- `deg` (*float*) — The rotation angle in degrees.

**Returns:** `Mat4`

### `rotate_y(deg)` — function

Return a new matrix rotated around the local Y axis.

**Parameters:**

- `deg` (*float*) — The rotation angle in degrees.

**Returns:** `Mat4`

### `rotate_z(deg)` — function

Return a new matrix rotated around the local Z axis.

**Parameters:**

- `deg` (*float*) — The rotation angle in degrees.

**Returns:** `Mat4`

### `scale_by(v)` — function

Return a new matrix scaled along the matrix's local axes.

**Parameters:**

- `v` (*Vec3*) — The per-axis scale.

**Returns:** `Mat4`

### `inverse()` — function

Return the inverse matrix. A singular matrix returns the identity.

**Returns:** `Mat4`

### `transpose()` — function *(Advanced)*

Return the transposed matrix.

**Returns:** `Mat4`

### `determinant()` — function *(Advanced)*

Return the determinant of the matrix.

**Returns:** `float`

### `to_local(mat)` — function *(Advanced)*

Convert a world-space transform into mat's local space.

**Parameters:**

- `mat` (*Mat4*) — The reference transform.

**Returns:** `Mat4`

### `to_world(mat)` — function *(Advanced)*

Convert a local-space transform into world space through mat.

**Parameters:**

- `mat` (*Mat4*) — The reference transform.

**Returns:** `Mat4`

### `to_local_dir(mat)` — function *(Advanced)*

Convert a world-space transform into mat's local space, ignoring translation.

**Parameters:**

- `mat` (*Mat4*) — The reference transform.

**Returns:** `Mat4`

### `to_world_dir(mat)` — function *(Advanced)*

Convert a local-space transform into world space, ignoring translation.

**Parameters:**

- `mat` (*Mat4*) — The reference transform.

**Returns:** `Mat4`

## Quat

### `Quat(x=0.0, y=0.0, z=0.0, w=1.0)` — class

A quaternion representing a rotation. The default is the identity rotation. Values are immutable; methods return a new Quat.

**Parameters:**

- `x` (*float*) — X component. Defaults to 0.0.
- `y` (*float*) — Y component. Defaults to 0.0.
- `z` (*float*) — Z component. Defaults to 0.0.
- `w` (*float*) — W component. Defaults to 1.0.

**Example:**

```python
from pyxel.cube import Quat, Vec3
q = Quat.from_axis_angle(Vec3.UP, 45)
rotated = q * Vec3.FORWARD
```

**Note:** Supports * with another Quat (composition) or a Vec3 (applies the rotation), unary -, ==, indexing, and iteration.

### `Quat.IDENTITY` — constant

The identity rotation.

- **Type:** `Quat`

### `x` — variable

The x component of the quaternion.

- **Type:** `float`

### `y` — variable

The y component of the quaternion.

- **Type:** `float`

### `z` — variable

The z component of the quaternion.

- **Type:** `float`

### `w` — variable

The w component of the quaternion.

- **Type:** `float`

### `Quat.from_axis_angle(axis, deg)` — class

Create a rotation around an axis.

**Parameters:**

- `axis` (*Vec3*) — The rotation axis.
- `deg` (*float*) — The rotation angle in degrees.

**Returns:** `Quat`

### `Quat.from_euler(rot)` — class

Create a rotation from Euler angles in degrees, applied around the world axes in X, Y, Z order.

**Parameters:**

- `rot` (*Vec3*) — Rotation around each axis in degrees.

**Returns:** `Quat`

### `Quat.from_two_vectors(from_vec, to_vec)` — class

Create the shortest rotation that turns from_vec to face the same direction as to_vec.

**Parameters:**

- `from_vec` (*Vec3*) — The starting direction.
- `to_vec` (*Vec3*) — The target direction.

**Returns:** `Quat`

### `Quat.from_matrix(mat)` — class *(Advanced)*

Extract the rotation of a matrix as a Quat.

**Parameters:**

- `mat` (*Mat4*) — The source matrix.

**Returns:** `Quat`

### `Quat.from_direction(forward, up=Vec3.UP)` — class

Create a rotation facing the forward direction (the forward axis is -Z), keeping up as close as possible.

**Parameters:**

- `forward` (*Vec3*) — The direction to face.
- `up` (*Vec3*) — The up direction. Defaults to Vec3.UP.

**Returns:** `Quat`

### `conjugate()` — function *(Advanced)*

Return the conjugate quaternion.

**Returns:** `Quat`

### `inverse()` — function

Return the inverse rotation.

**Returns:** `Quat`

### `normalize()` — function

Return a unit quaternion pointing in the same direction.

**Returns:** `Quat`

### `length()` — function *(Advanced)*

Return the length of the quaternion.

**Returns:** `float`

### `length_squared()` — function *(Advanced)*

Return the squared length of the quaternion.

**Returns:** `float`

### `dot(other)` — function *(Advanced)*

Return the dot product with another quaternion.

**Parameters:**

- `other` (*Quat*) — The other quaternion.

**Returns:** `float`

### `angle_to(other)` — function

Return the angle between two rotations in degrees.

**Parameters:**

- `other` (*Quat*) — The other rotation.

**Returns:** `float` — The angle in degrees.

### `to_matrix()` — function

Convert the rotation to a Mat4.

**Returns:** `Mat4`

### `to_euler()` — function

Convert the rotation to Euler angles in degrees (same convention as Quat.from_euler).

**Returns:** `Vec3` — Rotation around each axis in degrees.

### `to_axis_angle()` — function

Convert the rotation to an axis and an angle in degrees.

**Returns:** `(Vec3, float)` — A tuple of the axis and the angle in degrees.

### `slerp(other, t)` — function

Spherically interpolate toward another rotation.

**Parameters:**

- `other` (*Quat*) — The target rotation.
- `t` (*float*) — Interpolation factor from 0.0 to 1.0.

**Returns:** `Quat` — The interpolated rotation.

## Camera

### `Camera()` — class

View settings for rendering: position and orientation, field of view, clip range, projection mode, and clear color. Assign it to Node.camera.

**Example:**

```python
from pyxel.cube import Camera, Mat4, Vec3
camera = Camera()
camera.transform = Mat4.look_at(Vec3(0, 3, 8), Vec3.ZERO)
scene.camera = camera
```

### `transform` — variable

The camera's position and orientation in world space. Typically set with Mat4.look_at(). The default is the identity matrix.

- **Type:** `Mat4`

### `fov` — variable

The vertical field of view in degrees, used by the perspective projection. The default is 60.

- **Type:** `float`

### `near` — variable

The near clip distance. Geometry closer than this is not drawn. The default is 0.1.

- **Type:** `float`

### `far` — variable

The far clip distance. The default is 1000.

- **Type:** `float`

### `ortho_size` — variable

When set to a number, switches to an orthographic projection with this view height in world units. None (the default) keeps the perspective projection.

- **Type:** `float | None`

### `clear_color` — variable

When set to a color number, Node.draw() fills the target with it before rendering. None (the default) draws over the existing pixels.

- **Type:** `int | None`

## Shading

### `Shading(colors)` — class

A face-brightness lookup table with a scene-wide light direction. Built automatically from a palette: each color gets 4 brightness levels (0 = darkest, 3 = brightest), where a level is either a flat color or a 2x2 dither pair. Assign it to Node.shading.

**Parameters:**

- `colors` (*list*) — Display colors as 24-bit values. Typically pyxel.colors.

**Example:**

```python
from pyxel.cube import Shading, Vec3
scene.shading = Shading(pyxel.colors)
scene.shading.direction = Vec3(0.4, -0.8, -0.4)
```

**Note:** shading[(col, level)] reads or assigns the (primary, secondary) color pair of a table cell.

### `direction` — variable

The direction the light travels, in world space. Face brightness follows the angle between this and the face normal. The default is Vec3.DOWN.

- **Type:** `Vec3`

### `build(colors)` — function

Rebuild the lookup table from a palette. Manual edits through shading[(col, level)] are overwritten.

**Parameters:**

- `colors` (*list*) — Display colors as 24-bit values.

## Primitive

### `Primitive(mode, positions, indices, normals=[], uvs=[], cull=Primitive.CULL_BACK)` — class

A vertex-data asset: positions, optional normals and UVs, plus the topology (indices and draw mode). One Primitive can be shared by many Node.prim() draws and Mesh parts.

**Parameters:**

- `mode` (*int*) — Draw mode: MODE_POINTS, MODE_LINES, or MODE_TRIANGLES.
- `positions` (*list*) — Vertex coordinates as a flat list of x, y, z repeats.
- `indices` (*list*) — Vertex indices. An empty list consumes the vertices in order 0, 1, 2, ...
- `normals` (*list*) — Per-face normals as a flat list. Defaults to empty (= derived from the vertices while drawing).
- `uvs` (*list*) — Per-vertex UVs as a flat list of u, v repeats. Required for textured draws. Defaults to empty.
- `cull` (*int*) — Face culling mode. Defaults to Primitive.CULL_BACK.

**Example:**

```python
from pyxel.cube import Primitive
prim = Primitive(Primitive.MODE_TRIANGLES, [0, 1, 0, -1, -1, 0, 1, -1, 0], [])
node.prim(Mat4.IDENTITY, prim, 11)
```

**Note:** The list attributes are live proxies: edit elements or assign slices (positions[:] = ...); whole-attribute assignment is not supported.

### `plane(width=1.0, height=1.0)` — function

Create a textured XY-plane Primitive with flat normals.

**Parameters:**

- `width` (*float*) — Full width of the plane.
- `height` (*float*) — Full height of the plane.

**Returns:** `Primitive`

**Example:**

```python
prim = Primitive.plane(2, 2)
```

### `box(size=Vec3.ONE)` — function

Create a textured box Primitive with flat normals.

**Parameters:**

- `size` (*Vec3*) — Full width, height, and depth of the box. Defaults to Vec3.ONE.

**Returns:** `Primitive`

**Example:**

```python
prim = Primitive.box(Vec3(1, 2, 1))
```

### `sphere(radius=0.5)` — function

Create a textured low-poly sphere Primitive with flat normals.

**Parameters:**

- `radius` (*float*) — Sphere radius.

**Returns:** `Primitive`

**Example:**

```python
prim = Primitive.sphere(0.5)
```

### `Primitive.MODE_POINTS` — constant

Draw each vertex as a point.

- **Type:** `int`

### `Primitive.MODE_LINES` — constant

Draw every 2 vertices as a line segment.

- **Type:** `int`

### `Primitive.MODE_TRIANGLES` — constant

Draw every 3 vertices as a filled triangle.

- **Type:** `int`

### `Primitive.CULL_NONE` — constant

Draw both sides of each face.

- **Type:** `int`

### `Primitive.CULL_BACK` — constant

Skip faces seen from the back (counterclockwise winding is the front).

- **Type:** `int`

### `Primitive.CULL_FRONT` — constant

Skip faces seen from the front.

- **Type:** `int`

### `mode` — variable

Draw mode: MODE_POINTS, MODE_LINES, or MODE_TRIANGLES.

- **Type:** `int`

### `positions` — variable

Vertex coordinates as a flat list of x, y, z repeats.

- **Type:** `list`

### `indices` — variable

Vertex indices. An empty list consumes the vertices in order.

- **Type:** `list`

### `normals` — variable

Per-face normals as a flat list. Empty means derived from the vertices while drawing.

- **Type:** `list`

### `uvs` — variable

Per-vertex UVs as a flat list of u, v repeats.

- **Type:** `list`

### `cull` — variable

Face culling mode: CULL_NONE, CULL_BACK, or CULL_FRONT.

- **Type:** `int`

### `compute_normals()` — function

Compute flat per-face normals from positions and indices and store them in normals. Only meaningful for MODE_TRIANGLES.

## Mesh

### `Mesh(primitives=None, transforms=None, parents=None, names=None, col_img=7, colkey=None)` — class

A hierarchical 3D model asset. primitives, transforms, parents, and names are parallel arrays describing the part tree; parents must come before children (parents[i] < i). Instantiate it with Node.from_mesh(), load it with Mesh.from_glb(), or set it on Collider.mesh as static terrain.

**Parameters:**

- `primitives` (*list*) — Primitive per part, or None for a transform-only group part. Defaults to empty.
- `transforms` (*list*) — Local transform of each part relative to its parent. Defaults to empty.
- `parents` (*list*) — Parent index of each part. -1 is a root. Defaults to empty.
- `names` (*list*) — Node name per part. Defaults to empty strings.
- `col_img` (*int | Image*) — Flat color number, or a texture Image shared by every part. Defaults to 7.
- `colkey` (*int | None*) — Transparent color when col_img is an Image. Defaults to None.

**Example:**

```python
from pyxel.cube import Mat4, Mesh, Node, Primitive
mesh = Mesh(primitives=[prim], transforms=[Mat4.IDENTITY], parents=[-1], col_img=8)
mesh = Mesh.from_glb("actor.glb", colkey=0)
node = Node.from_mesh(mesh)
```

### `primitives` — variable

Primitive per part. None means a transform-only group part.

- **Type:** `list`

### `transforms` — variable

Local transform of each part relative to its parent.

- **Type:** `list`

### `parents` — variable

Parent index of each part. -1 is a root; parents[i] < i is required.

- **Type:** `list`

### `names` — variable

Node name per part.

- **Type:** `list`

### `motions` — variable

Transform animation clips imported with the mesh.

- **Type:** `list[Motion]`

### `col_img` — variable

Flat color number, or a texture Image shared by every part.

- **Type:** `int | Image`

### `colkey` — variable

Transparent color when col_img is an Image.

- **Type:** `int | None`

### `from_glb(filename, *, colkey=None, fps=30.0)` — function

Load a binary glTF (.glb) file into a Mesh. Embedded buffers and a single embedded texture are supported; imported transform animations are exposed through motions.

**Parameters:**

- `filename` (*str*) — Path to the .glb file.
- `colkey` (*int | None*) — Transparent palette color for the imported texture. Defaults to None.
- `fps` (*float*) — Frames per second used to convert glTF animation times into Pyxel frames. Defaults to 30.0.

**Returns:** `Mesh` — The loaded mesh.

**Example:**

```python
mesh = Mesh.from_glb("actor.glb", colkey=0)
actor = Node.from_mesh(mesh)
```

**Note:** Texture pixels must be fully opaque; use colkey for transparency. Multiple textures, external files, skins, morph targets, and material animation are rejected.

### `descendants(i)` — function *(Advanced)*

Return the indices of every part under part i.

**Parameters:**

- `i` (*int*) — The root part index.

**Returns:** `list` — Descendant part indices.

## Motion

### `Motion` — class

An imported transform animation clip owned by a Mesh. Use Node.apply_motion() for one-shot sampling or Node.play_motion() for per-update playback.

**Example:**

```python
motion = mesh.motions[0]
actor.apply_motion(motion, frame=0)
actor.play_motion(motion)
```

### `name` — variable

Animation name from the GLB file.

- **Type:** `str`

### `length` — variable

Clip length in Pyxel frames.

- **Type:** `float`

## Collider

### `Collider(size=Vec3.ZERO, radius=0.0, mesh=None, trigger=False, rolls=False, mass=1.0, restitution=0.0, friction=0.5, velocity=Vec3.ZERO, angular_velocity=Vec3.ZERO)` — class

Collision shape, physical coefficients, and motion state for a Node. The shape follows from size and radius: size of all zeros is a sphere, (0, h, 0) is a capsule of height h, and any other size is a box with rounded corners of the radius. Setting mesh turns the collider into static triangle terrain.

**Parameters:**

- `size` (*Vec3*) — Core dimensions of the shape. Defaults to Vec3.ZERO (a sphere).
- `radius` (*float*) — Sphere or capsule radius, and the corner rounding of a box. Defaults to 0.0.
- `mesh` (*Mesh | None*) — Static terrain mesh. Defaults to None.
- `trigger` (*bool*) — When True, reports contacts without any push-back. Defaults to False.
- `rolls` (*bool*) — When True, contacts also produce delta_angular_velocity. Defaults to False.
- `mass` (*float*) — Mass for contact resolution. 0.0 makes the body immovable. Defaults to 1.0.
- `restitution` (*float*) — Bounciness. The larger of the two contacting values is used. Defaults to 0.0.
- `friction` (*float*) — Friction. The average of the two contacting values is used. Defaults to 0.5.
- `velocity` (*Vec3*) — World-space displacement applied every update. Defaults to Vec3.ZERO.
- `angular_velocity` (*Vec3*) — Axis times angle (in degrees) applied as a local spin every update. Defaults to Vec3.ZERO.

**Example:**

```python
from pyxel.cube import Collider, Vec3
node.collider = Collider(radius=0.5)
floor.collider = Collider(size=Vec3(20, 0.5, 20), mass=0.0)
```

### `size` — variable

Core dimensions deciding the shape family.

- **Type:** `Vec3`

### `radius` — variable

Sphere or capsule radius, and the corner rounding of a box.

- **Type:** `float`

### `mesh` — variable

Static terrain mesh. A mesh collider never moves.

- **Type:** `Mesh | None`

### `trigger` — variable

When True, reports contacts without any push-back.

- **Type:** `bool`

### `rolls` — variable

When True, contacts also produce delta_angular_velocity.

- **Type:** `bool`

### `mass` — variable

Mass for contact resolution. 0.0 makes the body immovable.

- **Type:** `float`

### `restitution` — variable

Bounciness. The larger of the two contacting values is used.

- **Type:** `float`

### `friction` — variable

Friction. The average of the two contacting values is used.

- **Type:** `float`

### `velocity` — variable

World-space displacement applied to the node's transform every update.

- **Type:** `Vec3`

### `angular_velocity` — variable

Axis times angle (in degrees) applied as a local spin every update.

- **Type:** `Vec3`

## Contact

### `Contact()` — class

Collision payload passed to Node.on_collide(). Carries the contact geometry and the engine-resolved motion corrections for the receiving side.

**Example:**

```python
def on_collide(self, other, contact):
    push = Mat4.from_translation(contact.normal * contact.depth)
    self.transform = push * self.transform
    self.collider.velocity += contact.delta_velocity
```

### `point` — variable

The contact point in world space.

- **Type:** `Vec3`

### `normal` — variable

The contact normal in world space, pointing from the other node toward this node.

- **Type:** `Vec3`

### `depth` — variable

Penetration depth this side should resolve, already split by the mass ratio.

- **Type:** `float`

### `delta_rotation` — variable

Rotation correction reserved for a future response; currently always the identity, so applying it is a harmless no-op.

- **Type:** `Quat`

### `delta_velocity` — variable

Suggested additive velocity correction from the collision.

- **Type:** `Vec3`

### `delta_angular_velocity` — variable

Suggested additive angular velocity correction. Non-zero only when the collider has rolls=True.

- **Type:** `Vec3`

## RaycastHit

### `RaycastHit()` — class

Hit information returned by Node.raycast() and Node.raycast_all().

**Example:**

```python
hit = scene.raycast(origin, Vec3.DOWN, max_distance=10.0)
if hit is not None:
    ground_y = hit.point.y
```

### `node` — variable

The node that was hit.

- **Type:** `Node`

### `point` — variable

The hit position in world space.

- **Type:** `Vec3`

### `normal` — variable

The surface normal at the hit, facing the ray origin's side.

- **Type:** `Vec3`

### `distance` — variable

The distance from the ray origin to the hit.

- **Type:** `float`

## Node

### `Node()` — class

A scene-graph node. Subclass it and implement on_update / on_draw, instantiate Mesh assets with Node.from_mesh(), then drive the whole tree with update() and draw() on the root. Draw commands render relative to the node's world transform.

**Example:**

```python
from pyxel.cube import Camera, Mat4, Node, Shading, Vec3
class Scene(Node):
    def on_draw(self):
        self.box(Mat4.IDENTITY, Vec3(2, 2, 2), 8)
scene = Scene()
scene.shading = Shading(pyxel.colors)
scene.camera = Camera()
scene.camera.transform = Mat4.look_at(Vec3(3, 2, 4), Vec3.ZERO)
```

### `name` — variable

Node name, searchable with find_by_name().

- **Type:** `str`

### `transform` — variable

Local transform relative to the parent.

- **Type:** `Mat4`

### `active` — variable

When False, stops update and collision for this node and its descendants.

- **Type:** `bool`

### `visible` — variable

When False, stops drawing for this node and its descendants.

- **Type:** `bool`

### `camera` — variable

Camera used for drawing. None inherits from the closest ancestor that has one.

- **Type:** `Camera | None`

### `shading` — variable

Face-brightness table used for drawing. None inherits from the closest ancestor that has one.

- **Type:** `Shading | None`

### `collider` — variable

Collision shape and motion state. None keeps the node out of collision.

- **Type:** `Collider | None`

### `tags` — variable

Tag strings used by find_by_tags() and the tags filter of spatial queries.

- **Type:** `list`

### `parent` — variable

The parent node, or None for a root.

- **Type:** `Node | None`

### `children` — variable

The child nodes as a tuple.

- **Type:** `tuple`

### `destroyed` — variable

True after destroy() has been called, until the node is detached at the end of update().

- **Type:** `bool`

### `forward` — variable

The transform's forward direction (-Z axis) as a unit vector.

- **Type:** `Vec3`

### `right` — variable

The transform's right direction (+X axis) as a unit vector.

- **Type:** `Vec3`

### `up` — variable

The transform's up direction (+Y axis) as a unit vector.

- **Type:** `Vec3`

### `effective_camera` — variable *(Advanced)*

The camera after cascade resolution: this node's camera, or the closest ancestor's.

- **Type:** `Camera | None`

### `world_transform` — variable *(Advanced)*

The world transform composed from the root down to this node.

- **Type:** `Mat4`

### `add_child(node)` — function

Add a node as a child. A node that already has a parent is reparented.

**Parameters:**

- `node` (*Node*) — The node to add.

### `from_mesh(mesh)` — function

Create a Node tree from a Mesh and return its root node.

**Parameters:**

- `mesh` (*Mesh*) — The mesh asset to instantiate.

**Returns:** `Node` — The generated root node.

### `remove_child(node)` — function

Remove a child node.

**Parameters:**

- `node` (*Node*) — The node to remove.

### `destroy()` — function

Flag this node and its descendants for destruction. At the end of update(), on_destroy() fires on each flagged node and it is detached from the tree.

### `apply_motion(motion, frame, *, loop=True)` — function

Sample a Motion at the given frame and immediately apply it to this Node.from_mesh() subtree.

**Parameters:**

- `motion` (*Motion*) — Motion clip imported with the same Mesh as this subtree.
- `frame` (*float*) — Frame to sample.
- `loop` (*bool*) — When True, wrap the frame inside the clip length. Defaults to True.

### `play_motion(motion, *, loop=True, speed=1.0, start_frame=0.0)` — function

Start per-update playback of a Motion on this Node.from_mesh() subtree.

**Parameters:**

- `motion` (*Motion*) — Motion clip imported with the same Mesh as this subtree.
- `loop` (*bool*) — When True, wrap playback inside the clip length. Defaults to True.
- `speed` (*float*) — Frames advanced per update. Defaults to 1.0.
- `start_frame` (*float*) — Initial frame sampled when playback starts. Defaults to 0.0.

### `stop_motion()` — function

Stop the active Motion playback cursor on this node.

### `find_by_name(name)` — function

Return every node in this subtree whose name matches.

**Parameters:**

- `name` (*str*) — The name to search for.

**Returns:** `list` — The matching nodes.

### `find_by_tags(tags)` — function

Return every node in this subtree carrying any of the given tags.

**Parameters:**

- `tags` (*list*) — Tags to search for.

**Returns:** `list` — The matching nodes.

### `on_update()` — function

Per-frame update hook, called by update() in tree order. Override it in a subclass.

### `on_draw()` — function

Per-frame draw hook, called by draw() in tree order. Call draw commands from here.

### `on_collide(other, contact)` — function

Collision hook, called by update() once per contacting pair. Apply contact.depth and contact.delta_velocity here to resolve the hit.

**Parameters:**

- `other` (*Node*) — The other node of the contact.
- `contact` (*Contact*) — The contact information for this side.

### `on_destroy()` — function

Destruction hook, called at the end of update() before the node is detached.

### `dither(alpha)` — function

Apply dithered transparency to the following draw commands in this on_draw. Resets to 1.0 at the start of every node's on_draw.

**Parameters:**

- `alpha` (*float*) — Opacity from 0.0 to 1.0.

### `depth_test(on)` — function

Toggle the depth test for the following draw commands in this on_draw. Resets to True at the start of every node's on_draw.

**Parameters:**

- `on` (*bool*) — False draws over everything regardless of depth.

### `depth_write(on)` — function *(Advanced)*

Toggle depth-buffer writes for the following draw commands in this on_draw. Resets to True at the start of every node's on_draw.

**Parameters:**

- `on` (*bool*) — False leaves the depth buffer untouched.

### `depth_offset(offset)` — function *(Advanced)*

Shift only the depth of the following draw commands along the view direction, in world units. Negative is toward the camera. The screen position and size never change. Resets to 0.0 at the start of every node's on_draw.

**Parameters:**

- `offset` (*float*) — Depth shift in world units. Negative is toward the camera.

### `shaded(on)` — function

Toggle directional shading for the following draw commands in this on_draw. Resets to True at the start of every node's on_draw.

**Parameters:**

- `on` (*bool*) — False draws with the base color only.

### `pset(pos, col)` — function

Draw a single pixel at a position.

**Parameters:**

- `pos` (*Vec3*) — Position in node-local coordinates.
- `col` (*int*) — Color number.

### `line(p1, p2, col)` — function

Draw a 1-pixel-wide line segment.

**Parameters:**

- `p1` (*Vec3*) — Start position.
- `p2` (*Vec3*) — End position.
- `col` (*int*) — Color number.

### `tri(p1, p2, p3, col)` — function

Draw a filled triangle.

**Parameters:**

- `p1` (*Vec3*) — First vertex.
- `p2` (*Vec3*) — Second vertex.
- `p3` (*Vec3*) — Third vertex.
- `col` (*int*) — Color number.

### `trib(p1, p2, p3, col)` — function

Draw a triangle outline.

**Parameters:**

- `p1` (*Vec3*) — First vertex.
- `p2` (*Vec3*) — Second vertex.
- `p3` (*Vec3*) — Third vertex.
- `col` (*int*) — Color number.

### `rect(mat, w, h, col)` — function

Draw a filled rectangle of size w x h on mat's local XY plane.

**Parameters:**

- `mat` (*Mat4*) — Placement relative to the node.
- `w` (*float*) — Width in world units.
- `h` (*float*) — Height in world units.
- `col` (*int*) — Color number.

### `rectb(mat, w, h, col)` — function

Draw a rectangle outline on mat's local XY plane.

**Parameters:**

- `mat` (*Mat4*) — Placement relative to the node.
- `w` (*float*) — Width in world units.
- `h` (*float*) — Height in world units.
- `col` (*int*) — Color number.

### `circ(pos, r, col)` — function

Draw a filled circle that always faces the camera.

**Parameters:**

- `pos` (*Vec3*) — Center position.
- `r` (*float*) — Radius in world units.
- `col` (*int*) — Color number.

### `circb(pos, r, col)` — function

Draw a circle outline that always faces the camera.

**Parameters:**

- `pos` (*Vec3*) — Center position.
- `r` (*float*) — Radius in world units.
- `col` (*int*) — Color number.

### `elli(mat, w, h, col)` — function

Draw a filled ellipse of size w x h on mat's local XY plane.

**Parameters:**

- `mat` (*Mat4*) — Placement relative to the node.
- `w` (*float*) — Width in world units.
- `h` (*float*) — Height in world units.
- `col` (*int*) — Color number.

### `ellib(mat, w, h, col)` — function

Draw an ellipse outline on mat's local XY plane.

**Parameters:**

- `mat` (*Mat4*) — Placement relative to the node.
- `w` (*float*) — Width in world units.
- `h` (*float*) — Height in world units.
- `col` (*int*) — Color number.

### `box(mat, size, col_img=7, colkey=None)` — function

Draw a filled box. When col_img is an Image, every face is textured with the whole image.

**Parameters:**

- `mat` (*Mat4*) — Placement relative to the node.
- `size` (*Vec3*) — Edge lengths along each axis.
- `col_img` (*int | Image*) — Flat color number or a texture Image. Defaults to 7.
- `colkey` (*int | None*) — Transparent color of the texture. Defaults to None.

**Example:**

```python
self.box(Mat4.IDENTITY, Vec3(2, 2, 2), 8)
```

### `boxb(mat, size, col)` — function

Draw the 12 edges of a box.

**Parameters:**

- `mat` (*Mat4*) — Placement relative to the node.
- `size` (*Vec3*) — Edge lengths along each axis.
- `col` (*int*) — Color number.

### `sphere(pos, r, col_img=7, colkey=None)` — function

Draw a filled sphere (a subdivided icosahedron of 80 faces). When col_img is an Image, it is wrapped with an equirectangular mapping.

**Parameters:**

- `pos` (*Vec3*) — Center position.
- `r` (*float*) — Radius in world units.
- `col_img` (*int | Image*) — Flat color number or a texture Image. Defaults to 7.
- `colkey` (*int | None*) — Transparent color of the texture. Defaults to None.

### `sphereb(pos, r, col)` — function

Draw the wireframe edges of a sphere.

**Parameters:**

- `pos` (*Vec3*) — Center position.
- `r` (*float*) — Radius in world units.
- `col` (*int*) — Color number.

### `plane(mat, img, uvs, w, h, colkey=None)` — function

Draw a textured rectangle of size w x h on mat's local XY plane.

**Parameters:**

- `mat` (*Mat4*) — Placement relative to the node.
- `img` (*Image*) — Texture image.
- `uvs` (*tuple*) — UV coordinates of the four corners: top-left, top-right, bottom-left, bottom-right.
- `w` (*float*) — Width in world units.
- `h` (*float*) — Height in world units.
- `colkey` (*int | None*) — Transparent color. Defaults to None.

### `sprite(pos, img, uvs, w, h, colkey=None, angle=0.0)` — function

Draw a textured rectangle that always faces the camera. Sprites render unshaded.

**Parameters:**

- `pos` (*Vec3*) — Center position.
- `img` (*Image*) — Texture image.
- `uvs` (*tuple*) — UV coordinates of the four corners: top-left, top-right, bottom-left, bottom-right.
- `w` (*float*) — Width in world units.
- `h` (*float*) — Height in world units.
- `colkey` (*int | None*) — Transparent color. Defaults to None.
- `angle` (*float*) — Screen-space rotation in degrees. Defaults to 0.0.

### `prim(mat, primitive, col_img=7, colkey=None)` — function

Draw a Primitive asset with its own mode and culling.

**Parameters:**

- `mat` (*Mat4*) — Placement relative to the node.
- `primitive` (*Primitive*) — The vertex data to draw.
- `col_img` (*int | Image*) — Flat color number or a texture Image. Defaults to 7.
- `colkey` (*int | None*) — Transparent color of the texture. Defaults to None.

### `text(pos, s, col, font=None)` — function

Draw a screen-space string centered at the projected position. Glyphs keep their pixel size at any distance; the depth test still applies.

**Parameters:**

- `pos` (*Vec3*) — Anchor position.
- `s` (*str*) — The string to draw.
- `col` (*int*) — Color number.
- `font` (*Font | None*) — BDF font to use. Defaults to None (the built-in font).

### `update()` — function

Advance this subtree by one frame: on_update hooks, node motion playback, collider motion, collision detection, on_collide hooks, then on_destroy and detachment of destroyed nodes.

### `draw(x, y, w, h, target=None)` — function

Render this subtree into the viewport (x, y, w, h). A camera must be set on this node or an ancestor. When the camera has a clear_color, the target is filled with it first.

**Parameters:**

- `x` (*int*) — Left edge of the viewport.
- `y` (*int*) — Top edge of the viewport.
- `w` (*int*) — Viewport width in pixels.
- `h` (*int*) — Viewport height in pixels.
- `target` (*Image | None*) — Render target. Defaults to None (the screen).

**Example:**

```python
scene.draw(0, 0, pyxel.width, pyxel.height)
```

### `raycast(origin, direction, max_distance=None, hit_triggers=False, tags=None)` — function

Cast a ray against the colliders in this subtree and return the closest hit.

**Parameters:**

- `origin` (*Vec3*) — Ray origin in world space.
- `direction` (*Vec3*) — Ray direction.
- `max_distance` (*float | None*) — Maximum hit distance. Defaults to None (unlimited).
- `hit_triggers` (*bool*) — When True, trigger colliders can also be hit. Defaults to False.
- `tags` (*list | None*) — When set, only nodes carrying any of these tags are tested. Defaults to None.

**Returns:** `RaycastHit | None` — The closest hit, or None.

**Example:**

```python
hit = scene.raycast(self.transform.pos, Vec3.DOWN, max_distance=2.0)
```

### `raycast_all(origin, direction, max_distance=None, hit_triggers=False, tags=None)` — function

Cast a ray and return every hit sorted by distance.

**Parameters:**

- `origin` (*Vec3*) — Ray origin in world space.
- `direction` (*Vec3*) — Ray direction.
- `max_distance` (*float | None*) — Maximum hit distance. Defaults to None (unlimited).
- `hit_triggers` (*bool*) — When True, trigger colliders can also be hit. Defaults to False.
- `tags` (*list | None*) — When set, only nodes carrying any of these tags are tested. Defaults to None.

**Returns:** `list` — Every hit sorted by distance.

### `overlap_sphere(center, radius, hit_triggers=False, tags=None)` — function

Return every node in this subtree whose collider overlaps the given sphere.

**Parameters:**

- `center` (*Vec3*) — Sphere center in world space.
- `radius` (*float*) — Sphere radius.
- `hit_triggers` (*bool*) — When True, trigger colliders are also reported. Defaults to False.
- `tags` (*list | None*) — When set, only nodes carrying any of these tags are tested. Defaults to None.

**Returns:** `list` — The overlapping nodes.

### `overlap_box(mat, size, hit_triggers=False, tags=None)` — function

Return every node in this subtree whose collider overlaps the given box.

**Parameters:**

- `mat` (*Mat4*) — Box placement in world space.
- `size` (*Vec3*) — Edge lengths along each axis.
- `hit_triggers` (*bool*) — When True, trigger colliders are also reported. Defaults to False.
- `tags` (*list | None*) — When set, only nodes carrying any of these tags are tested. Defaults to None.

**Returns:** `list` — The overlapping nodes.
