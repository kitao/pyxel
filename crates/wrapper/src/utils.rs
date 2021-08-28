macro_rules! type_error {
    ($msg: expr) => {
        return Err(pyo3::exceptions::PyTypeError::new_err($msg))
    };
}

macro_rules! as_int {
    ($type: ty, $var: ident) => {
        if let Ok($var) = $var.cast_as::<pyo3::types::PyInt>() {
            $var.extract::<$type>().unwrap()
        } else if let Ok($var) = $var.cast_as::<pyo3::types::PyFloat>() {
            $var.extract::<f64>().unwrap() as $type
        } else {
            type_error!("must be real number");
        }
    };
}

macro_rules! as_i32 {
    ($var: ident) => {
        as_int!(i32, $var)
    };
}

macro_rules! as_u32 {
    ($var: ident) => {
        as_int!(u32, $var)
    };
}

macro_rules! type_switch {
    ($var: ident, $type1: ty, $block1: block, $type2: ty, $block2: block) => {
        if let Ok($var) = $var.extract::<$type1>() {
            $block1
        } else if let Ok($var) = $var.extract::<$type2>() {
            $block2
        } else {
            type_error!(format!(
                "must be {} or {}",
                stringify!($type1),
                stringify!($type2)
            ));
        }
    };

    ($var: ident, $type1: ty, $block1: block, $type2: ty, $block2: block, $type3: ty, $block3: block, $type4: ty, $block4: block) => {
        if let Ok($var) = $var.extract::<$type1>() {
            $block1
        } else if let Ok($var) = $var.extract::<$type2>() {
            $block2
        } else if let Ok($var) = $var.extract::<$type3>() {
            $block3
        } else if let Ok($var) = $var.extract::<$type4>() {
            $block4
        } else {
            type_error!(format!(
                "must be {}, {}, {}, or {}",
                stringify!($type1),
                stringify!($type2),
                stringify!($type3),
                stringify!($type4)
            ));
        }
    };
}
