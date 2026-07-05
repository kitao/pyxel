use super::{
    drop_int_seq, drop_music_seq, drop_music_seqs, ffi, int_list_from_ref,
    int_nested_list_from_ref, make_int_list, new_userdata, normalize_index, normalize_insert_index,
    rc_mut, rc_ref, return_int_list, userdata, value, IntSeq, MusicSeq, MusicSeqs, TP_INT_SEQ,
    TP_MUSIC_SEQ, TP_MUSIC_SEQS,
};

// Integer sequence wrappers

unsafe fn int_seq_len(seq: &IntSeq) -> usize {
    match seq {
        IntSeq::ToneWavetable(tone) => rc_ref(tone).wavetable.len(),
        IntSeq::SoundNotes(sound) => rc_ref(sound).notes.len(),
        IntSeq::SoundTones(sound) => rc_ref(sound).tones.len(),
        IntSeq::SoundVolumes(sound) => rc_ref(sound).volumes.len(),
        IntSeq::SoundEffects(sound) => rc_ref(sound).effects.len(),
    }
}

unsafe fn int_seq_get(seq: &IntSeq, index: usize) -> i64 {
    match seq {
        IntSeq::ToneWavetable(tone) => rc_ref(tone).wavetable[index] as i64,
        IntSeq::SoundNotes(sound) => rc_ref(sound).notes[index] as i64,
        IntSeq::SoundTones(sound) => rc_ref(sound).tones[index] as i64,
        IntSeq::SoundVolumes(sound) => rc_ref(sound).volumes[index] as i64,
        IntSeq::SoundEffects(sound) => rc_ref(sound).effects[index] as i64,
    }
}

unsafe fn int_seq_push(seq: &IntSeq, value: i64) {
    match seq {
        IntSeq::ToneWavetable(tone) => rc_mut(tone).wavetable.push(value as u32),
        IntSeq::SoundNotes(sound) => rc_mut(sound).notes.push(value as i8),
        IntSeq::SoundTones(sound) => rc_mut(sound).tones.push(value as u8),
        IntSeq::SoundVolumes(sound) => rc_mut(sound).volumes.push(value as u8),
        IntSeq::SoundEffects(sound) => rc_mut(sound).effects.push(value as u8),
    }
}

unsafe fn int_seq_set(seq: &IntSeq, index: usize, value: i64) {
    match seq {
        IntSeq::ToneWavetable(tone) => rc_mut(tone).wavetable[index] = value as u32,
        IntSeq::SoundNotes(sound) => rc_mut(sound).notes[index] = value as i8,
        IntSeq::SoundTones(sound) => rc_mut(sound).tones[index] = value as u8,
        IntSeq::SoundVolumes(sound) => rc_mut(sound).volumes[index] = value as u8,
        IntSeq::SoundEffects(sound) => rc_mut(sound).effects[index] = value as u8,
    }
}

unsafe fn int_seq_remove(seq: &IntSeq, index: usize) -> i64 {
    match seq {
        IntSeq::ToneWavetable(tone) => rc_mut(tone).wavetable.remove(index) as i64,
        IntSeq::SoundNotes(sound) => rc_mut(sound).notes.remove(index) as i64,
        IntSeq::SoundTones(sound) => rc_mut(sound).tones.remove(index) as i64,
        IntSeq::SoundVolumes(sound) => rc_mut(sound).volumes.remove(index) as i64,
        IntSeq::SoundEffects(sound) => rc_mut(sound).effects.remove(index) as i64,
    }
}

unsafe fn int_seq_insert_at(seq: &IntSeq, index: usize, value: i64) {
    match seq {
        IntSeq::ToneWavetable(tone) => rc_mut(tone).wavetable.insert(index, value as u32),
        IntSeq::SoundNotes(sound) => rc_mut(sound).notes.insert(index, value as i8),
        IntSeq::SoundTones(sound) => rc_mut(sound).tones.insert(index, value as u8),
        IntSeq::SoundVolumes(sound) => rc_mut(sound).volumes.insert(index, value as u8),
        IntSeq::SoundEffects(sound) => rc_mut(sound).effects.insert(index, value as u8),
    }
}

unsafe fn int_seq_values(seq: &IntSeq) -> Vec<i64> {
    (0..int_seq_len(seq))
        .map(|index| int_seq_get(seq, index))
        .collect()
}

unsafe fn make_int_seq(out: ffi::py_OutRef, seq: IntSeq) {
    new_userdata(out, TP_INT_SEQ, seq);
}

unsafe extern "C" fn int_seq_len_fn(_argc: i32, argv: ffi::py_StackRef) -> bool {
    value::return_int(int_seq_len(userdata::<IntSeq>(value::arg(argv, 0))) as i64);
    true
}

unsafe extern "C" fn int_seq_getitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seq = userdata::<IntSeq>(value::arg(argv, 0));
    let Some(index) = normalize_index(value::int_arg(argv, 1), int_seq_len(seq)) else {
        return value::raise_index_error("sequence index out of range");
    };
    value::return_int(int_seq_get(seq, index));
    true
}

unsafe extern "C" fn int_seq_setitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seq = userdata::<IntSeq>(value::arg(argv, 0));
    let Some(index) = normalize_index(value::int_arg(argv, 1), int_seq_len(seq)) else {
        return value::raise_index_error("sequence index out of range");
    };
    int_seq_set(seq, index, value::int_arg(argv, 2));
    value::return_none();
    true
}

unsafe extern "C" fn int_seq_delitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seq = userdata::<IntSeq>(value::arg(argv, 0));
    let Some(index) = normalize_index(value::int_arg(argv, 1), int_seq_len(seq)) else {
        return value::raise_index_error("sequence index out of range");
    };
    int_seq_remove(seq, index);
    value::return_none();
    true
}

unsafe extern "C" fn int_seq_iter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seq = userdata::<IntSeq>(value::arg(argv, 0));
    ffi::py_newlist(ffi::py_retval());
    for i in 0..int_seq_len(seq) {
        ffi::py_newint(ffi::py_list_emplace(ffi::py_retval()), int_seq_get(seq, i));
    }
    ffi::py_iter(ffi::py_retval());
    true
}

unsafe extern "C" fn int_seq_append(_argc: i32, argv: ffi::py_StackRef) -> bool {
    int_seq_push(
        userdata::<IntSeq>(value::arg(argv, 0)),
        value::int_arg(argv, 1),
    );
    value::return_none();
    true
}

unsafe extern "C" fn int_seq_extend(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seq = userdata::<IntSeq>(value::arg(argv, 0));
    let values = value::arg(argv, 1);
    for i in 0..ffi::py_list_len(values) {
        int_seq_push(seq, ffi::py_toint(ffi::py_list_getitem(values, i)));
    }
    value::return_none();
    true
}

unsafe extern "C" fn int_seq_insert(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seq = userdata::<IntSeq>(value::arg(argv, 0));
    let index = normalize_insert_index(value::int_arg(argv, 1), int_seq_len(seq));
    int_seq_insert_at(seq, index, value::int_arg(argv, 2));
    value::return_none();
    true
}

unsafe extern "C" fn int_seq_clear(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seq = userdata::<IntSeq>(value::arg(argv, 0));
    while int_seq_len(seq) > 0 {
        int_seq_remove(seq, int_seq_len(seq) - 1);
    }
    value::return_none();
    true
}

unsafe extern "C" fn int_seq_pop(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seq = userdata::<IntSeq>(value::arg(argv, 0));
    let index = if value::is_none(value::arg(argv, 1)) {
        -1
    } else {
        value::int_arg(argv, 1)
    };
    let Some(index) = normalize_index(index, int_seq_len(seq)) else {
        return value::raise_index_error("pop index out of range");
    };
    value::return_int(int_seq_remove(seq, index));
    true
}

unsafe extern "C" fn int_seq_contains(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seq = userdata::<IntSeq>(value::arg(argv, 0));
    let value = value::int_arg(argv, 1);
    value::return_bool((0..int_seq_len(seq)).any(|index| int_seq_get(seq, index) == value));
    true
}

unsafe extern "C" fn int_seq_eq(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seq = userdata::<IntSeq>(value::arg(argv, 0));
    let other = value::arg(argv, 1);
    let values = int_seq_values(seq);

    if value::is_list(other) {
        if values.len() != ffi::py_list_len(other) as usize {
            value::return_bool(false);
            return true;
        }
        value::return_bool(values.iter().enumerate().all(|(index, value)| {
            *value == ffi::py_toint(ffi::py_list_getitem(other, index as i32))
        }));
        return true;
    }

    if ffi::py_isinstance(other, TP_INT_SEQ) {
        value::return_bool(values == int_seq_values(userdata::<IntSeq>(other)));
        return true;
    }

    value::return_bool(false);
    true
}

unsafe extern "C" fn int_seq_bool(_argc: i32, argv: ffi::py_StackRef) -> bool {
    value::return_bool(int_seq_len(userdata::<IntSeq>(value::arg(argv, 0))) > 0);
    true
}

pub(super) unsafe extern "C" fn tone_wavetable_get(_argc: i32, argv: ffi::py_StackRef) -> bool {
    make_int_seq(
        ffi::py_retval(),
        IntSeq::ToneWavetable(userdata::<pyxel::RcTone>(value::arg(argv, 0)).clone()),
    );
    true
}

macro_rules! sound_seq_getter {
    ($name:ident, $variant:ident) => {
        pub(super) unsafe extern "C" fn $name(_argc: i32, argv: ffi::py_StackRef) -> bool {
            make_int_seq(
                ffi::py_retval(),
                IntSeq::$variant(userdata::<pyxel::RcSound>(value::arg(argv, 0)).clone()),
            );
            true
        }
    };
}

sound_seq_getter!(sound_notes_get, SoundNotes);
sound_seq_getter!(sound_tones_get, SoundTones);
sound_seq_getter!(sound_volumes_get, SoundVolumes);
sound_seq_getter!(sound_effects_get, SoundEffects);

pub(super) unsafe extern "C" fn music_seqs_get(_argc: i32, argv: ffi::py_StackRef) -> bool {
    new_userdata(
        ffi::py_retval(),
        TP_MUSIC_SEQS,
        MusicSeqs {
            music: userdata::<pyxel::RcMusic>(value::arg(argv, 0)).clone(),
        },
    );
    true
}

unsafe extern "C" fn music_seqs_len(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seqs = userdata::<MusicSeqs>(value::arg(argv, 0));
    value::return_int(rc_ref(&seqs.music).seqs.len() as i64);
    true
}

pub(super) unsafe extern "C" fn music_seqs_getitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seqs = userdata::<MusicSeqs>(value::arg(argv, 0));
    let len = rc_ref(&seqs.music).seqs.len();
    let Some(index) = normalize_index(value::int_arg(argv, 1), len) else {
        return value::raise_index_error("sequence index out of range");
    };
    new_userdata(
        ffi::py_retval(),
        TP_MUSIC_SEQ,
        MusicSeq {
            music: seqs.music.clone(),
            index,
        },
    );
    true
}

unsafe extern "C" fn music_seqs_setitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seqs = userdata::<MusicSeqs>(value::arg(argv, 0));
    let len = rc_ref(&seqs.music).seqs.len();
    let Some(index) = normalize_index(value::int_arg(argv, 1), len) else {
        return value::raise_index_error("sequence index out of range");
    };
    rc_mut(&seqs.music).seqs[index] = int_list_from_ref(value::arg(argv, 2));
    value::return_none();
    true
}

unsafe extern "C" fn music_seqs_delitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seqs = userdata::<MusicSeqs>(value::arg(argv, 0));
    let len = rc_ref(&seqs.music).seqs.len();
    let Some(index) = normalize_index(value::int_arg(argv, 1), len) else {
        return value::raise_index_error("sequence index out of range");
    };
    rc_mut(&seqs.music).seqs.remove(index);
    value::return_none();
    true
}

unsafe extern "C" fn music_seqs_iter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seqs = userdata::<MusicSeqs>(value::arg(argv, 0));
    let len = rc_ref(&seqs.music).seqs.len();
    ffi::py_newlist(ffi::py_retval());
    for index in 0..len {
        new_userdata(
            ffi::py_list_emplace(ffi::py_retval()),
            TP_MUSIC_SEQ,
            MusicSeq {
                music: seqs.music.clone(),
                index,
            },
        );
    }
    ffi::py_iter(ffi::py_retval());
    true
}

unsafe extern "C" fn music_seqs_bool(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seqs = userdata::<MusicSeqs>(value::arg(argv, 0));
    value::return_bool(!rc_ref(&seqs.music).seqs.is_empty());
    true
}

unsafe extern "C" fn music_seqs_append(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seqs = userdata::<MusicSeqs>(value::arg(argv, 0));
    rc_mut(&seqs.music)
        .seqs
        .push(int_list_from_ref(value::arg(argv, 1)));
    value::return_none();
    true
}

unsafe extern "C" fn music_seqs_extend(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seqs = userdata::<MusicSeqs>(value::arg(argv, 0));
    rc_mut(&seqs.music)
        .seqs
        .extend(int_nested_list_from_ref(value::arg(argv, 1)));
    value::return_none();
    true
}

unsafe extern "C" fn music_seqs_insert(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seqs = userdata::<MusicSeqs>(value::arg(argv, 0));
    let index = normalize_insert_index(value::int_arg(argv, 1), rc_ref(&seqs.music).seqs.len());
    rc_mut(&seqs.music)
        .seqs
        .insert(index, int_list_from_ref(value::arg(argv, 2)));
    value::return_none();
    true
}

unsafe extern "C" fn music_seqs_pop(argc: i32, argv: ffi::py_StackRef) -> bool {
    let seqs = userdata::<MusicSeqs>(value::arg(argv, 0));
    let len = rc_ref(&seqs.music).seqs.len();
    if len == 0 {
        return value::raise_index_error("pop from empty sequence");
    }
    let index = if argc <= 1 || value::is_none(value::arg(argv, 1)) {
        -1
    } else {
        value::int_arg(argv, 1)
    };
    let Some(index) = normalize_index(index, len) else {
        return value::raise_index_error("pop index out of range");
    };
    return_int_list(&rc_mut(&seqs.music).seqs.remove(index));
    true
}

unsafe extern "C" fn music_seqs_clear(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seqs = userdata::<MusicSeqs>(value::arg(argv, 0));
    rc_mut(&seqs.music).seqs.clear();
    value::return_none();
    true
}

unsafe extern "C" fn music_seq_len(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seq = userdata::<MusicSeq>(value::arg(argv, 0));
    value::return_int(rc_ref(&seq.music).seqs[seq.index].len() as i64);
    true
}

unsafe extern "C" fn music_seq_getitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seq = userdata::<MusicSeq>(value::arg(argv, 0));
    let values = &rc_ref(&seq.music).seqs[seq.index];
    let Some(index) = normalize_index(value::int_arg(argv, 1), values.len()) else {
        return value::raise_index_error("sequence index out of range");
    };
    value::return_int(values[index] as i64);
    true
}

unsafe extern "C" fn music_seq_setitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seq = userdata::<MusicSeq>(value::arg(argv, 0));
    let len = rc_ref(&seq.music).seqs[seq.index].len();
    let Some(index) = normalize_index(value::int_arg(argv, 1), len) else {
        return value::raise_index_error("sequence index out of range");
    };
    rc_mut(&seq.music).seqs[seq.index][index] = value::int_arg(argv, 2) as u32;
    value::return_none();
    true
}

unsafe extern "C" fn music_seq_iter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seq = userdata::<MusicSeq>(value::arg(argv, 0));
    make_int_list(ffi::py_retval(), &rc_ref(&seq.music).seqs[seq.index]);
    ffi::py_iter(ffi::py_retval());
    true
}

unsafe extern "C" fn music_seq_bool(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let seq = userdata::<MusicSeq>(value::arg(argv, 0));
    value::return_bool(!rc_ref(&seq.music).seqs[seq.index].is_empty());
    true
}

pub(super) unsafe fn register_int_seq_type(module: ffi::py_GlobalRef) {
    TP_INT_SEQ = value::new_type(module, c"IntSeq", Some(drop_int_seq));
    value::bind_magic(TP_INT_SEQ, c"__len__", Some(int_seq_len_fn));
    value::bind_magic(TP_INT_SEQ, c"__getitem__", Some(int_seq_getitem));
    value::bind_magic(TP_INT_SEQ, c"__setitem__", Some(int_seq_setitem));
    value::bind_magic(TP_INT_SEQ, c"__delitem__", Some(int_seq_delitem));
    value::bind_magic(TP_INT_SEQ, c"__iter__", Some(int_seq_iter));
    value::bind_magic(TP_INT_SEQ, c"__contains__", Some(int_seq_contains));
    value::bind_magic(TP_INT_SEQ, c"__eq__", Some(int_seq_eq));
    value::bind_magic(TP_INT_SEQ, c"__bool__", Some(int_seq_bool));
    ffi::py_bindmethod(TP_INT_SEQ, c"append".as_ptr(), Some(int_seq_append));
    ffi::py_bindmethod(TP_INT_SEQ, c"extend".as_ptr(), Some(int_seq_extend));
    ffi::py_bindmethod(TP_INT_SEQ, c"insert".as_ptr(), Some(int_seq_insert));
    ffi::py_bindmethod(TP_INT_SEQ, c"clear".as_ptr(), Some(int_seq_clear));
    ffi::py_bind(
        ffi::py_tpobject(TP_INT_SEQ),
        c"pop(self, index=None)".as_ptr(),
        Some(int_seq_pop),
    );
}

pub(super) unsafe fn register_music_seq_types(module: ffi::py_GlobalRef) {
    TP_MUSIC_SEQS = value::new_type(module, c"Seqs", Some(drop_music_seqs));
    value::bind_magic(TP_MUSIC_SEQS, c"__len__", Some(music_seqs_len));
    value::bind_magic(TP_MUSIC_SEQS, c"__getitem__", Some(music_seqs_getitem));
    value::bind_magic(TP_MUSIC_SEQS, c"__setitem__", Some(music_seqs_setitem));
    value::bind_magic(TP_MUSIC_SEQS, c"__delitem__", Some(music_seqs_delitem));
    value::bind_magic(TP_MUSIC_SEQS, c"__iter__", Some(music_seqs_iter));
    value::bind_magic(TP_MUSIC_SEQS, c"__bool__", Some(music_seqs_bool));
    ffi::py_bindmethod(TP_MUSIC_SEQS, c"append".as_ptr(), Some(music_seqs_append));
    ffi::py_bindmethod(TP_MUSIC_SEQS, c"extend".as_ptr(), Some(music_seqs_extend));
    ffi::py_bindmethod(TP_MUSIC_SEQS, c"insert".as_ptr(), Some(music_seqs_insert));
    ffi::py_bindmethod(TP_MUSIC_SEQS, c"clear".as_ptr(), Some(music_seqs_clear));
    ffi::py_bind(
        ffi::py_tpobject(TP_MUSIC_SEQS),
        c"pop(self, index=None)".as_ptr(),
        Some(music_seqs_pop),
    );

    TP_MUSIC_SEQ = value::new_type(module, c"Seq", Some(drop_music_seq));
    value::bind_magic(TP_MUSIC_SEQ, c"__len__", Some(music_seq_len));
    value::bind_magic(TP_MUSIC_SEQ, c"__getitem__", Some(music_seq_getitem));
    value::bind_magic(TP_MUSIC_SEQ, c"__setitem__", Some(music_seq_setitem));
    value::bind_magic(TP_MUSIC_SEQ, c"__iter__", Some(music_seq_iter));
    value::bind_magic(TP_MUSIC_SEQ, c"__bool__", Some(music_seq_bool));
}
