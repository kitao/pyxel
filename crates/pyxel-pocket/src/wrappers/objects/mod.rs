use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::mem::size_of;
use std::rc::Rc;

use crate::{ffi, value};

mod audio;
mod collections;
mod font;
mod image;
mod sequences;
mod tilemap;

static mut TP_FONT: ffi::py_Type = 0;
static mut TP_IMAGE: ffi::py_Type = 0;
static mut TP_TILEMAP: ffi::py_Type = 0;
static mut TP_CHANNEL: ffi::py_Type = 0;
static mut TP_TONE: ffi::py_Type = 0;
static mut TP_SOUND: ffi::py_Type = 0;
static mut TP_MUSIC: ffi::py_Type = 0;

static mut TP_COLORS: ffi::py_Type = 0;
static mut TP_IMAGES: ffi::py_Type = 0;
static mut TP_TILEMAPS: ffi::py_Type = 0;
static mut TP_CHANNELS: ffi::py_Type = 0;
static mut TP_TONES: ffi::py_Type = 0;
static mut TP_SOUNDS: ffi::py_Type = 0;
static mut TP_MUSICS: ffi::py_Type = 0;

static mut TP_INT_SEQ: ffi::py_Type = 0;
static mut TP_MUSIC_SEQS: ffi::py_Type = 0;
static mut TP_MUSIC_SEQ: ffi::py_Type = 0;

#[derive(Clone)]
enum IntSeq {
    ToneWavetable(pyxel::RcTone),
    SoundNotes(pyxel::RcSound),
    SoundTones(pyxel::RcSound),
    SoundVolumes(pyxel::RcSound),
    SoundEffects(pyxel::RcSound),
}

#[derive(Clone)]
struct MusicSeqs {
    music: pyxel::RcMusic,
}

#[derive(Clone)]
struct MusicSeq {
    music: pyxel::RcMusic,
    index: usize,
}

macro_rules! drop_userdata {
    ($name:ident, $type:ty) => {
        unsafe extern "C" fn $name(ptr: *mut c_void) {
            std::ptr::drop_in_place(ptr.cast::<$type>());
        }
    };
}

drop_userdata!(drop_font, pyxel::RcFont);
drop_userdata!(drop_image, pyxel::RcImage);
drop_userdata!(drop_tilemap, pyxel::RcTilemap);
drop_userdata!(drop_channel, pyxel::RcChannel);
drop_userdata!(drop_tone, pyxel::RcTone);
drop_userdata!(drop_sound, pyxel::RcSound);
drop_userdata!(drop_music, pyxel::RcMusic);
drop_userdata!(drop_int_seq, IntSeq);
drop_userdata!(drop_music_seqs, MusicSeqs);
drop_userdata!(drop_music_seq, MusicSeq);

unsafe fn rc_ref<T>(rc: &Rc<UnsafeCell<T>>) -> &T {
    &*rc.get()
}

// PocketPy methods mutate shared engine resources through C ABI receivers.
#[allow(clippy::mut_from_ref)]
pub(crate) unsafe fn rc_mut<T>(rc: &Rc<UnsafeCell<T>>) -> &mut T {
    &mut *rc.get()
}

unsafe fn new_userdata<T>(out: ffi::py_OutRef, type_: ffi::py_Type, value: T) {
    let userdata = ffi::py_newobject(out, type_, 0, size_of::<T>() as i32);
    userdata.cast::<T>().write(value);
}

unsafe fn userdata<T>(object: ffi::py_Ref) -> &'static T {
    &*ffi::py_touserdata(object).cast::<T>()
}

unsafe fn image_from_ref(object: ffi::py_Ref) -> pyxel::RcImage {
    userdata::<pyxel::RcImage>(object).clone()
}

unsafe fn tilemap_from_ref(object: ffi::py_Ref) -> pyxel::RcTilemap {
    userdata::<pyxel::RcTilemap>(object).clone()
}

unsafe fn sound_from_ref(object: ffi::py_Ref) -> pyxel::RcSound {
    userdata::<pyxel::RcSound>(object).clone()
}

unsafe fn font_from_ref(object: ffi::py_Ref) -> pyxel::RcFont {
    userdata::<pyxel::RcFont>(object).clone()
}

unsafe fn normalize_index(index: i64, len: usize) -> Option<usize> {
    let index = if index < 0 { index + len as i64 } else { index };
    if index < 0 || index as usize >= len {
        None
    } else {
        Some(index as usize)
    }
}

unsafe fn normalize_insert_index(index: i64, len: usize) -> usize {
    if index < 0 {
        (index + len as i64).clamp(0, len as i64) as usize
    } else {
        (index as usize).min(len)
    }
}

unsafe fn int_list_from_ref(object: ffi::py_Ref) -> Vec<u32> {
    (0..sequence_len(object))
        .map(|i| ffi::py_toint(sequence_getitem(object, i)) as u32)
        .collect()
}

unsafe fn int_nested_list_from_ref(object: ffi::py_Ref) -> Vec<Vec<u32>> {
    (0..sequence_len(object))
        .map(|i| int_list_from_ref(sequence_getitem(object, i)))
        .collect()
}

unsafe fn make_int_list(out: ffi::py_OutRef, values: &[u32]) {
    ffi::py_newlist(out);
    for value in values {
        ffi::py_newint(ffi::py_list_emplace(out), *value as i64);
    }
}

unsafe fn return_int_list(values: &[u32]) {
    make_int_list(ffi::py_retval(), values);
}

unsafe fn str_list_from_arg(argv: ffi::py_StackRef, index: usize) -> Vec<String> {
    let sequence = value::arg(argv, index);
    (0..sequence_len(sequence))
        .map(|i| {
            let sv = ffi::py_tosv(sequence_getitem(sequence, i));
            let bytes = std::slice::from_raw_parts(sv.data.cast::<u8>(), sv.size as usize);
            String::from_utf8_lossy(bytes).into_owned()
        })
        .collect()
}

unsafe fn sequence_len(object: ffi::py_Ref) -> i32 {
    if value::is_tuple(object) {
        ffi::py_tuple_len(object)
    } else {
        ffi::py_list_len(object)
    }
}

unsafe fn sequence_getitem(object: ffi::py_Ref, index: i32) -> ffi::py_Ref {
    if value::is_tuple(object) {
        ffi::py_tuple_getitem(object, index)
    } else {
        ffi::py_list_getitem(object, index)
    }
}

unsafe fn tile_from_ref(object: ffi::py_Ref) -> pyxel::Tile {
    if value::is_tuple(object) {
        (
            ffi::py_toint(ffi::py_tuple_getitem(object, 0)) as pyxel::ImageTileCoord,
            ffi::py_toint(ffi::py_tuple_getitem(object, 1)) as pyxel::ImageTileCoord,
        )
    } else {
        let value = ffi::py_toint(object);
        (
            ((value >> 16) & 0xffff) as pyxel::ImageTileCoord,
            (value & 0xffff) as pyxel::ImageTileCoord,
        )
    }
}

unsafe fn tile_arg(argv: ffi::py_StackRef, index: usize) -> pyxel::Tile {
    tile_from_ref(value::arg(argv, index))
}

unsafe fn farg(argv: ffi::py_StackRef, index: usize) -> f32 {
    value::float_arg(argv, index).unwrap_or(0.0)
}

unsafe fn color_arg(argv: ffi::py_StackRef, index: usize) -> pyxel::Color {
    value::int_arg(argv, index) as pyxel::Color
}

pub(crate) unsafe fn image_arg(object: ffi::py_Ref) -> Option<pyxel::RcImage> {
    if value::is_int(object) {
        pyxel::images().get(ffi::py_toint(object) as usize).cloned()
    } else if ffi::py_isinstance(object, TP_IMAGE) {
        Some(image_from_ref(object))
    } else {
        None
    }
}

pub(crate) unsafe fn tilemap_arg(object: ffi::py_Ref) -> Option<pyxel::RcTilemap> {
    if value::is_int(object) {
        pyxel::tilemaps()
            .get(ffi::py_toint(object) as usize)
            .cloned()
    } else if ffi::py_isinstance(object, TP_TILEMAP) {
        Some(tilemap_from_ref(object))
    } else {
        None
    }
}

pub(crate) unsafe fn font_arg(object: ffi::py_Ref) -> Option<pyxel::RcFont> {
    if ffi::py_isinstance(object, TP_FONT) {
        Some(font_from_ref(object))
    } else {
        None
    }
}

pub(crate) unsafe fn sound_arg(object: ffi::py_Ref) -> Option<pyxel::RcSound> {
    if value::is_int(object) {
        pyxel::sounds().get(ffi::py_toint(object) as usize).cloned()
    } else if ffi::py_isinstance(object, TP_SOUND) {
        Some(sound_from_ref(object))
    } else {
        None
    }
}

pub(crate) unsafe fn sound_list_arg(object: ffi::py_Ref) -> Option<Vec<pyxel::RcSound>> {
    if ffi::py_isinstance(object, TP_SOUND) || value::is_int(object) {
        return sound_arg(object).map(|sound| vec![sound]);
    }

    if !value::is_list(object) {
        return None;
    }

    let mut sounds = Vec::new();
    for i in 0..ffi::py_list_len(object) {
        let item = ffi::py_list_getitem(object, i);
        sounds.push(sound_arg(item)?);
    }
    Some(sounds)
}

unsafe fn make_image(out: ffi::py_OutRef, image: pyxel::RcImage) {
    new_userdata(out, TP_IMAGE, image);
}

unsafe fn make_tilemap(out: ffi::py_OutRef, tilemap: pyxel::RcTilemap) {
    new_userdata(out, TP_TILEMAP, tilemap);
}

unsafe fn make_channel(out: ffi::py_OutRef, channel: pyxel::RcChannel) {
    new_userdata(out, TP_CHANNEL, channel);
}

unsafe fn make_tone(out: ffi::py_OutRef, tone: pyxel::RcTone) {
    new_userdata(out, TP_TONE, tone);
}

unsafe fn make_sound(out: ffi::py_OutRef, sound: pyxel::RcSound) {
    new_userdata(out, TP_SOUND, sound);
}

unsafe fn make_music(out: ffi::py_OutRef, music: pyxel::RcMusic) {
    new_userdata(out, TP_MUSIC, music);
}

unsafe fn set_global_object<T>(
    module: ffi::py_GlobalRef,
    name: &std::ffi::CStr,
    type_: ffi::py_Type,
    value: T,
) {
    new_userdata(
        ffi::py_emplacedict(module, ffi::py_name(name.as_ptr())),
        type_,
        value,
    );
}

unsafe extern "C" fn noop_init(_argc: i32, _argv: ffi::py_StackRef) -> bool {
    value::return_none();
    true
}

pub unsafe fn register(module: ffi::py_GlobalRef) {
    font::register(module);
    image::register(module);
    tilemap::register(module);
    audio::register(module);
    collections::register(module);

    set_global_object(module, c"screen", TP_IMAGE, pyxel::screen().clone());
    set_global_object(module, c"cursor", TP_IMAGE, pyxel::cursor_image().clone());
    set_global_object(module, c"font", TP_IMAGE, pyxel::font_image().clone());
}
