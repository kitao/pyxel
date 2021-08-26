macro_rules! as_int {
    ($type: ident, $name: ident) => {
        if let Ok($name) = $name.cast_as::<pyo3::types::PyFloat>() {
            $name.extract::<f64>().unwrap() as $type
        } else {
            $name.extract::<$type>().unwrap()
        }
    };
}
