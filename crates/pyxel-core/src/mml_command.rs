use crate::sound::SoundTone;

#[derive(Debug, Clone)]
pub enum MmlCommand {
    // Playback parameters
    Tempo {
        clocks_per_tick: u32,
    },
    Quantize {
        gate_ratio: f32,
    },

    // Voice shaping
    Tone {
        tone: SoundTone,
    },
    Volume {
        level: f32,
    },

    // Pitch modulation
    Transpose {
        semitone_offset: f32,
    },
    Detune {
        semitone_offset: f32,
    },

    // Envelope control
    Envelope {
        slot: u32,
    },
    EnvelopeSet {
        slot: u32,
        initial_level: f32,
        segments: Vec<(u32, f32)>, // (duration_ticks, level)
    },

    // Vibrato control
    Vibrato {
        slot: u32,
    },
    VibratoSet {
        slot: u32,
        delay_ticks: u32,
        period_ticks: u32,
        semitone_depth: f32,
    },

    // Glide control
    Glide {
        slot: u32,
    },
    GlideSet {
        slot: u32,
        semitone_offset: Option<f32>,
        duration_ticks: Option<u32>,
    },

    // Timeline events
    Note {
        midi_note: u32,
        duration_ticks: u32,
    },
    Rest {
        duration_ticks: u32,
    },

    // Repeat control
    RepeatStart,
    RepeatEnd {
        play_count: u32,
    },
}
