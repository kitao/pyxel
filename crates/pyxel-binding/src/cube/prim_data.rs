use pyo3::prelude::*;

// Live proxy sequences onto an RcPrimData's vertex/topology fields.
// Mirrors `wrap_sound_as_python_list!` (sound_wrapper.rs): each proxy
// holds the shared RcPrimData and projects one of its plain Vec fields,
// so element writes / append / etc. mutate the PrimData in place. The
// PrimData getters hand out a fresh proxy carrying an Rc clone (a live
// view), and there is no whole-attribute setter — matching Sound.notes.
macro_rules! wrap_primitive_as_python_list {
    ($wrapper_name:ident, $value_type:ty, $field_name:ident) => {
        wrap_as_python_primitive_sequence!(
            $wrapper_name,
            pyxel::cube::RcPrimData,
            (|inner: &pyxel::cube::RcPrimData| rc_ref!(inner).$field_name.len()),
            $value_type,
            (|inner: &pyxel::cube::RcPrimData, index| rc_ref!(inner).$field_name[index]),
            $value_type,
            (|inner: &pyxel::cube::RcPrimData, index, value| rc_mut!(inner).$field_name[index] =
                value),
            (|inner: &pyxel::cube::RcPrimData| -> &mut Vec<$value_type> {
                &mut rc_mut!(inner).$field_name
            }),
            Vec<$value_type>,
            (|inner: &pyxel::cube::RcPrimData, list| rc_mut!(inner).$field_name = list),
            (|inner: &pyxel::cube::RcPrimData| rc_ref!(inner)
                .$field_name
                .iter()
                .copied()
                .collect::<Vec<$value_type>>())
        );
    };
}

wrap_primitive_as_python_list!(Positions, f32, positions);
wrap_primitive_as_python_list!(Indices, i32, indices);
wrap_primitive_as_python_list!(Normals, f32, normals);
wrap_primitive_as_python_list!(Uvs, f32, uvs);

define_wrapper!(PrimData, pyxel::cube::PrimData);

#[pymethods]
impl PrimData {
    // Topology mode constants

    #[classattr]
    const MODE_POINTS: i32 = pyxel::cube::prim_data::MODE_POINTS;
    #[classattr]
    const MODE_LINES: i32 = pyxel::cube::prim_data::MODE_LINES;
    #[classattr]
    const MODE_TRIANGLES: i32 = pyxel::cube::prim_data::MODE_TRIANGLES;

    // Back-face cull constants

    #[classattr]
    const CULL_NONE: i32 = pyxel::cube::prim_data::CULL_NONE;
    #[classattr]
    const CULL_BACK: i32 = pyxel::cube::prim_data::CULL_BACK;
    #[classattr]
    const CULL_FRONT: i32 = pyxel::cube::prim_data::CULL_FRONT;

    // Constructor

    #[new]
    #[pyo3(signature = (
        mode,
        positions,
        indices,
        normals=vec![],
        uvs=vec![],
        cull=pyxel::cube::prim_data::CULL_BACK,
    ))]
    fn new(
        mode: i32,
        positions: Vec<f32>,
        indices: Vec<i32>,
        normals: Vec<f32>,
        uvs: Vec<f32>,
        cull: i32,
    ) -> Self {
        let p = pyxel::cube::PrimData::new();
        {
            let p = rc_mut!(&p);
            p.mode = mode;
            p.positions = positions;
            p.indices = indices;
            p.normals = normals;
            p.uvs = uvs;
            p.cull = cull;
        }
        Self::wrap(p)
    }

    // Vertex attributes (live proxies; no whole-attribute setter, mirror
    // Sound.notes — write elements in place or reassign via slice).

    #[getter]
    fn positions(&self) -> Positions {
        Positions::wrap(self.inner.clone())
    }

    #[getter]
    fn normals(&self) -> Normals {
        Normals::wrap(self.inner.clone())
    }

    #[getter]
    fn uvs(&self) -> Uvs {
        Uvs::wrap(self.inner.clone())
    }

    // Topology

    #[getter]
    fn indices(&self) -> Indices {
        Indices::wrap(self.inner.clone())
    }

    #[getter]
    fn mode(&self) -> i32 {
        self.inner_ref().mode
    }

    #[setter]
    fn set_mode(&self, v: i32) {
        self.inner_mut().mode = v;
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

    // Dunder

    fn __repr__(&self) -> String {
        let p = self.inner_ref();
        format!(
            "PrimData(positions={}, mode={}, cull={})",
            p.positions.len(),
            p.mode,
            p.cull
        )
    }

    // Methods

    fn compute_normals(&self) {
        self.inner_mut().compute_normals();
    }
}

pub fn add_prim_data_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Positions>()?;
    m.add_class::<Indices>()?;
    m.add_class::<Normals>()?;
    m.add_class::<Uvs>()?;
    m.add_class::<PrimData>()?;
    Ok(())
}
