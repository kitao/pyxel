use crate::ffi;
use crate::helpers::*;

unsafe extern "C" fn pyxel_ceil(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_int(pyxel::Pyxel::ceil(arg_float(argv, 0) as f32) as i64);
    true
}

unsafe extern "C" fn pyxel_floor(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_int(pyxel::Pyxel::floor(arg_float(argv, 0) as f32) as i64);
    true
}

unsafe extern "C" fn pyxel_clamp(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let x = arg(argv, 0);
    if is_int(x) {
        let xi = ffi::py_toint(x);
        let lo = arg_int(argv, 1);
        let hi = arg_int(argv, 2);
        let (lo, hi) = if lo < hi { (lo, hi) } else { (hi, lo) };
        ret_int(xi.clamp(lo, hi));
    } else {
        let xf = arg_float(argv, 0);
        let lo = arg_float(argv, 1);
        let hi = arg_float(argv, 2);
        let (lo, hi) = if lo < hi { (lo, hi) } else { (hi, lo) };
        ret_float(xf.clamp(lo, hi));
    }
    true
}

unsafe extern "C" fn pyxel_sgn(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let a = arg(argv, 0);
    if is_int(a) {
        let v = ffi::py_toint(a);
        ret_int(if v > 0 { 1 } else if v < 0 { -1 } else { 0 });
    } else {
        let v = ffi::py_tofloat(a);
        ret_float(if v > 0.0 {
            1.0
        } else if v < 0.0 {
            -1.0
        } else {
            0.0
        });
    }
    true
}

unsafe extern "C" fn pyxel_sqrt(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_float(pyxel::Pyxel::sqrt(arg_float(argv, 0) as f32) as f64);
    true
}

unsafe extern "C" fn pyxel_sin(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_float(pyxel::Pyxel::sin(arg_float(argv, 0) as f32) as f64);
    true
}

unsafe extern "C" fn pyxel_cos(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_float(pyxel::Pyxel::cos(arg_float(argv, 0) as f32) as f64);
    true
}

unsafe extern "C" fn pyxel_atan2(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_float(
        pyxel::Pyxel::atan2(arg_float(argv, 0) as f32, arg_float(argv, 1) as f32) as f64,
    );
    true
}

unsafe extern "C" fn pyxel_rseed(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::Pyxel::random_seed(arg_int(argv, 0) as u32);
    ret_none();
    true
}

unsafe extern "C" fn pyxel_rndi(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_int(pyxel::Pyxel::random_int(arg_int(argv, 0) as i32, arg_int(argv, 1) as i32) as i64);
    true
}

unsafe extern "C" fn pyxel_rndf(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_float(
        pyxel::Pyxel::random_float(arg_float(argv, 0) as f32, arg_float(argv, 1) as f32) as f64,
    );
    true
}

unsafe extern "C" fn pyxel_nseed(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::Pyxel::noise_seed(arg_int(argv, 0) as u32);
    ret_none();
    true
}

unsafe extern "C" fn pyxel_noise(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let x = arg_float(argv, 0) as f32;
    let y = arg_opt_float(argv, 1).map(|v| v as f32).unwrap_or(0.0);
    let z = arg_opt_float(argv, 2).map(|v| v as f32).unwrap_or(0.0);
    ret_float(pyxel::Pyxel::noise(x, y, z) as f64);
    true
}

pub unsafe fn add_math_functions(m: ffi::py_GlobalRef) {
    bind(m, c"ceil(x)", Some(pyxel_ceil));
    bind(m, c"floor(x)", Some(pyxel_floor));
    bind(m, c"clamp(x, lower, upper)", Some(pyxel_clamp));
    bind(m, c"sgn(x)", Some(pyxel_sgn));
    bind(m, c"sqrt(x)", Some(pyxel_sqrt));
    bind(m, c"sin(deg)", Some(pyxel_sin));
    bind(m, c"cos(deg)", Some(pyxel_cos));
    bind(m, c"atan2(y, x)", Some(pyxel_atan2));
    bind(m, c"rseed(seed)", Some(pyxel_rseed));
    bind(m, c"rndi(a, b)", Some(pyxel_rndi));
    bind(m, c"rndf(a, b)", Some(pyxel_rndf));
    bind(m, c"nseed(seed)", Some(pyxel_nseed));
    bind(m, c"noise(x, y=None, z=None)", Some(pyxel_noise));
}
