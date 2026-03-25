use std::sync::Once;

static INIT: Once = Once::new();

fn ensure_init() {
    INIT.call_once(|| {
        pyxel::init(64, 64, None, None, None, None, None, None, Some(true));
    });
}

fn new_image(w: u32, h: u32) -> &'static mut pyxel::Image {
    unsafe { &mut *pyxel::Image::new(w, h) }
}

#[test]
fn test_clear_and_pget() {
    ensure_init();
    let img = new_image(32, 32);
    img.clear(5);
    assert_eq!(img.get_pixel(0.0, 0.0), 5);
    assert_eq!(img.get_pixel(15.0, 15.0), 5);
    assert_eq!(img.get_pixel(31.0, 31.0), 5);
}

#[test]
fn test_draw_pixel() {
    ensure_init();
    let img = new_image(32, 32);
    img.clear(0);
    img.set_pixel(10.0, 10.0, 7);
    assert_eq!(img.get_pixel(10.0, 10.0), 7);
    assert_eq!(img.get_pixel(9.0, 10.0), 0);
    assert_eq!(img.get_pixel(11.0, 10.0), 0);
}

#[test]
fn test_draw_rect_fills_area() {
    ensure_init();
    let img = new_image(32, 32);
    img.clear(0);
    img.draw_rect(5.0, 5.0, 4.0, 3.0, 8);
    assert_eq!(img.get_pixel(5.0, 5.0), 8);
    assert_eq!(img.get_pixel(8.0, 7.0), 8);
    assert_eq!(img.get_pixel(6.0, 6.0), 8);
    assert_eq!(img.get_pixel(4.0, 5.0), 0);
    assert_eq!(img.get_pixel(9.0, 5.0), 0);
}
