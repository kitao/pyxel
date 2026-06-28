use std::fs::File;
use std::path::Path;

use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, TrackType};
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;
use symphonia::default::{get_codecs, get_probe};

#[derive(Clone)]
pub struct PcmData {
    pub samples: Vec<i16>,
}

pub fn load_pcm(filename: &str, target_rate: u32) -> Result<PcmData, String> {
    // Open and probe audio file
    let file = File::open(filename).map_err(|_| format!("Failed to open file '{filename}'"))?;
    let media_stream = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

    let mut hint = Hint::new();
    if let Some(ext) = Path::new(filename).extension().and_then(|s| s.to_str()) {
        hint.with_extension(ext);
    }

    let mut format_reader = get_probe()
        .probe(
            &hint,
            media_stream,
            FormatOptions::default(),
            MetadataOptions::default(),
        )
        .map_err(|_| format!("Failed to probe file '{filename}'"))?;

    let track = format_reader
        .default_track(TrackType::Audio)
        .ok_or_else(|| format!("No audio track found in file '{filename}'"))?;
    let track_id = track.id;
    let codec_params = track
        .codec_params
        .as_ref()
        .and_then(|params| params.audio())
        .ok_or_else(|| format!("No audio track found in file '{filename}'"))?
        .clone();
    // Trim encoder delay/padding (gapless); pinned so a default change can't shift decoded length
    let mut decoder = get_codecs()
        .make_audio_decoder(&codec_params, &AudioDecoderOptions::default().gapless(true))
        .map_err(|_| format!("Failed to decode file '{filename}'"))?;

    // Decode audio packets into mono samples
    let mut sample_rate = codec_params
        .sample_rate
        .ok_or_else(|| format!("Unknown sample rate in file '{filename}'"))?;
    let mut mono_samples: Vec<f32> = Vec::new();
    let mut sample_buf: Vec<f32> = Vec::new();

    loop {
        let packet = match format_reader.next_packet() {
            Ok(Some(packet)) => packet,
            Ok(None) => break,
            Err(_) => return Err(format!("Failed to read file '{filename}'")),
        };

        if packet.track_id != track_id {
            continue;
        }

        let decoded = match decoder.decode(&packet) {
            Ok(decoded) => decoded,
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(_) => {
                return Err(format!("Failed to decode file '{filename}'"));
            }
        };

        let spec = decoded.spec();
        sample_rate = spec.rate();
        let channels = spec.channels().count();
        decoded.copy_to_vec_interleaved::<f32>(&mut sample_buf);

        if channels == 1 {
            mono_samples.extend_from_slice(&sample_buf);
        } else {
            for frame in sample_buf.chunks(channels) {
                let sum: f32 = frame.iter().sum();
                mono_samples.push(sum / channels as f32);
            }
        }
    }

    // Resample and convert to i16
    if mono_samples.is_empty() {
        return Err(format!("No audio data found in file '{filename}'"));
    }

    let mono_samples = if sample_rate == target_rate {
        mono_samples
    } else {
        resample_linear(&mono_samples, sample_rate, target_rate)
    };

    let samples = mono_samples.into_iter().map(f32_to_i16).collect();
    Ok(PcmData { samples })
}

// Helpers

fn resample_linear(input: &[f32], src_rate: u32, dst_rate: u32) -> Vec<f32> {
    if input.is_empty() || src_rate == dst_rate {
        return input.to_vec();
    }

    let ratio = src_rate as f64 / dst_rate as f64;
    let out_len = ((input.len() as f64) / ratio).ceil() as usize;
    let mut out = Vec::with_capacity(out_len);

    for i in 0..out_len {
        let src_pos = i as f64 * ratio;
        let idx = src_pos.floor() as usize;
        let frac = (src_pos - idx as f64) as f32;
        let s0 = input.get(idx).copied().unwrap_or(0.0);
        let s1 = input.get(idx + 1).copied().unwrap_or(s0);
        out.push(s0 + (s1 - s0) * frac);
    }

    out
}

fn f32_to_i16(sample: f32) -> i16 {
    if sample >= 1.0 {
        i16::MAX
    } else if sample <= -1.0 {
        i16::MIN
    } else {
        (sample * i16::MAX as f32) as i16
    }
}
