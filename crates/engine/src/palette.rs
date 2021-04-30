pub trait Palette<T> {
    fn get_render_value(&self, original_value: T) -> T;
}
