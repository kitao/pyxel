use pyo3::prelude::*;

use super::vec3::Vec3;

// Live proxy sequences onto an RcPrimitive's vertex/topology fields.
// Mirrors `wrap_sound_as_python_list!` (sound_wrapper.rs): each proxy
// holds the shared RcPrimitive and projects one of its plain Vec fields,
// so element writes / append / etc. mutate the Primitive in place. The
// Primitive getters hand out a fresh proxy carrying an Rc clone (a live
// view), and there is no whole-attribute setter — matching Sound.notes.
macro_rules! wrap_primitive_as_python_list {
    ($wrapper_name:ident, $value_type:ty, $field_name:ident) => {
        wrap_as_python_primitive_sequence!(
            $wrapper_name,
            pyxel::cube::RcPrimitive,
            (|inner: &pyxel::cube::RcPrimitive| rc_ref!(inner).$field_name.len()),
            $value_type,
            (|inner: &pyxel::cube::RcPrimitive, index| rc_ref!(inner).$field_name[index]),
            $value_type,
            (|inner: &pyxel::cube::RcPrimitive, index, value| rc_mut!(inner).$field_name[index] =
                value),
            (|inner: &pyxel::cube::RcPrimitive| -> &mut Vec<$value_type> {
                &mut rc_mut!(inner).$field_name
            }),
            Vec<$value_type>,
            (|inner: &pyxel::cube::RcPrimitive, list| rc_mut!(inner).$field_name = list),
            (|inner: &pyxel::cube::RcPrimitive| rc_ref!(inner)
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

define_wrapper!(Primitive, pyxel::cube::Primitive);

#[pymethods]
impl Primitive {
    // Topology mode constants

    #[classattr]
    const MODE_POINTS: i32 = pyxel::cube::primitive::MODE_POINTS;
    #[classattr]
    const MODE_LINES: i32 = pyxel::cube::primitive::MODE_LINES;
    #[classattr]
    const MODE_TRIANGLES: i32 = pyxel::cube::primitive::MODE_TRIANGLES;

    // Back-face cull constants

    #[classattr]
    const CULL_NONE: i32 = pyxel::cube::primitive::CULL_NONE;
    #[classattr]
    const CULL_BACK: i32 = pyxel::cube::primitive::CULL_BACK;
    #[classattr]
    const CULL_FRONT: i32 = pyxel::cube::primitive::CULL_FRONT;

    // Constructor

    #[new]
    #[pyo3(signature = (
        mode,
        positions,
        indices,
        normals=vec![],
        uvs=vec![],
        cull=pyxel::cube::primitive::CULL_BACK,
    ))]
    fn new(
        mode: i32,
        positions: Vec<f32>,
        indices: Vec<i32>,
        normals: Vec<f32>,
        uvs: Vec<f32>,
        cull: i32,
    ) -> Self {
        let p = pyxel::cube::Primitive::new();
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

    // Factories

    #[staticmethod]
    #[pyo3(signature = (width=1.0, height=1.0))]
    fn plane(width: f32, height: f32) -> Self {
        Self::wrap(pyxel::cube::Primitive::plane(width, height))
    }

    #[staticmethod]
    #[pyo3(signature = (size=None))]
    fn r#box(size: Option<PyRef<'_, Vec3>>) -> Self {
        let default_size = pyxel::cube::Vec3::one();
        let size_inner = size
            .as_ref()
            .map_or_else(|| rc_ref!(&default_size), |v| v.inner_ref());
        Self::wrap(pyxel::cube::Primitive::r#box(size_inner))
    }

    #[staticmethod]
    #[pyo3(signature = (radius=0.5))]
    fn sphere(radius: f32) -> Self {
        Self::wrap(pyxel::cube::Primitive::sphere(radius))
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
            "Primitive(positions={}, mode={}, cull={})",
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

pub fn add_primitive_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Positions>()?;
    m.add_class::<Indices>()?;
    m.add_class::<Normals>()?;
    m.add_class::<Uvs>()?;
    m.add_class::<Primitive>()?;
    Ok(())
}
