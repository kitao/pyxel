use pyo3::prelude::*;

use super::vec3::Vec3;

// Live sequence proxies retain their Primitive and mutate its Vec fields in place.
macro_rules! wrap_primitive_as_python_list {
    (
        $wrapper_name:ident,
        $python_name:literal,
        $value_type:ty,
        $field_name:ident,
        $field_mut:ident
    ) => {
        wrap_as_python_primitive_sequence!(
            $wrapper_name,
            pyxel::cube::RcPrimitive,
            (|inner: &pyxel::cube::RcPrimitive| rc_ref!(inner).$field_name.len()),
            $value_type,
            (|inner: &pyxel::cube::RcPrimitive, index| rc_ref!(inner).$field_name[index]),
            $value_type,
            (|inner: &pyxel::cube::RcPrimitive, index, value| $field_mut(inner)[index] = value),
            $field_mut,
            Vec<$value_type>,
            (|inner: &pyxel::cube::RcPrimitive, list| *$field_mut(inner) = list),
            (|inner: &pyxel::cube::RcPrimitive| rc_ref!(inner).$field_name.clone()),
            module = "pyxel.cube",
            name = $python_name
        );
    };
}

wrap_primitive_as_python_list!(Positions, "_Positions", f32, positions, positions_mut);
wrap_primitive_as_python_list!(Indices, "_Indices", i32, indices, indices_mut);
wrap_primitive_as_python_list!(Normals, "_Normals", f32, normals, normals_mut);
wrap_primitive_as_python_list!(Uvs, "_Uvs", f32, uvs, uvs_mut);

define_wrapper!(Primitive, pyxel::cube::Primitive, module = "pyxel.cube");

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
            let mut p = rc_mut!(&p);
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
        let size = size.map_or(
            pyxel::cube::Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            |v| *v.inner_ref(),
        );
        Self::wrap(pyxel::cube::Primitive::r#box(&size))
    }

    #[staticmethod]
    #[pyo3(signature = (radius=0.5))]
    fn sphere(radius: f32) -> Self {
        Self::wrap(pyxel::cube::Primitive::sphere(radius))
    }

    #[staticmethod]
    #[pyo3(signature = (height=1.0, radius=0.5))]
    fn capsule(height: f32, radius: f32) -> Self {
        Self::wrap(pyxel::cube::Primitive::capsule(height, radius))
    }

    // Vertex attributes (live proxies)

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
        let mut primitive = self.inner_mut();
        if primitive.mode != v {
            primitive.mode = v;
            primitive.mark_collision_geometry_changed();
        }
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

    fn __repr__(&self) -> String {
        let p = self.inner_ref();
        format!(
            "Primitive(positions={}, mode={}, cull={})",
            p.positions.len(),
            p.mode,
            p.cull
        )
    }

    fn compute_normals(&self) {
        self.inner_mut().compute_normals();
    }
}

fn positions_mut(inner: &pyxel::cube::RcPrimitive) -> std::cell::RefMut<'_, Vec<f32>> {
    let mut primitive = rc_mut!(inner);
    primitive.mark_collision_geometry_changed();
    std::cell::RefMut::map(primitive, |primitive| &mut primitive.positions)
}

fn indices_mut(inner: &pyxel::cube::RcPrimitive) -> std::cell::RefMut<'_, Vec<i32>> {
    let mut primitive = rc_mut!(inner);
    primitive.mark_collision_geometry_changed();
    std::cell::RefMut::map(primitive, |primitive| &mut primitive.indices)
}

fn normals_mut(inner: &pyxel::cube::RcPrimitive) -> std::cell::RefMut<'_, Vec<f32>> {
    std::cell::RefMut::map(rc_mut!(inner), |primitive| &mut primitive.normals)
}

fn uvs_mut(inner: &pyxel::cube::RcPrimitive) -> std::cell::RefMut<'_, Vec<f32>> {
    std::cell::RefMut::map(rc_mut!(inner), |primitive| &mut primitive.uvs)
}

pub fn add_primitive_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Positions>()?;
    m.add_class::<Indices>()?;
    m.add_class::<Normals>()?;
    m.add_class::<Uvs>()?;
    m.add_class::<Primitive>()?;
    Ok(())
}
