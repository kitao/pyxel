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

pub unsafe fn float_arg(argv: ffi::py_StackRef, index: usize) -> Option<f32> {
    let mut out = 0.0;
    if ffi::py_castfloat32(arg(argv, index), &mut out) {
        Some(out)
    } else {
        None
    }
}

pub unsafe fn opt_float_arg(argv: ffi::py_StackRef, index: usize) -> Option<f32> {
    let value = arg(argv, index);
    if is_none(value) {
        None
    } else {
        float_arg(argv, index)
    }
}

pub unsafe fn bool_arg(argv: ffi::py_StackRef, index: usize) -> bool {
    ffi::py_tobool(arg(argv, index))
}

pub unsafe fn str_arg(argv: ffi::py_StackRef, index: usize) -> String {
    opt_str_arg(argv, index).unwrap_or_default()
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

pub unsafe fn str_list_arg(argv: ffi::py_StackRef, index: usize) -> Vec<String> {
    let value = arg(argv, index);
    (0..ffi::py_list_len(value))
        .map(|i| {
            let sv = ffi::py_tosv(ffi::py_list_getitem(value, i));
            let bytes = std::slice::from_raw_parts(sv.data.cast::<u8>(), sv.size as usize);
            String::from_utf8_lossy(bytes).into_owned()
        })
        .collect()
}

pub unsafe fn int_list_arg(argv: ffi::py_StackRef, index: usize) -> Vec<u32> {
    let value = arg(argv, index);
    (0..ffi::py_list_len(value))
        .map(|i| ffi::py_toint(ffi::py_list_getitem(value, i)) as u32)
        .collect()
}

pub unsafe fn tuple3_float_arg(argv: ffi::py_StackRef, index: usize) -> Option<(f32, f32, f32)> {
    let value = arg(argv, index);
    if !ffi::py_istype(value, ffi::py_PredefinedTypes_tp_tuple as ffi::py_Type) {
        return None;
    }
    if ffi::py_tuple_len(value) != 3 {
        return None;
    }

    let mut out = [0.0; 3];
    for i in 0..3 {
        if !ffi::py_castfloat32(ffi::py_tuple_getitem(value, i), &mut out[i as usize]) {
            return None;
        }
    }
    Some((out[0], out[1], out[2]))
}

pub unsafe fn return_none() {
    ffi::py_newnone(ffi::py_retval());
}

pub unsafe fn return_int(value: i64) {
    ffi::py_newint(ffi::py_retval(), value);
}

pub unsafe fn return_float(value: f32) {
    ffi::py_newfloat(ffi::py_retval(), value as f64);
}

pub unsafe fn return_bool(value: bool) {
    ffi::py_newbool(ffi::py_retval(), value);
}

pub unsafe fn return_str(value: &str) {
    let value = CString::new(value).unwrap();
    ffi::py_newstr(ffi::py_retval(), value.as_ptr());
}

pub unsafe fn return_str_list(values: &[String]) {
    ffi::py_newlist(ffi::py_retval());
    let list = ffi::py_retval();
    for value in values {
        let value = CString::new(value.as_str()).unwrap();
        ffi::py_newstr(ffi::py_list_emplace(list), value.as_ptr());
    }
}

pub unsafe fn return_optional_play_pos(value: Option<(u32, f32)>) {
    match value {
        Some((sound_index, sec)) => {
            let tuple = ffi::py_newtuple(ffi::py_retval(), 2);
            ffi::py_newint(ffi::py_tuple_getitem(tuple, 0), sound_index as i64);
            ffi::py_newfloat(ffi::py_tuple_getitem(tuple, 1), sec as f64);
        }
        None => return_none(),
    }
}

pub unsafe fn raise_exception(message: &str) -> bool {
    let message = CString::new(message).unwrap();
    ffi::py_exception(
        ffi::py_PredefinedTypes_tp_Exception as ffi::py_Type,
        message.as_ptr(),
    )
}

pub unsafe fn raise_value_error(message: &str) -> bool {
    let message = CString::new(message).unwrap();
    ffi::py_exception(
        ffi::py_PredefinedTypes_tp_ValueError as ffi::py_Type,
        message.as_ptr(),
    )
}

pub unsafe fn set_module_value(module: ffi::py_GlobalRef, name: &CStr, value: ffi::py_Ref) {
    ffi::py_setdict(module, ffi::py_name(name.as_ptr()), value);
}

pub unsafe fn call_module_function(module: ffi::py_GlobalRef, name: &CStr) {
    let function = ffi::py_getdict(module, ffi::py_name(name.as_ptr()));
    if function.is_null() {
        eprintln!(
            "PocketPy callback '{}' is not registered",
            name.to_string_lossy()
        );
        std::process::exit(1);
    }
    if !ffi::py_call(function, 0, std::ptr::null_mut()) {
        ffi::py_printexc();
        std::process::exit(1);
    }
}

pub unsafe fn set_module_int(module: ffi::py_GlobalRef, name: &CStr, value: i64) {
    let temp = ffi::py_pushtmp();
    ffi::py_newint(temp, value);
    ffi::py_setattr(module, ffi::py_name(name.as_ptr()), temp);
    ffi::py_pop();
}

pub unsafe fn set_module_str(module: ffi::py_GlobalRef, name: &CStr, value: &str) {
    let temp = ffi::py_pushtmp();
    let value = CString::new(value).unwrap();
    ffi::py_newstr(temp, value.as_ptr());
    ffi::py_setattr(module, ffi::py_name(name.as_ptr()), temp);
    ffi::py_pop();
}

pub unsafe fn set_module_int_list(module: ffi::py_GlobalRef, name: &CStr, values: &[i64]) {
    let temp = ffi::py_pushtmp();
    ffi::py_newlist(temp);
    for value in values {
        ffi::py_newint(ffi::py_list_emplace(temp), *value);
    }
    ffi::py_setattr(module, ffi::py_name(name.as_ptr()), temp);
    ffi::py_pop();
}

pub unsafe fn set_module_str_list(module: ffi::py_GlobalRef, name: &CStr, values: &[String]) {
    let temp = ffi::py_pushtmp();
    ffi::py_newlist(temp);
    for value in values {
        let value = CString::new(value.as_str()).unwrap();
        ffi::py_newstr(ffi::py_list_emplace(temp), value.as_ptr());
    }
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

pub unsafe fn set_const_str(module: ffi::py_GlobalRef, name: &str, value: &str) {
    let name = CString::new(name).unwrap();
    let value = CString::new(value).unwrap();
    ffi::py_newstr(
        ffi::py_emplacedict(module, ffi::py_name(name.as_ptr())),
        value.as_ptr(),
    );
}

pub unsafe fn set_const_int_list(module: ffi::py_GlobalRef, name: &str, values: &[i64]) {
    let name = CString::new(name).unwrap();
    let list = ffi::py_emplacedict(module, ffi::py_name(name.as_ptr()));
    ffi::py_newlist(list);
    for value in values {
        ffi::py_newint(ffi::py_list_emplace(list), *value);
    }
}
