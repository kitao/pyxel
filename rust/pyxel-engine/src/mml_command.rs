#[derive(Clone, Debug)]
pub enum MmlCommand {
    RepeatStart,
    RepatEnd {
        count: u8,
    },

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
