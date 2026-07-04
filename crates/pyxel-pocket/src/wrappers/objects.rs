use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::mem::size_of;
use std::rc::Rc;

use crate::{ffi, value};

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

unsafe fn rc_mut<T>(rc: &Rc<UnsafeCell<T>>) -> &mut T {
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
    (0..ffi::py_list_len(object))
        .map(|i| ffi::py_toint(ffi::py_list_getitem(object, i)) as u32)
        .collect()
}

unsafe fn int_nested_list_from_ref(object: ffi::py_Ref) -> Vec<Vec<u32>> {
    (0..ffi::py_list_len(object))
        .map(|i| int_list_from_ref(ffi::py_list_getitem(object, i)))
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
    let list = value::arg(argv, index);
    (0..ffi::py_list_len(list))
        .map(|i| {
            let sv = ffi::py_tosv(ffi::py_list_getitem(list, i));
            let bytes = std::slice::from_raw_parts(sv.data.cast::<u8>(), sv.size as usize);
            String::from_utf8_lossy(bytes).into_owned()
        })
        .collect()
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

unsafe fn image_arg(object: ffi::py_Ref) -> Option<pyxel::RcImage> {
    if value::is_int(object) {
        pyxel::images().get(ffi::py_toint(object) as usize).cloned()
    } else if ffi::py_isinstance(object, TP_IMAGE) {
        Some(image_from_ref(object))
    } else {
        None
    }
}

unsafe fn tilemap_arg(object: ffi::py_Ref) -> Option<pyxel::RcTilemap> {
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

unsafe fn sound_arg(object: ffi::py_Ref) -> Option<pyxel::RcSound> {
    if value::is_int(object) {
        pyxel::sounds().get(ffi::py_toint(object) as usize).cloned()
    } else if ffi::py_isinstance(object, TP_SOUND) {
        Some(sound_from_ref(object))
    } else {
        None
    }
}

unsafe fn sound_list_arg(object: ffi::py_Ref) -> Option<Vec<pyxel::RcSound>> {
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

// Font

unsafe extern "C" fn font_new(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match pyxel::Font::new(&value::str_arg(argv, 1), value::opt_float_arg(argv, 2)) {
        Ok(font) => {
            let cls = ffi::py_totype(value::arg(argv, 0));
            new_userdata(ffi::py_retval(), cls, font);
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn font_text_width(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let font = userdata::<pyxel::RcFont>(value::arg(argv, 0));
    value::return_int(rc_mut(font).text_width(&value::str_arg(argv, 1)) as i64);
    true
}

// Image

unsafe extern "C" fn image_new(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let cls = ffi::py_totype(value::arg(argv, 0));
    new_userdata(
        ffi::py_retval(),
        cls,
        pyxel::Image::new(
            value::int_arg(argv, 1) as u32,
            value::int_arg(argv, 2) as u32,
        ),
    );
    true
}

unsafe extern "C" fn image_from_image(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match pyxel::Image::from_image(&value::str_arg(argv, 0), value::opt_bool_arg(argv, 1)) {
        Ok(image) => {
            make_image(ffi::py_retval(), image);
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn image_width(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    value::return_int(rc_ref(image).width() as i64);
    true
}

unsafe extern "C" fn image_height(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    value::return_int(rc_ref(image).height() as i64);
    true
}

unsafe extern "C" fn image_data_ptr(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    value::return_int(rc_mut(image).data_ptr() as usize as i64);
    true
}

unsafe extern "C" fn image_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    let data = str_list_from_arg(argv, 3);
    let data_refs = data.iter().map(String::as_str).collect::<Vec<_>>();
    rc_mut(image).set(
        value::int_arg(argv, 1) as i32,
        value::int_arg(argv, 2) as i32,
        &data_refs,
    );
    value::return_none();
    true
}

unsafe extern "C" fn image_load(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    match rc_mut(image).load(
        value::int_arg(argv, 1) as i32,
        value::int_arg(argv, 2) as i32,
        &value::str_arg(argv, 3),
        value::opt_bool_arg(argv, 4),
    ) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn image_save(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    match rc_ref(image).save(&value::str_arg(argv, 1), value::int_arg(argv, 2) as u32) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn image_clip(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    match (
        value::opt_float_arg(argv, 1),
        value::opt_float_arg(argv, 2),
        value::opt_float_arg(argv, 3),
        value::opt_float_arg(argv, 4),
    ) {
        (Some(x), Some(y), Some(width), Some(height)) => {
            rc_mut(image).set_clip_rect(x, y, width, height);
        }
        (None, None, None, None) => rc_mut(image).reset_clip_rect(),
        _ => return value::raise_exception("clip() takes 0 or 4 arguments"),
    }
    value::return_none();
    true
}

unsafe extern "C" fn image_camera(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    match (value::opt_float_arg(argv, 1), value::opt_float_arg(argv, 2)) {
        (Some(x), Some(y)) => rc_mut(image).set_camera(x, y),
        (None, None) => rc_mut(image).reset_camera(),
        _ => return value::raise_exception("camera() takes 0 or 2 arguments"),
    }
    value::return_none();
    true
}

unsafe extern "C" fn image_pal(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    match (value::opt_int_arg(argv, 1), value::opt_int_arg(argv, 2)) {
        (Some(src), Some(dst)) => rc_mut(image).map_color(src as pyxel::Color, dst as pyxel::Color),
        (None, None) => rc_mut(image).reset_color_map(),
        _ => return value::raise_exception("pal() takes 0 or 2 arguments"),
    }
    value::return_none();
    true
}

unsafe extern "C" fn image_dither(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(alpha) = value::float_arg(argv, 1) else {
        return false;
    };
    rc_mut(userdata::<pyxel::RcImage>(value::arg(argv, 0))).set_dithering(alpha);
    value::return_none();
    true
}

macro_rules! image_draw {
    ($name:ident, $method:ident, $($arg:expr),+ $(,)?) => {
        unsafe extern "C" fn $name(_argc: i32, argv: ffi::py_StackRef) -> bool {
            let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
            rc_mut(image).$method($($arg(argv)),+);
            value::return_none();
            true
        }
    };
}

unsafe fn farg(argv: ffi::py_StackRef, index: usize) -> f32 {
    value::float_arg(argv, index).unwrap_or(0.0)
}

unsafe fn color_arg(argv: ffi::py_StackRef, index: usize) -> pyxel::Color {
    value::int_arg(argv, index) as pyxel::Color
}

image_draw!(image_cls, clear, |argv| color_arg(argv, 1));
image_draw!(
    image_pset,
    set_pixel,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| color_arg(argv, 3)
);
image_draw!(
    image_line,
    draw_line,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| color_arg(argv, 5)
);
image_draw!(
    image_rect,
    draw_rect,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| color_arg(argv, 5)
);
image_draw!(
    image_rectb,
    draw_rect_border,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| color_arg(argv, 5)
);
image_draw!(
    image_circ,
    draw_circle,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| color_arg(argv, 4)
);
image_draw!(
    image_circb,
    draw_circle_border,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| color_arg(argv, 4)
);
image_draw!(
    image_elli,
    draw_ellipse,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| color_arg(argv, 5)
);
image_draw!(
    image_ellib,
    draw_ellipse_border,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| color_arg(argv, 5)
);
image_draw!(
    image_tri,
    draw_triangle,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| farg(argv, 5),
    |argv| farg(argv, 6),
    |argv| color_arg(argv, 7)
);
image_draw!(
    image_trib,
    draw_triangle_border,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| farg(argv, 5),
    |argv| farg(argv, 6),
    |argv| color_arg(argv, 7)
);
image_draw!(
    image_fill,
    flood_fill,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| color_arg(argv, 3)
);

unsafe extern "C" fn image_pget(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    value::return_int(rc_ref(image).pixel(farg(argv, 1), farg(argv, 2)) as i64);
    true
}

unsafe extern "C" fn image_blt(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(source) = image_arg(value::arg(argv, 3)) else {
        return value::raise_exception("Invalid image");
    };
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    rc_mut(image).draw_image(
        farg(argv, 1),
        farg(argv, 2),
        &source,
        farg(argv, 4),
        farg(argv, 5),
        farg(argv, 6),
        farg(argv, 7),
        value::opt_int_arg(argv, 8).map(|color| color as pyxel::Color),
        value::opt_float_arg(argv, 9),
        value::opt_float_arg(argv, 10),
    );
    value::return_none();
    true
}

unsafe extern "C" fn image_bltm(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(source) = tilemap_arg(value::arg(argv, 3)) else {
        return value::raise_exception("Invalid tilemap");
    };
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    rc_mut(image).draw_tilemap(
        farg(argv, 1),
        farg(argv, 2),
        &source,
        farg(argv, 4),
        farg(argv, 5),
        farg(argv, 6),
        farg(argv, 7),
        value::opt_int_arg(argv, 8).map(|color| color as pyxel::Color),
        value::opt_float_arg(argv, 9),
        value::opt_float_arg(argv, 10),
    );
    value::return_none();
    true
}

unsafe extern "C" fn image_blt3d(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(source) = image_arg(value::arg(argv, 5)) else {
        return value::raise_exception("Invalid image");
    };
    let Some(pos) = value::tuple3_float_arg(argv, 6) else {
        return false;
    };
    let Some(rot) = value::tuple3_float_arg(argv, 7) else {
        return false;
    };
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    rc_mut(image).draw_image_3d(
        farg(argv, 1),
        farg(argv, 2),
        farg(argv, 3),
        farg(argv, 4),
        &source,
        pos,
        rot,
        value::opt_float_arg(argv, 8),
        value::opt_int_arg(argv, 9).map(|color| color as pyxel::Color),
    );
    value::return_none();
    true
}

unsafe extern "C" fn image_bltm3d(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(source) = tilemap_arg(value::arg(argv, 5)) else {
        return value::raise_exception("Invalid tilemap");
    };
    let Some(pos) = value::tuple3_float_arg(argv, 6) else {
        return false;
    };
    let Some(rot) = value::tuple3_float_arg(argv, 7) else {
        return false;
    };
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    rc_mut(image).draw_tilemap_3d(
        farg(argv, 1),
        farg(argv, 2),
        farg(argv, 3),
        farg(argv, 4),
        &source,
        pos,
        rot,
        value::opt_float_arg(argv, 8),
        value::opt_int_arg(argv, 9).map(|color| color as pyxel::Color),
    );
    value::return_none();
    true
}

unsafe extern "C" fn image_text(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let font = if value::is_none(value::arg(argv, 5)) {
        None
    } else {
        Some(font_from_ref(value::arg(argv, 5)))
    };
    let image = userdata::<pyxel::RcImage>(value::arg(argv, 0));
    rc_mut(image).draw_text(
        farg(argv, 1),
        farg(argv, 2),
        &value::str_arg(argv, 3),
        color_arg(argv, 4),
        font.as_ref(),
    );
    value::return_none();
    true
}

// Tilemap

unsafe fn image_source_arg(object: ffi::py_Ref) -> Option<pyxel::ImageSource> {
    if value::is_int(object) {
        Some(pyxel::ImageSource::Index(ffi::py_toint(object) as u32))
    } else if ffi::py_isinstance(object, TP_IMAGE) {
        Some(pyxel::ImageSource::Image(image_from_ref(object)))
    } else {
        None
    }
}

unsafe extern "C" fn tilemap_new(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(imgsrc) = image_source_arg(value::arg(argv, 3)) else {
        return value::raise_exception("Invalid image source");
    };
    let cls = ffi::py_totype(value::arg(argv, 0));
    new_userdata(
        ffi::py_retval(),
        cls,
        pyxel::Tilemap::new(
            value::int_arg(argv, 1) as u32,
            value::int_arg(argv, 2) as u32,
            imgsrc,
        ),
    );
    true
}

unsafe extern "C" fn tilemap_from_tmx(_argc: i32, argv: ffi::py_StackRef) -> bool {
    match pyxel::Tilemap::from_tmx(&value::str_arg(argv, 0), value::int_arg(argv, 1) as u32) {
        Ok(tilemap) => {
            make_tilemap(ffi::py_retval(), tilemap);
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn tilemap_width(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    value::return_int(rc_ref(tilemap).width() as i64);
    true
}

unsafe extern "C" fn tilemap_height(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    value::return_int(rc_ref(tilemap).height() as i64);
    true
}

unsafe extern "C" fn tilemap_imgsrc_get(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    match &rc_ref(tilemap).imgsrc {
        pyxel::ImageSource::Index(index) => value::return_int(*index as i64),
        pyxel::ImageSource::Image(image) => make_image(ffi::py_retval(), image.clone()),
    }
    true
}

unsafe extern "C" fn tilemap_imgsrc_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(imgsrc) = image_source_arg(value::arg(argv, 1)) else {
        return value::raise_exception("Invalid image source");
    };
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    rc_mut(tilemap).imgsrc = imgsrc;
    value::return_none();
    true
}

unsafe extern "C" fn tilemap_image_get(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    let image = match &rc_ref(tilemap).imgsrc {
        pyxel::ImageSource::Index(index) => {
            let Some(image) = pyxel::images().get(*index as usize).cloned() else {
                return value::raise_exception("Invalid image index");
            };
            image
        }
        pyxel::ImageSource::Image(image) => image.clone(),
    };
    make_image(ffi::py_retval(), image);
    true
}

unsafe extern "C" fn tilemap_image_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    if !ffi::py_isinstance(value::arg(argv, 1), TP_IMAGE) {
        return value::raise_exception("Invalid image");
    }
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    rc_mut(tilemap).imgsrc = pyxel::ImageSource::Image(image_from_ref(value::arg(argv, 1)));
    value::return_none();
    true
}

unsafe extern "C" fn tilemap_refimg_get(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    match &rc_ref(tilemap).imgsrc {
        pyxel::ImageSource::Index(index) => value::return_int(*index as i64),
        pyxel::ImageSource::Image(_) => value::return_none(),
    }
    true
}

unsafe extern "C" fn tilemap_refimg_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    rc_mut(tilemap).imgsrc = pyxel::ImageSource::Index(value::int_arg(argv, 1) as u32);
    value::return_none();
    true
}

unsafe extern "C" fn tilemap_data_ptr(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    value::return_int(rc_mut(tilemap).data_ptr() as usize as i64);
    true
}

unsafe extern "C" fn tilemap_set(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    let data = str_list_from_arg(argv, 3);
    let data_refs = data.iter().map(String::as_str).collect::<Vec<_>>();
    rc_mut(tilemap).set(
        value::int_arg(argv, 1) as i32,
        value::int_arg(argv, 2) as i32,
        &data_refs,
    );
    value::return_none();
    true
}

unsafe extern "C" fn tilemap_load(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    match rc_mut(tilemap).load(
        value::int_arg(argv, 1) as i32,
        value::int_arg(argv, 2) as i32,
        &value::str_arg(argv, 3),
        value::int_arg(argv, 4) as u32,
    ) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

unsafe extern "C" fn tilemap_clip(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    match (
        value::opt_float_arg(argv, 1),
        value::opt_float_arg(argv, 2),
        value::opt_float_arg(argv, 3),
        value::opt_float_arg(argv, 4),
    ) {
        (Some(x), Some(y), Some(width), Some(height)) => {
            rc_mut(tilemap).set_clip_rect(x, y, width, height);
        }
        (None, None, None, None) => rc_mut(tilemap).reset_clip_rect(),
        _ => return value::raise_exception("clip() takes 0 or 4 arguments"),
    }
    value::return_none();
    true
}

unsafe extern "C" fn tilemap_camera(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    match (value::opt_float_arg(argv, 1), value::opt_float_arg(argv, 2)) {
        (Some(x), Some(y)) => rc_mut(tilemap).set_camera(x, y),
        (None, None) => rc_mut(tilemap).reset_camera(),
        _ => return value::raise_exception("camera() takes 0 or 2 arguments"),
    }
    value::return_none();
    true
}

macro_rules! tilemap_draw {
    ($name:ident, $method:ident, $($arg:expr),+ $(,)?) => {
        unsafe extern "C" fn $name(_argc: i32, argv: ffi::py_StackRef) -> bool {
            let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
            rc_mut(tilemap).$method($($arg(argv)),+);
            value::return_none();
            true
        }
    };
}

tilemap_draw!(tilemap_cls, clear, |argv| tile_arg(argv, 1));
tilemap_draw!(
    tilemap_pset,
    set_tile,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| tile_arg(argv, 3)
);
tilemap_draw!(
    tilemap_line,
    draw_line,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| tile_arg(argv, 5)
);
tilemap_draw!(
    tilemap_rect,
    draw_rect,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| tile_arg(argv, 5)
);
tilemap_draw!(
    tilemap_rectb,
    draw_rect_border,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| tile_arg(argv, 5)
);
tilemap_draw!(
    tilemap_circ,
    draw_circle,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| tile_arg(argv, 4)
);
tilemap_draw!(
    tilemap_circb,
    draw_circle_border,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| tile_arg(argv, 4)
);
tilemap_draw!(
    tilemap_elli,
    draw_ellipse,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| tile_arg(argv, 5)
);
tilemap_draw!(
    tilemap_ellib,
    draw_ellipse_border,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| tile_arg(argv, 5)
);
tilemap_draw!(
    tilemap_tri,
    draw_triangle,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| farg(argv, 5),
    |argv| farg(argv, 6),
    |argv| tile_arg(argv, 7)
);
tilemap_draw!(
    tilemap_trib,
    draw_triangle_border,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| farg(argv, 3),
    |argv| farg(argv, 4),
    |argv| farg(argv, 5),
    |argv| farg(argv, 6),
    |argv| tile_arg(argv, 7)
);
tilemap_draw!(
    tilemap_fill,
    flood_fill,
    |argv| farg(argv, 1),
    |argv| farg(argv, 2),
    |argv| tile_arg(argv, 3)
);

unsafe extern "C" fn tilemap_pget(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    value::return_tile(rc_ref(tilemap).tile(farg(argv, 1), farg(argv, 2)));
    true
}

unsafe extern "C" fn tilemap_collide(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let walls_ref = value::arg(argv, 7);
    let walls = (0..ffi::py_list_len(walls_ref))
        .map(|i| tile_from_ref(ffi::py_list_getitem(walls_ref, i)))
        .collect::<Vec<_>>();
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    value::return_float_pair(rc_ref(tilemap).collide(
        farg(argv, 1),
        farg(argv, 2),
        farg(argv, 3),
        farg(argv, 4),
        farg(argv, 5),
        farg(argv, 6),
        &walls,
    ));
    true
}

unsafe extern "C" fn tilemap_blt(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let Some(source) = tilemap_arg(value::arg(argv, 3)) else {
        return value::raise_exception("Invalid tilemap");
    };
    let tilemap = userdata::<pyxel::RcTilemap>(value::arg(argv, 0));
    rc_mut(tilemap).draw_tilemap(
        farg(argv, 1),
        farg(argv, 2),
        &source,
        farg(argv, 4),
        farg(argv, 5),
        farg(argv, 6),
        farg(argv, 7),
        if value::is_none(value::arg(argv, 8)) {
            None
        } else {
            Some(tile_arg(argv, 8))
        },
        value::opt_float_arg(argv, 9),
        value::opt_float_arg(argv, 10),
    );
    value::return_none();
    true
}

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

unsafe extern "C" fn tone_wavetable_get(_argc: i32, argv: ffi::py_StackRef) -> bool {
    make_int_seq(
        ffi::py_retval(),
        IntSeq::ToneWavetable(userdata::<pyxel::RcTone>(value::arg(argv, 0)).clone()),
    );
    true
}

macro_rules! sound_seq_getter {
    ($name:ident, $variant:ident) => {
        unsafe extern "C" fn $name(_argc: i32, argv: ffi::py_StackRef) -> bool {
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

unsafe extern "C" fn music_seqs_get(_argc: i32, argv: ffi::py_StackRef) -> bool {
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

unsafe extern "C" fn music_seqs_getitem(_argc: i32, argv: ffi::py_StackRef) -> bool {
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
    value::return_bool(pyxel::colors().iter().any(|value| *value == color));
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
        $len:ident,
        $iter:ident,
        $bool_fn:ident,
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
    };
}

collection_fns!(
    images_getitem,
    images_setitem,
    images_len,
    images_iter,
    images_bool,
    pyxel::images,
    make_image,
    pyxel::RcImage
);
collection_fns!(
    tilemaps_getitem,
    tilemaps_setitem,
    tilemaps_len,
    tilemaps_iter,
    tilemaps_bool,
    pyxel::tilemaps,
    make_tilemap,
    pyxel::RcTilemap
);
collection_fns!(
    channels_getitem,
    channels_setitem,
    channels_len,
    channels_iter,
    channels_bool,
    pyxel::channels,
    make_channel,
    pyxel::RcChannel
);
collection_fns!(
    tones_getitem,
    tones_setitem,
    tones_len,
    tones_iter,
    tones_bool,
    pyxel::tones,
    make_tone,
    pyxel::RcTone
);
collection_fns!(
    sounds_getitem,
    sounds_setitem,
    sounds_len,
    sounds_iter,
    sounds_bool,
    pyxel::sounds,
    make_sound,
    pyxel::RcSound
);
collection_fns!(
    musics_getitem,
    musics_setitem,
    musics_len,
    musics_iter,
    musics_bool,
    pyxel::musics,
    make_music,
    pyxel::RcMusic
);

unsafe fn register_collection_type(
    type_: ffi::py_Type,
    getitem: ffi::py_CFunction,
    setitem: ffi::py_CFunction,
    len: ffi::py_CFunction,
    iter: ffi::py_CFunction,
    bool_fn: ffi::py_CFunction,
) {
    value::bind_magic(type_, c"__getitem__", getitem);
    value::bind_magic(type_, c"__setitem__", setitem);
    value::bind_magic(type_, c"__len__", len);
    value::bind_magic(type_, c"__iter__", iter);
    value::bind_magic(type_, c"__bool__", bool_fn);
}

unsafe fn register_classes(module: ffi::py_GlobalRef) {
    TP_FONT = value::new_type(module, c"Font", Some(drop_font));
    let font_type = ffi::py_tpobject(TP_FONT);
    ffi::py_bind(
        font_type,
        c"__new__(cls, filename, font_size=None)".as_ptr(),
        Some(font_new),
    );
    ffi::py_bind(
        font_type,
        c"__init__(self, filename, font_size=None)".as_ptr(),
        Some(noop_init),
    );
    ffi::py_bindmethod(TP_FONT, c"text_width".as_ptr(), Some(font_text_width));

    TP_IMAGE = value::new_type(module, c"Image", Some(drop_image));
    let image_type = ffi::py_tpobject(TP_IMAGE);
    ffi::py_bind(
        image_type,
        c"__new__(cls, width, height)".as_ptr(),
        Some(image_new),
    );
    ffi::py_bind(
        image_type,
        c"__init__(self, width, height)".as_ptr(),
        Some(noop_init),
    );
    ffi::py_bind(
        image_type,
        c"from_image(filename, include_colors=None)".as_ptr(),
        Some(image_from_image),
    );
    ffi::py_bindproperty(TP_IMAGE, c"width".as_ptr(), Some(image_width), None);
    ffi::py_bindproperty(TP_IMAGE, c"height".as_ptr(), Some(image_height), None);
    ffi::py_bindmethod(TP_IMAGE, c"data_ptr".as_ptr(), Some(image_data_ptr));
    ffi::py_bindmethod(TP_IMAGE, c"set".as_ptr(), Some(image_set));
    ffi::py_bindmethod(TP_IMAGE, c"save".as_ptr(), Some(image_save));
    ffi::py_bindmethod(TP_IMAGE, c"cls".as_ptr(), Some(image_cls));
    ffi::py_bindmethod(TP_IMAGE, c"pget".as_ptr(), Some(image_pget));
    ffi::py_bindmethod(TP_IMAGE, c"pset".as_ptr(), Some(image_pset));
    ffi::py_bindmethod(TP_IMAGE, c"line".as_ptr(), Some(image_line));
    ffi::py_bindmethod(TP_IMAGE, c"rect".as_ptr(), Some(image_rect));
    ffi::py_bindmethod(TP_IMAGE, c"rectb".as_ptr(), Some(image_rectb));
    ffi::py_bindmethod(TP_IMAGE, c"circ".as_ptr(), Some(image_circ));
    ffi::py_bindmethod(TP_IMAGE, c"circb".as_ptr(), Some(image_circb));
    ffi::py_bindmethod(TP_IMAGE, c"elli".as_ptr(), Some(image_elli));
    ffi::py_bindmethod(TP_IMAGE, c"ellib".as_ptr(), Some(image_ellib));
    ffi::py_bindmethod(TP_IMAGE, c"tri".as_ptr(), Some(image_tri));
    ffi::py_bindmethod(TP_IMAGE, c"trib".as_ptr(), Some(image_trib));
    ffi::py_bindmethod(TP_IMAGE, c"fill".as_ptr(), Some(image_fill));
    ffi::py_bind(
        image_type,
        c"load(self, x, y, filename, include_colors=None)".as_ptr(),
        Some(image_load),
    );
    ffi::py_bind(
        image_type,
        c"clip(self, x=None, y=None, w=None, h=None)".as_ptr(),
        Some(image_clip),
    );
    ffi::py_bind(
        image_type,
        c"camera(self, x=None, y=None)".as_ptr(),
        Some(image_camera),
    );
    ffi::py_bind(
        image_type,
        c"pal(self, col1=None, col2=None)".as_ptr(),
        Some(image_pal),
    );
    ffi::py_bind(
        image_type,
        c"dither(self, alpha)".as_ptr(),
        Some(image_dither),
    );
    ffi::py_bind(
        image_type,
        c"blt(self, x, y, img, u, v, w, h, colkey=None, rotate=None, scale=None)".as_ptr(),
        Some(image_blt),
    );
    ffi::py_bind(
        image_type,
        c"bltm(self, x, y, tm, u, v, w, h, colkey=None, rotate=None, scale=None)".as_ptr(),
        Some(image_bltm),
    );
    ffi::py_bind(
        image_type,
        c"blt3d(self, x, y, w, h, img, pos, rot, fov=None, colkey=None)".as_ptr(),
        Some(image_blt3d),
    );
    ffi::py_bind(
        image_type,
        c"bltm3d(self, x, y, w, h, tm, pos, rot, fov=None, colkey=None)".as_ptr(),
        Some(image_bltm3d),
    );
    ffi::py_bind(
        image_type,
        c"text(self, x, y, s, col, font=None)".as_ptr(),
        Some(image_text),
    );

    TP_TILEMAP = value::new_type(module, c"Tilemap", Some(drop_tilemap));
    let tilemap_type = ffi::py_tpobject(TP_TILEMAP);
    ffi::py_bind(
        tilemap_type,
        c"__new__(cls, width, height, imgsrc)".as_ptr(),
        Some(tilemap_new),
    );
    ffi::py_bind(
        tilemap_type,
        c"__init__(self, width, height, imgsrc)".as_ptr(),
        Some(noop_init),
    );
    ffi::py_bind(
        tilemap_type,
        c"from_tmx(filename, layer)".as_ptr(),
        Some(tilemap_from_tmx),
    );
    ffi::py_bindproperty(TP_TILEMAP, c"width".as_ptr(), Some(tilemap_width), None);
    ffi::py_bindproperty(TP_TILEMAP, c"height".as_ptr(), Some(tilemap_height), None);
    ffi::py_bindproperty(
        TP_TILEMAP,
        c"imgsrc".as_ptr(),
        Some(tilemap_imgsrc_get),
        Some(tilemap_imgsrc_set),
    );
    ffi::py_bindproperty(
        TP_TILEMAP,
        c"image".as_ptr(),
        Some(tilemap_image_get),
        Some(tilemap_image_set),
    );
    ffi::py_bindproperty(
        TP_TILEMAP,
        c"refimg".as_ptr(),
        Some(tilemap_refimg_get),
        Some(tilemap_refimg_set),
    );
    ffi::py_bindmethod(TP_TILEMAP, c"data_ptr".as_ptr(), Some(tilemap_data_ptr));
    ffi::py_bindmethod(TP_TILEMAP, c"set".as_ptr(), Some(tilemap_set));
    ffi::py_bindmethod(TP_TILEMAP, c"load".as_ptr(), Some(tilemap_load));
    ffi::py_bindmethod(TP_TILEMAP, c"cls".as_ptr(), Some(tilemap_cls));
    ffi::py_bindmethod(TP_TILEMAP, c"pget".as_ptr(), Some(tilemap_pget));
    ffi::py_bindmethod(TP_TILEMAP, c"pset".as_ptr(), Some(tilemap_pset));
    ffi::py_bindmethod(TP_TILEMAP, c"line".as_ptr(), Some(tilemap_line));
    ffi::py_bindmethod(TP_TILEMAP, c"rect".as_ptr(), Some(tilemap_rect));
    ffi::py_bindmethod(TP_TILEMAP, c"rectb".as_ptr(), Some(tilemap_rectb));
    ffi::py_bindmethod(TP_TILEMAP, c"circ".as_ptr(), Some(tilemap_circ));
    ffi::py_bindmethod(TP_TILEMAP, c"circb".as_ptr(), Some(tilemap_circb));
    ffi::py_bindmethod(TP_TILEMAP, c"elli".as_ptr(), Some(tilemap_elli));
    ffi::py_bindmethod(TP_TILEMAP, c"ellib".as_ptr(), Some(tilemap_ellib));
    ffi::py_bindmethod(TP_TILEMAP, c"tri".as_ptr(), Some(tilemap_tri));
    ffi::py_bindmethod(TP_TILEMAP, c"trib".as_ptr(), Some(tilemap_trib));
    ffi::py_bindmethod(TP_TILEMAP, c"fill".as_ptr(), Some(tilemap_fill));
    ffi::py_bindmethod(TP_TILEMAP, c"collide".as_ptr(), Some(tilemap_collide));
    ffi::py_bind(
        tilemap_type,
        c"clip(self, x=None, y=None, w=None, h=None)".as_ptr(),
        Some(tilemap_clip),
    );
    ffi::py_bind(
        tilemap_type,
        c"camera(self, x=None, y=None)".as_ptr(),
        Some(tilemap_camera),
    );
    ffi::py_bind(
        tilemap_type,
        c"blt(self, x, y, tm, u, v, w, h, tilekey=None, rotate=None, scale=None)".as_ptr(),
        Some(tilemap_blt),
    );

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
        Some(tone_wavetable_get),
        None,
    );
    ffi::py_bindproperty(
        TP_TONE,
        c"waveform".as_ptr(),
        Some(tone_wavetable_get),
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
    ffi::py_bindproperty(TP_SOUND, c"notes".as_ptr(), Some(sound_notes_get), None);
    ffi::py_bindproperty(TP_SOUND, c"tones".as_ptr(), Some(sound_tones_get), None);
    ffi::py_bindproperty(TP_SOUND, c"volumes".as_ptr(), Some(sound_volumes_get), None);
    ffi::py_bindproperty(TP_SOUND, c"effects".as_ptr(), Some(sound_effects_get), None);
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

    TP_MUSIC = value::new_type(module, c"Music", Some(drop_music));
    let music_type = ffi::py_tpobject(TP_MUSIC);
    ffi::py_bind(music_type, c"__new__(cls)".as_ptr(), Some(music_new));
    ffi::py_bind(music_type, c"__init__(self)".as_ptr(), Some(noop_init));
    ffi::py_bindproperty(TP_MUSIC, c"seqs".as_ptr(), Some(music_seqs_get), None);
    ffi::py_bindproperty(TP_MUSIC, c"snds_list".as_ptr(), Some(music_seqs_get), None);
    ffi::py_bindmethod(TP_MUSIC, c"set".as_ptr(), Some(music_set));
    ffi::py_bind(
        music_type,
        c"save(self, filename, sec, ffmpeg=None)".as_ptr(),
        Some(music_save),
    );
}

unsafe fn register_collections(module: ffi::py_GlobalRef) {
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
        Some(images_len),
        Some(images_iter),
        Some(images_bool),
    );
    TP_TILEMAPS = value::new_type(module, c"Tilemaps", None);
    register_collection_type(
        TP_TILEMAPS,
        Some(tilemaps_getitem),
        Some(tilemaps_setitem),
        Some(tilemaps_len),
        Some(tilemaps_iter),
        Some(tilemaps_bool),
    );
    TP_CHANNELS = value::new_type(module, c"Channels", None);
    register_collection_type(
        TP_CHANNELS,
        Some(channels_getitem),
        Some(channels_setitem),
        Some(channels_len),
        Some(channels_iter),
        Some(channels_bool),
    );
    TP_TONES = value::new_type(module, c"Tones", None);
    register_collection_type(
        TP_TONES,
        Some(tones_getitem),
        Some(tones_setitem),
        Some(tones_len),
        Some(tones_iter),
        Some(tones_bool),
    );
    TP_SOUNDS = value::new_type(module, c"Sounds", None);
    register_collection_type(
        TP_SOUNDS,
        Some(sounds_getitem),
        Some(sounds_setitem),
        Some(sounds_len),
        Some(sounds_iter),
        Some(sounds_bool),
    );
    TP_MUSICS = value::new_type(module, c"Musics", None);
    register_collection_type(
        TP_MUSICS,
        Some(musics_getitem),
        Some(musics_setitem),
        Some(musics_len),
        Some(musics_iter),
        Some(musics_bool),
    );

    value::set_module_object(module, c"colors", TP_COLORS);
    value::set_module_object(module, c"images", TP_IMAGES);
    value::set_module_object(module, c"tilemaps", TP_TILEMAPS);
    value::set_module_object(module, c"channels", TP_CHANNELS);
    value::set_module_object(module, c"tones", TP_TONES);
    value::set_module_object(module, c"sounds", TP_SOUNDS);
    value::set_module_object(module, c"musics", TP_MUSICS);
}

pub unsafe fn register(module: ffi::py_GlobalRef) {
    register_classes(module);
    register_collections(module);

    set_global_object(module, c"screen", TP_IMAGE, pyxel::screen().clone());
    set_global_object(module, c"cursor", TP_IMAGE, pyxel::cursor_image().clone());
    set_global_object(module, c"font", TP_IMAGE, pyxel::font_image().clone());
}
