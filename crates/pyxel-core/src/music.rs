use blip_buf::BlipBuf;

use crate::audio::Audio;
use crate::channel::Channel;
use crate::pyxel;
use crate::settings::{AUDIO_CLOCK_RATE, AUDIO_SAMPLE_RATE, NUM_CHANNELS};

#[derive(Clone)]
pub struct Music {
    pub seqs: Vec<Vec<u32>>,
}

define_audio_type!(RcMusic, Music);

impl Music {
    pub fn new() -> RcMusic {
        new_audio_type!(Self { seqs: Vec::new() })
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

        let pyxel_sounds = pyxel::sounds();
        let render_channels: Vec<_> = self
            .seqs
            .iter()
            .enumerate()
            .map(|(i, seq)| {
                let sounds = seq
                    .iter()
                    .map(|&index| pyxel_sounds[index as usize].clone())
                    .collect();
                let stagger_sec = channel_stagger_clocks(i) as f32 / AUDIO_CLOCK_RATE as f32;
                let channel = Channel::new();
                audio_mut!(channel).play(sounds, Some(stagger_sec), true, false);
                channel
            })
            .collect();
        drop(pyxel_sounds);

        if !render_channels.is_empty() {
            Audio::render_samples(&render_channels, &mut blip_buf, &mut samples);
        }
        Audio::save_samples(filename, &samples, use_ffmpeg.unwrap_or(false))
    }
}

// Stagger music channel starts so phase-aligned waveforms do not mask sound effects.
pub(crate) fn channel_stagger_clocks(channel_index: usize) -> u32 {
    let step = AUDIO_CLOCK_RATE / 500; // 2ms
    let cycle_index = (channel_index % (2 * NUM_CHANNELS as usize)) as u32;
    (cycle_index % NUM_CHANNELS) * step + (cycle_index / NUM_CHANNELS) * (step / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_stagger_interleaves_distinct_offsets() {
        let step = AUDIO_CLOCK_RATE / 500;
        let expected: Vec<u32> = [0, 2, 4, 6, 1, 3, 5, 7, 0, 2]
            .iter()
            .map(|&half_steps| half_steps * step / 2)
            .collect();
        let actual: Vec<u32> = (0..10).map(channel_stagger_clocks).collect();
        assert_eq!(actual, expected);
    }
}
