use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::{get_codecs, get_probe};

#[derive(Clone)]
pub struct PcmData {
    pub samples: Vec<i16>,
}

pub fn load_pcm(path: &str, target_rate: u32) -> Result<PcmData, String> {
    let file = File::open(path).map_err(|_e| format!("Failed to open '{path}'"))?;
    let mss = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

    let mut hint = Hint::new();
    if let Some(ext) = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
    {
        hint.with_extension(ext);
    }

    let probed = get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|_e| format!("Failed to probe '{path}'"))?;
    let mut format = probed.format;

    let track = format
        .default_track()
        .ok_or_else(|| format!("No supported audio tracks in '{path}'"))?;
    let track_id = track.id;
    let codec_params = track.codec_params.clone();
    let mut decoder = get_codecs()
        .make(&codec_params, &DecoderOptions::default())
        .map_err(|_e| format!("Failed to create decoder for '{path}'"))?;

    let mut sample_rate = codec_params
        .sample_rate
        .ok_or_else(|| format!("Unknown sample rate in '{path}'"))?;
    let mut mono_samples: Vec<f32> = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::IoError(e)) if e.kind() == ErrorKind::UnexpectedEof => break,
            Err(SymphoniaError::ResetRequired) => {
                return Err(format!("Stream reset required for '{path}'"));
            }
            Err(_e) => return Err(format!("Failed to read audio packet in '{path}'")),
        };

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = match decoder.decode(&packet) {
            Ok(decoded) => decoded,
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(_e) => {
                return Err(format!("Failed to decode audio packet in '{path}'"));
            }
        };

        let spec = *decoded.spec();
        sample_rate = spec.rate;
        let mut sample_buf = SampleBuffer::<f32>::new(decoded.capacity() as u64, spec);
        sample_buf.copy_interleaved_ref(decoded);

        let channels = spec.channels.count();
        let data = sample_buf.samples();
        if channels == 1 {
            mono_samples.extend_from_slice(data);
        } else {
            for frame in data.chunks(channels) {
                let mut sum = 0.0f32;
                for sample in frame {
                    sum += *sample;
                }
                mono_samples.push(sum / channels as f32);
            }
        }
    }

    if mono_samples.is_empty() {
        return Err(format!("No audio samples decoded from '{path}'"));
    }

    let mono_samples = if sample_rate == target_rate {
        mono_samples
    } else {
        resample_linear(&mono_samples, sample_rate, target_rate)
    };

    let samples = mono_samples.into_iter().map(f32_to_i16).collect();
    Ok(PcmData { samples })
}

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
