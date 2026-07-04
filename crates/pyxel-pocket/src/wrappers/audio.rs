use crate::{ffi, value};

unsafe extern "C" fn play(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let channel = value::int_arg(argv, 0) as u32;
    let sound_arg = value::arg(argv, 1);
    let start_sec = value::opt_int_arg(argv, 5)
        .map(|tick| tick as f32 / 120.0)
        .or_else(|| value::opt_float_arg(argv, 2));
    let should_loop = value::opt_bool_arg(argv, 3).unwrap_or(false);
    let should_resume = value::opt_bool_arg(argv, 4).unwrap_or(false);

    if ffi::py_istype(sound_arg, ffi::py_PredefinedTypes_tp_list as ffi::py_Type) {
        pyxel::pyxel().play(
            channel,
            &value::int_list_arg(argv, 1),
            start_sec,
            should_loop,
            should_resume,
        );
    } else if ffi::py_istype(sound_arg, ffi::py_PredefinedTypes_tp_str as ffi::py_Type) {
        if let Err(err) = pyxel::pyxel().play_mml(
            channel,
            &value::str_arg(argv, 1),
            start_sec,
            should_loop,
            should_resume,
        ) {
            return value::raise_exception(&err);
        }
    } else {
        pyxel::pyxel().play_sound(
            channel,
            value::int_arg(argv, 1) as u32,
            start_sec,
            should_loop,
            should_resume,
        );
    }

    value::return_none();
    true
}

unsafe extern "C" fn playm(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let start_sec = value::opt_int_arg(argv, 3)
        .map(|tick| tick as f32 / 120.0)
        .or_else(|| value::opt_float_arg(argv, 1));
    pyxel::pyxel().play_music(
        value::int_arg(argv, 0) as u32,
        start_sec,
        value::opt_bool_arg(argv, 2).unwrap_or(false),
    );
    value::return_none();
    true
}

unsafe extern "C" fn stop(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if let Some(channel) = value::opt_int_arg(argv, 0) {
        pyxel::pyxel().stop_channel(channel as u32);
    } else {
        pyxel::pyxel().stop_all_channels();
    }
    value::return_none();
    true
}

unsafe extern "C" fn play_pos(_argc: i32, argv: ffi::py_StackRef) -> bool {
    value::return_optional_play_pos(pyxel::pyxel().play_position(value::int_arg(argv, 0) as u32));
    true
}

unsafe extern "C" fn gen_bgm(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let mml = pyxel::pyxel().gen_bgm(
        value::int_arg(argv, 0) as i32,
        value::int_arg(argv, 1) as i32,
        value::int_arg(argv, 2) as i32,
        value::int_arg(argv, 3) as u64,
        value::opt_bool_arg(argv, 4),
    );
    value::return_str_list(&mml);
    true
}

pub unsafe fn add_functions(module: ffi::py_GlobalRef) {
    ffi::py_bind(
        module,
        c"play(ch, snd, sec=None, loop=None, resume=None, tick=None)".as_ptr(),
        Some(play),
    );
    ffi::py_bind(
        module,
        c"playm(msc, sec=None, loop=None, tick=None)".as_ptr(),
        Some(playm),
    );
    ffi::py_bind(module, c"stop(ch=None)".as_ptr(), Some(stop));
    ffi::py_bind(module, c"play_pos(ch)".as_ptr(), Some(play_pos));
    ffi::py_bind(
        module,
        c"gen_bgm(preset, transp, instr, seed, play=None)".as_ptr(),
        Some(gen_bgm),
    );
}
