use crate::ffi;
use crate::helpers::*;
use crate::image_wrapper::new_image_obj;

pub static mut TP_COLORS: ffi::py_Type = 0;

// Colors collection — full sequence operations

unsafe extern "C" fn colors_getitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let key = arg(argv, 1);
    let colors = pyxel::colors();
    if is_slice(key) {
        let (start, stop, step) = slice_indices(key, colors.len());
        let indices = collect_indices(start, stop, step);
        let items: Vec<i64> = indices.iter().map(|&i| colors[i] as i64).collect();
        ret_int_list(&items);
    } else {
        let idx = ffi::py_toint(key);
        match resolve_index(idx, colors.len()) {
            Some(i) => ret_int(colors[i] as i64),
            None => return raise_index(),
        }
    }
    true
}

unsafe extern "C" fn colors_setitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let key = arg(argv, 1);
    let colors = pyxel::colors();
    if is_slice(key) {
        let (start, stop, step) = slice_indices(key, colors.len());
        let val_list = arg(argv, 2);
        if step == 1 {
            let new_len = ffi::py_list_len(val_list);
            let start = start as usize;
            let end = stop as usize;
            let new_vals: Vec<u32> = (0..new_len)
                .map(|i| ffi::py_toint(ffi::py_list_getitem(val_list, i)) as u32)
                .collect();
            colors.splice(start..end, new_vals);
        } else {
            let indices = collect_indices(start, stop, step);
            for (pos, idx) in indices.iter().enumerate() {
                colors[*idx] = ffi::py_toint(ffi::py_list_getitem(val_list, pos as i32)) as u32;
            }
        }
    } else {
        let idx = ffi::py_toint(key);
        match resolve_index(idx, colors.len()) {
            Some(i) => colors[i] = arg_int(argv, 2) as u32,
            None => return raise_index(),
        }
    }
    ret_none();
    true
}

unsafe extern "C" fn colors_delitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let key = arg(argv, 1);
    let colors = pyxel::colors();
    if is_slice(key) {
        let (start, stop, step) = slice_indices(key, colors.len());
        let mut indices = collect_indices(start, stop, step);
        indices.sort_unstable_by(|a, b| b.cmp(a));
        for i in indices {
            colors.remove(i);
        }
    } else {
        let idx = ffi::py_toint(key);
        match resolve_index(idx, colors.len()) {
            Some(i) => { colors.remove(i); }
            None => return raise_index(),
        }
    }
    ret_none();
    true
}

unsafe extern "C" fn colors_len(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    ret_int(pyxel::colors().len() as i64);
    true
}

unsafe extern "C" fn colors_iter(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    let colors = pyxel::colors();
    let tmp = ffi::py_pushtmp();
    ffi::py_newlist(tmp);
    for &c in colors.iter() {
        ffi::py_newint(ffi::py_list_emplace(tmp), c as i64);
    }
    ffi::py_iter(tmp);
    ffi::py_pop();
    true
}

unsafe extern "C" fn colors_contains(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let value = arg_int(argv, 1) as u32;
    ret_bool(pyxel::colors().contains(&value));
    true
}

unsafe extern "C" fn colors_bool(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    ret_bool(!pyxel::colors().is_empty());
    true
}

unsafe extern "C" fn colors_repr(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    let colors = pyxel::colors();
    let items: Vec<String> = colors.iter().map(|c| c.to_string()).collect();
    let s = format!("Colors[{}]", items.join(", "));
    ret_str(&s);
    true
}

unsafe extern "C" fn colors_reversed(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    let tmp = ffi::py_pushtmp();
    ffi::py_newlist(tmp);
    for &c in pyxel::colors().iter().rev() {
        ffi::py_newint(ffi::py_list_emplace(tmp), c as i64);
    }
    ffi::py_iter(tmp);
    ffi::py_pop();
    true
}

unsafe extern "C" fn colors_eq(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let other = arg(argv, 1);
    let colors = pyxel::colors();
    if is_list(other) {
        let len = ffi::py_list_len(other) as usize;
        if colors.len() != len {
            ret_bool(false);
        } else {
            let eq = (0..len).all(|i| colors[i] as i64 == ffi::py_toint(ffi::py_list_getitem(other, i as i32)));
            ret_bool(eq);
        }
    } else {
        ret_bool(false);
    }
    true
}

unsafe extern "C" fn colors_append(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::colors().push(arg_int(argv, 1) as u32);
    ret_none();
    true
}

unsafe extern "C" fn colors_extend(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let val_list = arg(argv, 1);
    for i in 0..ffi::py_list_len(val_list) {
        pyxel::colors().push(ffi::py_toint(ffi::py_list_getitem(val_list, i)) as u32);
    }
    ret_none();
    true
}

unsafe extern "C" fn colors_insert(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let index = arg_int(argv, 1) as isize;
    let value = arg_int(argv, 2) as u32;
    let colors = pyxel::colors();
    let len = colors.len();
    let i = if index < 0 {
        let r = index + len as isize;
        if r < 0 { 0 } else { r as usize }
    } else if index as usize > len { len } else { index as usize };
    colors.insert(i, value);
    ret_none();
    true
}

unsafe extern "C" fn colors_pop(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let colors = pyxel::colors();
    if colors.is_empty() {
        return raise_index();
    }
    let idx = if arg_is_none(argv, 1) { -1i64 } else { arg_int(argv, 1) };
    match resolve_index(idx, colors.len()) {
        Some(i) => {
            let val = colors.remove(i);
            ret_int(val as i64);
        }
        None => return raise_index(),
    }
    true
}

unsafe extern "C" fn colors_clear(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    pyxel::colors().clear();
    ret_none();
    true
}

pub unsafe fn add_module_variables(m: ffi::py_GlobalRef) {
    TP_COLORS = new_type(c"Colors", m);
    ffi::py_bindmagic(TP_COLORS, ffi::py_name(c"__getitem__".as_ptr()), Some(colors_getitem));
    ffi::py_bindmagic(TP_COLORS, ffi::py_name(c"__setitem__".as_ptr()), Some(colors_setitem));
    ffi::py_bindmagic(TP_COLORS, ffi::py_name(c"__delitem__".as_ptr()), Some(colors_delitem));
    ffi::py_bindmagic(TP_COLORS, ffi::py_name(c"__len__".as_ptr()), Some(colors_len));
    ffi::py_bindmagic(TP_COLORS, ffi::py_name(c"__contains__".as_ptr()), Some(colors_contains));
    ffi::py_bindmagic(TP_COLORS, ffi::py_name(c"__bool__".as_ptr()), Some(colors_bool));
    ffi::py_bindmagic(TP_COLORS, ffi::py_name(c"__iter__".as_ptr()), Some(colors_iter));
    ffi::py_bindmagic(TP_COLORS, ffi::py_name(c"__reversed__".as_ptr()), Some(colors_reversed));
    ffi::py_bindmagic(TP_COLORS, ffi::py_name(c"__eq__".as_ptr()), Some(colors_eq));
    ffi::py_bindmagic(TP_COLORS, ffi::py_name(c"__repr__".as_ptr()), Some(colors_repr));
    ffi::py_bindmethod(TP_COLORS, c"append".as_ptr(), Some(colors_append));
    ffi::py_bindmethod(TP_COLORS, c"extend".as_ptr(), Some(colors_extend));
    ffi::py_bindmethod(TP_COLORS, c"insert".as_ptr(), Some(colors_insert));
    ffi::py_bindmethod(TP_COLORS, c"clear".as_ptr(), Some(colors_clear));
    let tp_obj = ffi::py_tpobject(TP_COLORS);
    bind(tp_obj, c"pop(self, index=None)", Some(colors_pop));
}

/// Set screen/cursor/font Image objects on the module after init
pub unsafe fn set_screen_objects(m: ffi::py_GlobalRef) {
    new_image_obj(
        ffi::py_emplacedict(m, ffi::py_name(c"screen".as_ptr())),
        std::ptr::from_mut(pyxel::screen()),
    );
    new_image_obj(
        ffi::py_emplacedict(m, ffi::py_name(c"cursor".as_ptr())),
        std::ptr::from_mut(pyxel::cursor_image()),
    );
    new_image_obj(
        ffi::py_emplacedict(m, ffi::py_name(c"font".as_ptr())),
        std::ptr::from_mut(pyxel::font_image()),
    );
}
