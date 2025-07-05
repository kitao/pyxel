use std::cmp::min;

use crate::audio::Audio;
use crate::blippers::BlipBuf;
use crate::pyxel::{CHANNELS, SOUNDS};
use crate::settings::{AUDIO_CLOCK_RATE, AUDIO_SAMPLE_RATE};

pub type SharedSeq = shared_type!(Vec<u32>);

#[derive(Clone)]
pub struct Music {
    pub seqs: Vec<SharedSeq>,
}

pub type SharedMusic = shared_type!(Music);

impl Music {
    pub fn new() -> SharedMusic {
        new_shared_type!(Self { seqs: Vec::new() })
    }

    pub fn set(&mut self, seqs: &[Vec<u32>]) {
        self.seqs = seqs
            .iter()
            .map(|seq| new_shared_type!(seq.clone()))
            .collect();

        let num_channels = CHANNELS.lock().len();
        while self.seqs.len() < num_channels {
            self.seqs.push(new_shared_type!(Vec::new()));
        }
    }

    pub fn save(&self, filename: &str, duration_sec: f32, use_ffmpeg: Option<bool>) {
        assert!(duration_sec > 0.0);

        let num_samples = (duration_sec * AUDIO_SAMPLE_RATE as f32).round() as u32;
        if num_samples == 0 {
            return;
        }

        let seqs: Vec<_> = (0..self.seqs.len())
            .map(|i| {
                let pyxel_sounds = SOUNDS.lock();
                self.seqs[i]
                    .lock()
                    .iter()
                    .map(|&sound_index| pyxel_sounds[sound_index as usize].clone())
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut samples = vec![0; num_samples as usize];
        let mut blip_buf = BlipBuf::new(num_samples as usize);
        blip_buf.set_rates(AUDIO_CLOCK_RATE as f64, AUDIO_SAMPLE_RATE as f64);

        let channels = CHANNELS.lock();
        channels.iter().for_each(|channel| channel.lock().stop());

        {
            let mut channels: Vec<_> = channels.iter().map(|channel| channel.lock()).collect();
            for i in 0..min(channels.len(), seqs.len()) {
                channels[i].play(seqs[i].clone(), None, true, false);
            }
        }

        Audio::render_samples(&channels, &mut blip_buf, &mut samples);
        Audio::save_samples(filename, &samples, use_ffmpeg.unwrap_or(false));
        channels.iter().for_each(|channel| channel.lock().stop());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_music_new() {
        let music = Music::new();
        assert_eq!(music.lock().seqs.len(), 0);
    }

    #[test]
    fn test_music_set() {
        let music = Music::new();
        music
            .lock()
            .set(&[vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4]]);

        for i in 0..3 {
            assert_eq!(
                &*music.lock().seqs[i as usize].lock(),
                &vec![i, i + 1, i + 2]
            );
        }
    }
}
