use std::ffi::CString;

use crate::ffi;
use crate::helpers::*;
use crate::sound_wrapper::TP_SOUND;

unsafe extern "C" fn pyxel_play(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let ch = arg_int(argv, 0) as u32;
    let snd_ref = arg(argv, 1);
    let sec = arg_opt_float(argv, 2).map(|v| v as f32);
    let r#loop = arg_opt_bool(argv, 3).unwrap_or(false);
    let resume = arg_opt_bool(argv, 4).unwrap_or(false);

    if is_int(snd_ref) {
        pyxel::pyxel().play_sound(ch, ffi::py_toint(snd_ref) as u32, sec, r#loop, resume);
    } else if ffi::py_isinstance(snd_ref, TP_SOUND) {
        let snd_ptr = *(ffi::py_touserdata(snd_ref) as *mut *mut pyxel::Sound);
        (&mut *pyxel::channels()[ch as usize]).play_sound(snd_ptr, sec, r#loop, resume);
    } else if is_list(snd_ref) {
        let len = ffi::py_list_len(snd_ref);
        let first = ffi::py_list_getitem(snd_ref, 0);
        if ffi::py_isinstance(first, TP_SOUND) {
            let sounds: Vec<*mut pyxel::Sound> = (0..len)
                .map(|i| *(ffi::py_touserdata(ffi::py_list_getitem(snd_ref, i)) as *mut *mut pyxel::Sound))
                .collect();
            (&mut *pyxel::channels()[ch as usize]).play(sounds, sec, r#loop, resume);
        } else {
            let snds: Vec<u32> = (0..len)
                .map(|i| ffi::py_toint(ffi::py_list_getitem(snd_ref, i)) as u32)
                .collect();
            pyxel::pyxel().play(ch, &snds, sec, r#loop, resume);
        }
    } else if is_str(snd_ref) {
        let mml = arg_str(argv, 1);
        if let Err(e) = pyxel::pyxel().play_mml(ch, mml, sec, r#loop, resume) {
            return raise_exc(&e);
        }
    }
    ret_none();
    true
}

unsafe extern "C" fn pyxel_playm(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let msc = arg_int(argv, 0) as u32;
    let sec = arg_opt_float(argv, 1).map(|v| v as f32);
    let r#loop = arg_opt_bool(argv, 2).unwrap_or(false);
    pyxel::pyxel().play_music(msc, sec, r#loop);
    ret_none();
    true
}

unsafe extern "C" fn pyxel_stop(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if arg_is_none(argv, 0) {
        pyxel::pyxel().stop_all_channels();
    } else {
        pyxel::pyxel().stop_channel(arg_int(argv, 0) as u32);
    }
    ret_none();
    true
}

unsafe extern "C" fn pyxel_play_pos(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let ch = arg_int(argv, 0) as u32;
    match pyxel::pyxel().play_position(ch) {
        Some((snd, sec)) => {
            let out = ffi::py_newtuple(ffi::py_retval(), 2);
            ffi::py_newint(out.add(0), snd as i64);
            ffi::py_newfloat(out.add(1), sec as f64);
        }
        None => ret_none(),
    }
    true
}

unsafe extern "C" fn pyxel_gen_bgm(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let preset = arg_int(argv, 0) as i32;
    let instr = arg_int(argv, 1) as i32;
    let seed = arg_opt_int(argv, 2).map(|v| v as u64);
    let play = arg_opt_bool(argv, 3);
    let result = pyxel::pyxel().gen_bgm(preset, instr, seed, play);
    ffi::py_newlist(ffi::py_retval());
    let list = ffi::py_retval();
    for s in &result {
        let cs = CString::new(s.as_str()).unwrap();
        ffi::py_newstr(ffi::py_list_emplace(list), cs.as_ptr());
    }
    true
}

pub unsafe fn add_audio_functions(m: ffi::py_GlobalRef) {
    bind(m, c"play(ch, snd, sec=None, loop=None, resume=None)", Some(pyxel_play));
    bind(m, c"playm(msc, sec=None, loop=None)", Some(pyxel_playm));
    bind(m, c"stop(ch=None)", Some(pyxel_stop));
    bind(m, c"play_pos(ch)", Some(pyxel_play_pos));
    bind(m, c"gen_bgm(preset, instr, seed=None, play=None)", Some(pyxel_gen_bgm));

    // Deprecated functions
    bind(m, c"channel(ch)", Some(pyxel_channel_deprecated));
    bind(m, c"sound(snd)", Some(pyxel_sound_deprecated));
    bind(m, c"music(msc)", Some(pyxel_music_deprecated));
}

unsafe extern "C" fn pyxel_channel_deprecated(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let index = arg_int(argv, 0) as usize;
    let channels = pyxel::channels();
    if index >= channels.len() {
        return raise_exc("Invalid channel index");
    }
    crate::channel_wrapper::new_channel_obj(ffi::py_retval(), channels[index]);
    true
}

unsafe extern "C" fn pyxel_sound_deprecated(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let index = arg_int(argv, 0) as usize;
    let sounds = pyxel::sounds();
    if index >= sounds.len() {
        return raise_exc("Invalid sound index");
    }
    crate::sound_wrapper::new_sound_obj(ffi::py_retval(), sounds[index]);
    true
}

unsafe extern "C" fn pyxel_music_deprecated(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let index = arg_int(argv, 0) as usize;
    let musics = pyxel::musics();
    if index >= musics.len() {
        return raise_exc("Invalid music index");
    }
    crate::music_wrapper::new_music_obj(ffi::py_retval(), musics[index]);
    true
}
