use pyo3::prelude::*;

define_wrapper!(Geometry, pyxel::cube::Geometry);

#[pymethods]
impl Geometry {
    // Topology mode constants

    #[classattr]
    const PRIM_POINTS: i32 = pyxel::cube::geometry::PRIM_POINTS;
    #[classattr]
    const PRIM_LINES: i32 = pyxel::cube::geometry::PRIM_LINES;
    #[classattr]
    const PRIM_TRIANGLES: i32 = pyxel::cube::geometry::PRIM_TRIANGLES;

    // Back-face cull constants

    #[classattr]
    const CULL_NONE: i32 = pyxel::cube::geometry::CULL_NONE;
    #[classattr]
    const CULL_BACK: i32 = pyxel::cube::geometry::CULL_BACK;
    #[classattr]
    const CULL_FRONT: i32 = pyxel::cube::geometry::CULL_FRONT;

    // Constructor

    #[new]
    #[pyo3(signature = (
        positions=None,
        normals=None,
        uvs=None,
        indices=None,
        prim=pyxel::cube::geometry::PRIM_TRIANGLES,
        cull=pyxel::cube::geometry::CULL_BACK,
    ))]
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
