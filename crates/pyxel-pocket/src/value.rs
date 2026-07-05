use std::ffi::{c_void, CStr, CString};

use crate::ffi;

const VALUE_SIZE: usize = 16;

extern "C" {
    fn free(ptr: *mut c_void);
}

pub unsafe fn arg(argv: ffi::py_StackRef, index: usize) -> ffi::py_Ref {
    argv.cast::<u8>().add(index * VALUE_SIZE).cast()
}

unsafe fn tuple_item(tuple: ffi::py_ObjectRef, index: usize) -> ffi::py_Ref {
    tuple.cast::<u8>().add(index * VALUE_SIZE).cast()
}

pub unsafe fn is_none(value: ffi::py_Ref) -> bool {
    ffi::py_istype(value, ffi::py_PredefinedTypes_tp_NoneType as ffi::py_Type)
}

pub unsafe fn is_int(value: ffi::py_Ref) -> bool {
    ffi::py_istype(value, ffi::py_PredefinedTypes_tp_int as ffi::py_Type)
}

pub unsafe fn is_list(value: ffi::py_Ref) -> bool {
    ffi::py_istype(value, ffi::py_PredefinedTypes_tp_list as ffi::py_Type)
}

pub unsafe fn is_str(value: ffi::py_Ref) -> bool {
    ffi::py_istype(value, ffi::py_PredefinedTypes_tp_str as ffi::py_Type)
}

pub unsafe fn is_tuple(value: ffi::py_Ref) -> bool {
    ffi::py_istype(value, ffi::py_PredefinedTypes_tp_tuple as ffi::py_Type)
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
            ffi::py_newint(tuple_item(tuple, 0), sound_index as i64);
            ffi::py_newfloat(tuple_item(tuple, 1), sec as f64);
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

pub unsafe fn raise_index_error(message: &str) -> bool {
    let message = CString::new(message).unwrap();
    ffi::py_exception(
        ffi::py_PredefinedTypes_tp_IndexError as ffi::py_Type,
        message.as_ptr(),
    )
}

pub unsafe fn set_module_value(module: ffi::py_GlobalRef, name: &CStr, value: ffi::py_Ref) {
    ffi::py_setdict(module, ffi::py_name(name.as_ptr()), value);
}

pub unsafe fn bind_magic(type_: ffi::py_Type, name: &CStr, function: ffi::py_CFunction) {
    ffi::py_newnativefunc(
        ffi::py_tpgetmagic(type_, ffi::py_name(name.as_ptr())),
        function,
    );
}

pub unsafe fn new_type(
    module: ffi::py_GlobalRef,
    name: &CStr,
    destructor: ffi::py_Dtor,
) -> ffi::py_Type {
    ffi::py_newtype(
        name.as_ptr(),
        ffi::py_PredefinedTypes_tp_object as ffi::py_Type,
        module,
        destructor,
    )
}

pub unsafe fn call_module_function(module: ffi::py_GlobalRef, name: &CStr) -> Result<(), String> {
    let function = ffi::py_getdict(module, ffi::py_name(name.as_ptr()));
    if function.is_null() {
        return Err(format!(
            "PocketPy callback '{}' is not registered",
            name.to_string_lossy()
        ));
    }
    if !ffi::py_call(function, 0, std::ptr::null_mut()) {
        return Err(format_exception());
    }
    Ok(())
}

pub(crate) unsafe fn format_exception() -> String {
    let message = ffi::py_formatexc();
    if message.is_null() {
        return "PocketPy failed without an active exception".to_owned();
    }

    let result = CStr::from_ptr(message).to_string_lossy().into_owned();
    free(message.cast::<c_void>());
    result
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

pub unsafe fn set_module_object(module: ffi::py_GlobalRef, name: &CStr, type_: ffi::py_Type) {
    ffi::py_newobject(
        ffi::py_emplacedict(module, ffi::py_name(name.as_ptr())),
        type_,
        0,
        0,
    );
}

pub unsafe fn return_tile(value: pyxel::Tile) {
    let tuple = ffi::py_newtuple(ffi::py_retval(), 2);
    ffi::py_newint(tuple_item(tuple, 0), value.0 as i64);
    ffi::py_newint(tuple_item(tuple, 1), value.1 as i64);
}

pub unsafe fn return_float_pair(value: (f32, f32)) {
    let tuple = ffi::py_newtuple(ffi::py_retval(), 2);
    ffi::py_newfloat(tuple_item(tuple, 0), value.0 as f64);
    ffi::py_newfloat(tuple_item(tuple, 1), value.1 as f64);
}
