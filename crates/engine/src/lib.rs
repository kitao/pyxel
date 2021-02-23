macro_rules! global_instance {
    ($struct_name:ident, $instance_name:ident) => {
        static mut INSTANCE: Option<$struct_name> = None;

        fn set_instance($instance_name: $struct_name) {
            unsafe {
                /*if INSTANCE.is_none() {
                    panic!(concat!(
                        stringify!($instance_name),
                        " is already initialized"
                    ));
                }*/

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

mod resource;
pub use resource::{init_resource, resource, Resource};

mod input;
pub use input::{init_input, input, Input};

mod graphics;
pub use graphics::{graphics, init_graphics, Graphics};

mod audio;
pub use audio::{audio, init_audio, Audio};
