pub mod camera;
pub mod light;
pub mod math;
pub mod model;
pub mod rasterizer;
pub mod scene;

pub use camera::Camera;
pub use light::Light;
pub use math::{Mat4, Vec3};
pub use model::{Face, FaceMaterial, Model, ModelNode, Uv};
pub use rasterizer::{ShadePalette, DEFAULT_SHADE_PALETTE, NUM_SHADE_LEVELS};
pub use scene::Scene;
