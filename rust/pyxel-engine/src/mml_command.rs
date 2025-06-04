macro_rules! ascii_to_i16 {
    ($a:expr) => {
        $a as i16
    };
    ($a:expr, $b:expr) => {
        (($a as i16) << 8) + ($b as i16)
    };
}

#[repr(i16)]
pub enum MmlCommandId {
    Note = ascii_to_i16!(b'N'),
    ContinueNote = ascii_to_i16!(b'&', b'N'),
    Rest = ascii_to_i16!(b'R'),
    Quantize = ascii_to_i16!(b'Q'),
    Volume = ascii_to_i16!(b'V'),
    Tempo = ascii_to_i16!(b'T'),
    Tone = ascii_to_i16!(b'@'),
    Detune = ascii_to_i16!(b'D', b':'),
    EnvelopeSet = ascii_to_i16!(b'X', b':'),
    Envelope = ascii_to_i16!(b'X'),
    VibratoParams = ascii_to_i16!(b'V', b':'),
    VibratoOn = ascii_to_i16!(b'V', b'+'),
    VibratoOff = ascii_to_i16!(b'V', b'-'),
    GlideParams = ascii_to_i16!(b'G', b':'),
    GlideOn = ascii_to_i16!(b'G', b'+'),
    GlideOff = ascii_to_i16!(b'G', b'-'),
}

pub enum MmlCommand {
    Note {
        midi_note: u8,
        duration_ticks: u16,
    },
    ContinueNote {
        midi_note: u8,
        duration_ticks: u16,
    },
    Rest {
        duration_ticks: u16,
    },

    Quantize {
        value: u8,
    },
    Volume {
        value: u8,
    },
    Tempo {
        bpm: u16,
    },

    Tone {
        tone_id: u8,
    },
    Detune {
        cents: i16,
    },

    EnvelopeSet {
        pattern_id: u8,
        initial_volume: u8,
        segments: Vec<(u16, u8)>, // (duration_ticks, target_volume)
    },
    Envelope {
        pattern_id: u8,
    },

    VibratoParams {
        delay_ticks: u16,
        frequency_chz: u16,
        depth_cents: i16,
    },
    VibratoOn,
    VibratoOff,

    GlideParams {
        offset_cents: i16,
        time_ticks: u16,
    },
    GlideOn,
    GlideOff,
}

impl MmlCommand {
    pub fn parse(data: &[i16]) -> Result<(Self, usize), ()> {
        let id = *data.first().ok_or(())?;

        if id == MmlCommandId::Note as i16 {
            Ok((
                Self::Note {
                    midi_note: data[1] as u8,
                    duration_ticks: data[2] as u16,
                },
                3,
            ))
        } else if id == MmlCommandId::ContinueNote as i16 {
            Ok((
                Self::ContinueNote {
                    midi_note: data[1] as u8,
                    duration_ticks: data[2] as u16,
                },
                3,
            ))
        } else if id == MmlCommandId::Rest as i16 {
            Ok((
                Self::Rest {
                    duration_ticks: data[1] as u16,
                },
                2,
            ))
        } else if id == MmlCommandId::Quantize as i16 {
            Ok((
                Self::Quantize {
                    value: data[1] as u8,
                },
                2,
            ))
        } else if id == MmlCommandId::Volume as i16 {
            Ok((
                Self::Volume {
                    value: data[1] as u8,
                },
                2,
            ))
        } else if id == MmlCommandId::Tempo as i16 {
            Ok((
                Self::Tempo {
                    bpm: data[1] as u16,
                },
                2,
            ))
        } else if id == MmlCommandId::Tone as i16 {
            Ok((
                Self::Tone {
                    tone_id: data[1] as u8,
                },
                2,
            ))
        } else if id == MmlCommandId::Detune as i16 {
            Ok((Self::Detune { cents: data[1] }, 2))
        } else if id == MmlCommandId::EnvelopeSet as i16 {
            let pattern_id = data[1] as u8;
            let initial_volume = data[2] as u8;
            let num_segments = data[3] as usize;
            let mut segments = Vec::with_capacity(num_segments);
            for i in 0..num_segments {
                let dur = data[4 + i * 2] as u16;
                let vol = data[4 + i * 2 + 1] as u8;
                segments.push((dur, vol));
            }
            Ok((
                Self::EnvelopeSet {
                    pattern_id,
                    initial_volume,
                    segments,
                },
                4 + num_segments * 2,
            ))
        } else if id == MmlCommandId::Envelope as i16 {
            Ok((
                Self::Envelope {
                    pattern_id: data[1] as u8,
                },
                2,
            ))
        } else if id == MmlCommandId::VibratoParams as i16 {
            Ok((
                Self::VibratoParams {
                    delay_ticks: data[1] as u16,
                    frequency_chz: data[2] as u16,
                    depth_cents: data[3],
                },
                4,
            ))
        } else if id == MmlCommandId::VibratoOn as i16 {
            Ok((Self::VibratoOn, 1))
        } else if id == MmlCommandId::VibratoOff as i16 {
            Ok((Self::VibratoOff, 1))
        } else if id == MmlCommandId::GlideParams as i16 {
            Ok((
                Self::GlideParams {
                    offset_cents: data[1],
                    time_ticks: data[2] as u16,
                },
                3,
            ))
        } else if id == MmlCommandId::GlideOn as i16 {
            Ok((Self::GlideOn, 1))
        } else if id == MmlCommandId::GlideOff as i16 {
            Ok((Self::GlideOff, 1))
        } else {
            Err(())
        }
    }
}
