use std::cmp::Ordering;

use crate::{ffi, value};

unsafe extern "C" fn ceil(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(x) = value::float_arg(argv, 0) else {
        return false;
    };
    value::return_int(pyxel::Pyxel::ceil(x) as i64);
    true
}

unsafe extern "C" fn floor(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(x) = value::float_arg(argv, 0) else {
        return false;
    };
    value::return_int(pyxel::Pyxel::floor(x) as i64);
    true
}

unsafe extern "C" fn clamp(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let x = value::arg(argv, 0);
    let lower = value::arg(argv, 1);
    let upper = value::arg(argv, 2);

    if ffi::py_istype(x, ffi::py_PredefinedTypes_tp_int as ffi::py_Type)
        && ffi::py_istype(lower, ffi::py_PredefinedTypes_tp_int as ffi::py_Type)
        && ffi::py_istype(upper, ffi::py_PredefinedTypes_tp_int as ffi::py_Type)
    {
        let x = ffi::py_toint(x);
        let lower = ffi::py_toint(lower);
        let upper = ffi::py_toint(upper);
        let (lower, upper) = if lower < upper {
            (lower, upper)
        } else {
            (upper, lower)
        };
        value::return_int(x.clamp(lower, upper));
    } else {
        let Some(x) = value::float_arg(argv, 0) else {
            return false;
        };
        let Some(lower) = value::float_arg(argv, 1) else {
            return false;
        };
        let Some(upper) = value::float_arg(argv, 2) else {
            return false;
        };
        let (lower, upper) = if lower < upper {
            (lower, upper)
        } else {
            (upper, lower)
        };
        value::return_float(x.clamp(lower, upper));
    }
    true
}

unsafe extern "C" fn sgn(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let x = value::arg(argv, 0);
    if ffi::py_istype(x, ffi::py_PredefinedTypes_tp_int as ffi::py_Type) {
        value::return_int(match ffi::py_toint(x).cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        });
    } else {
        let Some(x) = value::float_arg(argv, 0) else {
            return false;
        };
        value::return_float(match x.partial_cmp(&0.0) {
            Some(Ordering::Greater) => 1.0,
            Some(Ordering::Less) => -1.0,
            _ => 0.0,
        });
    }
    true
}

unsafe extern "C" fn sqrt(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(x) = value::float_arg(argv, 0) else {
        return false;
    };
    value::return_float(pyxel::Pyxel::sqrt(x));
    true
}

unsafe extern "C" fn sin(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(degrees) = value::float_arg(argv, 0) else {
        return false;
    };
    value::return_float(pyxel::Pyxel::sin(degrees));
    true
}

unsafe extern "C" fn cos(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(degrees) = value::float_arg(argv, 0) else {
        return false;
    };
    value::return_float(pyxel::Pyxel::cos(degrees));
    true
}

unsafe extern "C" fn atan2(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(y) = value::float_arg(argv, 0) else {
        return false;
    };
    let Some(x) = value::float_arg(argv, 1) else {
        return false;
    };
    value::return_float(pyxel::Pyxel::atan2(y, x));
    true
}

unsafe extern "C" fn rseed(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::Pyxel::random_seed(value::int_arg(argv, 0) as u32);
    value::return_none();
    true
}

unsafe extern "C" fn rndi(_argc: i32, argv: ffi::py_StackRef) -> bool {
    value::return_int(pyxel::Pyxel::random_int(
        value::int_arg(argv, 0) as i32,
        value::int_arg(argv, 1) as i32,
    ) as i64);
    true
}

unsafe extern "C" fn rndf(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(min) = value::float_arg(argv, 0) else {
        return false;
    };
    let Some(max) = value::float_arg(argv, 1) else {
        return false;
    };
    value::return_float(pyxel::Pyxel::random_float(min, max));
    true
}

unsafe extern "C" fn nseed(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::Pyxel::noise_seed(value::int_arg(argv, 0) as u32);
    value::return_none();
    true
}

unsafe extern "C" fn noise(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(x) = value::float_arg(argv, 0) else {
        return false;
    };
    value::return_float(pyxel::Pyxel::noise(
        x,
        value::opt_float_arg(argv, 1).unwrap_or(0.0),
        value::opt_float_arg(argv, 2).unwrap_or(0.0),
    ));
    true
}

pub unsafe fn add_functions(module: ffi::py_GlobalRef) {
    ffi::py_bind(module, c"ceil(x)".as_ptr(), Some(ceil));
    ffi::py_bind(module, c"floor(x)".as_ptr(), Some(floor));
    ffi::py_bind(module, c"clamp(x, lower, upper)".as_ptr(), Some(clamp));
    ffi::py_bind(module, c"sgn(x)".as_ptr(), Some(sgn));
    ffi::py_bind(module, c"sqrt(x)".as_ptr(), Some(sqrt));
    ffi::py_bind(module, c"sin(deg)".as_ptr(), Some(sin));
    ffi::py_bind(module, c"cos(deg)".as_ptr(), Some(cos));
    ffi::py_bind(module, c"atan2(y, x)".as_ptr(), Some(atan2));
    ffi::py_bind(module, c"rseed(seed)".as_ptr(), Some(rseed));
    ffi::py_bind(module, c"rndi(a, b)".as_ptr(), Some(rndi));
    ffi::py_bind(module, c"rndf(a, b)".as_ptr(), Some(rndf));
    ffi::py_bind(module, c"nseed(seed)".as_ptr(), Some(nseed));
    ffi::py_bind(module, c"noise(x, y=0, z=0)".as_ptr(), Some(noise));
}
