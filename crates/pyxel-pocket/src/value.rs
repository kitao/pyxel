use std::ffi::{CStr, CString};

use crate::ffi;

const VALUE_SIZE: usize = 16;

pub unsafe fn arg(argv: ffi::py_StackRef, index: usize) -> ffi::py_Ref {
    argv.cast::<u8>().add(index * VALUE_SIZE).cast()
}

pub unsafe fn is_none(value: ffi::py_Ref) -> bool {
    ffi::py_istype(value, ffi::py_PredefinedTypes_tp_NoneType as ffi::py_Type)
}

pub unsafe fn int_arg(argv: ffi::py_StackRef, index: usize) -> i64 {
    ffi::py_toint(arg(argv, index))
}

pub unsafe fn opt_int_arg(argv: ffi::py_StackRef, index: usize) -> Option<i64> {
    let value = arg(argv, index);
    if is_none(value) {
        None
    } else {
        Some(ffi::py_toint(value))
    }
}

pub unsafe fn opt_bool_arg(argv: ffi::py_StackRef, index: usize) -> Option<bool> {
    let value = arg(argv, index);
    if is_none(value) {
        None
    } else {
        Some(ffi::py_tobool(value))
    }
}

pub unsafe fn opt_str_arg(argv: ffi::py_StackRef, index: usize) -> Option<String> {
    let value = arg(argv, index);
    if is_none(value) {
        None
    } else {
        let sv = ffi::py_tosv(value);
        let bytes = std::slice::from_raw_parts(sv.data.cast::<u8>(), sv.size as usize);
        Some(String::from_utf8_lossy(bytes).into_owned())
    }
}

pub unsafe fn return_none() {
    ffi::py_newnone(ffi::py_retval());
}

pub unsafe fn set_module_int(module: ffi::py_GlobalRef, name: &CStr, value: i64) {
    let temp = ffi::py_pushtmp();
    ffi::py_newint(temp, value);
    ffi::py_setattr(module, ffi::py_name(name.as_ptr()), temp);
    ffi::py_pop();
}

pub unsafe fn set_const_int(module: ffi::py_GlobalRef, name: &str, value: i64) {
    let name = CString::new(name).unwrap();
    ffi::py_newint(
        ffi::py_emplacedict(module, ffi::py_name(name.as_ptr())),
        value,
    );
}
