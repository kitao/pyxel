macro_rules! as_int {
    ($type: ty, $var: ident) => {
        if let Ok($var) = $var.cast_as::<pyo3::types::PyInt>() {
            $var.extract::<$type>().unwrap()
        } else {
            $var.extract::<f64>().unwrap() as $type
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
        } else {
            let $var = $var.extract::<$type2>().unwrap();
            $block2
        }
    };

    ($var: ident, $type1: ty, $block1: block, $type2: ty, $block2: block, $type3: ty, $block3: block, $type4: ty, $block4: block) => {
        if let Ok($var) = $var.extract::<$type1>() {
            $block1
        } else if let Ok($var) = $var.extract::<$type2>() {
            $block2
        } else if let Ok($var) = $var.extract::<$type3>() {
            $block3
        } else {
            let $var = $var.extract::<$type4>().unwrap();
            $block4
        }
    };
}
