use std::ffi::CString;

use crate::ffi;
use crate::helpers::*;

pub static mut TP_MUSIC: ffi::py_Type = 0;
pub static mut TP_MUSICS: ffi::py_Type = 0;
static mut TP_SEQS: ffi::py_Type = 0;
static mut TP_SEQ: ffi::py_Type = 0;

unsafe fn mus(argv: ffi::py_StackRef) -> &'static mut pyxel::Music {
    &mut *(*(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Music))
}

pub unsafe fn new_music_obj(out: ffi::py_OutRef, ptr: *mut pyxel::Music) {
    let ud = ffi::py_newobject(out, TP_MUSIC, 0, size_of::<*mut pyxel::Music>() as i32);
    *(ud as *mut *mut pyxel::Music) = ptr;
}

// SeqRef: stores music pointer + sequence index
#[repr(C)]
struct SeqRef {
    music: *mut pyxel::Music,
    index: usize,
}

// Seq — a single sequence (list of sound indices) in a Music
unsafe extern "C" fn seq_getitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let sr = &*(ffi::py_touserdata(arg(argv, 0)) as *const SeqRef);
    let index = arg_int(argv, 1) as usize;
    let music = &mut *sr.music;
    let seq = &music.seqs[sr.index];
    if index >= seq.len() {
        return raise_index();
    }
    ret_int(seq[index] as i64);
    true
}

unsafe extern "C" fn seq_setitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let sr = &*(ffi::py_touserdata(arg(argv, 0)) as *const SeqRef);
    let index = arg_int(argv, 1) as usize;
    let music = &mut *sr.music;
    let seq = &mut music.seqs[sr.index];
    if index >= seq.len() {
        return raise_index();
    }
    seq[index] = arg_int(argv, 2) as u32;
    ret_none();
    true
}

unsafe extern "C" fn seq_len(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let sr = &*(ffi::py_touserdata(arg(argv, 0)) as *const SeqRef);
    ret_int((&mut *sr.music).seqs[sr.index].len() as i64);
    true
}

// Seqs — the list of sequences in a Music
unsafe extern "C" fn seqs_getitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let music_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Music);
    let index = arg_int(argv, 1) as usize;
    if index >= (&mut *music_ptr).seqs.len() {
        return raise_index();
    }
    let ud = ffi::py_newobject(ffi::py_retval(), TP_SEQ, 0, size_of::<SeqRef>() as i32);
    let sr = &mut *(ud as *mut SeqRef);
    sr.music = music_ptr;
    sr.index = index;
    true
}

unsafe extern "C" fn seqs_setitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let music_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Music);
    let index = arg_int(argv, 1) as usize;
    if index >= (&mut *music_ptr).seqs.len() {
        return raise_index();
    }
    (&mut *music_ptr).seqs[index] = arg_int_list(argv, 2);
    ret_none();
    true
}

unsafe extern "C" fn seqs_len(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let music_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Music);
    ret_int((&mut *music_ptr).seqs.len() as i64);
    true
}

unsafe extern "C" fn seqs_delitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let music_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Music);
    let key = arg(argv, 1);
    let seqs = &mut (&mut *music_ptr).seqs;
    if is_slice(key) {
        let (start, stop, step) = slice_indices(key, seqs.len());
        let mut indices = collect_indices(start, stop, step);
        indices.sort_unstable_by(|a, b| b.cmp(a));
        for i in indices { seqs.remove(i); }
    } else {
        match resolve_index(ffi::py_toint(key), seqs.len()) {
            Some(i) => { seqs.remove(i); }
            None => return raise_index(),
        }
    }
    ret_none();
    true
}

unsafe extern "C" fn seqs_append(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let music_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Music);
    (&mut *music_ptr).seqs.push(arg_int_list(argv, 1));
    ret_none();
    true
}

unsafe extern "C" fn seqs_clear(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let music_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Music);
    (&mut *music_ptr).seqs.clear();
    ret_none();
    true
}

unsafe extern "C" fn seqs_bool(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let music_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Music);
    ret_bool(!(&mut *music_ptr).seqs.is_empty());
    true
}

unsafe extern "C" fn seqs_getter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let music_ptr = *(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Music);
    let ud = ffi::py_newobject(ffi::py_retval(), TP_SEQS, 0, size_of::<*mut pyxel::Music>() as i32);
    *(ud as *mut *mut pyxel::Music) = music_ptr;
    true
}

// music.set([snd0, snd1], [snd2, snd3], ...) — variadic
unsafe extern "C" fn music_set(argc: i32, argv: ffi::py_StackRef) -> bool {
    let mut seqs: Vec<Vec<u32>> = Vec::new();
    for i in 1..argc {
        seqs.push(arg_int_list(argv, i as usize));
    }
    mus(argv).set(&seqs);
    ret_none();
    true
}

unsafe extern "C" fn music_save(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let filename = arg_str(argv, 1);
    let sec = arg_float(argv, 2) as f32;
    let ffmpeg = arg_opt_bool(argv, 3);
    if let Err(e) = mus(argv).save(filename, sec, ffmpeg) {
        let msg = CString::new(e).unwrap();
        return ffi::py_exception(
            ffi::py_PredefinedType_tp_Exception as ffi::py_Type,
            msg.as_ptr(),
        );
    }
    ret_none();
    true
}

unsafe extern "C" fn music_new(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let cls = ffi::py_totype(arg(argv, 0));
    let ptr = pyxel::Music::new();
    let ud = ffi::py_newobject(ffi::py_retval(), cls, 0, size_of::<*mut pyxel::Music>() as i32);
    *(ud as *mut *mut pyxel::Music) = ptr;
    true
}

pub unsafe fn add_music_class(m: ffi::py_GlobalRef) {
    // Seq type (single sequence)
    TP_SEQ = new_type(c"Seq", m);
    ffi::py_bindmagic(TP_SEQ, ffi::py_name(c"__getitem__".as_ptr()), Some(seq_getitem));
    ffi::py_bindmagic(TP_SEQ, ffi::py_name(c"__setitem__".as_ptr()), Some(seq_setitem));
    ffi::py_bindmagic(TP_SEQ, ffi::py_name(c"__len__".as_ptr()), Some(seq_len));

    // Seqs type (list of sequences)
    TP_SEQS = new_type(c"Seqs", m);
    ffi::py_bindmagic(TP_SEQS, ffi::py_name(c"__getitem__".as_ptr()), Some(seqs_getitem));
    ffi::py_bindmagic(TP_SEQS, ffi::py_name(c"__setitem__".as_ptr()), Some(seqs_setitem));
    ffi::py_bindmagic(TP_SEQS, ffi::py_name(c"__delitem__".as_ptr()), Some(seqs_delitem));
    ffi::py_bindmagic(TP_SEQS, ffi::py_name(c"__len__".as_ptr()), Some(seqs_len));
    ffi::py_bindmagic(TP_SEQS, ffi::py_name(c"__bool__".as_ptr()), Some(seqs_bool));
    ffi::py_bindmethod(TP_SEQS, c"append".as_ptr(), Some(seqs_append));
    ffi::py_bindmethod(TP_SEQS, c"clear".as_ptr(), Some(seqs_clear));

    // Music type
    TP_MUSIC = new_type(c"Music", m);
    ffi::py_bindproperty(TP_MUSIC, c"seqs".as_ptr(), Some(seqs_getter), None);
    ffi::py_bindproperty(TP_MUSIC, c"snds_list".as_ptr(), Some(seqs_getter), None);
    ffi::py_bindmethod(TP_MUSIC, c"set".as_ptr(), Some(music_set));
    let tp_obj = ffi::py_tpobject(TP_MUSIC);
    bind(tp_obj, c"save(self, filename, sec, ffmpeg=None)", Some(music_save));
    bind(tp_obj, c"__new__(cls)", Some(music_new));

    // Musics collection
    impl_object_collection!(pyxel::musics, new_music_obj, musics_getitem, musics_setitem, musics_len, musics_iter);
    register_collection!(TP_MUSICS, c"Musics", m, musics_getitem, musics_setitem, musics_len, musics_iter);
}
