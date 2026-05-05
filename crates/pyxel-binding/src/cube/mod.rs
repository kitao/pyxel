use pyo3::prelude::*;

mod camera;
mod light;
mod mat4;
mod quat;
mod ramp;
mod vec3;

pub fn add_cube_submodule(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent.py(), "cube")?;
    vec3::add_vec3_class(&m)?;
    mat4::add_mat4_class(&m)?;
    quat::add_quat_class(&m)?;
    camera::add_camera_class(&m)?;
    light::add_light_class(&m)?;
    ramp::add_ramp_class(&m)?;
    parent.add_submodule(&m)?;
    Ok(())
}
