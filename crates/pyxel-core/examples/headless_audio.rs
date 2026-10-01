// Pulls audio samples from pyxel-core in headless mode, for hosts that own the
// audio device themselves (e.g. a libretro frontend).
//
// cargo run --example headless_audio --features sdl2_static

use pyxel::{init, pyxel, sounds, AudioRenderer, AUDIO_SAMPLE_RATE};

const FPS: u32 = 60;

fn main() {
    // Keep SDL2 away from the real audio device the host already owns.
    std::env::set_var("SDL_AUDIODRIVER", "dummy");

    init(
        128,
        128,
        None,
        Some(FPS),
        None,
        None,
        None,
        None,
        Some(true),
    )
    .expect("failed to initialize Pyxel");

    sounds()[0]
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .set("c3e3g3", "t", "7", "n", 20)
        .expect("failed to set sound");
    pyxel()
        .play_sound(0, 0, None, false, false)
        .expect("failed to play sound");

    let mut renderer = AudioRenderer::new();
    let samples_per_frame = AUDIO_SAMPLE_RATE.div_ceil(FPS) as usize;

    for frame in 1..=10 {
        let mut samples = vec![0i16; samples_per_frame];
        renderer.render(&mut samples);

        let peak = samples.iter().map(|s| s.unsigned_abs()).max().unwrap_or(0);
        println!("frame {frame:2}: {samples_per_frame} samples, peak = {peak}");
    }
}
