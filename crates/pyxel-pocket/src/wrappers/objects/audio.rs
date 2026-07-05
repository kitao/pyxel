use super::{
    drop_channel, drop_music, drop_sound, drop_tone, farg, ffi, int_list_from_ref, new_userdata,
    noop_init, rc_mut, rc_ref, sequences, sound_list_arg, userdata, value, TP_CHANNEL, TP_MUSIC,
    TP_SOUND, TP_TONE,
};

// Channel, Tone, Sound, Music

unsafe extern "C" fn channel_new(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let cls = ffi::py_totype(value::arg(argv, 0));
    new_userdata(ffi::py_retval(), cls, pyxel::Channel::new());
    true
}

unsafe extern "C" fn channel_gain_get(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let channel = userdata::<pyxel::RcChannel>(value::arg(argv, 0));
    value::return_float(rc_ref(channel).gain);
    true
}

unsafe extern "C" fn channel_gain_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let channel = userdata::<pyxel::RcChannel>(value::arg(argv, 0));
    rc_mut(channel).gain = farg(argv, 1);
    value::return_none();
    true
}

unsafe extern "C" fn channel_detune_get(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let channel = userdata::<pyxel::RcChannel>(value::arg(argv, 0));
    value::return_int(rc_ref(channel).detune as i64);
    true
}

unsafe extern "C" fn channel_detune_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let channel = userdata::<pyxel::RcChannel>(value::arg(argv, 0));
    rc_mut(channel).detune = value::int_arg(argv, 1) as pyxel::ChannelDetune;
    value::return_none();
    true
}

unsafe extern "C" fn channel_play(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let channel = userdata::<pyxel::RcChannel>(value::arg(argv, 0));
    let snd = value::arg(argv, 1);
    let sec = value::opt_float_arg(argv, 2);
    let should_loop = value::opt_bool_arg(argv, 3).unwrap_or(false);
    let should_resume = value::opt_bool_arg(argv, 4).unwrap_or(false);
    let _lock = pyxel::AudioLock::lock();

    if value::is_str(snd) {
        match rc_mut(channel).play_mml(&value::str_arg(argv, 1), sec, should_loop, should_resume) {
            Ok(()) => {}
            Err(err) => return value::raise_exception(&err),
        }
    } else if let Some(sounds) = sound_list_arg(snd) {
        rc_mut(channel).play(sounds, sec, should_loop, should_resume);
    } else {
        return value::raise_exception("Invalid sound");
    }
    value::return_none();
    true
}

unsafe extern "C" fn channel_stop(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let _lock = pyxel::AudioLock::lock();
    let channel = userdata::<pyxel::RcChannel>(value::arg(argv, 0));
    rc_mut(channel).stop();
    value::return_none();
    true
}

unsafe extern "C" fn channel_play_pos(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let _lock = pyxel::AudioLock::lock();
    let channel = userdata::<pyxel::RcChannel>(value::arg(argv, 0));
    value::return_optional_play_pos(rc_mut(channel).play_position());
    true
}

unsafe extern "C" fn tone_new(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let cls = ffi::py_totype(value::arg(argv, 0));
    new_userdata(ffi::py_retval(), cls, pyxel::Tone::new());
    true
}

unsafe extern "C" fn tone_mode_get(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone = userdata::<pyxel::RcTone>(value::arg(argv, 0));
    let mode: u32 = rc_ref(tone).mode.into();
    value::return_int(mode as i64);
    true
}

unsafe extern "C" fn tone_mode_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone = userdata::<pyxel::RcTone>(value::arg(argv, 0));
    rc_mut(tone).mode = pyxel::ToneMode::from(value::int_arg(argv, 1) as u32);
    value::return_none();
    true
}

unsafe extern "C" fn tone_sample_bits_get(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone = userdata::<pyxel::RcTone>(value::arg(argv, 0));
    value::return_int(rc_ref(tone).sample_bits as i64);
    true
}

unsafe extern "C" fn tone_sample_bits_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone = userdata::<pyxel::RcTone>(value::arg(argv, 0));
    rc_mut(tone).sample_bits = value::int_arg(argv, 1) as u32;
    value::return_none();
    true
}

unsafe extern "C" fn tone_gain_get(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone = userdata::<pyxel::RcTone>(value::arg(argv, 0));
    value::return_float(rc_ref(tone).gain);
    true
}

unsafe extern "C" fn tone_gain_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tone = userdata::<pyxel::RcTone>(value::arg(argv, 0));
    rc_mut(tone).gain = farg(argv, 1);
    value::return_none();
    true
}

unsafe extern "C" fn sound_new(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let cls = ffi::py_totype(value::arg(argv, 0));
    new_userdata(ffi::py_retval(), cls, pyxel::Sound::new());
    true
}

unsafe extern "C" fn sound_speed_get(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let sound = userdata::<pyxel::RcSound>(value::arg(argv, 0));
    value::return_int(rc_ref(sound).speed as i64);
    true
}

unsafe extern "C" fn sound_speed_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let sound = userdata::<pyxel::RcSound>(value::arg(argv, 0));
    rc_mut(sound).speed = value::int_arg(argv, 1) as pyxel::SoundSpeed;
    value::return_none();
    true
}

unsafe extern "C" fn sound_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    for index in 1..=5 {
        if value::is_none(value::arg(argv, index)) {
            return value::raise_exception("Sound.set() missing required argument");
        }
    }

    let sound = userdata::<pyxel::RcSound>(value::arg(argv, 0));
    match rc_mut(sound).set(
        &value::str_arg(argv, 1),
        &value::str_arg(argv, 2),
        &value::str_arg(argv, 3),
        &value::str_arg(argv, 4),
        value::int_arg(argv, 5) as pyxel::SoundSpeed,
    ) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

macro_rules! sound_setter {
    ($name:ident, $method:ident) => {
        unsafe extern "C" fn $name(_argc: i32, argv: ffi::py_StackRef) -> bool {
            let sound = userdata::<pyxel::RcSound>(value::arg(argv, 0));
            match rc_mut(sound).$method(&value::str_arg(argv, 1)) {
                Ok(()) => {
                    value::return_none();
                    true
                }
                Err(err) => value::raise_exception(&err),
            }
        }
    };
}

sound_setter!(sound_set_notes, set_notes);
sound_setter!(sound_set_tones, set_tones);
sound_setter!(sound_set_volumes, set_volumes);
sound_setter!(sound_set_effects, set_effects);

unsafe extern "C" fn sound_mml(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let sound = userdata::<pyxel::RcSound>(value::arg(argv, 0));
    if value::is_none(value::arg(argv, 1)) {
        rc_mut(sound).clear_mml();
        value::return_none();
        return true;
    }
    match rc_mut(sound).set_mml(&value::str_arg(argv, 1)) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn sound_old_mml(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let sound = userdata::<pyxel::RcSound>(value::arg(argv, 0));
    if value::is_none(value::arg(argv, 1)) {
        rc_mut(sound).clear_mml();
        value::return_none();
        return true;
    }
    match rc_mut(sound).old_mml(&value::str_arg(argv, 1)) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn sound_pcm(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let sound = userdata::<pyxel::RcSound>(value::arg(argv, 0));
    if value::is_none(value::arg(argv, 1)) {
        rc_mut(sound).clear_pcm();
        value::return_none();
        return true;
    }
    match rc_mut(sound).load_pcm(&value::str_arg(argv, 1)) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn sound_save(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let sound = userdata::<pyxel::RcSound>(value::arg(argv, 0));
    match rc_ref(sound).save(
        &value::str_arg(argv, 1),
        farg(argv, 2),
        value::opt_bool_arg(argv, 3),
    ) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn sound_total_sec(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let sound = userdata::<pyxel::RcSound>(value::arg(argv, 0));
    match rc_ref(sound).total_seconds() {
        Some(sec) => value::return_float(sec),
        None => value::return_none(),
    }
    true
}

unsafe extern "C" fn music_new(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let cls = ffi::py_totype(value::arg(argv, 0));
    new_userdata(ffi::py_retval(), cls, pyxel::Music::new());
    true
}

unsafe extern "C" fn music_set(argc: i32, argv: ffi::py_StackRef) -> bool {
    let music = userdata::<pyxel::RcMusic>(value::arg(argv, 0));
    let mut seqs = Vec::new();
    for i in 1..argc {
        seqs.push(int_list_from_ref(value::arg(argv, i as usize)));
    }
    rc_mut(music).set(&seqs);
    value::return_none();
    true
}

unsafe extern "C" fn music_save(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let music = userdata::<pyxel::RcMusic>(value::arg(argv, 0));
    match rc_ref(music).save(
        &value::str_arg(argv, 1),
        farg(argv, 2),
        value::opt_bool_arg(argv, 3),
    ) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

pub(super) unsafe fn register(module: ffi::py_GlobalRef) {
    TP_CHANNEL = value::new_type(module, c"Channel", Some(drop_channel));
    let channel_type = ffi::py_tpobject(TP_CHANNEL);
    ffi::py_bind(channel_type, c"__new__(cls)".as_ptr(), Some(channel_new));
    ffi::py_bind(channel_type, c"__init__(self)".as_ptr(), Some(noop_init));
    ffi::py_bindproperty(
        TP_CHANNEL,
        c"gain".as_ptr(),
        Some(channel_gain_get),
        Some(channel_gain_set),
    );
    ffi::py_bindproperty(
        TP_CHANNEL,
        c"detune".as_ptr(),
        Some(channel_detune_get),
        Some(channel_detune_set),
    );
    ffi::py_bind(
        channel_type,
        c"play(self, snd, sec=None, loop=None, resume=None)".as_ptr(),
        Some(channel_play),
    );
    ffi::py_bindmethod(TP_CHANNEL, c"stop".as_ptr(), Some(channel_stop));
    ffi::py_bindmethod(TP_CHANNEL, c"play_pos".as_ptr(), Some(channel_play_pos));

    sequences::register_int_seq_type(module);
    TP_TONE = value::new_type(module, c"Tone", Some(drop_tone));
    let tone_type = ffi::py_tpobject(TP_TONE);
    ffi::py_bind(tone_type, c"__new__(cls)".as_ptr(), Some(tone_new));
    ffi::py_bind(tone_type, c"__init__(self)".as_ptr(), Some(noop_init));
    ffi::py_bindproperty(
        TP_TONE,
        c"mode".as_ptr(),
        Some(tone_mode_get),
        Some(tone_mode_set),
    );
    ffi::py_bindproperty(
        TP_TONE,
        c"sample_bits".as_ptr(),
        Some(tone_sample_bits_get),
        Some(tone_sample_bits_set),
    );
    ffi::py_bindproperty(
        TP_TONE,
        c"wavetable".as_ptr(),
        Some(sequences::tone_wavetable_get),
        None,
    );
    ffi::py_bindproperty(
        TP_TONE,
        c"waveform".as_ptr(),
        Some(sequences::tone_wavetable_get),
        None,
    );
    ffi::py_bindproperty(
        TP_TONE,
        c"gain".as_ptr(),
        Some(tone_gain_get),
        Some(tone_gain_set),
    );
    ffi::py_bindproperty(
        TP_TONE,
        c"noise".as_ptr(),
        Some(tone_mode_get),
        Some(tone_mode_set),
    );

    TP_SOUND = value::new_type(module, c"Sound", Some(drop_sound));
    let sound_type = ffi::py_tpobject(TP_SOUND);
    ffi::py_bind(sound_type, c"__new__(cls)".as_ptr(), Some(sound_new));
    ffi::py_bind(sound_type, c"__init__(self)".as_ptr(), Some(noop_init));
    ffi::py_bindproperty(
        TP_SOUND,
        c"notes".as_ptr(),
        Some(sequences::sound_notes_get),
        None,
    );
    ffi::py_bindproperty(
        TP_SOUND,
        c"tones".as_ptr(),
        Some(sequences::sound_tones_get),
        None,
    );
    ffi::py_bindproperty(
        TP_SOUND,
        c"volumes".as_ptr(),
        Some(sequences::sound_volumes_get),
        None,
    );
    ffi::py_bindproperty(
        TP_SOUND,
        c"effects".as_ptr(),
        Some(sequences::sound_effects_get),
        None,
    );
    ffi::py_bindproperty(
        TP_SOUND,
        c"speed".as_ptr(),
        Some(sound_speed_get),
        Some(sound_speed_set),
    );
    ffi::py_bind(
        sound_type,
        c"set(self, notes=None, tones=None, volumes=None, effects=None, speed=None)".as_ptr(),
        Some(sound_set),
    );
    ffi::py_bindmethod(TP_SOUND, c"set_notes".as_ptr(), Some(sound_set_notes));
    ffi::py_bindmethod(TP_SOUND, c"set_tones".as_ptr(), Some(sound_set_tones));
    ffi::py_bindmethod(TP_SOUND, c"set_volumes".as_ptr(), Some(sound_set_volumes));
    ffi::py_bindmethod(TP_SOUND, c"set_effects".as_ptr(), Some(sound_set_effects));
    ffi::py_bind(
        sound_type,
        c"mml(self, code=None)".as_ptr(),
        Some(sound_mml),
    );
    ffi::py_bind(
        sound_type,
        c"old_mml(self, code=None)".as_ptr(),
        Some(sound_old_mml),
    );
    ffi::py_bind(
        sound_type,
        c"pcm(self, filename=None)".as_ptr(),
        Some(sound_pcm),
    );
    ffi::py_bind(
        sound_type,
        c"save(self, filename, sec, ffmpeg=None)".as_ptr(),
        Some(sound_save),
    );
    ffi::py_bindmethod(TP_SOUND, c"total_sec".as_ptr(), Some(sound_total_sec));

    sequences::register_music_seq_types(module);
    TP_MUSIC = value::new_type(module, c"Music", Some(drop_music));
    let music_type = ffi::py_tpobject(TP_MUSIC);
    ffi::py_bind(music_type, c"__new__(cls)".as_ptr(), Some(music_new));
    ffi::py_bind(music_type, c"__init__(self)".as_ptr(), Some(noop_init));
    ffi::py_bindproperty(
        TP_MUSIC,
        c"seqs".as_ptr(),
        Some(sequences::music_seqs_get),
        None,
    );
    ffi::py_bindproperty(
        TP_MUSIC,
        c"snds_list".as_ptr(),
        Some(sequences::music_seqs_get),
        None,
    );
    ffi::py_bindmethod(TP_MUSIC, c"set".as_ptr(), Some(music_set));
    ffi::py_bind(
        music_type,
        c"save(self, filename, sec, ffmpeg=None)".as_ptr(),
        Some(music_save),
    );
}
