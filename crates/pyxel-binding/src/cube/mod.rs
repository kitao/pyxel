pub mod camera_wrapper;
pub mod light_wrapper;
pub mod math_wrapper;
pub mod model_wrapper;
pub mod scene_wrapper;

use pyo3::prelude::*;

pub fn add_cube_classes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    math_wrapper::add_cube_math_classes(m)?;
    camera_wrapper::add_cube_camera_class(m)?;
    light_wrapper::add_cube_light_class(m)?;
    model_wrapper::add_cube_model_class(m)?;
    scene_wrapper::add_cube_scene_class(m)?;
    Ok(())
}
