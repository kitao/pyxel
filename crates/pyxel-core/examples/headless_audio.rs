// SPDX-License-Identifier: MIT
//
// examples/headless_audio.rs
//
// Demonstrates how to use pyxel-core in headless mode and pull PCM samples
// into an external audio pipeline instead of letting SDL2 push them directly
// to an ALSA / CoreAudio / WASAPI device.
//
// Typical use-cases:
//   - libretro cores  (samples go to retro_audio_sample_batch_t)
//   - WebAssembly     (samples go to the Web Audio API)
//   - Offline render  (samples are written to a WAV file)
//   - Unit tests      (sound output is verified without a real device)
//
// Build:
//   cargo run --example headless_audio --features sdl2_static
//
// Expected output:
//   frame  1: 368 samples rendered, first sample = 0
//   frame  2: 368 samples rendered, first sample = <non-zero while beep plays>
//   ...

use blip_buf::BlipBuf;
use pyxel::{
    Audio, AUDIO_CLOCK_RATE, AUDIO_SAMPLE_RATE,
    channels, init, pyxel, sounds,
};

fn main() {
    // ------------------------------------------------------------------
    // 1. Tell SDL2 to use the null audio driver so it does not attempt
    //    to open /dev/snd (or CoreAudio / WASAPI on other platforms).
    //    This avoids device conflicts when the host process already owns
    //    the audio device (e.g. a libretro frontend).
    // ------------------------------------------------------------------
    std::env::set_var("SDL_AUDIODRIVER", "dummy");

    // ------------------------------------------------------------------
    // 2. Initialize Pyxel in headless mode.
    //    headless = Some(true) skips window creation and GL context setup.
    // ------------------------------------------------------------------
    init(
        128, 128,           // width, height (unused in headless mode)
        None,               // title
        Some(60),           // fps
        None,               // quit_key
        None,               // display_scale
        None,               // capture_scale
        None,               // capture_sec
        Some(true),         // headless = true
    );

    // ------------------------------------------------------------------
    // 3. Define a short beep in sound bank 0 using MML notation.
    //    (C4 - E4 - G4, triangle wave, max volume, no effect, speed 20)
    // ------------------------------------------------------------------
    {
        let rc_sound = &sounds()[0];
        let sound = unsafe { &mut *rc_sound.get() };
        sound
            .set("c3e3g3", "t", "7", "n", 20)
            .expect("Failed to set sound");
    }

    // ------------------------------------------------------------------
    // 4. Trigger playback on channel 0.
    // ------------------------------------------------------------------
    pyxel().play_sound(0, 0, Some(0.0), false, false);

    // ------------------------------------------------------------------
    // 5. Set up a BlipBuf resampler matching Pyxel's internal clock.
    //    AUDIO_CLOCK_RATE = 1_789_773 (NTSC NES APU clock)
    //    AUDIO_SAMPLE_RATE = 22_050 Hz
    // ------------------------------------------------------------------
    let samples_per_frame = (AUDIO_SAMPLE_RATE as f64 / 60.0).ceil() as usize; // ≈ 368
    let mut blip = BlipBuf::new((samples_per_frame * 2) as u32);
    blip.set_rates(AUDIO_CLOCK_RATE as f64, AUDIO_SAMPLE_RATE as f64);

    // ------------------------------------------------------------------
    // 6. Simulate 10 frames.  Each call to Audio::render_samples() pulls
    //    the next batch of PCM data — exactly what a libretro core passes
    //    to retro_audio_sample_batch_t, or a WASM port feeds to
    //    AudioWorkletProcessor.process().
    // ------------------------------------------------------------------
    for frame in 1..=10 {
        let mut mono_buf = vec![0i16; samples_per_frame];

        // This is the key call exposed by this PR.
        Audio::render_samples(channels(), &mut blip, &mut mono_buf);

        println!(
            "frame {:2}: {} samples rendered, first sample = {}",
            frame,
            mono_buf.len(),
            mono_buf[0],
        );

        // In a real integration you would convert mono -> stereo here
        // and hand the buffer to the external audio sink:
        //
        //   let stereo: Vec<i16> = mono_buf.iter()
        //       .flat_map(|&s| [s, s])
        //       .collect();
        //   external_audio_sink.write(&stereo);
    }
}
