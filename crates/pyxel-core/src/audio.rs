#[cfg(pyxel_core)]
use std::env::temp_dir;
#[cfg(pyxel_core)]
use std::fs::{remove_file, write};
#[cfg(pyxel_core)]
use std::process::Command;

use blip_buf::BlipBuf;
#[cfg(pyxel_core)]
use hound::{SampleFormat, WavSpec, WavWriter};

use crate::channel::{Channel, RcChannel};
use crate::pyxel::{self, Pyxel};
use crate::settings::{
    AUDIO_BUFFER_SAMPLES, AUDIO_CLOCKS_PER_SAMPLE, AUDIO_CLOCK_RATE, AUDIO_RENDER_STEP_SAMPLES,
    AUDIO_SAMPLE_BITS, AUDIO_SAMPLE_RATE, NUM_CHANNELS,
};
use crate::sound::RcSound;
use crate::{platform, utils};

pub struct Audio;

pub struct AudioLock;

struct AudioStreamRenderer {
    blip_buf: BlipBuf,
}

// Audio locking

impl AudioLock {
    pub fn lock() -> Self {
        platform::lock_audio();
        Self
    }
}

impl Drop for AudioLock {
    fn drop(&mut self) {
        platform::unlock_audio();
    }
}

// Stream rendering

impl AudioStreamRenderer {
    fn new() -> Self {
        let mut blip_buf = BlipBuf::new(AUDIO_BUFFER_SAMPLES);
        blip_buf.set_rates(AUDIO_CLOCK_RATE as f64, AUDIO_SAMPLE_RATE as f64);

        Self { blip_buf }
    }

    fn render(&mut self, out: &mut [i16]) {
        let channels = pyxel::channels();
        Audio::render_samples(&channels, &mut self.blip_buf, out);
    }
}

// Audio output

impl Audio {
    pub fn start() {
        let mut stream_renderer = AudioStreamRenderer::new();

        platform::start_audio(
            AUDIO_SAMPLE_RATE,
            AUDIO_BUFFER_SAMPLES,
            move |out: &mut [i16]| {
                stream_renderer.render(out);
            },
        );
    }

    // Render generated samples and mix PCM playback.
    pub fn render_samples(channels: &[RcChannel], blip_buf: &mut BlipBuf, out: &mut [i16]) {
        if channels.len() <= NUM_CHANNELS as usize {
            let mut pcm_mix_starts = [usize::MAX; NUM_CHANNELS as usize];
            Self::render_samples_with_mix_starts(channels, blip_buf, out, &mut pcm_mix_starts);
        } else {
            let mut pcm_mix_starts = vec![usize::MAX; channels.len()];
            Self::render_samples_with_mix_starts(channels, blip_buf, out, &mut pcm_mix_starts);
        }
    }

    fn render_samples_with_mix_starts(
        channels: &[RcChannel],
        blip_buf: &mut BlipBuf,
        out: &mut [i16],
        pcm_mix_starts: &mut [usize],
    ) {
        let mut written = blip_buf.read_samples(out, false);
        if written > 0 {
            Self::mix_pcm_channels(channels, &mut out[..written]);
        }

        while written < out.len() {
            pcm_mix_starts.fill(usize::MAX);
            let mut target_samples = ((out.len() - written) as u32).min(AUDIO_RENDER_STEP_SAMPLES);
            let mut needs_blip = false;
            let mut needs_pcm = false;
            for (i, ch) in channels.iter().enumerate() {
                let mut channel = audio_mut!(ch);
                channel.prepare_pcm();
                needs_blip |= channel.needs_blip_processing();
                needs_pcm |= channel.is_playing_pcm();
                if channel.is_playing_pcm() {
                    pcm_mix_starts[i] = 0;
                    target_samples = target_samples
                        .min(channel.pcm_samples_until_mode_change(target_samples as usize) as u32);
                }
            }

            let step_start = written;

            let clocks = match blip_buf.clocks_needed(target_samples) {
                0 => AUDIO_CLOCKS_PER_SAMPLE,
                clocks => clocks,
            };
            if needs_blip {
                for (i, ch) in channels.iter().enumerate() {
                    let mut channel = audio_mut!(ch);
                    if channel.needs_blip_processing() {
                        let was_playing_pcm = channel.is_playing_pcm();
                        let elapsed_before = channel.total_elapsed_clocks();
                        channel.process(Some(blip_buf), clocks);
                        if !was_playing_pcm && channel.is_playing_pcm() {
                            let consumed_clocks = channel
                                .total_elapsed_clocks()
                                .saturating_sub(elapsed_before)
                                .min(u64::from(clocks))
                                as u32;
                            pcm_mix_starts[i] =
                                Self::samples_for_clocks(blip_buf, consumed_clocks, target_samples);
                            needs_pcm = true;
                        }
                    }
                }
            }
            blip_buf.end_frame(clocks);
            written += blip_buf.read_samples(&mut out[written..], false);

            if needs_pcm {
                Self::mix_pcm_channels_from(
                    channels,
                    &mut out[step_start..written],
                    pcm_mix_starts,
                );
            }
        }
    }

    fn samples_for_clocks(blip_buf: &BlipBuf, clocks: u32, max_samples: u32) -> usize {
        let mut low = 0;
        let mut high = max_samples;
        while low < high {
            let mid = u32::midpoint(low, high);
            if blip_buf.clocks_needed(mid) < clocks {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low as usize
    }

    fn mix_pcm_channels(channels: &[RcChannel], out: &mut [i16]) {
        for ch in channels {
            let mut channel = audio_mut!(ch);
            if channel.is_playing_pcm() {
                channel.mix_pcm(out);
            }
        }
    }

    fn mix_pcm_channels_from(channels: &[RcChannel], out: &mut [i16], starts: &[usize]) {
        for (ch, &start) in channels.iter().zip(starts) {
            if start >= out.len() {
                continue;
            }

            let mut channel = audio_mut!(ch);
            if channel.is_playing_pcm() {
                channel.mix_pcm(&mut out[start..]);
            }
        }
    }

    // File export

    pub(crate) fn duration_samples(duration_sec: f32) -> Result<u32, String> {
        if !duration_sec.is_finite() {
            return Err("duration_sec must be finite".to_string());
        }
        if duration_sec <= 0.0 {
            return Err("duration_sec must be greater than 0".to_string());
        }

        let num_samples = (duration_sec * AUDIO_SAMPLE_RATE as f32).round() as u32;
        if num_samples == 0 {
            return Err("duration_sec is too short to produce an audio sample".to_string());
        }
        Ok(num_samples)
    }

    #[cfg(pyxel_core)]
    pub fn save_samples(filename: &str, samples: &[i16], use_ffmpeg: bool) -> Result<(), String> {
        // Save WAV file
        let spec = WavSpec {
            channels: 1,
            sample_rate: AUDIO_SAMPLE_RATE,
            bits_per_sample: AUDIO_SAMPLE_BITS as u16,
            sample_format: SampleFormat::Int,
        };
        let filename = utils::add_file_extension(filename, ".wav");
        let save_err = || format!("Failed to save file '{filename}'");
        let mut writer = WavWriter::create(&filename, spec)
            .map_err(|_| format!("Failed to create file '{filename}'"))?;

        for &sample in samples {
            writer.write_sample(sample).map_err(|_| save_err())?;
        }
        writer.finalize().map_err(|_| save_err())?;

        // Save MP4 file
        if !use_ffmpeg {
            return Ok(());
        }

        let image_data = include_bytes!("assets/pyxel_logo_152x64.png");
        let image_path = temp_dir().join("pyxel_mp4_image.png");
        let png_file = image_path
            .to_str()
            .ok_or_else(|| "Failed to create temporary file path".to_string())?;
        let wav_file = &filename;
        let mp4_file = filename.replace(".wav", ".mp4");

        write(&image_path, image_data).map_err(|_| "Failed to save temporary file".to_string())?;
        let output = Command::new("ffmpeg")
            .args([
                "-loop",
                "1",
                "-i",
                png_file,
                "-f",
                "lavfi",
                "-i",
                "color=c=black:s=480x360",
                "-i",
                wav_file,
                "-filter_complex",
                "[1][0]overlay=(W-w)/2:(H-h)/2",
                "-c:v",
                "libx264",
                "-c:a",
                "aac",
                "-b:a",
                "192k",
                "-shortest",
                &mp4_file,
                "-y",
            ])
            .output();

        let _ = remove_file(png_file);
        let output = output.map_err(|_| "Failed to execute FFmpeg".to_string())?;
        if !output.status.success() {
            return Err("Failed to convert file with FFmpeg".to_string());
        }
        Ok(())
    }
}

impl Pyxel {
    // Playback

    pub fn play(
        &self,
        channel_index: u32,
        sequence: &[u32],
        start_sec: Option<f32>,
        should_loop: bool,
        should_resume: bool,
    ) -> Result<(), String> {
        Channel::validate_sec(start_sec)?;
        if sequence.is_empty() {
            return Ok(());
        }

        let pyxel_sounds = pyxel::sounds();
        let sounds: Vec<RcSound> = sequence
            .iter()
            .map(|&index| pyxel_sounds[index as usize].clone())
            .collect();

        let _lock = AudioLock::lock();
        audio_mut!(pyxel::channels()[channel_index as usize]).play(
            sounds,
            start_sec,
            should_loop,
            should_resume,
        )
    }

    pub fn play_sound(
        &self,
        channel_index: u32,
        sound_index: u32,
        start_sec: Option<f32>,
        should_loop: bool,
        should_resume: bool,
    ) -> Result<(), String> {
        let sound = pyxel::sounds()[sound_index as usize].clone();

        let _lock = AudioLock::lock();
        audio_mut!(pyxel::channels()[channel_index as usize]).play_sound(
            sound,
            start_sec,
            should_loop,
            should_resume,
        )
    }

    pub fn play_mml(
        &mut self,
        channel_index: u32,
        code: &str,
        start_sec: Option<f32>,
        should_loop: bool,
        should_resume: bool,
    ) -> Result<(), String> {
        let _lock = AudioLock::lock();
        audio_mut!(pyxel::channels()[channel_index as usize]).play_mml(
            code,
            start_sec,
            should_loop,
            should_resume,
        )
    }

    pub fn play_music(
        &self,
        music_index: u32,
        start_sec: Option<f32>,
        should_loop: bool,
    ) -> Result<(), String> {
        Channel::validate_sec(start_sec)?;
        let music_rc = pyxel::musics()[music_index as usize].clone();
        let music = audio_ref!(music_rc);
        let channels = pyxel::channels();
        let channel_count = channels.len();
        let pyxel_sounds = pyxel::sounds();

        let channel_sounds: Vec<(usize, Vec<RcSound>)> = music
            .seqs
            .iter()
            .enumerate()
            .take(channel_count)
            .filter(|(_, seq)| !seq.is_empty())
            .map(|(i, seq)| {
                let sounds = seq
                    .iter()
                    .map(|&index| {
                        pyxel_sounds
                            .get(index as usize)
                            .cloned()
                            .ok_or_else(|| "Music contains an invalid sound index".to_string())
                    })
                    .collect::<Result<_, _>>()?;
                Ok((i, sounds))
            })
            .collect::<Result<_, String>>()?;

        for (_, sounds) in &channel_sounds {
            Channel::validate_tones(sounds)?;
        }

        let _lock = AudioLock::lock();
        for (i, sounds) in channel_sounds {
            audio_mut!(channels[i]).play(sounds, start_sec, should_loop, false)?;
        }
        Ok(())
    }

    // Stop

    pub fn stop_channel(&self, channel_index: u32) {
        let _lock = AudioLock::lock();
        audio_mut!(pyxel::channels()[channel_index as usize]).stop();
    }

    pub fn stop_all_channels(&self) {
        let _lock = AudioLock::lock();
        for ch in pyxel::channels().iter() {
            audio_mut!(ch).stop();
        }
    }

    // Position

    pub fn play_position(&self, channel_index: u32) -> Option<(u32, f32)> {
        let _lock = AudioLock::lock();
        audio_mut!(pyxel::channels()[channel_index as usize]).play_position()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::channel::Channel;
    use crate::pcm_decoder::PcmData;
    use crate::sound::Sound;

    fn note_sound_with_note(note: i8) -> RcSound {
        let sound = Sound::new();
        {
            let mut sound = audio_mut!(sound);
            sound.notes = vec![note];
            sound.tones = vec![1];
            sound.speed = 1;
        }
        sound
    }

    fn note_sound() -> RcSound {
        note_sound_with_note(0)
    }

    fn silent_pcm_sound(num_samples: usize) -> RcSound {
        let sound = Sound::new();
        audio_mut!(sound).pcm = Some(PcmData {
            samples: vec![0; num_samples],
        });
        sound
    }

    fn render_in_chunks(
        sounds: Vec<RcSound>,
        num_samples: usize,
        chunk_samples: usize,
    ) -> Vec<i16> {
        let channel = Channel::new();
        audio_mut!(channel)
            .play(sounds, None, false, false)
            .unwrap();

        let mut blip_buf = BlipBuf::new(num_samples as u32);
        blip_buf.set_rates(AUDIO_CLOCK_RATE as f64, AUDIO_SAMPLE_RATE as f64);
        let mut samples = vec![0; num_samples];
        for chunk in samples.chunks_mut(chunk_samples) {
            Audio::render_samples(std::slice::from_ref(&channel), &mut blip_buf, chunk);
        }
        samples
    }

    fn render(sounds: Vec<RcSound>, num_samples: usize) -> Vec<i16> {
        render_in_chunks(sounds, num_samples, num_samples)
    }

    #[test]
    fn test_synth_to_pcm_transition_starts_at_boundary_sample() {
        let pcm = Sound::new();
        audio_mut!(pcm).pcm = Some(PcmData {
            samples: vec![30_000; 256],
        });

        let samples = render(vec![note_sound(), pcm], 512);
        let pcm_start = samples.iter().position(|&sample| sample > 2_000).unwrap();
        let note_clocks = AUDIO_CLOCK_RATE / crate::settings::SOUND_TICKS_PER_SECOND;
        let expected_start = (u64::from(note_clocks) * u64::from(AUDIO_SAMPLE_RATE))
            .div_ceil(u64::from(AUDIO_CLOCK_RATE)) as usize;

        assert_eq!(pcm_start, expected_start);
    }

    #[test]
    fn test_short_pcm_transition_is_render_step_invariant() {
        let pcm = Sound::new();
        audio_mut!(pcm).pcm = Some(PcmData {
            samples: vec![30_000],
        });
        let sounds = vec![note_sound(), pcm, note_sound_with_note(12)];

        let whole = render_in_chunks(sounds.clone(), 512, 512);
        let split = render_in_chunks(sounds, 512, 32);

        assert_eq!(
            whole
                .iter()
                .zip(&split)
                .position(|(left, right)| left != right),
            None
        );
    }

    #[test]
    fn test_looping_empty_pcm_stops_without_hanging() {
        let channel = Channel::new();
        audio_mut!(channel)
            .play(vec![silent_pcm_sound(0)], None, true, false)
            .unwrap();
        let mut blip_buf = BlipBuf::new(64);
        blip_buf.set_rates(AUDIO_CLOCK_RATE as f64, AUDIO_SAMPLE_RATE as f64);
        let mut samples = [0; 64];

        Audio::render_samples(std::slice::from_ref(&channel), &mut blip_buf, &mut samples);

        assert!(!audio_ref!(channel).is_playing_pcm());
        assert!(samples.iter().all(|&sample| sample == 0));
    }

    #[test]
    fn test_render_supports_more_than_runtime_channels() {
        let channels: Vec<_> = (0..=NUM_CHANNELS)
            .map(|_| {
                let channel = Channel::new();
                audio_mut!(channel)
                    .play(vec![silent_pcm_sound(64)], None, false, false)
                    .unwrap();
                channel
            })
            .collect();
        let mut blip_buf = BlipBuf::new(64);
        blip_buf.set_rates(AUDIO_CLOCK_RATE as f64, AUDIO_SAMPLE_RATE as f64);
        let mut samples = [0; 64];

        Audio::render_samples(&channels, &mut blip_buf, &mut samples);

        assert!(samples.iter().all(|&sample| sample == 0));
    }
}
