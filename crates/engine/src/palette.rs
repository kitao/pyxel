pub trait Palette<T> {
    fn get_render_color(&self, original_color: T) -> T;
}
