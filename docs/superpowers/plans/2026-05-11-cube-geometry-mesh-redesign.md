# Pyxel Cube Geometry / Mesh Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Restructure cube's asset layer per `docs/superpowers/specs/2026-05-11-cube-geometry-mesh-redesign-design.md`: introduce `Geometry` class, redesign `Mesh` as a hierarchical container of geometries, drop `FloatBuffer` / `IntBuffer` in favor of native `list[float]` / `list[int]`, and update `node.prim` / `node.mesh` accordingly.

**Architecture:** Pyxel's three-layer build pyxel-core (Rust, business logic) → pyxel-binding (Rust, PyO3 wrapper) → pyxel (Python, public API). Each new class adds: core struct + `define_rc_type!`, binding wrapper + `define_wrapper!` + `#[pymethods]`, Python re-export in `cube/__init__.py`, type hint in `cube/__init__.pyi`, and Python tests under `python/tests/cube/`. Implementation order keeps the build green between commits by adding new classes before removing old ones.

**Tech Stack:** Rust (edition 2021), PyO3, maturin, Python ≥ 3.11, pytest

**Reference:** Spec at `docs/superpowers/specs/2026-05-11-cube-geometry-mesh-redesign-design.md`. Read it before starting any task.

---

## File Structure

### Files to Create

- `crates/pyxel-core/src/cube/geometry.rs` — Geometry struct, RcGeometry, normal auto-compute logic
- `crates/pyxel-binding/src/cube/geometry.rs` — PyO3 wrapper for Geometry with class constants and property access
- `python/tests/cube/test_geometry.py` — Geometry public API tests
- `docs/superpowers/plans/2026-05-11-cube-geometry-mesh-redesign.md` — this plan

### Files to Modify (replace contents)

- `crates/pyxel-core/src/cube/mesh.rs` — replace old field set with `geometries / transforms / parents / col_img / colkey`, add `descendants` method, parent topological-order validation
- `crates/pyxel-binding/src/cube/mesh.rs` — new wrapper matching the rewritten core Mesh
- `crates/pyxel-core/src/cube/mod.rs` — register `geometry` module; remove `float_buffer` / `int_buffer` modules after dependents are migrated
- `crates/pyxel-binding/src/cube/mod.rs` — register `geometry`, register order updates; remove `float_buffer` / `int_buffer` after dependents are migrated
- `crates/pyxel-binding/src/cube/node.rs` — update `prim` signature (single `Geometry` argument) and `mesh` signature (drop `col`); both rename `col` → `col_img`
- `crates/pyxel-core/src/cube/draw.rs` — adjust prim path to read mode / cull / vertex attrs from Geometry; adjust mesh path to walk parallel arrays and compose world transforms
- `python/pyxel/cube/__init__.pyi` — new Geometry / Mesh type hints; new prim / mesh signatures; remove FloatBuffer / IntBuffer
- `python/pyxel/cube/__init__.py` — add `Geometry` re-export, remove `FloatBuffer` / `IntBuffer`
- `python/tests/cube/test_mesh.py` — rewrite against new Mesh shape
- `python/tests/cube/test_node.py` — update prim / mesh test cases against new signatures
- `docs/cube-design.md` — delete §10 (buffers); replace §11 (Mesh); adjust §12.1 (move PRIM_* constants off Node); adjust §12.5 (node.prim and node.mesh signatures); update §16 with new rejected alternatives

### Files to Delete (after dependents are migrated)

- `crates/pyxel-core/src/cube/float_buffer.rs`
- `crates/pyxel-core/src/cube/int_buffer.rs`
- `crates/pyxel-binding/src/cube/float_buffer.rs`
- `crates/pyxel-binding/src/cube/int_buffer.rs`
- `python/tests/cube/test_float_buffer.py`
- `python/tests/cube/test_int_buffer.py`

---

## Phase 1: Geometry Core

Introduce `Geometry` core struct + tests. This phase adds new code without touching existing classes; the build stays green.

### Task 1: Create Geometry core struct

**Files:**
- Create: `crates/pyxel-core/src/cube/geometry.rs`
- Modify: `crates/pyxel-core/src/cube/mod.rs`

- [ ] **Step 1: Write the failing tests**

```rust
// crates/pyxel-core/src/cube/geometry.rs

use crate::cube::vec3::Vec3;

pub const PRIM_POINTS: i32 = 0;
pub const PRIM_LINES: i32 = 1;
pub const PRIM_TRIANGLES: i32 = 2;

pub const CULL_NONE: i32 = 0;
pub const CULL_BACK: i32 = 1;
pub const CULL_FRONT: i32 = 2;

pub struct Geometry {
    pub positions: Vec<f32>,
    pub normals: Option<Vec<f32>>,
    pub uvs: Option<Vec<f32>>,
    pub indices: Option<Vec<i32>>,
    pub prim: i32,
    pub cull: i32,
}

define_rc_type!(RcGeometry, Geometry);

impl Geometry {
    pub fn new() -> RcGeometry {
        new_rc_type!(Geometry {
            positions: Vec::new(),
            normals: None,
            uvs: None,
            indices: None,
            prim: PRIM_TRIANGLES,
            cull: CULL_BACK,
        })
    }

    pub fn compute_normals(&mut self, smooth: bool) {
        let n = self.positions.len() / 3;
        if n < 3 {
            self.normals = Some(vec![0.0; self.positions.len()]);
            return;
        }
        // Build per-vertex normals from face normals, flat or smoothed.
        let mut out = vec![0.0f32; self.positions.len()];
        let triangles: Vec<[usize; 3]> = match &self.indices {
            Some(idx) if self.prim == PRIM_TRIANGLES => idx
                .chunks_exact(3)
                .map(|c| [c[0] as usize, c[1] as usize, c[2] as usize])
                .collect(),
            _ if self.prim == PRIM_TRIANGLES => (0..n / 3)
                .map(|i| [i * 3, i * 3 + 1, i * 3 + 2])
                .collect(),
            _ => Vec::new(),
        };
        for &[a, b, c] in &triangles {
            let pa = read_vec3(&self.positions, a);
            let pb = read_vec3(&self.positions, b);
            let pc = read_vec3(&self.positions, c);
            let n = (pb - pa).cross(&(pc - pa)).normalize_or_zero();
            if smooth {
                add_vec3(&mut out, a, &n);
                add_vec3(&mut out, b, &n);
                add_vec3(&mut out, c, &n);
            } else {
                write_vec3(&mut out, a, &n);
                write_vec3(&mut out, b, &n);
                write_vec3(&mut out, c, &n);
            }
        }
        if smooth {
            for i in 0..n {
                let v = read_vec3(&out, i).normalize_or_zero();
                write_vec3(&mut out, i, &v);
            }
        }
        self.normals = Some(out);
    }
}

fn read_vec3(buf: &[f32], i: usize) -> Vec3 {
    Vec3::new(buf[i * 3], buf[i * 3 + 1], buf[i * 3 + 2])
}

fn write_vec3(buf: &mut [f32], i: usize, v: &Vec3) {
    buf[i * 3] = v.x;
    buf[i * 3 + 1] = v.y;
    buf[i * 3 + 2] = v.z;
}

fn add_vec3(buf: &mut [f32], i: usize, v: &Vec3) {
    buf[i * 3] += v.x;
    buf[i * 3 + 1] += v.y;
    buf[i * 3 + 2] += v.z;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_defaults() {
        let g = Geometry::new();
        let g = rc_ref!(&g);
        assert!(g.positions.is_empty());
        assert!(g.normals.is_none());
        assert!(g.uvs.is_none());
        assert!(g.indices.is_none());
        assert_eq!(g.prim, PRIM_TRIANGLES);
        assert_eq!(g.cull, CULL_BACK);
    }

    #[test]
    fn test_compute_normals_flat_triangle() {
        let g = Geometry::new();
        {
            let g = rc_mut!(&g);
            g.positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
            g.compute_normals(false);
        }
        let g = rc_ref!(&g);
        let n = g.normals.as_ref().unwrap();
        assert_eq!(n.len(), 9);
        // Normal of +Z face: (0, 0, 1)
        for v in n.chunks(3) {
            assert!((v[0]).abs() < 1e-5);
            assert!((v[1]).abs() < 1e-5);
            assert!((v[2] - 1.0).abs() < 1e-5);
        }
    }
}
```

Add to `crates/pyxel-core/src/cube/mod.rs`:
```rust
pub mod geometry;
pub use geometry::{Geometry, RcGeometry};
```

(The `PRIM_*` / `CULL_*` constants stay namespaced under `pyxel::cube::geometry::PRIM_TRIANGLES` etc. — no top-level re-export needed.)

In Task 4 (Mesh rewrite), also add to the same file:
```rust
pub use mesh::{ColImage, Mesh, RcMesh};
```
(adding `ColImage` to the existing re-export line).

- [ ] **Step 2: Run tests to verify they pass**

Run: `cd crates && cargo test -p pyxel-core cube::geometry -- --nocapture`
Expected: 2 tests pass.

- [ ] **Step 3: Commit**

```bash
git add crates/pyxel-core/src/cube/geometry.rs crates/pyxel-core/src/cube/mod.rs
git commit -m "Add Pyxel Cube Geometry core struct with compute_normals"
```

---

## Phase 2: Geometry Binding

Wire Geometry to PyO3 with property access and class constants.

### Task 2: Create Geometry binding wrapper

**Files:**
- Create: `crates/pyxel-binding/src/cube/geometry.rs`
- Modify: `crates/pyxel-binding/src/cube/mod.rs`

- [ ] **Step 1: Write the binding wrapper**

```rust
// crates/pyxel-binding/src/cube/geometry.rs

use pyo3::prelude::*;

define_wrapper!(Geometry, pyxel::cube::Geometry);

#[pymethods]
impl Geometry {
    // Constants — topology mode

    #[classattr]
    const PRIM_POINTS: i32 = pyxel::cube::geometry::PRIM_POINTS;

    #[classattr]
    const PRIM_LINES: i32 = pyxel::cube::geometry::PRIM_LINES;

    #[classattr]
    const PRIM_TRIANGLES: i32 = pyxel::cube::geometry::PRIM_TRIANGLES;

    // Constants — back-face cull

    #[classattr]
    const CULL_NONE: i32 = pyxel::cube::geometry::CULL_NONE;

    #[classattr]
    const CULL_BACK: i32 = pyxel::cube::geometry::CULL_BACK;

    #[classattr]
    const CULL_FRONT: i32 = pyxel::cube::geometry::CULL_FRONT;

    // Constructor

    #[new]
    #[pyo3(signature = (positions=None, normals=None, uvs=None, indices=None, prim=pyxel::cube::geometry::PRIM_TRIANGLES, cull=pyxel::cube::geometry::CULL_BACK))]
    fn new(
        positions: Option<Vec<f32>>,
        normals: Option<Vec<f32>>,
        uvs: Option<Vec<f32>>,
        indices: Option<Vec<i32>>,
        prim: i32,
        cull: i32,
    ) -> Self {
        let g = pyxel::cube::Geometry::new();
        {
            let g = rc_mut!(&g);
            if let Some(p) = positions {
                g.positions = p;
            }
            g.normals = normals;
            g.uvs = uvs;
            g.indices = indices;
            g.prim = prim;
            g.cull = cull;
        }
        Self::wrap(g)
    }

    // Vertex attributes

    #[getter]
    fn positions(&self) -> Vec<f32> {
        self.inner_ref().positions.clone()
    }

    #[setter]
    fn set_positions(&self, v: Vec<f32>) {
        self.inner_mut().positions = v;
    }

    #[getter]
    fn normals(&self) -> Option<Vec<f32>> {
        self.inner_ref().normals.clone()
    }

    #[setter]
    fn set_normals(&self, v: Option<Vec<f32>>) {
        self.inner_mut().normals = v;
    }

    #[getter]
    fn uvs(&self) -> Option<Vec<f32>> {
        self.inner_ref().uvs.clone()
    }

    #[setter]
    fn set_uvs(&self, v: Option<Vec<f32>>) {
        self.inner_mut().uvs = v;
    }

    // Topology

    #[getter]
    fn indices(&self) -> Option<Vec<i32>> {
        self.inner_ref().indices.clone()
    }

    #[setter]
    fn set_indices(&self, v: Option<Vec<i32>>) {
        self.inner_mut().indices = v;
    }

    #[getter]
    fn prim(&self) -> i32 {
        self.inner_ref().prim
    }

    #[setter]
    fn set_prim(&self, v: i32) {
        self.inner_mut().prim = v;
    }

    // Back-face cull

    #[getter]
    fn cull(&self) -> i32 {
        self.inner_ref().cull
    }

    #[setter]
    fn set_cull(&self, v: i32) {
        self.inner_mut().cull = v;
    }

    // Methods

    #[pyo3(signature = (smooth=false))]
    fn compute_normals(&self, smooth: bool) {
        self.inner_mut().compute_normals(smooth);
    }

    // Dunder

    fn __repr__(&self) -> String {
        let g = self.inner_ref();
        format!(
            "Geometry(positions={}, prim={}, cull={})",
            g.positions.len(),
            g.prim,
            g.cull
        )
    }
}

pub fn add_geometry_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Geometry>()?;
    Ok(())
}
```

Edit `crates/pyxel-binding/src/cube/mod.rs`:
1. Add `mod geometry;` before `mod mesh;`
2. Add `geometry::add_geometry_class(&m)?;` inside `add_cube_submodule` after `int_buffer::add_int_buffer_class(&m)?;`

- [ ] **Step 2: Build the binding**

Run: `make build` (or `cd crates && cargo build -p pyxel-binding`)
Expected: builds successfully, no warnings.

- [ ] **Step 3: Commit**

```bash
git add crates/pyxel-binding/src/cube/geometry.rs crates/pyxel-binding/src/cube/mod.rs
git commit -m "Add Pyxel Cube Geometry binding wrapper"
```

---

## Phase 3: Python Tests for Geometry

### Task 3: Write Geometry public API tests

**Files:**
- Create: `python/tests/cube/test_geometry.py`

- [ ] **Step 1: Write the tests**

```python
# python/tests/cube/test_geometry.py

import pytest

from pyxel.cube import Geometry


def test_default_construction():
    geom = Geometry()
    assert geom.positions == []
    assert geom.normals is None
    assert geom.uvs is None
    assert geom.indices is None
    assert geom.prim == Geometry.PRIM_TRIANGLES
    assert geom.cull == Geometry.CULL_BACK


def test_full_construction():
    geom = Geometry(
        positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        indices=[0, 1, 2],
        uvs=[0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
        prim=Geometry.PRIM_TRIANGLES,
        cull=Geometry.CULL_NONE,
    )
    assert len(geom.positions) == 9
    assert geom.indices == [0, 1, 2]
    assert geom.uvs == [0.0, 0.0, 1.0, 0.0, 0.0, 1.0]
    assert geom.cull == Geometry.CULL_NONE


def test_compute_normals_flat():
    geom = Geometry(positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0])
    geom.compute_normals()
    assert geom.normals is not None
    assert len(geom.normals) == 9
    for i in range(3):
        nx, ny, nz = geom.normals[i * 3 : i * 3 + 3]
        assert abs(nx) < 1e-5
        assert abs(ny) < 1e-5
        assert abs(nz - 1.0) < 1e-5


def test_set_normals_to_none_clears_cache():
    geom = Geometry(positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0])
    geom.compute_normals()
    assert geom.normals is not None
    geom.normals = None
    assert geom.normals is None


def test_prim_constants():
    assert Geometry.PRIM_POINTS == 0
    assert Geometry.PRIM_LINES == 1
    assert Geometry.PRIM_TRIANGLES == 2


def test_cull_constants():
    assert Geometry.CULL_NONE == 0
    assert Geometry.CULL_BACK == 1
    assert Geometry.CULL_FRONT == 2


def test_repr():
    geom = Geometry(positions=[0.0] * 9)
    r = repr(geom)
    assert "Geometry(" in r
    assert "positions=9" in r
```

- [ ] **Step 2: Add re-export to cube/__init__.py temporarily**

Edit `python/pyxel/cube/__init__.py` to add `Geometry` next to existing entries (do not yet remove FloatBuffer / IntBuffer):

```python
Geometry = _binding_cube.Geometry
```

Add `"Geometry"` to `__all__`.

- [ ] **Step 3: Run the tests**

Run: `make test` (or `cd python && pytest tests/cube/test_geometry.py -v`)
Expected: 7 tests pass.

- [ ] **Step 4: Commit**

```bash
git add python/tests/cube/test_geometry.py python/pyxel/cube/__init__.py
git commit -m "Add Geometry public API tests and re-export"
```

---

## Phase 4: Mesh Core Rewrite

Replace the existing `crates/pyxel-core/src/cube/mesh.rs` with the parallel-array layout.

### Task 4: Rewrite Mesh core struct

**Files:**
- Modify: `crates/pyxel-core/src/cube/mesh.rs` (replace contents)

- [ ] **Step 1: Replace mesh.rs with the new implementation**

```rust
// crates/pyxel-core/src/cube/mesh.rs

use crate::cube::geometry::RcGeometry;
use crate::cube::mat4::{Mat4, RcMat4};
use crate::image::RcImage;

// Asset container for a hierarchical 3D model. geometries / transforms /
// parents are parallel arrays; col_img holds either a flat color
// (ColImage::Color) or a shared texture (ColImage::Image). parents[i] < i
// is required (topological order); the constructor enforces this.

pub enum ColImage {
    Color(i32),
    Image(RcImage),
}

pub struct Mesh {
    pub geometries: Vec<Option<RcGeometry>>,
    pub transforms: Vec<RcMat4>,
    pub parents: Vec<i32>,
    pub col_img: ColImage,
    pub colkey: Option<i32>,
}

define_rc_type!(RcMesh, Mesh);

impl Mesh {
    pub fn new() -> RcMesh {
        new_rc_type!(Mesh {
            geometries: Vec::new(),
            transforms: Vec::new(),
            parents: Vec::new(),
            col_img: ColImage::Color(7),
            colkey: None,
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        let n = self.geometries.len();
        if self.transforms.len() != n || self.parents.len() != n {
            return Err(format!(
                "Mesh parallel arrays length mismatch: geometries={}, transforms={}, parents={}",
                n,
                self.transforms.len(),
                self.parents.len(),
            ));
        }
        for (i, &p) in self.parents.iter().enumerate() {
            if p < -1 {
                return Err(format!("Mesh.parents[{i}] = {p} < -1"));
            }
            if p >= i as i32 {
                return Err(format!(
                    "Mesh.parents[{i}] = {p} violates topological order (must be < {i})"
                ));
            }
        }
        Ok(())
    }

    pub fn descendants(&self, root: i32) -> Vec<i32> {
        let n = self.parents.len();
        if root < 0 || root >= n as i32 {
            return Vec::new();
        }
        let mut in_subtree = vec![false; n];
        in_subtree[root as usize] = true;
        let mut result = Vec::new();
        for j in (root as usize + 1)..n {
            if in_subtree[self.parents[j].max(0) as usize] && self.parents[j] != -1 {
                in_subtree[j] = true;
                result.push(j as i32);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::geometry::Geometry;

    #[test]
    fn test_new_empty() {
        let m = Mesh::new();
        let m = rc_ref!(&m);
        assert!(m.geometries.is_empty());
        assert!(m.transforms.is_empty());
        assert!(m.parents.is_empty());
        assert!(matches!(m.col_img, ColImage::Color(7)));
        assert!(m.colkey.is_none());
    }

    #[test]
    fn test_validate_topological_order() {
        let m = Mesh::new();
        {
            let m = rc_mut!(&m);
            m.geometries = vec![Some(Geometry::new()), Some(Geometry::new())];
            m.transforms = vec![Mat4::identity(), Mat4::identity()];
            m.parents = vec![-1, 0];
        }
        assert!(rc_ref!(&m).validate().is_ok());
    }

    #[test]
    fn test_validate_rejects_forward_parent() {
        let m = Mesh::new();
        {
            let m = rc_mut!(&m);
            m.geometries = vec![Some(Geometry::new()), Some(Geometry::new())];
            m.transforms = vec![Mat4::identity(), Mat4::identity()];
            m.parents = vec![1, -1];  // part 0 says parent is 1, but 1 > 0
        }
        assert!(rc_ref!(&m).validate().is_err());
    }

    #[test]
    fn test_descendants() {
        let m = Mesh::new();
        // tree: 0 -> 1, 0 -> 3, 3 -> 2 (but indices ordered 0,1,2,3 with parents -1,0,3,0)
        // descendants(0) should return [1,2,3] in topological order
        {
            let m = rc_mut!(&m);
            m.geometries = vec![None, None, None, None];
            m.transforms = vec![
                Mat4::identity(),
                Mat4::identity(),
                Mat4::identity(),
                Mat4::identity(),
            ];
            m.parents = vec![-1, 0, 3, 0];
        }
        // The constraint parents[i] < i requires part 2's parent (3) to be < 2, fail.
        // Fix: reorder so all descendants come after their parents in array order.
        // Use parents = [-1, 0, 0, 2] (child of 2 is 3).
        {
            let m = rc_mut!(&m);
            m.parents = vec![-1, 0, 0, 2];
        }
        assert_eq!(rc_ref!(&m).descendants(0), vec![1, 2, 3]);
        assert_eq!(rc_ref!(&m).descendants(2), vec![3]);
        assert_eq!(rc_ref!(&m).descendants(3), Vec::<i32>::new());
    }
}
```

- [ ] **Step 2: Run core tests**

Run: `cd crates && cargo test -p pyxel-core cube::mesh -- --nocapture`
Expected: 4 tests pass.

- [ ] **Step 3: Commit**

```bash
git add crates/pyxel-core/src/cube/mesh.rs
git commit -m "Rewrite Pyxel Cube Mesh as hierarchical Geometry container"
```

---

## Phase 5: Mesh Binding Rewrite

### Task 5: Rewrite Mesh binding wrapper

**Files:**
- Modify: `crates/pyxel-binding/src/cube/mesh.rs` (replace contents)

- [ ] **Step 1: Replace binding mesh.rs**

```rust
// crates/pyxel-binding/src/cube/mesh.rs

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyList;

use super::geometry::Geometry;
use super::mat4::Mat4;
use crate::image_wrapper::Image;

define_wrapper!(Mesh, pyxel::cube::Mesh);

#[pymethods]
impl Mesh {
    // Constructor

    #[new]
    #[pyo3(signature = (geometries=None, transforms=None, parents=None, col_img=None, colkey=None))]
    fn new(
        py: Python<'_>,
        geometries: Option<Vec<Option<PyRef<'_, Geometry>>>>,
        transforms: Option<Vec<PyRef<'_, Mat4>>>,
        parents: Option<Vec<i32>>,
        col_img: Option<Bound<'_, PyAny>>,
        colkey: Option<i32>,
    ) -> PyResult<Self> {
        let mesh = pyxel::cube::Mesh::new();
        {
            let m = rc_mut!(&mesh);
            if let Some(gs) = geometries {
                m.geometries = gs
                    .into_iter()
                    .map(|g| g.map(|g| g.inner.clone()))
                    .collect();
            }
            if let Some(ts) = transforms {
                m.transforms = ts.iter().map(|t| t.inner.clone()).collect();
            }
            if let Some(ps) = parents {
                m.parents = ps;
            }
            if let Some(ci) = col_img {
                m.col_img = parse_col_img(py, &ci)?;
            }
            m.colkey = colkey;
        }
        rc_ref!(&mesh)
            .validate()
            .map_err(PyValueError::new_err)?;
        Ok(Self::wrap(mesh))
    }

    // Parts

    #[getter]
    fn geometries(&self, py: Python<'_>) -> PyResult<Py<PyList>> {
        let inner = self.inner_ref();
        let items: Vec<PyObject> = inner
            .geometries
            .iter()
            .map(|g| match g {
                Some(g) => Geometry::wrap(g.clone())
                    .into_pyobject(py)
                    .map(|b| b.into_any().unbind())
                    .unwrap_or_else(|_| py.None()),
                None => py.None(),
            })
            .collect();
        Ok(PyList::new(py, items)?.unbind())
    }

    #[setter]
    fn set_geometries(&self, v: Vec<Option<PyRef<'_, Geometry>>>) -> PyResult<()> {
        self.inner_mut().geometries = v
            .into_iter()
            .map(|g| g.map(|g| g.inner.clone()))
            .collect();
        self.inner_ref()
            .validate()
            .map_err(PyValueError::new_err)
    }

    #[getter]
    fn transforms(&self) -> Vec<Mat4> {
        self.inner_ref()
            .transforms
            .iter()
            .map(|t| Mat4::wrap(t.clone()))
            .collect()
    }

    #[setter]
    fn set_transforms(&self, v: Vec<PyRef<'_, Mat4>>) -> PyResult<()> {
        self.inner_mut().transforms = v.iter().map(|t| t.inner.clone()).collect();
        self.inner_ref()
            .validate()
            .map_err(PyValueError::new_err)
    }

    #[getter]
    fn parents(&self) -> Vec<i32> {
        self.inner_ref().parents.clone()
    }

    #[setter]
    fn set_parents(&self, v: Vec<i32>) -> PyResult<()> {
        self.inner_mut().parents = v;
        self.inner_ref()
            .validate()
            .map_err(PyValueError::new_err)
    }

    // Shared material

    #[getter]
    fn col_img(&self, py: Python<'_>) -> PyResult<PyObject> {
        match &self.inner_ref().col_img {
            pyxel::cube::mesh::ColImage::Color(c) => Ok(c
                .into_pyobject(py)?
                .into_any()
                .unbind()),
            pyxel::cube::mesh::ColImage::Image(img) => Ok(Image::wrap(img.clone())
                .into_pyobject(py)?
                .into_any()
                .unbind()),
        }
    }

    #[setter]
    fn set_col_img(&self, py: Python<'_>, v: Bound<'_, PyAny>) -> PyResult<()> {
        self.inner_mut().col_img = parse_col_img(py, &v)?;
        Ok(())
    }

    #[getter]
    fn colkey(&self) -> Option<i32> {
        self.inner_ref().colkey
    }

    #[setter]
    fn set_colkey(&self, v: Option<i32>) {
        self.inner_mut().colkey = v;
    }

    // Methods

    fn descendants(&self, i: i32) -> Vec<i32> {
        self.inner_ref().descendants(i)
    }

    // Dunder

    fn __repr__(&self) -> String {
        let m = self.inner_ref();
        format!("Mesh(parts={})", m.geometries.len())
    }
}

fn parse_col_img(
    py: Python<'_>,
    v: &Bound<'_, PyAny>,
) -> PyResult<pyxel::cube::mesh::ColImage> {
    let _ = py;
    if let Ok(c) = v.extract::<i32>() {
        return Ok(pyxel::cube::mesh::ColImage::Color(c));
    }
    if let Ok(img) = v.extract::<PyRef<'_, Image>>() {
        return Ok(pyxel::cube::mesh::ColImage::Image(img.inner.clone()));
    }
    Err(PyTypeError::new_err(
        "col_img must be int or Image",
    ))
}

pub fn add_mesh_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Mesh>()?;
    Ok(())
}
```

- [ ] **Step 2: Build the binding**

Run: `make build`
Expected: builds successfully. (Old `node.mesh` and `node.prim` still call into Mesh's old API — fix in Phase 6 / 7. If the binding stops compiling because of those callers, proceed to the next task; do not commit until callers are updated.)

If build fails because `node.rs` references removed Mesh fields (`positions`, `indices`, etc.), defer the commit and move on to Phase 6. Build and commit Phase 4-7 together.

---

## Phase 6: node.prim Signature Update

### Task 6: Update node.prim binding

**Files:**
- Modify: `crates/pyxel-binding/src/cube/node.rs` (replace the `prim` method)

- [ ] **Step 1: Replace the prim method**

Find the existing `fn prim(...)` in `crates/pyxel-binding/src/cube/node.rs` and replace its signature and body with:

```rust
    // node.prim — draw a Geometry with optional flat color or texture

    #[pyo3(signature = (
        mat,
        geom,
        *,
        col_img=None,
        colkey=None,
        shaded=true,
        dither_alpha=1.0,
        depth_test=true,
        depth_write=true,
        billboard=0,
    ))]
    fn prim(
        &self,
        py: Python<'_>,
        mat: PyRef<'_, Mat4>,
        geom: PyRef<'_, Geometry>,
        col_img: Option<Bound<'_, PyAny>>,
        colkey: Option<i32>,
        shaded: bool,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
        billboard: i32,
    ) -> PyResult<()> {
        let col_img_value = match col_img {
            Some(v) => super::mesh::parse_col_img(py, &v)?,
            None => pyxel::cube::mesh::ColImage::Color(7),
        };
        pyxel::cube::draw::draw_geometry(
            self.inner.clone(),
            mat.inner.clone(),
            geom.inner.clone(),
            col_img_value,
            colkey,
            shaded,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
        );
        Ok(())
    }
```

Note: `parse_col_img` must be made `pub(super)` or `pub` in `mesh.rs`. Update its signature:
```rust
pub(super) fn parse_col_img(...) -> PyResult<...>
```

- [ ] **Step 2: Adapt the existing core prim function**

The existing core function is at `crates/pyxel-core/src/cube/draw.rs:209` with this signature:

```rust
pub fn prim(
    ctx: &mut DrawContext,
    world_mat: &Mat4,
    mode: i32,
    positions: &[f32],
    indices: Option<&[i32]>,
    normals: Option<&[f32]>,
    uvs: Option<&[f32]>,
    first: usize,
    count: Option<usize>,
    col_flat: i32,
    col_image: Option<&RcImage>,
    colkey: Option<i32>,
    state: DrawState,
) -> Result<(), &'static str>
```

Modify it as follows:

1. **Add a `cull: i32` parameter** between `mode` and `positions`.
2. **Remove `first` and `count` parameters** (Geometry draws are whole-buffer; subrange slicing is dropped per spec §3.4 / §5.1).
3. **Inside the `PRIM_TRIANGLES` arm**, before rasterizing each face, add a cull test:
   - For `cull == CULL_NONE`: no test, draw all faces.
   - For `cull == CULL_BACK`: skip faces whose signed area in screen space is negative (back-facing).
   - For `cull == CULL_FRONT`: skip faces whose signed area in screen space is positive (front-facing).
   - The signed-area test reuses the existing rasterizer's edge-function setup (signed area = sum of edge cross products) — the test is a single sign check before fragment emission.

The binding side calls this function with arguments extracted from `Geometry`:

```rust
let g = rc_ref!(&geom.inner);
let result = pyxel::cube::draw::prim(
    ctx,
    &mat_view,
    g.prim,
    g.cull,
    &g.positions,
    g.indices.as_deref(),
    g.normals.as_deref(),
    g.uvs.as_deref(),
    col_flat,
    col_image.as_ref(),
    colkey,
    draw_state,
)
.map_err(PyValueError::new_err)?;
```

The binding's `parse_col_img` helper (Phase 5) returns `ColImage`; for prim, extract `(i32, Option<RcImage>)` from it (add a public method on `ColImage` like `as_flat_and_image() -> (i32, Option<RcImage>)`, or destructure inline).

(The `prepare_draw` and rasterizer-internal logic do not change — only the prim function's argument-handling and pre-rasterization cull test are touched.)

- [ ] **Step 3: Build**

Run: `make build`
Expected: builds successfully.

- [ ] **Step 4: Commit (combined with Phase 4/5 if those still uncommitted)**

If Phase 4 and 5 are uncommitted because of build failures, combine into one commit:
```bash
git add crates/pyxel-core/src/cube/mesh.rs crates/pyxel-binding/src/cube/mesh.rs crates/pyxel-binding/src/cube/node.rs crates/pyxel-core/src/cube/draw.rs
git commit -m "Update Mesh and node.prim for Geometry-based asset model"
```

Otherwise:
```bash
git add crates/pyxel-binding/src/cube/node.rs crates/pyxel-core/src/cube/draw.rs
git commit -m "Update node.prim signature to take Geometry directly"
```

---

## Phase 7: node.mesh Signature Update

### Task 7: Update node.mesh binding and draw path

**Files:**
- Modify: `crates/pyxel-binding/src/cube/node.rs` (replace the `mesh` method)
- Modify: `crates/pyxel-core/src/cube/draw.rs` (add `draw_mesh` that walks parallel arrays)

- [ ] **Step 1: Replace the mesh method**

```rust
    // node.mesh — draw a hierarchical Mesh asset

    #[pyo3(signature = (
        mat,
        mesh_asset,
        *,
        shaded=true,
        dither_alpha=1.0,
        depth_test=true,
        depth_write=true,
        billboard=0,
    ))]
    fn mesh(
        &self,
        mat: PyRef<'_, Mat4>,
        mesh_asset: PyRef<'_, super::mesh::Mesh>,
        shaded: bool,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
        billboard: i32,
    ) {
        pyxel::cube::draw::draw_mesh(
            self.inner.clone(),
            mat.inner.clone(),
            mesh_asset.inner.clone(),
            shaded,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
        );
    }
```

- [ ] **Step 2: Add draw_mesh in draw.rs**

```rust
pub fn draw_mesh(
    node: RcNode,
    mat: RcMat4,
    mesh: RcMesh,
    shaded: bool,
    dither_alpha: f32,
    depth_test: bool,
    depth_write: bool,
    billboard: i32,
) {
    let mesh_ref = rc_ref!(&mesh);
    let n = mesh_ref.geometries.len();
    // Compute world transforms in topological order (parents[i] < i).
    // Mat4::mul_mat returns RcMat4 directly (see crates/pyxel-core/src/cube/mat4.rs).
    let mut world: Vec<RcMat4> = Vec::with_capacity(n);
    for i in 0..n {
        let local = mesh_ref.transforms[i].clone();
        let combined: RcMat4 = if mesh_ref.parents[i] == -1 {
            rc_ref!(&mat).mul_mat(&rc_ref!(&local))
        } else {
            let parent_world = world[mesh_ref.parents[i] as usize].clone();
            rc_ref!(&parent_world).mul_mat(&rc_ref!(&local))
        };
        world.push(combined);
    }
    // Draw each part with its world transform via the prim path.
    let (col_flat, col_image) = mesh_ref.col_img.as_flat_and_image();
    for i in 0..n {
        let Some(geom) = mesh_ref.geometries[i].as_ref() else {
            continue;
        };
        let g = rc_ref!(geom);
        prim(
            ctx,
            &rc_ref!(&world[i]),
            g.prim,
            g.cull,
            &g.positions,
            g.indices.as_deref(),
            g.normals.as_deref(),
            g.uvs.as_deref(),
            col_flat,
            col_image.as_ref(),
            mesh_ref.colkey,
            DrawState { shaded, dither_alpha, depth_test, depth_write, billboard, /* ... */ },
        )
        .ok();
    }
}
```

Note: `ColImage::as_flat_and_image(&self) -> (i32, Option<RcImage>)` is a new helper on the `ColImage` enum (defined in Task 4); it returns `(c, None)` for `Color(c)` and `(0, Some(img))` for `Image(img)`. Mat4 multiplication uses `mul_mat` (verified in `mat4.rs:158`); no Mat4 wrapping helper is needed because `mul_mat` already returns `RcMat4`. The `DrawState` field set may include other fields (e.g., `shading: &Shading`) — read the existing struct at `draw.rs:50` and populate accordingly.

- [ ] **Step 3: Build and run core tests**

Run: `make build && cd crates && cargo test -p pyxel-core cube -- --nocapture`
Expected: builds, all cube core tests pass.

- [ ] **Step 4: Commit**

```bash
git add crates/pyxel-binding/src/cube/node.rs crates/pyxel-core/src/cube/{draw.rs,mesh.rs}
git commit -m "Update node.mesh to walk Mesh parallel arrays for hierarchical draw"
```

---

## Phase 8: Python tests for Mesh

### Task 8: Rewrite test_mesh.py

**Files:**
- Modify: `python/tests/cube/test_mesh.py` (replace contents)

- [ ] **Step 1: Replace test contents**

```python
# python/tests/cube/test_mesh.py

import pytest

import pyxel
from pyxel.cube import Geometry, Mat4, Mesh, Vec3


def _square_geom():
    return Geometry(
        positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        indices=[0, 1, 2],
    )


def test_default_construction():
    mesh = Mesh()
    assert mesh.geometries == []
    assert mesh.transforms == []
    assert mesh.parents == []
    assert mesh.col_img == 7
    assert mesh.colkey is None


def test_full_construction():
    g0, g1 = _square_geom(), _square_geom()
    mesh = Mesh(
        geometries=[g0, g1, None],
        transforms=[Mat4(), Mat4(), Mat4()],
        parents=[-1, 0, 1],
        col_img=8,
        colkey=0,
    )
    assert len(mesh.geometries) == 3
    assert mesh.geometries[2] is None
    assert mesh.parents == [-1, 0, 1]
    assert mesh.col_img == 8
    assert mesh.colkey == 0


def test_topological_order_validation():
    g = _square_geom()
    with pytest.raises(ValueError):
        Mesh(
            geometries=[g, g],
            transforms=[Mat4(), Mat4()],
            parents=[1, -1],  # parents[0] = 1 violates parents[i] < i
        )


def test_parallel_array_length_validation():
    g = _square_geom()
    with pytest.raises(ValueError):
        Mesh(
            geometries=[g, g],
            transforms=[Mat4()],  # length mismatch
            parents=[-1, 0],
        )


def test_descendants():
    g = _square_geom()
    mesh = Mesh(
        geometries=[g, g, g, g],
        transforms=[Mat4(), Mat4(), Mat4(), Mat4()],
        parents=[-1, 0, 0, 2],
    )
    assert mesh.descendants(0) == [1, 2, 3]
    assert mesh.descendants(2) == [3]
    assert mesh.descendants(3) == []


def test_descendants_out_of_range():
    g = _square_geom()
    mesh = Mesh(
        geometries=[g],
        transforms=[Mat4()],
        parents=[-1],
    )
    assert mesh.descendants(-1) == []
    assert mesh.descendants(5) == []


def test_col_img_image_type():
    pyxel.init(64, 64)
    g = _square_geom()
    img = pyxel.images[0]
    mesh = Mesh(
        geometries=[g],
        transforms=[Mat4()],
        parents=[-1],
        col_img=img,
        colkey=0,
    )
    assert mesh.col_img is img


def test_repr():
    g = _square_geom()
    mesh = Mesh(geometries=[g], transforms=[Mat4()], parents=[-1])
    assert "Mesh(parts=1)" in repr(mesh)
```

- [ ] **Step 2: Run the tests**

Run: `make test` (or `cd python && pytest tests/cube/test_mesh.py -v`)
Expected: 8 tests pass.

- [ ] **Step 3: Commit**

```bash
git add python/tests/cube/test_mesh.py
git commit -m "Rewrite Mesh tests for parallel-array hierarchy"
```

---

## Phase 9: Update test_node.py

### Task 9: Update prim / mesh tests in test_node.py

**Files:**
- Modify: `python/tests/cube/test_node.py`

- [ ] **Step 1: Find and replace prim / mesh test cases**

Open `python/tests/cube/test_node.py`. Locate any tests calling `node.prim(...)` or `node.mesh(...)`. Replace their call shapes:

Old prim:
```python
node.prim(Mat4(), Geometry.PRIM_TRIANGLES, positions, indices=..., normals=..., uvs=..., col=..., colkey=...)
```
New prim:
```python
geom = Geometry(positions=positions, indices=indices, normals=normals, uvs=uvs)
node.prim(Mat4(), geom, col_img=..., colkey=...)
```

Old mesh:
```python
node.mesh(Mat4(), mesh_asset, col=..., shaded=...)
```
New mesh:
```python
node.mesh(Mat4(), mesh_asset, shaded=...)
```

Where the old test constructed a `Mesh` using the old field set (positions / indices / normals / uvs / image / colkey), rewrite the Mesh construction to use Geometry instances and the new Mesh shape.

- [ ] **Step 2: Run tests**

Run: `make test`
Expected: All cube tests pass.

- [ ] **Step 3: Commit**

```bash
git add python/tests/cube/test_node.py
git commit -m "Update Node prim and mesh tests for redesigned signatures"
```

---

## Phase 10: Python .pyi and __init__.py Update

### Task 10: Replace cube/__init__.pyi

**Files:**
- Modify: `python/pyxel/cube/__init__.pyi` (full replacement)

- [ ] **Step 1: Open the spec and copy the final class definitions**

Open `docs/superpowers/specs/2026-05-11-cube-geometry-mesh-redesign-design.md` and find the `Geometry` (§3.1) and `Mesh` (§4.1) class definitions. The full .pyi is constructed by:

- Keeping `Vec3`, `Mat4`, `Quat`, `Camera`, `Shading`, `Contact`, `Collider`, `Scene` blocks unchanged.
- **Removing** `FloatBuffer` and `IntBuffer` blocks entirely.
- **Replacing** the `Mesh` block with the §4.1 form.
- **Adding** a new `Geometry` block per §3.1.
- In the `Node` block:
  - **Removing** `PRIM_POINTS / PRIM_LINES / PRIM_TRIANGLES` class-level constants (relocated to `Geometry`).
  - **Replacing** the `prim` method signature with:

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

  - **Replacing** the `mesh` method signature with:

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

- Updating imports: add `from typing import overload` if not present, ensure `Image` and `Font` are imported.

- [ ] **Step 2: Update cube/__init__.py**

```python
# python/pyxel/cube/__init__.py

from ..pyxel_binding import cube as _binding_cube  # type: ignore

Camera = _binding_cube.Camera
Collider = _binding_cube.Collider
Contact = _binding_cube.Contact
Geometry = _binding_cube.Geometry
Mat4 = _binding_cube.Mat4
Mesh = _binding_cube.Mesh
Node = _binding_cube.Node
Quat = _binding_cube.Quat
Scene = _binding_cube.Scene
Shading = _binding_cube.Shading
Vec3 = _binding_cube.Vec3

__all__ = [
    "Camera",
    "Collider",
    "Contact",
    "Geometry",
    "Mat4",
    "Mesh",
    "Node",
    "Quat",
    "Scene",
    "Shading",
    "Vec3",
]
```

- [ ] **Step 3: Verify imports work**

Run: `cd python && python -c "from pyxel.cube import Geometry, Mesh, Mat4, Vec3; print('OK')"`
Expected: `OK`.

- [ ] **Step 4: Commit**

```bash
git add python/pyxel/cube/__init__.pyi python/pyxel/cube/__init__.py
git commit -m "Update cube .pyi and __init__ for Geometry / Mesh redesign"
```

---

## Phase 11: Remove FloatBuffer and IntBuffer

### Task 11: Delete buffer classes and their tests

**Files:**
- Delete: `crates/pyxel-core/src/cube/float_buffer.rs`
- Delete: `crates/pyxel-core/src/cube/int_buffer.rs`
- Delete: `crates/pyxel-binding/src/cube/float_buffer.rs`
- Delete: `crates/pyxel-binding/src/cube/int_buffer.rs`
- Delete: `python/tests/cube/test_float_buffer.py`
- Delete: `python/tests/cube/test_int_buffer.py`
- Modify: `crates/pyxel-core/src/cube/mod.rs` (remove `float_buffer` / `int_buffer` mod and re-exports)
- Modify: `crates/pyxel-binding/src/cube/mod.rs` (remove module declarations and `add_*` calls)

- [ ] **Step 1: Delete buffer files**

```bash
rm crates/pyxel-core/src/cube/float_buffer.rs
rm crates/pyxel-core/src/cube/int_buffer.rs
rm crates/pyxel-binding/src/cube/float_buffer.rs
rm crates/pyxel-binding/src/cube/int_buffer.rs
rm python/tests/cube/test_float_buffer.py
rm python/tests/cube/test_int_buffer.py
```

- [ ] **Step 2: Update mod.rs files**

In `crates/pyxel-core/src/cube/mod.rs`, remove these lines:
```
pub mod float_buffer;
pub mod int_buffer;
pub use float_buffer::{FloatBuffer, RcFloatBuffer};
pub use int_buffer::{IntBuffer, RcIntBuffer};
```

In `crates/pyxel-binding/src/cube/mod.rs`, remove these lines:
```
mod float_buffer;
mod int_buffer;
float_buffer::add_float_buffer_class(&m)?;
int_buffer::add_int_buffer_class(&m)?;
```

- [ ] **Step 3: Build and test**

Run: `make build && make lint && make test`
Expected: All commands succeed, no warnings, all tests pass.

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "Remove FloatBuffer and IntBuffer from Pyxel Cube"
```

---

## Phase 12: WASM Lint and Verification

### Task 12: Verify WASM target builds

**Files:**
- (none modified; build verification only)

- [ ] **Step 1: Run lint-wasm**

Run: `make lint-wasm`
Expected: succeeds without warnings. If failures occur (e.g., cfg-gated paths that referenced FloatBuffer / IntBuffer), fix them and re-run.

- [ ] **Step 2: Run full test suite**

Run: `make test`
Expected: all tests pass (Python tests for Geometry / Mesh / Node, all unchanged tests).

- [ ] **Step 3: Run native lint**

Run: `make lint`
Expected: succeeds without warnings.

- [ ] **Step 4: Commit any cleanup**

If any cleanup edits were made during Steps 1-3:
```bash
git add -A
git commit -m "Apply lint fixes for cube redesign"
```

---

## Phase 13: Update cube-design.md

### Task 13: Reflect the redesign in cube-design.md

**Files:**
- Modify: `docs/cube-design.md`

- [ ] **Step 1: Delete §10 (Buffers)**

In `docs/cube-design.md`, find `## 10. Buffers` and delete the entire §10 section through `## 11. Mesh` (exclusive). Renumber subsequent sections accordingly.

- [ ] **Step 2: Replace §11 (Mesh) with the new Mesh design**

Replace the content of `## 11. Mesh` with a summary mirroring §4 of the spec. Key points:

- `Mesh` is a hierarchical container of `Geometry` parts with a shared texture / flat color.
- Parallel arrays: `geometries`, `transforms`, `parents`.
- Topological order constraint: `parents[i] < i`, or `-1` for root.
- `col_img: int | Image` and `colkey: int | None`.
- `descendants(i)` helper.
- Rationale for parallel arrays vs. a `MeshPart` class, and for no `names` array (deferred to animation phase).

Then add a new section `## 11A. Geometry` (or insert before Mesh as a new §11) with the Geometry design:

- Vertex attributes (`positions / normals / uvs`) as `list[float]`; `indices` as `list[int]`.
- `prim` topology (PRIM_POINTS / LINES / TRIANGLES).
- `cull` back-face mode (CULL_NONE / BACK / FRONT).
- Normal auto-cache on first draw; `compute_normals(smooth)` for explicit recalculation.

- [ ] **Step 3: Update §12.1 (Node class-level constants)**

Find the table of `Node.PRIM_POINTS / PRIM_LINES / PRIM_TRIANGLES`. Replace it with a brief note: "Primitive mode constants are class attributes on `Geometry` (§11A); they are no longer exposed on `Node`."

- [ ] **Step 4: Update §12.5 (draw command signatures)**

Find the `node.prim(...)` signature in §12.5. Replace it with:
```python
self.prim(mat, geom, *, col_img, colkey, shaded, dither_alpha, depth_test, depth_write, billboard)
```
Find the `node.mesh(...)` signature. Replace it with:
```python
self.mesh(mat, mesh_asset, *, shaded, dither_alpha, depth_test, depth_write, billboard)
```
Update prose to reflect that prim now takes a single `Geometry` argument and that mesh draws by walking `Mesh` parts in topological order.

- [ ] **Step 5: Add new rejected items to §16**

In §16.3 (Drawing rejected items), append entries for:
- `col_tex` argument name — rejected because cube lacks a `Texture` class; `tex` is justified in business engines only when paired with a Texture class instance.
- `col_image` argument name — rejected as short+full mix (col_image) is less consistent than the all-short `col_img`.
- `mode` as the topology attribute name on Geometry — `prim` chosen for parallelism with `Node.prim(...)` and `PRIM_*` constants.

In §16.1 (Math classes rejected), add no new entries (the redesign is mostly drawing-side).

In §16.4 (Scene structure rejected), add:
- `MeshPart` / `MeshNode` as a separate class — rejected in favor of parallel arrays on `Mesh` to keep public class count down and avoid nested navigation.
- Recursive `Mesh` tree (each Mesh has `children: list[Mesh]`) — rejected as overly nested for what is functionally a flat parts array.
- `names: list[str]` on `Mesh` — deferred until the animation pipeline (§15 open item).
- Pure-Mesh per-part image override — rejected; mixed-image models are expressed as multiple `Mesh` instances combined via a `Node` hierarchy.

- [ ] **Step 6: Commit**

```bash
git add docs/cube-design.md
git commit -m "Update cube-design.md for Geometry / Mesh redesign"
```

---

## Phase 14: Final Verification

### Task 14: Full project verification

**Files:**
- (verification only)

- [ ] **Step 1: Format**

Run: `make format`
Expected: applies any pending formatting (none expected if previous tasks ran `make format` per `coding-policy.md`).

- [ ] **Step 2: Lint both targets**

Run: `make lint && make lint-wasm`
Expected: both succeed without warnings.

- [ ] **Step 3: Test**

Run: `make test`
Expected: all tests pass.

- [ ] **Step 4: Manual smoke test (user)**

Inform the user: "cube redesign implementation complete. Please run a manual smoke test of a cube example script (any script under `python/pyxel/examples/` using the cube module) to confirm visual output. The `mcp__pyxel` toolset cannot run cube scripts because it uses the released pyxel wheel, not the in-progress branch build."

The user runs a smoke test and confirms visual output.

- [ ] **Step 5: Commit any final cleanup**

If the smoke test surfaced issues that required code edits, commit them. Otherwise no commit needed at this phase.

---

## Done

All redesign tasks complete. Summary:

- 2 classes removed (`FloatBuffer`, `IntBuffer`)
- 1 class added (`Geometry`)
- 1 class redesigned in place (`Mesh`)
- `node.prim` / `node.mesh` signatures updated
- `Node.PRIM_*` constants relocated to `Geometry`
- `docs/cube-design.md` updated
- All tests pass, lint clean, manual smoke test confirmed

The cube module is now ready for downstream use (game examples, future animation pipeline, future Texture class additions if needed).
