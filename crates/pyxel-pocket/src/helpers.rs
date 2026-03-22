// Argument extraction and return value helpers for PocketPy C API

use std::ffi::{CStr, CString};

use crate::ffi;

// Argument extraction
pub unsafe fn arg(argv: ffi::py_StackRef, i: usize) -> ffi::py_Ref {
    argv.add(i)
}

pub unsafe fn arg_int(argv: ffi::py_StackRef, i: usize) -> i64 {
    ffi::py_toint(arg(argv, i))
}

pub unsafe fn arg_float(argv: ffi::py_StackRef, i: usize) -> f64 {
    let a = arg(argv, i);
    let mut out: f64 = 0.0;
    if ffi::py_castfloat(a, &mut out) {
        out
    } else {
        ffi::py_tofloat(a)
    }
}

pub unsafe fn arg_str<'a>(argv: ffi::py_StackRef, i: usize) -> &'a str {
    let sv = ffi::py_tosv(arg(argv, i));
    let slice = std::slice::from_raw_parts(sv.data.cast::<u8>(), sv.size as usize);
    std::str::from_utf8_unchecked(slice)
}

pub unsafe fn arg_is_none(argv: ffi::py_StackRef, i: usize) -> bool {
    is_none(arg(argv, i))
}

pub unsafe fn is_none(r: ffi::py_Ref) -> bool {
    ffi::py_istype(r, ffi::py_PredefinedType_tp_NoneType as ffi::py_Type)
}

pub unsafe fn is_int(r: ffi::py_Ref) -> bool {
    ffi::py_istype(r, ffi::py_PredefinedType_tp_int as ffi::py_Type)
}

pub unsafe fn is_list(r: ffi::py_Ref) -> bool {
    ffi::py_istype(r, ffi::py_PredefinedType_tp_list as ffi::py_Type)
}

pub unsafe fn is_str(r: ffi::py_Ref) -> bool {
    ffi::py_istype(r, ffi::py_PredefinedType_tp_str as ffi::py_Type)
}

pub unsafe fn arg_opt_int(argv: ffi::py_StackRef, i: usize) -> Option<i64> {
    if arg_is_none(argv, i) {
        None
    } else {
        Some(arg_int(argv, i))
    }
}

pub unsafe fn arg_opt_float(argv: ffi::py_StackRef, i: usize) -> Option<f64> {
    if arg_is_none(argv, i) {
        None
    } else {
        Some(arg_float(argv, i))
    }
}

pub unsafe fn arg_opt_str<'a>(argv: ffi::py_StackRef, i: usize) -> Option<&'a str> {
    if arg_is_none(argv, i) {
        None
    } else {
        Some(arg_str(argv, i))
    }
}

pub unsafe fn arg_opt_bool(argv: ffi::py_StackRef, i: usize) -> Option<bool> {
    if arg_is_none(argv, i) {
        None
    } else {
        Some(ffi::py_tobool(arg(argv, i)))
    }
}

pub unsafe fn arg_str_list(argv: ffi::py_StackRef, i: usize) -> Vec<String> {
    let list_ref = arg(argv, i);
    let len = ffi::py_list_len(list_ref);
    (0..len)
        .map(|j| {
            let sv = ffi::py_tosv(ffi::py_list_getitem(list_ref, j));
            let slice = std::slice::from_raw_parts(sv.data.cast::<u8>(), sv.size as usize);
            std::str::from_utf8_unchecked(slice).to_owned()
        })
        .collect()
}

pub unsafe fn arg_int_list(argv: ffi::py_StackRef, i: usize) -> Vec<u32> {
    let list_ref = arg(argv, i);
    let len = ffi::py_list_len(list_ref);
    (0..len)
        .map(|j| ffi::py_toint(ffi::py_list_getitem(list_ref, j)) as u32)
        .collect()
}

// Return value helpers
pub unsafe fn ret_none() {
    ffi::py_newnone(ffi::py_retval());
}

pub unsafe fn ret_int(v: i64) {
    ffi::py_newint(ffi::py_retval(), v);
}

pub unsafe fn ret_float(v: f64) {
    ffi::py_newfloat(ffi::py_retval(), v);
}

pub unsafe fn ret_bool(v: bool) {
    ffi::py_newbool(ffi::py_retval(), v);
}

pub unsafe fn ret_str(s: &str) {
    let cs = CString::new(s).unwrap();
    ffi::py_newstr(ffi::py_retval(), cs.as_ptr());
}

// Exception helpers
pub unsafe fn raise_exc(msg: &str) -> bool {
    let cs = CString::new(msg).unwrap();
    ffi::py_exception(
        ffi::py_PredefinedType_tp_Exception as ffi::py_Type,
        cs.as_ptr(),
    )
}

pub unsafe fn raise_index() -> bool {
    let cs = CString::new("list index out of range").unwrap();
    ffi::py_exception(
        ffi::py_PredefinedType_tp_IndexError as ffi::py_Type,
        cs.as_ptr(),
    )
}

// Binding helpers
pub unsafe fn bind(module: ffi::py_GlobalRef, sig: &CStr, f: ffi::py_CFunction) {
    ffi::py_bind(module, sig.as_ptr(), f);
}

pub unsafe fn bindfunc(module: ffi::py_GlobalRef, name: &CStr, f: ffi::py_CFunction) {
    ffi::py_bindfunc(module, name.as_ptr(), f);
}

// Module attribute helpers
pub unsafe fn set_module_int(module: ffi::py_GlobalRef, name: &CStr, value: i64) {
    let tmp = ffi::py_pushtmp();
    ffi::py_newint(tmp, value);
    ffi::py_setattr(module, ffi::py_name(name.as_ptr()), tmp);
    ffi::py_pop();
}

pub unsafe fn set_const_int(module: ffi::py_GlobalRef, name: &str, value: i64) {
    let n = ffi::py_name(CString::new(name).unwrap().as_ptr());
    ffi::py_newint(ffi::py_emplacedict(module, n), value);
}

pub unsafe fn set_const_str(module: ffi::py_GlobalRef, name: &str, value: &str) {
    let n = ffi::py_name(CString::new(name).unwrap().as_ptr());
    let cs = CString::new(value).unwrap();
    ffi::py_newstr(ffi::py_emplacedict(module, n), cs.as_ptr());
}

// Type registration helpers
pub unsafe fn new_type(name: &CStr, module: ffi::py_GlobalRef) -> ffi::py_Type {
    ffi::py_newtype(
        name.as_ptr(),
        ffi::py_PredefinedType_tp_object as ffi::py_Type,
        module,
        None,
    )
}

// Call a PocketPy function stored in a module dict entry
pub unsafe fn call_py_func(name: ffi::py_Name) {
    let module = ffi::py_getmodule(c"pyxel".as_ptr());
    let func = ffi::py_getdict(module, name);
    if func.is_null() {
        return;
    }
    if !ffi::py_call(func, 0, std::ptr::null_mut()) {
        ffi::py_printexc();
        std::process::exit(1);
    }
}

// Slice index computation (Python's slice.indices(length) algorithm)
pub unsafe fn is_slice(r: ffi::py_Ref) -> bool {
    ffi::py_istype(r, ffi::py_PredefinedType_tp_slice as ffi::py_Type)
}

pub unsafe fn slice_indices(key: ffi::py_Ref, length: usize) -> (isize, isize, isize) {
    let start_ref = ffi::py_getslot(key, 0);
    let stop_ref = ffi::py_getslot(key, 1);
    let step_ref = ffi::py_getslot(key, 2);

    let step = if is_none(step_ref) { 1 } else { ffi::py_toint(step_ref) as isize };
    let len = length as isize;

    let start = if is_none(start_ref) {
        if step > 0 { 0 } else { len - 1 }
    } else {
        let mut s = ffi::py_toint(start_ref) as isize;
        if s < 0 { s += len; }
        if step > 0 { s.clamp(0, len) } else { s.clamp(-1, len - 1) }
    };

    let stop = if is_none(stop_ref) {
        if step > 0 { len } else { -1 }
    } else {
        let mut s = ffi::py_toint(stop_ref) as isize;
        if s < 0 { s += len; }
        if step > 0 { s.clamp(0, len) } else { s.clamp(-1, len - 1) }
    };

    (start, stop, step)
}

pub fn collect_indices(start: isize, stop: isize, step: isize) -> Vec<usize> {
    let mut indices = Vec::new();
    let mut i = start;
    if step > 0 {
        while i < stop {
            indices.push(i as usize);
            i += step;
        }
    } else {
        while i > stop {
            indices.push(i as usize);
            i += step;
        }
    }
    indices
}

pub unsafe fn resolve_index(index: i64, length: usize) -> Option<usize> {
    let i = if index < 0 { index + length as i64 } else { index };
    if i < 0 || i as usize >= length { None } else { Some(i as usize) }
}

// Return a Python list from iterator items using a closure to create each element
pub unsafe fn ret_int_list(items: &[i64]) {
    ffi::py_newlist(ffi::py_retval());
    let list = ffi::py_retval();
    for &v in items {
        ffi::py_newint(ffi::py_list_emplace(list), v);
    }
}

// Macro: generate getitem/setitem/len/iter for an object collection (Images, Sounds, etc.)
macro_rules! impl_object_collection {
    ($global_fn:path, $new_obj:path, $getitem:ident, $setitem:ident, $len:ident, $iter:ident) => {
        unsafe extern "C" fn $getitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
            let key = arg(argv, 1);
            let items = $global_fn();
            if is_slice(key) {
                let (start, stop, step) = slice_indices(key, items.len());
                let indices = collect_indices(start, stop, step);
                ffi::py_newlist(ffi::py_retval());
                let list = ffi::py_retval();
                for &i in &indices {
                    $new_obj(ffi::py_list_emplace(list), items[i]);
                }
            } else {
                let idx = ffi::py_toint(key);
                match resolve_index(idx, items.len()) {
                    Some(i) => $new_obj(ffi::py_retval(), items[i]),
                    None => return raise_index(),
                }
            }
            true
        }

        unsafe extern "C" fn $setitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
            let key = arg(argv, 1);
            let items = $global_fn();
            if is_slice(key) {
                let (start, stop, step) = slice_indices(key, items.len());
                let val_list = arg(argv, 2);
                let new_len = ffi::py_list_len(val_list);
                if step == 1 {
                    for (pos, idx) in (start as usize..stop as usize).enumerate() {
                        if pos < new_len as usize {
                            let obj = ffi::py_list_getitem(val_list, pos as i32);
                            items[idx] = *(ffi::py_touserdata(obj) as *mut _);
                        }
                    }
                } else {
                    let indices = collect_indices(start, stop, step);
                    for (pos, &idx) in indices.iter().enumerate() {
                        let obj = ffi::py_list_getitem(val_list, pos as i32);
                        items[idx] = *(ffi::py_touserdata(obj) as *mut _);
                    }
                }
            } else {
                let idx = ffi::py_toint(key);
                match resolve_index(idx, items.len()) {
                    Some(i) => {
                        let obj = arg(argv, 2);
                        items[i] = *(ffi::py_touserdata(obj) as *mut _);
                    }
                    None => return raise_index(),
                }
            }
            ret_none();
            true
        }

        unsafe extern "C" fn $len(_argc: i32, _argv: ffi::py_StackRef) -> bool {
            ret_int($global_fn().len() as i64);
            true
        }

        unsafe extern "C" fn $iter(_argc: i32, _argv: ffi::py_StackRef) -> bool {
            let items = $global_fn();
            let tmp = ffi::py_pushtmp();
            ffi::py_newlist(tmp);
            for &item in items.iter() {
                $new_obj(ffi::py_list_emplace(tmp), item);
            }
            ffi::py_iter(tmp);
            ffi::py_pop();
            true
        }

        paste::paste! {
        unsafe extern "C" fn [<$getitem _reversed>](_argc: i32, _argv: ffi::py_StackRef) -> bool {
            let items = $global_fn();
            let tmp = ffi::py_pushtmp();
            ffi::py_newlist(tmp);
            for &item in items.iter().rev() {
                $new_obj(ffi::py_list_emplace(tmp), item);
            }
            ffi::py_iter(tmp);
            ffi::py_pop();
            true
        }

        unsafe extern "C" fn [<$getitem _bool>](_argc: i32, _argv: ffi::py_StackRef) -> bool {
            ret_bool(!$global_fn().is_empty());
            true
        }
        }
    };
}

// Macro: register a collection type with full magic methods
macro_rules! register_collection {
    ($tp:expr, $name:literal, $m:expr, $getitem:ident, $setitem:ident, $len:ident, $iter:ident) => {
        $tp = new_type($name, $m);
        ffi::py_bindmagic($tp, ffi::py_name(c"__getitem__".as_ptr()), Some($getitem));
        ffi::py_bindmagic($tp, ffi::py_name(c"__setitem__".as_ptr()), Some($setitem));
        ffi::py_bindmagic($tp, ffi::py_name(c"__len__".as_ptr()), Some($len));
        ffi::py_bindmagic($tp, ffi::py_name(c"__iter__".as_ptr()), Some($iter));
        paste::paste! {
        ffi::py_bindmagic($tp, ffi::py_name(c"__reversed__".as_ptr()), Some([<$getitem _reversed>]));
        ffi::py_bindmagic($tp, ffi::py_name(c"__bool__".as_ptr()), Some([<$getitem _bool>]));
        }
    };
}

// Update dynamic module attributes
pub unsafe fn sync_module_vars() {
    let m = ffi::py_getmodule(c"pyxel".as_ptr());
    set_module_int(m, c"frame_count", *pyxel::frame_count() as i64);
    set_module_int(m, c"mouse_x", *pyxel::mouse_x() as i64);
    set_module_int(m, c"mouse_y", *pyxel::mouse_y() as i64);
    set_module_int(m, c"mouse_wheel", *pyxel::mouse_wheel() as i64);

    // input_keys
    let n = ffi::py_name(c"input_keys".as_ptr());
    let tmp = ffi::py_pushtmp();
    ffi::py_newlist(tmp);
    for key in pyxel::input_keys().iter() {
        ffi::py_newint(ffi::py_list_emplace(tmp), *key as i64);
    }
    ffi::py_setattr(m, n, tmp);
    ffi::py_pop();

    // input_text
    let n = ffi::py_name(c"input_text".as_ptr());
    let tmp = ffi::py_pushtmp();
    let cs = CString::new(pyxel::input_text().as_str()).unwrap();
    ffi::py_newstr(tmp, cs.as_ptr());
    ffi::py_setattr(m, n, tmp);
    ffi::py_pop();

    // dropped_files
    let n = ffi::py_name(c"dropped_files".as_ptr());
    let tmp = ffi::py_pushtmp();
    ffi::py_newlist(tmp);
    for f in pyxel::dropped_files() {
        let cs = CString::new(f.as_str()).unwrap();
        ffi::py_newstr(ffi::py_list_emplace(tmp), cs.as_ptr());
    }
    ffi::py_setattr(m, n, tmp);
    ffi::py_pop();
}
