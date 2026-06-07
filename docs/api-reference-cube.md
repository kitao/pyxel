<!-- This file is generated from web/api-reference/cube/api-reference.json. -->

# Pyxel Cube API Reference

*This document was auto-generated from the [Pyxel Cube API Reference](https://kitao.github.io/pyxel/web/api-reference/cube/) web page, which also offers multilingual support.*

## Vec3

### `Vec3(x, y, z)` — class

A 3D vector with x, y, and z components.

**Parameters:**

- `x` (*float*) — X component (default 0.0).
- `y` (*float*) — Y component (default 0.0).
- `z` (*float*) — Z component (default 0.0).

**Example:**

```python
from pyxel.cube import Vec3
v = Vec3(1.0, 2.0, 3.0)
length = v.length()
```

**Note:** Import it from the pyxel.cube module.

### `x` — variable

The x component of the vector.

- **Type:** `float`

### `Vec3.UP` — constant

Unit vector pointing up.

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

### `normalize()` — function

Return a unit vector pointing in the same direction.

**Returns:** `Vec3` — The normalized vector.

### `lerp(other, t)` — function

Linearly interpolate toward another vector.

**Parameters:**

- `other` (*Vec3*) — The target vector.
- `t` (*float*) — Interpolation factor from 0.0 to 1.0.

**Returns:** `Vec3` — The interpolated vector.
