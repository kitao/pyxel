use once_cell::sync::Lazy;

use crate::blip_buf::BlipBuf;
use crate::channel::{Channel, SharedChannels};
use crate::pyxel::Pyxel;
use crate::settings::{CLOCK_RATE, NUM_CHANNELS, NUM_CLOCKS_PER_TICK, NUM_SAMPLES, SAMPLE_RATE};

pub(crate) static CHANNELS: Lazy<SharedChannels> =
    Lazy::new(|| new_shared_type!((0..NUM_CHANNELS).map(|_| Channel::new()).collect()));

struct AudioCore {
    blip_buf: BlipBuf,
    channels: SharedChannels,
}

impl pyxel_platform::AudioCallback for AudioCore {
    fn update(&mut self, out: &mut [i16]) {
        let mut samples = self.blip_buf.read_samples(out, false);
        while samples < out.len() {
            for channel in &*self.channels.lock() {
                channel.lock().update(&mut self.blip_buf);
            }
            self.blip_buf.end_frame(NUM_CLOCKS_PER_TICK as u64);
            samples += self.blip_buf.read_samples(&mut out[samples..], false);
        }
    }
}

pub struct Audio {}

impl Audio {
    pub fn new() -> Self {
        let mut blip_buf = BlipBuf::new(NUM_SAMPLES as usize);
        blip_buf.set_rates(CLOCK_RATE as f64, SAMPLE_RATE as f64);
        pyxel_platform::start_audio(
            SAMPLE_RATE,
            1,
            NUM_SAMPLES as u16,
            new_shared_type!(AudioCore {
                blip_buf,
                channels: CHANNELS.clone()
            }),
        );
        Self {}
    }
}

impl Pyxel {
    pub fn play(
        &self,
        channel_no: u32,
        sequence: &[u32],
        start_tick: Option<u32>,
        should_loop: bool,
    ) {
        if sequence.is_empty() {
            return;
        }
        let sounds = sequence
            .iter()
            .map(|sound_no| self.sounds[*sound_no as usize].clone())
            .collect();
        self.channels.lock()[channel_no as usize]
            .lock()
            .play(sounds, start_tick, should_loop);
    }

    pub fn play1(
        &self,
        channel_no: u32,
        sound_no: u32,
        start_tick: Option<u32>,
        should_loop: bool,
    ) {
        self.channels.lock()[channel_no as usize].lock().play1(
            self.sounds[sound_no as usize].clone(),
            start_tick,
            should_loop,
        );
    }

    pub fn playm(&self, music_no: u32, start_tick: Option<u32>, should_loop: bool) {
        let music = self.musics[music_no as usize].lock();
        for i in 0..NUM_CHANNELS {
            self.play(i, &music.seqs[i as usize].lock(), start_tick, should_loop);
        }
    }

    pub fn stop(&self, channel_no: u32) {
        self.channels.lock()[channel_no as usize].lock().stop();
    }

    pub fn stop0(&self) {
        for i in 0..NUM_CHANNELS {
            self.stop(i);
        }
    }

    pub fn play_pos(&self, channel_no: u32) -> Option<(u32, u32)> {
        self.channels.lock()[channel_no as usize].lock().play_pos()
    }
}
