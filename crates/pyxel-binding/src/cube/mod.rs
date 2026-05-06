use pyo3::prelude::*;

mod camera;
mod color_ramp;
mod light;
mod mat4;
mod mesh;
mod node;
mod quat;
mod scene;
mod vec3;

pub fn add_cube_submodule(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent.py(), "cube")?;
    vec3::add_vec3_class(&m)?;
    mat4::add_mat4_class(&m)?;
    quat::add_quat_class(&m)?;
    camera::add_camera_class(&m)?;
    light::add_light_class(&m)?;
    color_ramp::add_color_ramp_class(&m)?;
    mesh::add_mesh_class(&m)?;
    node::add_node_class(&m)?;
    scene::add_scene_class(&m)?;
    parent.add_submodule(&m)?;
    Ok(())
}
