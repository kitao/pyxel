macro_rules! global_instance {
    ($struct_name:ident, $instance_name:ident) => {
        static mut INSTANCE: Option<$struct_name> = None;

        fn set_instance($instance_name: $struct_name) {
            unsafe {
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
pub use system::init_system;
pub use system::system;
pub use system::System;

mod resource;
pub use resource::init_resource;
pub use resource::resource;
pub use resource::Resource;

mod input;
pub use input::init_input;
pub use input::input;
pub use input::Input;

mod graphics;
pub use graphics::graphics;
pub use graphics::init_graphics;
pub use graphics::Graphics;

mod audio;
pub use audio::audio;
pub use audio::init_audio;
pub use audio::Audio;
