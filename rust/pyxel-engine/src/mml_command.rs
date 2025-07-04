#[derive(Clone, Debug)]
pub enum MmlCommand {
    RepeatStart,
    RepeatEnd {
        repeat_count: u32,
    },

    Tempo {
        clocks_per_tick: u32,
    },
    Quantize {
        gate_ratio: f32,
    },

    Tone {
        tone_index: u32,
    },
    Volume {
        level: f32,
    },

    Transpose {
        semitone_offset: f32,
    },
    Detune {
        semitone_offset: f32,
    },

    Envelope {
        slot: u32,
    },
    EnvelopeSet {
        slot: u32,
        initial_level: f32,
        segments: Vec<(u32, f32)>, // (duration_ticks, level)
    },

    Vibrato {
        slot: u32,
    },
    VibratoSet {
        slot: u32,
        delay_ticks: u32,
        period_ticks: u32,
        semitone_depth: f32,
    },

    Glide {
        slot: u32,
    },
    GlideSet {
        slot: u32,
        semitone_offset: f32,
        duration_ticks: u32,
    },

    Note {
        midi_note: u32,
        duration_ticks: u32,
    },
    Rest {
        duration_ticks: u32,
    },
}
