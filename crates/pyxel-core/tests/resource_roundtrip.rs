use std::sync::Once;

static INIT: Once = Once::new();

fn ensure_init() {
    INIT.call_once(|| {
        pyxel::init(64, 64, None, None, None, None, None, None, Some(true));
    });
}

#[test]
fn test_image_pixel_roundtrip() {
    ensure_init();
    let img = unsafe { &mut *pyxel::images()[0] };
    img.set_pixel(0.0, 0.0, 7);
    assert_eq!(img.get_pixel(0.0, 0.0), 7);

    // Neighboring pixel remains untouched
    assert_eq!(img.get_pixel(1.0, 0.0), 0);
}

#[test]
fn test_image_multiple_pixels() {
    ensure_init();
    let img = unsafe { &mut *pyxel::images()[1] };
    img.set_pixel(10.0, 20.0, 3);
    img.set_pixel(11.0, 20.0, 5);
    img.set_pixel(10.0, 21.0, 9);

    assert_eq!(img.get_pixel(10.0, 20.0), 3);
    assert_eq!(img.get_pixel(11.0, 20.0), 5);
    assert_eq!(img.get_pixel(10.0, 21.0), 9);
}

#[test]
fn test_sound_note_roundtrip() {
    ensure_init();
    let snd = unsafe { &mut *pyxel::sounds()[0] };
    snd.notes.push(24);
    snd.notes.push(36);

    assert_eq!(snd.notes.len(), 2);
    assert_eq!(snd.notes[0], 24);
    assert_eq!(snd.notes[1], 36);
}

#[test]
fn test_sound_set_and_read() {
    ensure_init();
    let snd = unsafe { &mut *pyxel::sounds()[1] };
    snd.set("c2e2g2c3", "s", "7776", "nnnn", 8).unwrap();

    assert_eq!(snd.notes.len(), 4);
    assert_eq!(snd.speed, 8);
}
