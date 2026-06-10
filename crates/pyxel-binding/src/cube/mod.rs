use pyo3::prelude::*;

mod camera;
mod collider;
mod contact;
mod mat4;
mod mesh_data;
mod node;
mod prim_data;
mod quat;
mod raycast_hit;
mod shading;
mod vec3;

pub fn add_cube_submodule(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent.py(), "cube")?;
    vec3::add_vec3_class(&m)?;
    mat4::add_mat4_class(&m)?;
    quat::add_quat_class(&m)?;
    camera::add_camera_class(&m)?;
    shading::add_shading_class(&m)?;
    prim_data::add_prim_data_class(&m)?;
    mesh_data::add_mesh_data_class(&m)?;
    collider::add_collider_class(&m)?;
    contact::add_contact_class(&m)?;
    raycast_hit::add_raycast_hit_class(&m)?;
    node::add_node_class(&m)?;
    parent.add_submodule(&m)?;
    Ok(())
}
