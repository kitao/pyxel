pub mod color_palette;
pub mod graphics;
pub mod graphics_buffer;
pub mod image_buffer;
pub mod rectarea;
pub mod system;
pub mod tilemap_buffer;

/*
macro_rules! global_instance {
    ($struct_name:ident, $instance_name:ident) => {
        static mut INSTANCE: Option<$struct_name> = None;

        fn set_instance($instance_name: $struct_name) {
            unsafe {
                if INSTANCE.is_some() {
                    panic!(concat!(
                        stringify!($instance_name),
                        " is already initialized"
                    ));
                }

                INSTANCE = Some($instance_name);
            }
        }

        #[inline]
        pub fn $instance_name() -> &'static mut $struct_name {
            unsafe {
                INSTANCE
                    .as_mut()
                    .expect(concat!(stringify!($struct_name), " is not initialized"))
            }
        }
    };
}

mod system;
pub use system::{init_system, system, System};
*/
