use crate::ffi;
use crate::helpers::*;

pub static mut TP_SOUND: ffi::py_Type = 0;
pub static mut TP_SOUNDS: ffi::py_Type = 0;
static mut TP_NOTES: ffi::py_Type = 0;
static mut TP_SOUND_TONES: ffi::py_Type = 0;
static mut TP_VOLUMES: ffi::py_Type = 0;
static mut TP_EFFECTS: ffi::py_Type = 0;

unsafe fn snd(argv: ffi::py_StackRef) -> &'static mut pyxel::Sound {
    &mut *(*(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound))
}

pub unsafe fn new_sound_obj(out: ffi::py_OutRef, ptr: *mut pyxel::Sound) {
    let ud = ffi::py_newobject(out, TP_SOUND, 0, size_of::<*mut pyxel::Sound>() as i32);
    *(ud as *mut *mut pyxel::Sound) = ptr;
}

unsafe extern "C" fn sound_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if let Err(e) = snd(argv).set(
        arg_str(argv, 1),
        arg_str(argv, 2),
        arg_str(argv, 3),
        arg_str(argv, 4),
        arg_float(argv, 5) as u16,
    ) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

unsafe extern "C" fn sound_set_notes(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if let Err(e) = snd(argv).set_notes(arg_str(argv, 1)) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

unsafe extern "C" fn sound_set_tones(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if let Err(e) = snd(argv).set_tones(arg_str(argv, 1)) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

unsafe extern "C" fn sound_set_volumes(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if let Err(e) = snd(argv).set_volumes(arg_str(argv, 1)) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

unsafe extern "C" fn sound_set_effects(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if let Err(e) = snd(argv).set_effects(arg_str(argv, 1)) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

unsafe extern "C" fn sound_mml(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if arg_is_none(argv, 1) {
        snd(argv).clear_mml();
    } else {
        let code = arg_str(argv, 1);
        if let Err(e) = snd(argv).set_mml(code) {
            return raise_exc(&e);
        }
    }
    ret_none();
    true
}

unsafe extern "C" fn sound_speed_getter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_int(snd(argv).speed as i64);
    true
}

unsafe extern "C" fn sound_speed_setter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    snd(argv).speed = arg_int(argv, 1) as u16;
    ret_none();
    true
}

// Sound sub-collection with full sequence operations matching pyxel-binding
macro_rules! sound_seq_fns {
    ($field:ident, $prefix:ident, $tp:ident) => { paste::paste! {
        // Read operations
        unsafe extern "C" fn [<$prefix _getitem>](_argc: i32, argv: ffi::py_StackRef) -> bool {
            let snd_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound);
            let key = arg(argv, 1);
            let seq = &(*snd_ptr).$field;
            if is_slice(key) {
                let (start, stop, step) = slice_indices(key, seq.len());
                let items: Vec<i64> = collect_indices(start, stop, step).iter().map(|&i| seq[i] as i64).collect();
                ret_int_list(&items);
            } else {
                match resolve_index(ffi::py_toint(key), seq.len()) {
                    Some(i) => ret_int(seq[i] as i64),
                    None => return raise_index(),
                }
            }
            true
        }
        unsafe extern "C" fn [<$prefix _len>](_argc: i32, argv: ffi::py_StackRef) -> bool {
            let snd_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound);
            ret_int((*snd_ptr).$field.len() as i64);
            true
        }
        unsafe extern "C" fn [<$prefix _iter>](_argc: i32, argv: ffi::py_StackRef) -> bool {
            let snd_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound);
            let tmp = ffi::py_pushtmp();
            ffi::py_newlist(tmp);
            for &v in (*snd_ptr).$field.iter() {
                ffi::py_newint(ffi::py_list_emplace(tmp), v as i64);
            }
            ffi::py_iter(tmp);
            ffi::py_pop();
            true
        }
        unsafe extern "C" fn [<$prefix _reversed>](_argc: i32, argv: ffi::py_StackRef) -> bool {
            let snd_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound);
            let tmp = ffi::py_pushtmp();
            ffi::py_newlist(tmp);
            for &v in (*snd_ptr).$field.iter().rev() {
                ffi::py_newint(ffi::py_list_emplace(tmp), v as i64);
            }
            ffi::py_iter(tmp);
            ffi::py_pop();
            true
        }
        unsafe extern "C" fn [<$prefix _bool>](_argc: i32, argv: ffi::py_StackRef) -> bool {
            let snd_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound);
            ret_bool(!(*snd_ptr).$field.is_empty());
            true
        }
        unsafe extern "C" fn [<$prefix _repr>](_argc: i32, argv: ffi::py_StackRef) -> bool {
            let snd_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound);
            let items: Vec<String> = (*snd_ptr).$field.iter().map(|v| format!("{v}")).collect();
            ret_str(&format!("{}[{}]", stringify!($prefix), items.join(", ")));
            true
        }
        // Comparison operations
        unsafe extern "C" fn [<$prefix _contains>](_argc: i32, argv: ffi::py_StackRef) -> bool {
            let snd_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound);
            let val = arg_int(argv, 1);
            ret_bool((*snd_ptr).$field.iter().any(|&v| v as i64 == val));
            true
        }
        unsafe extern "C" fn [<$prefix _eq>](_argc: i32, argv: ffi::py_StackRef) -> bool {
            let snd_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound);
            let other = arg(argv, 1);
            if is_list(other) {
                let len = ffi::py_list_len(other) as usize;
                let seq = &(*snd_ptr).$field;
                if seq.len() != len {
                    ret_bool(false);
                } else {
                    let eq = (0..len).all(|i| seq[i] as i64 == ffi::py_toint(ffi::py_list_getitem(other, i as i32)));
                    ret_bool(eq);
                }
            } else {
                ret_bool(false);
            }
            true
        }
        // Write operations
        unsafe extern "C" fn [<$prefix _setitem>](_argc: i32, argv: ffi::py_StackRef) -> bool {
            let snd_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound);
            let key = arg(argv, 1);
            let seq = &mut (*snd_ptr).$field;
            if is_slice(key) {
                let (start, stop, step) = slice_indices(key, seq.len());
                let val_list = arg(argv, 2);
                if step == 1 {
                    let new_vals: Vec<_> = (0..ffi::py_list_len(val_list))
                        .map(|i| ffi::py_toint(ffi::py_list_getitem(val_list, i)) as _)
                        .collect();
                    seq.splice(start as usize..stop as usize, new_vals);
                } else {
                    let indices = collect_indices(start, stop, step);
                    for (pos, &idx) in indices.iter().enumerate() {
                        seq[idx] = ffi::py_toint(ffi::py_list_getitem(val_list, pos as i32)) as _;
                    }
                }
            } else {
                match resolve_index(ffi::py_toint(key), seq.len()) {
                    Some(i) => seq[i] = arg_int(argv, 2) as _,
                    None => return raise_index(),
                }
            }
            ret_none();
            true
        }
        unsafe extern "C" fn [<$prefix _delitem>](_argc: i32, argv: ffi::py_StackRef) -> bool {
            let snd_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound);
            let key = arg(argv, 1);
            let seq = &mut (*snd_ptr).$field;
            if is_slice(key) {
                let (start, stop, step) = slice_indices(key, seq.len());
                let mut indices = collect_indices(start, stop, step);
                indices.sort_unstable_by(|a, b| b.cmp(a));
                for i in indices { seq.remove(i); }
            } else {
                match resolve_index(ffi::py_toint(key), seq.len()) {
                    Some(i) => { seq.remove(i); }
                    None => return raise_index(),
                }
            }
            ret_none();
            true
        }
        unsafe extern "C" fn [<$prefix _append>](_argc: i32, argv: ffi::py_StackRef) -> bool {
            let snd_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound);
            (*snd_ptr).$field.push(arg_int(argv, 1) as _);
            ret_none();
            true
        }
        unsafe extern "C" fn [<$prefix _extend>](_argc: i32, argv: ffi::py_StackRef) -> bool {
            let snd_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound);
            let val_list = arg(argv, 1);
            for i in 0..ffi::py_list_len(val_list) {
                (*snd_ptr).$field.push(ffi::py_toint(ffi::py_list_getitem(val_list, i)) as _);
            }
            ret_none();
            true
        }
        unsafe extern "C" fn [<$prefix _insert>](_argc: i32, argv: ffi::py_StackRef) -> bool {
            let snd_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound);
            let index = arg_int(argv, 1) as isize;
            let value = arg_int(argv, 2);
            let seq = &mut (*snd_ptr).$field;
            let len = seq.len();
            let i = if index < 0 {
                let r = index + len as isize;
                if r < 0 { 0 } else { r as usize }
            } else if index as usize > len { len } else { index as usize };
            seq.insert(i, value as _);
            ret_none();
            true
        }
        unsafe extern "C" fn [<$prefix _pop>](_argc: i32, argv: ffi::py_StackRef) -> bool {
            let snd_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound);
            let seq = &mut (*snd_ptr).$field;
            if seq.is_empty() {
                return raise_index();
            }
            let idx = if arg_is_none(argv, 1) { -1i64 } else { arg_int(argv, 1) };
            match resolve_index(idx, seq.len()) {
                Some(i) => {
                    let val = seq.remove(i);
                    ret_int(val as i64);
                }
                None => return raise_index(),
            }
            true
        }
        unsafe extern "C" fn [<$prefix _clear>](_argc: i32, argv: ffi::py_StackRef) -> bool {
            let snd_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound);
            (*snd_ptr).$field.clear();
            ret_none();
            true
        }
        // Property getter for Sound.notes etc.
        unsafe extern "C" fn [<$prefix _getter>](_argc: i32, argv: ffi::py_StackRef) -> bool {
            let snd_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Sound);
            let ud = ffi::py_newobject(ffi::py_retval(), $tp, 0, size_of::<*mut pyxel::Sound>() as i32);
            *(ud as *mut *mut pyxel::Sound) = snd_ptr;
            true
        }
    } };
}

sound_seq_fns!(notes, notes, TP_NOTES);
sound_seq_fns!(tones, stones, TP_SOUND_TONES);
sound_seq_fns!(volumes, volumes, TP_VOLUMES);
sound_seq_fns!(effects, effects, TP_EFFECTS);

unsafe extern "C" fn sound_save_file(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let filename = arg_str(argv, 1);
    let sec = arg_float(argv, 2) as f32;
    let ffmpeg = arg_opt_bool(argv, 3);
    if let Err(e) = snd(argv).save(filename, sec, ffmpeg) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

unsafe extern "C" fn sound_pcm(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if arg_is_none(argv, 1) {
        snd(argv).clear_pcm();
    } else if let Err(e) = snd(argv).load_pcm(arg_str(argv, 1)) {
        return raise_exc(&e);
    }
    ret_none();
    true
}

unsafe extern "C" fn sound_total_sec(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match snd(argv).total_seconds() {
        Some(sec) => ret_float(sec as f64),
        None => ret_none(),
    }
    true
}

unsafe extern "C" fn sound_new(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let cls = ffi::py_totype(arg(argv, 0));
    let ptr = pyxel::Sound::new();
    let ud = ffi::py_newobject(
        ffi::py_retval(),
        cls,
        0,
        size_of::<*mut pyxel::Sound>() as i32,
    );
    *(ud as *mut *mut pyxel::Sound) = ptr;
    true
}

pub unsafe fn add_sound_class(m: ffi::py_GlobalRef) {
    TP_SOUND = new_type(c"Sound", m);

    // Sub-collection types
    macro_rules! register_prim_seq {
        ($tp:expr, $name:literal, $prefix:ident) => { paste::paste! {
            $tp = new_type($name, m);
            ffi::py_bindmagic($tp, ffi::py_name(c"__getitem__".as_ptr()), Some([<$prefix _getitem>]));
            ffi::py_bindmagic($tp, ffi::py_name(c"__setitem__".as_ptr()), Some([<$prefix _setitem>]));
            ffi::py_bindmagic($tp, ffi::py_name(c"__delitem__".as_ptr()), Some([<$prefix _delitem>]));
            ffi::py_bindmagic($tp, ffi::py_name(c"__len__".as_ptr()), Some([<$prefix _len>]));
            ffi::py_bindmagic($tp, ffi::py_name(c"__iter__".as_ptr()), Some([<$prefix _iter>]));
            ffi::py_bindmagic($tp, ffi::py_name(c"__reversed__".as_ptr()), Some([<$prefix _reversed>]));
            ffi::py_bindmagic($tp, ffi::py_name(c"__contains__".as_ptr()), Some([<$prefix _contains>]));
            ffi::py_bindmagic($tp, ffi::py_name(c"__eq__".as_ptr()), Some([<$prefix _eq>]));
            ffi::py_bindmagic($tp, ffi::py_name(c"__bool__".as_ptr()), Some([<$prefix _bool>]));
            ffi::py_bindmagic($tp, ffi::py_name(c"__repr__".as_ptr()), Some([<$prefix _repr>]));
            ffi::py_bindmethod($tp, c"append".as_ptr(), Some([<$prefix _append>]));
            ffi::py_bindmethod($tp, c"extend".as_ptr(), Some([<$prefix _extend>]));
            ffi::py_bindmethod($tp, c"insert".as_ptr(), Some([<$prefix _insert>]));
            ffi::py_bindmethod($tp, c"clear".as_ptr(), Some([<$prefix _clear>]));
            let tp_obj = ffi::py_tpobject($tp);
            bind(tp_obj, c"pop(self, index=None)", Some([<$prefix _pop>]));
        } };
    }
    register_prim_seq!(TP_NOTES, c"Notes", notes);
    register_prim_seq!(TP_SOUND_TONES, c"SoundTones", stones);
    register_prim_seq!(TP_VOLUMES, c"Volumes", volumes);
    register_prim_seq!(TP_EFFECTS, c"Effects", effects);

    ffi::py_bindproperty(TP_SOUND, c"notes".as_ptr(), Some(notes_getter), None);
    ffi::py_bindproperty(TP_SOUND, c"tones".as_ptr(), Some(stones_getter), None);
    ffi::py_bindproperty(TP_SOUND, c"volumes".as_ptr(), Some(volumes_getter), None);
    ffi::py_bindproperty(TP_SOUND, c"effects".as_ptr(), Some(effects_getter), None);
    ffi::py_bindproperty(
        TP_SOUND,
        c"speed".as_ptr(),
        Some(sound_speed_getter),
        Some(sound_speed_setter),
    );
    // Use default=None to force FuncType_NORMAL so kwargs work
    bind(
        ffi::py_tpobject(TP_SOUND),
        c"set(self, notes=None, tones=None, volumes=None, effects=None, speed=None)",
        Some(sound_set),
    );
    ffi::py_bindmethod(TP_SOUND, c"set_notes".as_ptr(), Some(sound_set_notes));
    ffi::py_bindmethod(TP_SOUND, c"set_tones".as_ptr(), Some(sound_set_tones));
    ffi::py_bindmethod(TP_SOUND, c"set_volumes".as_ptr(), Some(sound_set_volumes));
    ffi::py_bindmethod(TP_SOUND, c"set_effects".as_ptr(), Some(sound_set_effects));

    let tp_obj = ffi::py_tpobject(TP_SOUND);
    bind(tp_obj, c"mml(self, code=None)", Some(sound_mml));
    bind(tp_obj, c"old_mml(self, code=None)", Some(sound_mml));
    bind(
        tp_obj,
        c"save(self, filename, sec, ffmpeg=None)",
        Some(sound_save_file),
    );
    bind(tp_obj, c"pcm(self, filename=None)", Some(sound_pcm));
    bind(tp_obj, c"total_sec(self)", Some(sound_total_sec));
    bind(tp_obj, c"__new__(cls)", Some(sound_new));

    // Sounds collection
    impl_object_collection!(
        pyxel::sounds,
        new_sound_obj,
        sounds_getitem,
        sounds_setitem,
        sounds_len,
        sounds_iter
    );
    register_collection!(
        TP_SOUNDS,
        c"Sounds",
        m,
        sounds_getitem,
        sounds_setitem,
        sounds_len,
        sounds_iter
    );
}
