use crate::ffi;
use crate::helpers::*;

pub static mut TP_CHANNEL: ffi::py_Type = 0;
pub static mut TP_CHANNELS: ffi::py_Type = 0;

unsafe fn ch(argv: ffi::py_StackRef) -> &'static mut pyxel::Channel {
    &mut *(*(ffi::py_touserdata(arg(argv, 0)) as *mut *mut pyxel::Channel))
}

pub unsafe fn new_channel_obj(out: ffi::py_OutRef, ptr: *mut pyxel::Channel) {
    let ud = ffi::py_newobject(out, TP_CHANNEL, 0, size_of::<*mut pyxel::Channel>() as i32);
    *(ud as *mut *mut pyxel::Channel) = ptr;
}

unsafe extern "C" fn channel_gain_getter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_float(ch(argv).gain as f64);
    true
}

unsafe extern "C" fn channel_gain_setter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ch(argv).gain = arg_float(argv, 1) as f32;
    ret_none();
    true
}

unsafe extern "C" fn channel_detune_getter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ret_float(ch(argv).detune as f64);
    true
}

unsafe extern "C" fn channel_detune_setter(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ch(argv).detune = arg_int(argv, 1) as i32;
    ret_none();
    true
}

unsafe extern "C" fn channel_play(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let snd_ref = arg(argv, 1);
    let sec = arg_opt_float(argv, 2).map(|v| v as f32);
    let r#loop = arg_opt_bool(argv, 3).unwrap_or(false);
    let resume = arg_opt_bool(argv, 4).unwrap_or(false);

    if is_int(snd_ref) {
        let snd_idx = ffi::py_toint(snd_ref) as usize;
        let sound_ptr = pyxel::sounds()[snd_idx];
        ch(argv).play_sound(sound_ptr, sec, r#loop, resume);
    } else if is_list(snd_ref) {
        let len = ffi::py_list_len(snd_ref);
        let sounds: Vec<*mut pyxel::Sound> = (0..len)
            .map(|i| {
                let idx = ffi::py_toint(ffi::py_list_getitem(snd_ref, i)) as usize;
                pyxel::sounds()[idx]
            })
            .collect();
        ch(argv).play(sounds, sec, r#loop, resume);
    } else if is_str(snd_ref) {
        let mml = arg_str(argv, 1);
        if let Err(e) = ch(argv).play_mml(mml, sec, r#loop, resume) {
            return raise_exc(&e);
        }
    }
    ret_none();
    true
}

unsafe extern "C" fn channel_stop(_argc: i32, argv: ffi::py_StackRef) -> bool {
    ch(argv).stop();
    ret_none();
    true
}

unsafe extern "C" fn channel_play_pos(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match ch(argv).play_position() {
        Some((snd, sec)) => {
            let out = ffi::py_newtuple(ffi::py_retval(), 2);
            ffi::py_newint(out.add(0), snd as i64);
            ffi::py_newfloat(out.add(1), sec as f64);
        }
        None => ret_none(),
    }
    true
}

unsafe extern "C" fn channel_new(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let cls = ffi::py_totype(arg(argv, 0));
    let ptr = pyxel::Channel::new();
    let ud = ffi::py_newobject(ffi::py_retval(), cls, 0, size_of::<*mut pyxel::Channel>() as i32);
    *(ud as *mut *mut pyxel::Channel) = ptr;
    true
}

pub unsafe fn add_channel_class(m: ffi::py_GlobalRef) {
    TP_CHANNEL = new_type(c"Channel", m);
    ffi::py_bindproperty(TP_CHANNEL, c"gain".as_ptr(), Some(channel_gain_getter), Some(channel_gain_setter));
    ffi::py_bindproperty(TP_CHANNEL, c"detune".as_ptr(), Some(channel_detune_getter), Some(channel_detune_setter));
    bindfunc(ffi::py_tpobject(TP_CHANNEL), c"stop", Some(channel_stop));
    bindfunc(ffi::py_tpobject(TP_CHANNEL), c"play_pos", Some(channel_play_pos));

    let tp_obj = ffi::py_tpobject(TP_CHANNEL);
    bind(tp_obj, c"play(self, snd, sec=None, loop=None, resume=None)", Some(channel_play));
    bind(tp_obj, c"__new__(cls)", Some(channel_new));

    // Channels collection
    impl_object_collection!(pyxel::channels, new_channel_obj, channels_getitem, channels_setitem, channels_len, channels_iter);
    register_collection!(TP_CHANNELS, c"Channels", m, channels_getitem, channels_setitem, channels_len, channels_iter);
}
