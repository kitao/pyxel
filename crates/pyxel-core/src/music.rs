use std::cmp::min;

use blip_buf::BlipBuf;

use crate::audio::Audio;
use crate::channel::Channel;
use crate::pyxel;
use crate::settings::{AUDIO_CLOCK_RATE, AUDIO_SAMPLE_RATE};

#[derive(Clone)]
pub struct Music {
    pub seqs: Vec<Vec<u32>>,
}

impl Music {
    pub fn new() -> *mut Music {
        Box::into_raw(Box::new(Self { seqs: Vec::new() }))
    }

    pub fn set(&mut self, seqs: &[Vec<u32>]) {
        self.seqs = seqs.to_vec();

        let num_channels = pyxel::channels().len();
        while self.seqs.len() < num_channels {
            self.seqs.push(Vec::new());
        }
    }

    pub fn save(
        &self,
        filename: &str,
        duration_sec: f32,
        use_ffmpeg: Option<bool>,
    ) -> Result<(), String> {
        assert!(duration_sec > 0.0);

        let num_samples = (duration_sec * AUDIO_SAMPLE_RATE as f32).round() as u32;
        if num_samples == 0 {
            return Ok(());
        }

        let seqs: Vec<_> = (0..self.seqs.len())
            .map(|i| {
                let pyxel_sounds = pyxel::sounds();
                self.seqs[i]
                    .iter()
                    .map(|&sound_index| pyxel_sounds[sound_index as usize])
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut samples = vec![0; num_samples as usize];
        let mut blip_buf = BlipBuf::new(num_samples);
        blip_buf.set_rates(AUDIO_CLOCK_RATE as f64, AUDIO_SAMPLE_RATE as f64);

        let channels = pyxel::channels();
        for &channel in channels.iter() {
            unsafe { &mut *channel }.stop();
        }

        {
            let mut channels: Vec<&mut Channel> = channels
                .iter()
                .map(|&channel| unsafe { &mut *channel })
                .collect();
            for i in 0..min(channels.len(), seqs.len()) {
                channels[i].play(seqs[i].clone(), None, true, false);
            }
        }

        Audio::render_samples(channels.as_slice(), &mut blip_buf, &mut samples);
        let result = Audio::save_samples(filename, &samples, use_ffmpeg.unwrap_or(false));
        for &channel in channels.iter() {
            unsafe { &mut *channel }.stop();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_music_new() {
        let music = Music::new();
        let music = unsafe { &*music };
        assert_eq!(music.seqs.len(), 0);
    }

    #[test]
    fn test_music_set() {
        let music = Music::new();
        let music = unsafe { &mut *music };
        music.set(&[vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4]]);

        for i in 0..3 {
            assert_eq!(&music.seqs[i as usize], &vec![i, i + 1, i + 2]);
        }
    }
}
