use blip_buf::BlipBuf;

use crate::audio::Audio;
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
        self.seqs.resize_with(num_channels, Vec::new);
    }

    pub fn save(
        &self,
        filename: &str,
        duration_sec: f32,
        use_ffmpeg: Option<bool>,
    ) -> Result<(), String> {
        if duration_sec <= 0.0 {
            return Err("duration_sec must be greater than 0".to_string());
        }

        let num_samples = (duration_sec * AUDIO_SAMPLE_RATE as f32).round() as u32;
        if num_samples == 0 {
            return Ok(());
        }

        let pyxel_sounds = pyxel::sounds();
        let seqs: Vec<Vec<_>> = self
            .seqs
            .iter()
            .map(|seq| {
                seq.iter()
                    .map(|&index| pyxel_sounds[index as usize])
                    .collect()
            })
            .collect();

        let mut samples = vec![0; num_samples as usize];
        let mut blip_buf = BlipBuf::new(num_samples);
        blip_buf.set_rates(AUDIO_CLOCK_RATE as f64, AUDIO_SAMPLE_RATE as f64);

        let channels = pyxel::channels();
        for &ch in channels.iter() {
            unsafe { &mut *ch }.stop();
        }
        for (ch, seq) in channels.iter().zip(seqs.iter()) {
            unsafe { &mut **ch }.play(seq.clone(), None, true, false);
        }

        Audio::render_samples(channels.as_slice(), &mut blip_buf, &mut samples);
        let result = Audio::save_samples(filename, &samples, use_ffmpeg.unwrap_or(false));
        for &ch in channels.iter() {
            unsafe { &mut *ch }.stop();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_music() -> &'static mut Music {
        unsafe { &mut *Music::new() }
    }

    #[test]
    fn test_new() {
        let music = new_music();
        assert!(music.seqs.is_empty());
    }

    #[test]
    fn test_set() {
        let music = new_music();
        music.set(&[vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4]]);
        assert_eq!(music.seqs[0], [0, 1, 2]);
        assert_eq!(music.seqs[1], [1, 2, 3]);
        assert_eq!(music.seqs[2], [2, 3, 4]);
    }

    #[test]
    fn test_set_pads_channels() {
        let music = new_music();
        music.set(&[vec![0]]);
        // Should be padded to at least NUM_CHANNELS
        assert!(music.seqs.len() >= 2);
        assert_eq!(music.seqs[0], [0]);
        for seq in &music.seqs[1..] {
            assert!(seq.is_empty());
        }
    }
}
