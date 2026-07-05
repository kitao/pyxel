use super::{
    ffi, make_channel, make_image, make_music, make_sound, make_tilemap, make_tone,
    normalize_index, normalize_insert_index, userdata, value, TP_CHANNELS, TP_COLORS, TP_IMAGES,
    TP_MUSICS, TP_SOUNDS, TP_TILEMAPS, TP_TONES,
};

// Global collections

unsafe extern "C" fn colors_len(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    value::return_int(pyxel::colors().len() as i64);
    true
}

unsafe extern "C" fn colors_getitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let colors = pyxel::colors();
    let Some(index) = normalize_index(value::int_arg(argv, 1), colors.len()) else {
        return value::raise_index_error("color index out of range");
    };
    value::return_int(colors[index] as i64);
    true
}

unsafe extern "C" fn colors_setitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let colors = pyxel::colors();
    let Some(index) = normalize_index(value::int_arg(argv, 1), colors.len()) else {
        return value::raise_index_error("color index out of range");
    };
    colors[index] = value::int_arg(argv, 2) as pyxel::Rgb24;
    value::return_none();
    true
}

unsafe extern "C" fn colors_delitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let colors = pyxel::colors();
    let Some(index) = normalize_index(value::int_arg(argv, 1), colors.len()) else {
        return value::raise_index_error("color index out of range");
    };
    colors.remove(index);
    value::return_none();
    true
}

unsafe extern "C" fn colors_iter(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    ffi::py_newlist(ffi::py_retval());
    for color in pyxel::colors() {
        ffi::py_newint(ffi::py_list_emplace(ffi::py_retval()), *color as i64);
    }
    ffi::py_iter(ffi::py_retval());
    true
}

unsafe extern "C" fn colors_contains(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let color = value::int_arg(argv, 1) as pyxel::Rgb24;
    value::return_bool(pyxel::colors().contains(&color));
    true
}

unsafe extern "C" fn colors_bool(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    value::return_bool(!pyxel::colors().is_empty());
    true
}

unsafe extern "C" fn colors_append(_argc: i32, argv: ffi::py_StackRef) -> bool {
    pyxel::colors().push(value::int_arg(argv, 1) as pyxel::Rgb24);
    value::return_none();
    true
}

unsafe extern "C" fn colors_extend(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let values = value::arg(argv, 1);
    for i in 0..ffi::py_list_len(values) {
        pyxel::colors().push(ffi::py_toint(ffi::py_list_getitem(values, i)) as pyxel::Rgb24);
    }
    value::return_none();
    true
}

unsafe extern "C" fn colors_insert(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let index = normalize_insert_index(value::int_arg(argv, 1), pyxel::colors().len());
    pyxel::colors().insert(index, value::int_arg(argv, 2) as pyxel::Rgb24);
    value::return_none();
    true
}

unsafe extern "C" fn colors_pop(argc: i32, argv: ffi::py_StackRef) -> bool {
    let colors = pyxel::colors();
    if colors.is_empty() {
        return value::raise_index_error("pop from empty sequence");
    }
    let index = if argc <= 1 || value::is_none(value::arg(argv, 1)) {
        -1
    } else {
        value::int_arg(argv, 1)
    };
    let Some(index) = normalize_index(index, colors.len()) else {
        return value::raise_index_error("pop index out of range");
    };
    value::return_int(colors.remove(index) as i64);
    true
}

unsafe extern "C" fn colors_clear(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    pyxel::colors().clear();
    value::return_none();
    true
}

macro_rules! collection_fns {
    (
        $getitem:ident,
        $setitem:ident,
        $delitem:ident,
        $len:ident,
        $iter:ident,
        $bool_fn:ident,
        $append:ident,
        $extend:ident,
        $insert:ident,
        $pop:ident,
        $clear:ident,
        $global:expr,
        $make:ident,
        $type:ty
    ) => {
        unsafe extern "C" fn $getitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
            let items = $global();
            let Some(index) = normalize_index(value::int_arg(argv, 1), items.len()) else {
                return value::raise_index_error("collection index out of range");
            };
            $make(ffi::py_retval(), items[index].clone());
            true
        }

        unsafe extern "C" fn $setitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
            let items = $global();
            let Some(index) = normalize_index(value::int_arg(argv, 1), items.len()) else {
                return value::raise_index_error("collection index out of range");
            };
            items[index] = userdata::<$type>(value::arg(argv, 2)).clone();
            value::return_none();
            true
        }

        unsafe extern "C" fn $delitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
            let items = $global();
            let Some(index) = normalize_index(value::int_arg(argv, 1), items.len()) else {
                return value::raise_index_error("collection index out of range");
            };
            items.remove(index);
            value::return_none();
            true
        }

        unsafe extern "C" fn $len(_argc: i32, _argv: ffi::py_StackRef) -> bool {
            value::return_int($global().len() as i64);
            true
        }

        unsafe extern "C" fn $iter(_argc: i32, _argv: ffi::py_StackRef) -> bool {
            ffi::py_newlist(ffi::py_retval());
            for item in $global().iter() {
                $make(ffi::py_list_emplace(ffi::py_retval()), item.clone());
            }
            ffi::py_iter(ffi::py_retval());
            true
        }

        unsafe extern "C" fn $bool_fn(_argc: i32, _argv: ffi::py_StackRef) -> bool {
            value::return_bool(!$global().is_empty());
            true
        }

        unsafe extern "C" fn $append(_argc: i32, argv: ffi::py_StackRef) -> bool {
            $global().push(userdata::<$type>(value::arg(argv, 1)).clone());
            value::return_none();
            true
        }

        unsafe extern "C" fn $extend(_argc: i32, argv: ffi::py_StackRef) -> bool {
            let values = value::arg(argv, 1);
            for i in 0..ffi::py_list_len(values) {
                $global().push(userdata::<$type>(ffi::py_list_getitem(values, i)).clone());
            }
            value::return_none();
            true
        }

        unsafe extern "C" fn $insert(_argc: i32, argv: ffi::py_StackRef) -> bool {
            let index = normalize_insert_index(value::int_arg(argv, 1), $global().len());
            $global().insert(index, userdata::<$type>(value::arg(argv, 2)).clone());
            value::return_none();
            true
        }

        unsafe extern "C" fn $pop(argc: i32, argv: ffi::py_StackRef) -> bool {
            let items = $global();
            if items.is_empty() {
                return value::raise_index_error("pop from empty sequence");
            }
            let index = if argc <= 1 || value::is_none(value::arg(argv, 1)) {
                -1
            } else {
                value::int_arg(argv, 1)
            };
            let Some(index) = normalize_index(index, items.len()) else {
                return value::raise_index_error("pop index out of range");
            };
            $make(ffi::py_retval(), items.remove(index));
            true
        }

        unsafe extern "C" fn $clear(_argc: i32, _argv: ffi::py_StackRef) -> bool {
            $global().clear();
            value::return_none();
            true
        }
    };
}

collection_fns!(
    images_getitem,
    images_setitem,
    images_delitem,
    images_len,
    images_iter,
    images_bool,
    images_append,
    images_extend,
    images_insert,
    images_pop,
    images_clear,
    pyxel::images,
    make_image,
    pyxel::RcImage
);
collection_fns!(
    tilemaps_getitem,
    tilemaps_setitem,
    tilemaps_delitem,
    tilemaps_len,
    tilemaps_iter,
    tilemaps_bool,
    tilemaps_append,
    tilemaps_extend,
    tilemaps_insert,
    tilemaps_pop,
    tilemaps_clear,
    pyxel::tilemaps,
    make_tilemap,
    pyxel::RcTilemap
);
collection_fns!(
    channels_getitem,
    channels_setitem,
    channels_delitem,
    channels_len,
    channels_iter,
    channels_bool,
    channels_append,
    channels_extend,
    channels_insert,
    channels_pop,
    channels_clear,
    pyxel::channels,
    make_channel,
    pyxel::RcChannel
);
collection_fns!(
    tones_getitem,
    tones_setitem,
    tones_delitem,
    tones_len,
    tones_iter,
    tones_bool,
    tones_append,
    tones_extend,
    tones_insert,
    tones_pop,
    tones_clear,
    pyxel::tones,
    make_tone,
    pyxel::RcTone
);
collection_fns!(
    sounds_getitem,
    sounds_setitem,
    sounds_delitem,
    sounds_len,
    sounds_iter,
    sounds_bool,
    sounds_append,
    sounds_extend,
    sounds_insert,
    sounds_pop,
    sounds_clear,
    pyxel::sounds,
    make_sound,
    pyxel::RcSound
);
collection_fns!(
    musics_getitem,
    musics_setitem,
    musics_delitem,
    musics_len,
    musics_iter,
    musics_bool,
    musics_append,
    musics_extend,
    musics_insert,
    musics_pop,
    musics_clear,
    pyxel::musics,
    make_music,
    pyxel::RcMusic
);

unsafe fn register_collection_type(
    type_: ffi::py_Type,
    getitem: ffi::py_CFunction,
    setitem: ffi::py_CFunction,
    delitem: ffi::py_CFunction,
    len: ffi::py_CFunction,
    iter: ffi::py_CFunction,
    bool_fn: ffi::py_CFunction,
    append: ffi::py_CFunction,
    extend: ffi::py_CFunction,
    insert: ffi::py_CFunction,
    pop: ffi::py_CFunction,
    clear: ffi::py_CFunction,
) {
    value::bind_magic(type_, c"__getitem__", getitem);
    value::bind_magic(type_, c"__setitem__", setitem);
    value::bind_magic(type_, c"__delitem__", delitem);
    value::bind_magic(type_, c"__len__", len);
    value::bind_magic(type_, c"__iter__", iter);
    value::bind_magic(type_, c"__bool__", bool_fn);
    ffi::py_bindmethod(type_, c"append".as_ptr(), append);
    ffi::py_bindmethod(type_, c"extend".as_ptr(), extend);
    ffi::py_bindmethod(type_, c"insert".as_ptr(), insert);
    ffi::py_bindmethod(type_, c"clear".as_ptr(), clear);
    ffi::py_bind(
        ffi::py_tpobject(type_),
        c"pop(self, index=None)".as_ptr(),
        pop,
    );
}

pub(super) unsafe fn register(module: ffi::py_GlobalRef) {
    TP_COLORS = value::new_type(module, c"Colors", None);
    value::bind_magic(TP_COLORS, c"__len__", Some(colors_len));
    value::bind_magic(TP_COLORS, c"__getitem__", Some(colors_getitem));
    value::bind_magic(TP_COLORS, c"__setitem__", Some(colors_setitem));
    value::bind_magic(TP_COLORS, c"__delitem__", Some(colors_delitem));
    value::bind_magic(TP_COLORS, c"__iter__", Some(colors_iter));
    value::bind_magic(TP_COLORS, c"__contains__", Some(colors_contains));
    value::bind_magic(TP_COLORS, c"__bool__", Some(colors_bool));
    ffi::py_bindmethod(TP_COLORS, c"append".as_ptr(), Some(colors_append));
    ffi::py_bindmethod(TP_COLORS, c"extend".as_ptr(), Some(colors_extend));
    ffi::py_bindmethod(TP_COLORS, c"insert".as_ptr(), Some(colors_insert));
    ffi::py_bindmethod(TP_COLORS, c"clear".as_ptr(), Some(colors_clear));
    ffi::py_bind(
        ffi::py_tpobject(TP_COLORS),
        c"pop(self, index=None)".as_ptr(),
        Some(colors_pop),
    );

    TP_IMAGES = value::new_type(module, c"Images", None);
    register_collection_type(
        TP_IMAGES,
        Some(images_getitem),
        Some(images_setitem),
        Some(images_delitem),
        Some(images_len),
        Some(images_iter),
        Some(images_bool),
        Some(images_append),
        Some(images_extend),
        Some(images_insert),
        Some(images_pop),
        Some(images_clear),
    );
    TP_TILEMAPS = value::new_type(module, c"Tilemaps", None);
    register_collection_type(
        TP_TILEMAPS,
        Some(tilemaps_getitem),
        Some(tilemaps_setitem),
        Some(tilemaps_delitem),
        Some(tilemaps_len),
        Some(tilemaps_iter),
        Some(tilemaps_bool),
        Some(tilemaps_append),
        Some(tilemaps_extend),
        Some(tilemaps_insert),
        Some(tilemaps_pop),
        Some(tilemaps_clear),
    );
    TP_CHANNELS = value::new_type(module, c"Channels", None);
    register_collection_type(
        TP_CHANNELS,
        Some(channels_getitem),
        Some(channels_setitem),
        Some(channels_delitem),
        Some(channels_len),
        Some(channels_iter),
        Some(channels_bool),
        Some(channels_append),
        Some(channels_extend),
        Some(channels_insert),
        Some(channels_pop),
        Some(channels_clear),
    );
    TP_TONES = value::new_type(module, c"Tones", None);
    register_collection_type(
        TP_TONES,
        Some(tones_getitem),
        Some(tones_setitem),
        Some(tones_delitem),
        Some(tones_len),
        Some(tones_iter),
        Some(tones_bool),
        Some(tones_append),
        Some(tones_extend),
        Some(tones_insert),
        Some(tones_pop),
        Some(tones_clear),
    );
    TP_SOUNDS = value::new_type(module, c"Sounds", None);
    register_collection_type(
        TP_SOUNDS,
        Some(sounds_getitem),
        Some(sounds_setitem),
        Some(sounds_delitem),
        Some(sounds_len),
        Some(sounds_iter),
        Some(sounds_bool),
        Some(sounds_append),
        Some(sounds_extend),
        Some(sounds_insert),
        Some(sounds_pop),
        Some(sounds_clear),
    );
    TP_MUSICS = value::new_type(module, c"Musics", None);
    register_collection_type(
        TP_MUSICS,
        Some(musics_getitem),
        Some(musics_setitem),
        Some(musics_delitem),
        Some(musics_len),
        Some(musics_iter),
        Some(musics_bool),
        Some(musics_append),
        Some(musics_extend),
        Some(musics_insert),
        Some(musics_pop),
        Some(musics_clear),
    );

    value::set_module_object(module, c"colors", TP_COLORS);
    value::set_module_object(module, c"images", TP_IMAGES);
    value::set_module_object(module, c"tilemaps", TP_TILEMAPS);
    value::set_module_object(module, c"channels", TP_CHANNELS);
    value::set_module_object(module, c"tones", TP_TONES);
    value::set_module_object(module, c"sounds", TP_SOUNDS);
    value::set_module_object(module, c"musics", TP_MUSICS);
}
