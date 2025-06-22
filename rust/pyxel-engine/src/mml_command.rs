use crate::settings::AUDIO_CLOCK_RATE;

#[derive(Clone, Debug)]
pub enum MmlCommand {
    Tempo {
        bpm: u16,
    },
    Quantize {
        gate_1_8: u8,
    },

    Tone {
        tone_index: u8,
    },
    Volume {
        volume_0_15: u8,
    },

    Transpose {
        key_offset: i8,
    },
    Detune {
        offset_cents: i8,
    },

    Envelope {
        slot: u8,
    },
    EnvelopeSet {
        slot: u8,
        volume_0_15: u8,
        segments: Vec<(u16, u8)>, // (duration_ticks, volume_0_15)
    },

    Vibrato {
        slot: u8,
    },
    VibratoSet {
        slot: u8,
        delay_ticks: u16,
        frequency_chz: u16,
        depth_cents: u16,
    },

    Glide {
        slot: u8,
    },
    GlideSet {
        slot: u8,
        offset_cents: i16,
        duration_ticks: u16,
    },

    Note {
        midi_note: u8,
        duration_ticks: u16,
    },
    Rest {
        duration_ticks: u16,
    },
}

impl MmlCommand {
    pub fn bpm_to_cpt(bpm: u16) -> u32 {
        (AUDIO_CLOCK_RATE as f64 * 60.0 / bpm as f64).round() as u32
    }

    pub fn gate_to_ratio(gate: u8) -> f64 {
        (gate as f64 / 8.0).round()
    }

    pub fn volume_to_level(volume: u8) -> f64 {
        (volume as f64 / 15.0).round()
    }

    pub fn cents_to_semitones(cents: impl Into<f64>) -> f64 {
        (cents.into() / 100.0).round()
    }

    pub fn ticks_to_clocks(ticks: u16, clocks_per_tick: u32) -> u32 {
        ticks as u32 * clocks_per_tick
    }

    pub fn convert_segments(segments: &[(u16, u8)], clocks_per_tick: u32) -> Vec<(u32, f64)> {
        segments
            .iter()
            .map(|&(duration_ticks, volume)| {
                (
                    Self::ticks_to_clocks(duration_ticks, clocks_per_tick),
                    Self::volume_to_level(volume),
                )
            })
            .collect()
    }
}
