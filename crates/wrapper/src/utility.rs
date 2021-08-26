macro_rules! as_int {
    ($type: ident, $name: ident) => {
        if let Ok($name) = $name.cast_as::<PyInt>() {
            $name.extract::<$type>().unwrap()
        } else {
            $name.extract::<f64>().unwrap() as $type
        }
    };
}
