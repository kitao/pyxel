use std::cmp::min;
use std::env::temp_dir;
use std::fs::{remove_file, write};
use std::process::Command;

use blip_buf::BlipBuf;
use hound::{SampleFormat, WavSpec, WavWriter};

use crate::adaptive_buffer::AdaptiveBuffer;
use crate::channel::SharedChannel;
use crate::pyxel::{Pyxel, CHANNELS};
use crate::settings::{
    AUDIO_CLOCKS_PER_SAMPLE, AUDIO_CLOCK_RATE, AUDIO_MIN_BUFFER_SAMPLES, AUDIO_RENDER_STEP_SAMPLES,
    AUDIO_SAMPLE_RATE,
};
use crate::utils;

pub struct Audio {}

struct AudioStreamRenderer {
    blip_buf: BlipBuf,
    adaptive_buffer: AdaptiveBuffer,
    channels: Vec<SharedChannel>,
}

impl AudioStreamRenderer {
    fn new() -> Self {
        let mut blip_buf = BlipBuf::new(AUDIO_MIN_BUFFER_SAMPLES);
        blip_buf.set_rates(AUDIO_CLOCK_RATE as f64, AUDIO_SAMPLE_RATE as f64);

        let adaptive_buffer = AdaptiveBuffer::new();

        Self {
            blip_buf,
            adaptive_buffer,
            channels: Vec::new(),
        }
    }

    fn render(&mut self, out: &mut [i16]) {
        let callback_start_time = std::time::Instant::now();
        self.snapshot_channels();
        let (adaptive_buffer, blip_buf, channels) = (
            &mut self.adaptive_buffer,
            &mut self.blip_buf,
            &self.channels,
        );
        adaptive_buffer.process(out, callback_start_time, |samples| {
            Audio::render_samples(channels, blip_buf, samples);
        });
    }

    fn snapshot_channels(&mut self) {
        self.channels.clear();

        let channels = CHANNELS.lock();
        self.channels.extend(channels.iter().cloned());
    }
}

impl Audio {
    pub fn new() -> Self {
        let mut stream_renderer = AudioStreamRenderer::new();

        pyxel_platform::start_audio(
            AUDIO_SAMPLE_RATE,
            AUDIO_MIN_BUFFER_SAMPLES,
            move |out: &mut [i16]| {
                stream_renderer.render(out);
            },
        );

        Self {}
    }

    pub fn render_samples(channels: &[SharedChannel], blip_buf: &mut BlipBuf, samples: &mut [i16]) {
        let mut channels: Vec<_> = channels.iter().map(|channel| channel.lock()).collect();
        let needs_blip = channels
            .iter()
            .any(|channel| channel.needs_blip_processing());
        let needs_pcm = channels.iter().any(|channel| channel.is_playing_pcm());
        let mut num_samples = blip_buf.read_samples(samples, false);

        if needs_blip {
            while num_samples < samples.len() {
                let target_samples =
                    ((samples.len() - num_samples) as u32).min(AUDIO_RENDER_STEP_SAMPLES);
                let clock_count = match blip_buf.clocks_needed(target_samples) {
                    0 => AUDIO_CLOCKS_PER_SAMPLE,
                    clocks => clocks,
                };

                for channel in &mut *channels {
                    if channel.needs_blip_processing() {
                        channel.process(Some(blip_buf), clock_count);
                    }
                }

                blip_buf.end_frame(clock_count);
                num_samples += blip_buf.read_samples(&mut samples[num_samples..], false);
            }
        } else if num_samples < samples.len() {
            samples[num_samples..].fill(0);
        }

        if needs_pcm {
            for channel in &mut *channels {
                if channel.is_playing_pcm() {
                    channel.mix_pcm(samples);
                }
            }
        }
    }

    pub fn save_samples(filename: &str, samples: &[i16], use_ffmpeg: bool) -> Result<(), String> {
        // Save WAV file
        let spec = WavSpec {
            channels: 1,
            sample_rate: AUDIO_SAMPLE_RATE,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };
        let filename = utils::add_file_extension(filename, ".wav");
        let mut writer = WavWriter::create(&filename, spec)
            .map_err(|_e| format!("Failed to open file '{filename}'"))?;

        for sample in samples {
            writer.write_sample(*sample).unwrap();
        }
        writer.finalize().unwrap();

        // Save MP4 file
        if !use_ffmpeg {
            return Ok(());
        }

        let image_data = include_bytes!("assets/pyxel_logo_152x64.png");
        let image_path = temp_dir().join("pyxel_mp4_image.png");
        let png_file = image_path.to_str().unwrap();
        let wav_file = &filename;
        let mp4_file = filename.replace(".wav", ".mp4");

        write(&image_path, image_data).unwrap();
        Command::new("ffmpeg")
            .arg("-loop")
            .arg("1")
            .arg("-i")
            .arg(png_file)
            .arg("-f")
            .arg("lavfi")
            .arg("-i")
            .arg("color=c=black:s=480x360")
            .arg("-i")
            .arg(wav_file)
            .arg("-filter_complex")
            .arg("[1][0]overlay=(W-w)/2:(H-h)/2")
            .arg("-c:v")
            .arg("libx264")
            .arg("-c:a")
            .arg("aac")
            .arg("-b:a")
            .arg("192k")
            .arg("-shortest")
            .arg(mp4_file)
            .arg("-y")
            .output()
            .map_err(|_e| "Failed to execute FFmpeg".to_string())?;

        remove_file(png_file).unwrap();
        Ok(())
    }
}

impl Pyxel {
    pub fn play(
        &self,
        channel_index: u32,
        sequence: &[u32],
        start_sec: Option<f32>,
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
            start_sec,
            should_loop,
            should_resume,
        );
    }

    pub fn play1(
        &self,
        channel_index: u32,
        sound_index: u32,
        start_sec: Option<f32>,
        should_loop: bool,
        should_resume: bool,
    ) {
        self.channels.lock()[channel_index as usize].lock().play1(
            self.sounds.lock()[sound_index as usize].clone(),
            start_sec,
            should_loop,
            should_resume,
        );
    }

    pub fn play_mml(
        &mut self,
        channel_index: u32,
        code: &str,
        start_sec: Option<f32>,
        should_loop: bool,
        should_resume: bool,
    ) -> Result<(), String> {
        self.channels.lock()[channel_index as usize]
            .lock()
            .play_mml(code, start_sec, should_loop, should_resume)
    }

    pub fn playm(&self, music_index: u32, start_sec: Option<f32>, should_loop: bool) {
        let num_channels = self.channels.lock().len();
        let musics = self.musics.lock();
        let music = musics[music_index as usize].lock();

        for i in 0..min(num_channels, music.seqs.len()) {
            self.play(
                i as u32,
                &music.seqs[i].lock(),
                start_sec,
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

    pub fn play_pos(&self, channel_index: u32) -> Option<(u32, f32)> {
        self.channels.lock()[channel_index as usize]
            .lock()
            .play_pos()
    }
}
