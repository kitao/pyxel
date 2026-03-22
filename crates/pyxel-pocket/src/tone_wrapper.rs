use crate::ffi;
use crate::helpers::*;

pub static mut TP_TONE: ffi::py_Type = 0;
pub static mut TP_TONES: ffi::py_Type = 0;
static mut TP_WAVETABLE: ffi::py_Type = 0;

unsafe fn tn(argv: ffi::py_StackRef) -> &'static mut pyxel::Tone {
    &mut *(*(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Tone))
}

pub unsafe fn new_tone_obj(out: ffi::py_OutRef, ptr: *mut pyxel::Tone) {
    let ud = ffi::py_newobject(out, TP_TONE, 0, size_of::<*mut pyxel::Tone>() as i32);
    *(ud as *mut *mut pyxel::Tone) = ptr;
}

unsafe extern "C" fn tone_mode_getter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let mode: u32 = tn(argv).mode.into();
    ret_int(mode as i64);
    true
}

unsafe extern "C" fn tone_mode_setter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    tn(argv).mode = pyxel::ToneMode::from(arg_int(argv, 1) as u32);
    ret_none();
    true
}

unsafe extern "C" fn tone_sample_bits_getter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_int(tn(argv).sample_bits as i64);
    true
}

unsafe extern "C" fn tone_sample_bits_setter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    tn(argv).sample_bits = arg_int(argv, 1) as pyxel::ToneSample;
    ret_none();
    true
}

unsafe extern "C" fn tone_gain_getter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_float(tn(argv).gain as f64);
    true
}

unsafe extern "C" fn tone_gain_setter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    tn(argv).gain = arg_float(argv, 1) as pyxel::ToneGain;
    ret_none();
    true
}

// Wavetable sub-collection
unsafe extern "C" fn wavetable_getitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Tone);
    let key = arg(argv, 1);
    let wt = &(*tone_ptr).wavetable;
    if is_slice(key) {
        let (start, stop, step) = slice_indices(key, wt.len());
        let indices = collect_indices(start, stop, step);
        let items: Vec<i64> = indices.iter().map(|&i| wt[i] as i64).collect();
        ret_int_list(&items);
    } else {
        let idx = ffi::py_toint(key);
        match resolve_index(idx, wt.len()) {
            Some(i) => ret_int(wt[i] as i64),
            None => return raise_index(),
        }
    }
    true
}

unsafe extern "C" fn wavetable_setitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Tone);
    let key = arg(argv, 1);
    let wt = &mut (*tone_ptr).wavetable;
    if is_slice(key) {
        let (start, stop, step) = slice_indices(key, wt.len());
        let val_list = arg(argv, 2);
        if step == 1 {
            let new_vals: Vec<pyxel::ToneSample> = (0..ffi::py_list_len(val_list))
                .map(|i| ffi::py_toint(ffi::py_list_getitem(val_list, i)) as pyxel::ToneSample)
                .collect();
            wt.splice(start as usize..stop as usize, new_vals);
        } else {
            let indices = collect_indices(start, stop, step);
            for (pos, &idx) in indices.iter().enumerate() {
                wt[idx] = ffi::py_toint(ffi::py_list_getitem(val_list, pos as i32)) as pyxel::ToneSample;
            }
        }
    } else {
        let idx = ffi::py_toint(key);
        match resolve_index(idx, wt.len()) {
            Some(i) => wt[i] = arg_int(argv, 2) as pyxel::ToneSample,
            None => return raise_index(),
        }
    }
    ret_none();
    true
}

unsafe extern "C" fn wavetable_len(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Tone);
    ret_int((*tone_ptr).wavetable.len() as i64);
    true
}

unsafe extern "C" fn wavetable_iter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Tone);
    let tmp = ffi::py_pushtmp();
    ffi::py_newlist(tmp);
    for &v in (*tone_ptr).wavetable.iter() {
        ffi::py_newint(ffi::py_list_emplace(tmp), v as i64);
    }
    ffi::py_iter(tmp);
    ffi::py_pop();
    true
}

unsafe extern "C" fn wavetable_reversed(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Tone);
    let tmp = ffi::py_pushtmp();
    ffi::py_newlist(tmp);
    for &v in (*tone_ptr).wavetable.iter().rev() {
        ffi::py_newint(ffi::py_list_emplace(tmp), v as i64);
    }
    ffi::py_iter(tmp);
    ffi::py_pop();
    true
}

unsafe extern "C" fn wavetable_contains(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Tone);
    let val = arg_int(argv, 1) as pyxel::ToneSample;
    ret_bool((*tone_ptr).wavetable.contains(&val));
    true
}

unsafe extern "C" fn wavetable_bool(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Tone);
    ret_bool(!(*tone_ptr).wavetable.is_empty());
    true
}

unsafe extern "C" fn wavetable_delitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Tone);
    let key = arg(argv, 1);
    let wt = &mut (*tone_ptr).wavetable;
    if is_slice(key) {
        let (start, stop, step) = slice_indices(key, wt.len());
        let mut indices = collect_indices(start, stop, step);
        indices.sort_unstable_by(|a, b| b.cmp(a));
        for i in indices { wt.remove(i); }
    } else {
        match resolve_index(ffi::py_toint(key), wt.len()) {
            Some(i) => { wt.remove(i); }
            None => return raise_index(),
        }
    }
    ret_none();
    true
}

unsafe extern "C" fn wavetable_append(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Tone);
    (*tone_ptr).wavetable.push(arg_int(argv, 1) as pyxel::ToneSample);
    ret_none();
    true
}

unsafe extern "C" fn wavetable_clear(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Tone);
    (*tone_ptr).wavetable.clear();
    ret_none();
    true
}

unsafe extern "C" fn wavetable_getter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Tone);
    let ud = ffi::py_newobject(ffi::py_retval(), TP_WAVETABLE, 0, size_of::<*mut pyxel::Tone>() as i32);
    *(ud as *mut *mut pyxel::Tone) = tone_ptr;
    true
}

unsafe extern "C" fn tone_new(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let cls = ffi::py_totype(arg(argv, 0));
    let ptr = pyxel::Tone::new();
    let ud = ffi::py_newobject(ffi::py_retval(), cls, 0, size_of::<*mut pyxel::Tone>() as i32);
    *(ud as *mut *mut pyxel::Tone) = ptr;
    true
}

pub unsafe fn add_tone_class(m: ffi::py_GlobalRef) {
    // Wavetable type
    TP_WAVETABLE = new_type(c"Wavetable", m);
    ffi::py_bindmagic(TP_WAVETABLE, ffi::py_name(c"__getitem__".as_ptr()), Some(wavetable_getitem));
    ffi::py_bindmagic(TP_WAVETABLE, ffi::py_name(c"__setitem__".as_ptr()), Some(wavetable_setitem));
    ffi::py_bindmagic(TP_WAVETABLE, ffi::py_name(c"__delitem__".as_ptr()), Some(wavetable_delitem));
    ffi::py_bindmagic(TP_WAVETABLE, ffi::py_name(c"__len__".as_ptr()), Some(wavetable_len));
    ffi::py_bindmagic(TP_WAVETABLE, ffi::py_name(c"__iter__".as_ptr()), Some(wavetable_iter));
    ffi::py_bindmagic(TP_WAVETABLE, ffi::py_name(c"__reversed__".as_ptr()), Some(wavetable_reversed));
    ffi::py_bindmagic(TP_WAVETABLE, ffi::py_name(c"__contains__".as_ptr()), Some(wavetable_contains));
    ffi::py_bindmagic(TP_WAVETABLE, ffi::py_name(c"__bool__".as_ptr()), Some(wavetable_bool));
    ffi::py_bindmethod(TP_WAVETABLE, c"append".as_ptr(), Some(wavetable_append));
    ffi::py_bindmethod(TP_WAVETABLE, c"clear".as_ptr(), Some(wavetable_clear));

    TP_TONE = new_type(c"Tone", m);
    ffi::py_bindproperty(TP_TONE, c"mode".as_ptr(), Some(tone_mode_getter), Some(tone_mode_setter));
    ffi::py_bindproperty(TP_TONE, c"sample_bits".as_ptr(), Some(tone_sample_bits_getter), Some(tone_sample_bits_setter));
    ffi::py_bindproperty(TP_TONE, c"wavetable".as_ptr(), Some(wavetable_getter), None);
    ffi::py_bindproperty(TP_TONE, c"gain".as_ptr(), Some(tone_gain_getter), Some(tone_gain_setter));

    // Deprecated aliases
    ffi::py_bindproperty(TP_TONE, c"noise".as_ptr(), Some(tone_mode_getter), Some(tone_mode_setter));
    ffi::py_bindproperty(TP_TONE, c"waveform".as_ptr(), Some(wavetable_getter), None);

    let tp_obj = ffi::py_tpobject(TP_TONE);
    bind(tp_obj, c"__new__(cls)", Some(tone_new));

    // Tones collection
    impl_object_collection!(pyxel::tones, new_tone_obj, tones_getitem, tones_setitem, tones_len, tones_iter);
    register_collection!(TP_TONES, c"Tones", m, tones_getitem, tones_setitem, tones_len, tones_iter);
}
