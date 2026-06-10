use pyo3::prelude::*;

use super::mesh_data::MeshData;
use super::vec3::Vec3;

define_wrapper!(Collider, pyxel::cube::Collider);

#[pymethods]
impl Collider {
    #[new]
    #[pyo3(signature = (
        size=None,
        radius=0.0,
        mesh=None,
        trigger=false,
        rolls=false,
        mass=1.0,
        restitution=0.0,
        friction=0.5,
        velocity=None,
        angular_velocity=None,
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        size: Option<PyRef<'_, Vec3>>,
        radius: f32,
        mesh: Option<PyRef<'_, MeshData>>,
        trigger: bool,
        rolls: bool,
        mass: f32,
        restitution: f32,
        friction: f32,
        velocity: Option<PyRef<'_, Vec3>>,
        angular_velocity: Option<PyRef<'_, Vec3>>,
    ) -> Self {
        let size_rc = size
            .as_ref()
            .map_or_else(pyxel::cube::Vec3::zero, |v| v.inner.clone());
        let mesh_rc = mesh.as_ref().map(|m| m.inner.clone());
        let velocity_rc = velocity
            .as_ref()
            .map_or_else(pyxel::cube::Vec3::zero, |v| v.inner.clone());
        let angular_velocity_rc = angular_velocity
            .as_ref()
            .map_or_else(pyxel::cube::Vec3::zero, |v| v.inner.clone());
        Self::wrap(pyxel::cube::Collider::new(
            size_rc,
            radius,
            mesh_rc,
            trigger,
            rolls,
            mass,
            restitution,
            friction,
            velocity_rc,
            angular_velocity_rc,
        ))
    }

    #[getter]
    fn size(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().size.clone())
    }

    #[setter]
    fn set_size(&self, v: PyRef<'_, Vec3>) {
        self.inner_mut().size = v.inner.clone();
    }

    #[getter]
    fn radius(&self) -> f32 {
        self.inner_ref().radius
    }

    #[setter]
    fn set_radius(&self, v: f32) {
        self.inner_mut().radius = v;
    }

    #[getter]
    fn mesh(&self) -> Option<MeshData> {
        self.inner_ref()
            .mesh
            .as_ref()
            .map(|m| MeshData::wrap(m.clone()))
    }

    #[setter]
    fn set_mesh(&self, v: Option<PyRef<'_, MeshData>>) {
        self.inner_mut().mesh = v.as_ref().map(|m| m.inner.clone());
    }

    #[getter]
    fn trigger(&self) -> bool {
        self.inner_ref().trigger
    }

    #[setter]
    fn set_trigger(&self, v: bool) {
        self.inner_mut().trigger = v;
    }

    #[getter]
    fn rolls(&self) -> bool {
        self.inner_ref().rolls
    }

    #[setter]
    fn set_rolls(&self, v: bool) {
        self.inner_mut().rolls = v;
    }

    #[getter]
    fn mass(&self) -> f32 {
        self.inner_ref().mass
    }

    #[setter]
    fn set_mass(&self, v: f32) {
        self.inner_mut().mass = v;
    }

    #[getter]
    fn restitution(&self) -> f32 {
        self.inner_ref().restitution
    }

    #[setter]
    fn set_restitution(&self, v: f32) {
        self.inner_mut().restitution = v;
    }

    #[getter]
    fn friction(&self) -> f32 {
        self.inner_ref().friction
    }

    #[setter]
    fn set_friction(&self, v: f32) {
        self.inner_mut().friction = v;
    }

    #[getter]
    fn velocity(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().velocity.clone())
    }

    #[setter]
    fn set_velocity(&self, v: PyRef<'_, Vec3>) {
        self.inner_mut().velocity = v.inner.clone();
    }

    #[getter]
    fn angular_velocity(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().angular_velocity.clone())
    }

    #[setter]
    fn set_angular_velocity(&self, v: PyRef<'_, Vec3>) {
        self.inner_mut().angular_velocity = v.inner.clone();
    }

    fn __repr__(&self) -> String {
        let c = self.inner_ref();
        let s = rc_ref!(&c.size);
        format!(
            "Collider(size=Vec3({}, {}, {}), radius={}, mass={})",
            s.x, s.y, s.z, c.radius, c.mass,
        )
    }
}

pub fn add_collider_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Collider>()?;
    Ok(())
}
