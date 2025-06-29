use crate::settings::AUDIO_CLOCK_RATE;

#[derive(Clone, Debug)]
pub enum MmlCommand {
    RepeatStart, // [
    RepeatEnd {
        // ][repat_count:0 is infinite]
        count: u32,
    },

    Tempo {
        // T[bpm]
        clocks_per_tick: u32,
    },
    Quantize {
        // Q[gate:1-8]
        gate_ratio: f64,
    },

    Tone {
        // @[tone_index]
        tone_index: u8,
    },
    Volume {
        // V[volume:0-15]
        level: f64,
    },

    Transpose {
        // K[key_offset]
        semitone_offset: f64,
    },
    Detune {
        // Y[offset_cents]
        semitone_offset: f64,
    },

    Envelope {
        // @ENV[slot]
        slot: u8,
    },
    EnvelopeSet {
        // @ENV[slot] { initial_volume, duration_ticks1, target_volume1, ... }
        slot: u8,
        level: f64,
        segments: Vec<(u16, f64)>, // (duration_ticks, level)
    },

    Vibrato {
        // @VIB[slot]
        slot: u8,
    },
    VibratoSet {
        // @VIB[slot] { delay_ticks, frequency_chz, depth_cents }
        slot: u8,
        delay_ticks: u16,
        period_clocks: u32,
        semitone_depth: f64,
    },

    Glide {
        // @GLI[slot]
        slot: u8,
    },
    GlideSet {
        // @GLI[slot] { offset_cents, duration_ticks }
        slot: u8,
        semitone_offset: f64,
        duration_ticks: u16,
    },

    Note {
        // CDEFGAB
        midi_note: u8,
        duration_ticks: u16,
    },
    Rest {
        // R
        duration_ticks: u16,
    },
}

impl MmlCommand {
    pub fn bpm_to_cpt(bpm: impl Into<f64>) -> u32 {
        (AUDIO_CLOCK_RATE as f64 * 60.0 / bpm.into()).round() as u32
    }

    pub fn cents_to_semitones(cents: impl Into<f64>) -> f64 {
        cents.into() / 100.0
    }

    pub fn ticks_to_clocks(ticks: impl Into<u32>, clocks_per_tick: u32) -> u32 {
        clocks_per_tick * ticks.into()
    }

    pub fn freq_to_clocks(frequency_chz: impl Into<u32>) -> u32 {
        AUDIO_CLOCK_RATE * 100 / frequency_chz.into()
    }

    pub fn convert_segments(segments: &[(u16, f64)], clocks_per_tick: u32) -> Vec<(u32, f64)> {
        segments
            .iter()
            .map(|&(duration_ticks, level)| (clocks_per_tick * duration_ticks as u32, level))
            .collect()
    }
}
