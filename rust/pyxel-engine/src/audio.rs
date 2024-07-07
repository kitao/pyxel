use std::cmp::min;

use crate::blip_buf::BlipBuf;
use crate::pyxel::{Pyxel, CHANNELS};
use crate::settings::{CLOCK_RATE, NUM_CLOCKS_PER_TICK, NUM_SAMPLES, SAMPLE_RATE};
use crate::SharedChannel;

struct AudioCore {
    blip_buf: BlipBuf,
    channels: shared_type!(Vec<SharedChannel>),
}

impl pyxel_platform::AudioCallback for AudioCore {
    fn update(&mut self, out: &mut [i16]) {
        let channels_ = self.channels.lock();
        let mut channels: Vec<_> = channels_.iter().map(|channel| channel.lock()).collect();
        let mut samples = self.blip_buf.read_samples(out, false);
        while samples < out.len() {
            for channel in &mut *channels {
                channel.update(&mut self.blip_buf);
            }
            self.blip_buf.end_frame(NUM_CLOCKS_PER_TICK as u64);
            samples += self.blip_buf.read_samples(&mut out[samples..], false);
        }
    }
}

pub struct Audio {}

impl Audio {
    pub fn new(sample_rate: u32, num_samples: u32) -> Self {
        let mut blip_buf = BlipBuf::new(NUM_SAMPLES as usize);
        blip_buf.set_rates(CLOCK_RATE as f64, SAMPLE_RATE as f64);
        pyxel_platform::start_audio(
            sample_rate,
            1,
            num_samples as u16,
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
        channel_index: u32,
        sequence: &[u32],
        start_tick: Option<u32>,
        should_loop: bool,
        should_resume: bool,
    ) {
        if sequence.is_empty() {
            return;
        }
        let sounds = sequence
            .iter()
            .map(|sound_index| self.sounds.lock()[*sound_index as usize].clone())
            .collect();
        self.channels.lock()[channel_index as usize].lock().play(
            sounds,
            start_tick,
            should_loop,
            should_resume,
        );
    }

    pub fn play1(
        &self,
        channel_index: u32,
        sound_index: u32,
        start_tick: Option<u32>,
        should_loop: bool,
        should_resume: bool,
    ) {
        self.channels.lock()[channel_index as usize].lock().play1(
            self.sounds.lock()[sound_index as usize].clone(),
            start_tick,
            should_loop,
            should_resume,
        );
    }

    pub fn playm(&self, music_index: u32, start_tick: Option<u32>, should_loop: bool) {
        let num_channels = self.channels.lock().len();
        let musics = self.musics.lock();
        let music = musics[music_index as usize].lock();
        for i in 0..min(num_channels, music.seqs.len()) {
            self.play(
                i as u32,
                &music.seqs[i].lock(),
                start_tick,
                should_loop,
                false,
            );
        }
    }

    pub fn stop(&self, channel_index: u32) {
        self.channels.lock()[channel_index as usize].lock().stop();
    }

    pub fn stop0(&self) {
        let num_channels = self.channels.lock().len();
        for i in 0..num_channels {
            self.stop(i as u32);
        }
    }

    pub fn play_pos(&self, channel_index: u32) -> Option<(u32, u32)> {
        self.channels.lock()[channel_index as usize]
            .lock()
            .play_pos()
    }
}
