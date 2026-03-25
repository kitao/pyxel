use std::sync::Once;

static INIT: Once = Once::new();

fn ensure_init() {
    INIT.call_once(|| {
        pyxel::init(64, 64, None, None, None, None, None, None, Some(true));
    });
}

#[test]
fn test_channels_initialized() {
    ensure_init();
    let channels = pyxel::channels();

    // Default channel count is NUM_CHANNELS (4)
    assert!(channels.len() >= 4);

    // Each channel pointer is valid and accessible
    for &ch in channels.iter() {
        let channel = unsafe { &*ch };
        assert!(channel.gain > 0.0);
    }
}

#[test]
fn test_channel_play_mml() {
    ensure_init();
    let ch = unsafe { &mut *pyxel::channels()[0] };

    // Channel starts idle
    assert!(ch.play_position().is_none());

    // Play a short MML sequence
    ch.play_mml("T120 O4 C4", None, false, false).unwrap();

    // After play_mml, the channel should be playing
    assert!(ch.play_position().is_some());

    // Stop and verify idle
    ch.stop();
    assert!(ch.play_position().is_none());
}

#[test]
fn test_tones_accessible() {
    ensure_init();
    let tones = pyxel::tones();

    // Default tone count is NUM_TONES (4)
    assert!(tones.len() >= 4);

    // First tone should have non-empty wavetable (initialized from DEFAULT_TONE_0)
    let tone0 = unsafe { &*tones[0] };
    assert!(!tone0.wavetable.is_empty());
}
