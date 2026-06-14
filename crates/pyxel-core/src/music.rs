use blip_buf::BlipBuf;

use crate::audio::Audio;
use crate::pyxel;
use crate::settings::{AUDIO_CLOCK_RATE, AUDIO_SAMPLE_RATE};

#[derive(Clone)]
pub struct Music {
    pub seqs: Vec<Vec<u32>>,
}

define_rc_type!(RcMusic, Music);

impl Music {
    pub fn new() -> RcMusic {
        new_rc_type!(Self { seqs: Vec::new() })
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

        let mut samples = vec![0; num_samples as usize];
        let mut blip_buf = BlipBuf::new(num_samples);
        blip_buf.set_rates(AUDIO_CLOCK_RATE as f64, AUDIO_SAMPLE_RATE as f64);

        {
            let _lock = crate::audio::AudioLock::lock();
            let pyxel_sounds = pyxel::sounds();
            let channels = pyxel::channels();
            for ch in channels.iter() {
                rc_mut!(ch).stop();
            }
            for (ch, seq) in channels.iter().zip(self.seqs.iter()) {
                let sounds = seq
                    .iter()
                    .map(|&index| pyxel_sounds[index as usize].clone())
                    .collect();
                rc_mut!(ch).play(sounds, None, true, false);
            }

            Audio::render_samples(channels.as_slice(), &mut blip_buf, &mut samples);
            for ch in channels.iter() {
                rc_mut!(ch).stop();
            }
        }
        Audio::save_samples(filename, &samples, use_ffmpeg.unwrap_or(false))
    }
}
