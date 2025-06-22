use crate::audio::Audio;
use crate::blip_buf::BlipBuf;
use crate::mml_command::MmlCommand;
use crate::pyxel::{CHANNELS, TONES};
use crate::settings::{
    AUDIO_CLOCK_RATE, AUDIO_SAMPLE_RATE, CLOCKS_PER_SPEED, DEFAULT_CLOCKS_PER_TICK,
    DEFAULT_SOUND_SPEED, EFFECT_FADEOUT, EFFECT_HALF_FADEOUT, EFFECT_NONE, EFFECT_QUARTER_FADEOUT,
    EFFECT_SLIDE, EFFECT_VIBRATO, MAX_VOLUME, SOUND_TICKS_PER_SECOND, TONE_NOISE, TONE_PULSE,
    TONE_SQUARE, TONE_TRIANGLE, VIBRATO_DEPTH_CENTS, VIBRATO_FREQUNCY_CHZ,
};
use crate::tone::Noise;
use crate::utils::simplify_string;

pub type Note = i16;
pub type ToneIndex = u16;
pub type Volume = u16;
pub type Effect = u16;
pub type Speed = u32;

#[derive(Clone)]
pub struct Sound {
    pub notes: Vec<Note>,
    pub tones: Vec<ToneIndex>,
    pub volumes: Vec<Volume>,
    pub effects: Vec<Effect>,
    pub speed: Speed,

    pub(crate) commands: Vec<MmlCommand>,
}

pub type SharedSound = shared_type!(Sound);

impl Sound {
    pub fn new() -> SharedSound {
        new_shared_type!(Self {
            notes: Vec::new(),
            tones: Vec::new(),
            volumes: Vec::new(),
            effects: Vec::new(),
            speed: DEFAULT_SOUND_SPEED,

            commands: Vec::new(),
        })
    }

    pub fn set(
        &mut self,
        note_str: &str,
        tone_str: &str,
        volume_str: &str,
        effect_str: &str,
        speed: Speed,
    ) {
        self.set_notes(note_str);
        self.set_tones(tone_str);
        self.set_volumes(volume_str);
        self.set_effects(effect_str);
        self.speed = speed;
    }

    pub fn set_notes(&mut self, note_str: &str) {
        let note_str = simplify_string(note_str);
        let mut chars = note_str.chars();
        self.notes.clear();

        while let Some(c) = chars.next() {
            let mut note: Note;
            if ('a'..='g').contains(&c) {
                note = match c {
                    'c' => 0,
                    'd' => 2,
                    'e' => 4,
                    'f' => 5,
                    'g' => 7,
                    'a' => 9,
                    'b' => 11,
                    _ => panic!("Invalid sound note '{c}'"),
                };

                let mut c = chars.next().unwrap_or(0 as char);
                if c == '#' {
                    note += 1;
                    c = chars.next().unwrap_or(0 as char);
                } else if c == '-' {
                    note -= 1;
                    c = chars.next().unwrap_or(0 as char);
                }

                if ('0'..='4').contains(&c) {
                    note += (c.to_digit(10).unwrap() as Note) * 12;
                } else {
                    panic!("Invalid sound note '{c}'");
                }
            } else if c == 'r' {
                note = -1;
            } else {
                panic!("Invalid sound note '{c}'");
            }
            self.notes.push(note);
        }
    }

    pub fn set_tones(&mut self, tone_str: &str) {
        self.tones.clear();
        for c in simplify_string(tone_str).chars() {
            let tone = match c {
                't' => TONE_TRIANGLE,
                's' => TONE_SQUARE,
                'p' => TONE_PULSE,
                'n' => TONE_NOISE,
                '0'..='9' => c.to_digit(10).unwrap() as ToneIndex,
                _ => panic!("Invalid sound tone '{c}'"),
            };
            self.tones.push(tone);
        }
    }

    pub fn set_volumes(&mut self, volume_str: &str) {
        self.volumes.clear();
        for c in simplify_string(volume_str).chars() {
            if ('0'..='7').contains(&c) {
                self.volumes.push(c.to_digit(10).unwrap() as Volume);
            } else {
                panic!("Invalid sound volume '{c}'");
            }
        }
    }

    pub fn set_effects(&mut self, effect_str: &str) {
        self.effects.clear();
        for c in simplify_string(effect_str).chars() {
            let effect = match c {
                'n' => EFFECT_NONE,
                's' => EFFECT_SLIDE,
                'v' => EFFECT_VIBRATO,
                'f' => EFFECT_FADEOUT,
                'h' => EFFECT_HALF_FADEOUT,
                'q' => EFFECT_QUARTER_FADEOUT,
                _ => panic!("Invalid sound effect '{c}'"),
            };
            self.effects.push(effect);
        }
    }

    pub fn save(&self, filename: &str, count: u32, ffmpeg: Option<bool>) {
        assert!(count > 0);

        let num_samples = self.calc_total_clocks() * AUDIO_SAMPLE_RATE / AUDIO_CLOCK_RATE * count;

        if num_samples == 0 {
            return;
        }

        let mut samples = vec![0; num_samples as usize];
        let mut blip_buf = BlipBuf::new(num_samples as usize);
        blip_buf.set_rates(AUDIO_CLOCK_RATE as f64, AUDIO_SAMPLE_RATE as f64);

        let channels = CHANNELS.lock();
        channels.iter().for_each(|channel| channel.lock().stop());

        {
            let mut channels: Vec<_> = channels.iter().map(|channel| channel.lock()).collect();
            let sounds = vec![new_shared_type!(self.clone())];
            channels[0].play(sounds, None, true, false);
        }

        Audio::render_samples(&channels, &mut blip_buf, &mut samples);
        Audio::save_samples(filename, &samples, ffmpeg.unwrap_or(false));
        channels.iter().for_each(|channel| channel.lock().stop());
    }

    pub(crate) fn calc_total_clocks(&self) -> u32 {
        if self.commands.is_empty() {
            return self.speed * CLOCKS_PER_SPEED * self.notes.len() as u32;
        }

        let mut total_clocks = 0;
        let mut clocks_per_tick = DEFAULT_CLOCKS_PER_TICK;

        for command in &self.commands {
            match command {
                MmlCommand::Tempo { bpm } => {
                    clocks_per_tick = (AUDIO_CLOCK_RATE as f64 * 60.0 / *bpm as f64).round() as u32;
                }
                MmlCommand::Note { duration_ticks, .. } | MmlCommand::Rest { duration_ticks } => {
                    total_clocks += *duration_ticks as u32 * clocks_per_tick;
                }
                _ => {}
            }
        }

        total_clocks
    }

    pub(crate) fn calc_tick_at_clock(&self, clock: u32) -> (u32, Option<u32>) {
        if self.commands.is_empty() {
            let total_ticks = self.speed * self.notes.len() as u32;
            let total_clocks = total_ticks * CLOCKS_PER_SPEED;

            if clock < total_clocks {
                return (clock / CLOCKS_PER_SPEED, None);
            } else {
                return (total_ticks, Some(clock - total_clocks));
            }
        }

        let mut remaining_clocks = clock;
        let mut consumed_ticks = 0;
        let mut clocks_per_tick = DEFAULT_CLOCKS_PER_TICK;

        for command in &self.commands {
            match command {
                MmlCommand::Tempo { bpm } => {
                    clocks_per_tick = (AUDIO_CLOCK_RATE as f64 * 60.0 / *bpm as f64).round() as u32;
                }
                MmlCommand::Note { duration_ticks, .. } | MmlCommand::Rest { duration_ticks } => {
                    let duration_clocks = *duration_ticks as u32 * clocks_per_tick;

                    if remaining_clocks < duration_clocks {
                        return (consumed_ticks + remaining_clocks / clocks_per_tick, None);
                    }

                    consumed_ticks += *duration_ticks as u32;
                    remaining_clocks -= duration_clocks;
                }
                _ => {}
            }
        }

        (consumed_ticks, Some(remaining_clocks))
    }

    pub(crate) fn calc_clock_at_tick(&self, tick: u32) -> (u32, Option<u32>) {
        if self.commands.is_empty() {
            let total_ticks = self.speed * self.notes.len() as u32;

            if tick < total_ticks {
                return (tick * CLOCKS_PER_SPEED, None);
            } else {
                return (total_ticks * CLOCKS_PER_SPEED, Some(tick - total_ticks));
            }
        }

        let mut remaining_ticks = tick;
        let mut consumed_clocks = 0;
        let mut clocks_per_tick = DEFAULT_CLOCKS_PER_TICK;

        for command in &self.commands {
            match command {
                MmlCommand::Tempo { bpm } => {
                    clocks_per_tick = (AUDIO_CLOCK_RATE as f64 * 60.0 / *bpm as f64).round() as u32;
                }
                MmlCommand::Note { duration_ticks, .. } | MmlCommand::Rest { duration_ticks } => {
                    if remaining_ticks < *duration_ticks as u32 {
                        return (consumed_clocks + remaining_ticks * clocks_per_tick, None);
                    }

                    consumed_clocks += *duration_ticks as u32 * clocks_per_tick;
                    remaining_ticks -= *duration_ticks as u32;
                }
                _ => {}
            }
        }

        (consumed_clocks, Some(remaining_ticks))
    }

    pub(crate) fn to_commands(&self) -> Vec<MmlCommand> {
        let mut commands = Vec::new();
        let mut prev_note = 0;
        let tones = TONES.lock();

        commands.push(MmlCommand::Tempo {
            bpm: (SOUND_TICKS_PER_SECOND * 60) as u16,
        });
        commands.push(MmlCommand::Quantize { gate_1_8: 8 });
        commands.push(MmlCommand::Transpose { key_offset: 0 });
        commands.push(MmlCommand::Detune { offset_cents: 0 });
        commands.push(MmlCommand::Envelope { slot: 0 });
        commands.push(MmlCommand::Vibrato { slot: 0 });
        commands.push(MmlCommand::Glide { slot: 0 });

        for (i, note) in self.notes.iter().enumerate() {
            // Rest
            if *note < 0 {
                commands.push(MmlCommand::Rest {
                    duration_ticks: self.speed as u16,
                });
                continue;
            }

            let tone_index = if self.tones.is_empty() {
                TONE_TRIANGLE
            } else {
                self.tones[i % self.tones.len()]
            };

            let volume = if self.volumes.is_empty() {
                MAX_VOLUME
            } else {
                self.volumes[i % self.volumes.len()]
            };

            let effect = if self.effects.is_empty() {
                EFFECT_NONE
            } else {
                self.effects[i % self.effects.len()]
            };

            // Volume
            commands.push(MmlCommand::Volume {
                volume_0_15: (volume as f64 * 15.0 / MAX_VOLUME as f64).round() as u8,
            });

            // Tone
            commands.push(MmlCommand::Tone {
                tone_index: tone_index as u8,
            });

            // Fade out
            if effect == EFFECT_FADEOUT {
                commands.push(MmlCommand::EnvelopeSet {
                    slot: 1,
                    volume_0_15: 15,
                    segments: vec![(self.speed as u16, 0)],
                });
            } else if effect == EFFECT_HALF_FADEOUT {
                let ticks = (self.speed as f64 / 2.0).round().max(1.0) as u16;
                commands.push(MmlCommand::EnvelopeSet {
                    slot: 1,
                    volume_0_15: 15,
                    segments: vec![(ticks, 127), (ticks, 0)],
                });
            } else if effect == EFFECT_QUARTER_FADEOUT {
                let ticks1 = (self.speed as f64 * 3.0 / 4.0).round().max(1.0) as u16;
                let ticks2 = (self.speed as f64 / 4.0).round().max(1.0) as u16;
                commands.push(MmlCommand::EnvelopeSet {
                    slot: 1,
                    volume_0_15: 15,
                    segments: vec![(ticks1, 127), (ticks2, 0)],
                });
            } else {
                commands.push(MmlCommand::Envelope { slot: 0 });
            }

            // Vibrato
            if effect == EFFECT_VIBRATO {
                commands.push(MmlCommand::VibratoSet {
                    slot: 1,
                    delay_ticks: 0,
                    frequency_chz: VIBRATO_FREQUNCY_CHZ as u16,
                    depth_cents: VIBRATO_DEPTH_CENTS as u16,
                });
            } else {
                commands.push(MmlCommand::Vibrato { slot: 0 });
            }

            // Slide
            if effect == EFFECT_SLIDE {
                commands.push(MmlCommand::GlideSet {
                    slot: 1,
                    offset_cents: (prev_note - *note) as i16,
                    duration_ticks: self.speed as u16,
                });
            } else {
                commands.push(MmlCommand::Glide { slot: 0 });
            }

            // Note
            let tone = tones[tone_index as usize].lock();
            let midi_note = *note + if tone.noise == Noise::Off { 36 } else { 60 };
            commands.push(MmlCommand::Note {
                midi_note: midi_note as u8,
                duration_ticks: self.speed as u16,
            });

            prev_note = *note;
        }

        println!("commmands: {:?}", commands);
        commands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sound_new() {
        let sound = Sound::new();
        assert_eq!(sound.lock().notes.len(), 0);
        assert_eq!(sound.lock().tones.len(), 0);
        assert_eq!(sound.lock().volumes.len(), 0);
        assert_eq!(sound.lock().effects.len(), 0);
        assert_eq!(sound.lock().speed, DEFAULT_SOUND_SPEED);
    }

    #[test]
    fn test_sound_set() {
        let sound = Sound::new();
        sound
            .lock()
            .set("c0d-0d0d#0", "tspn", "012345", "nsvfhq", 123);
        assert_eq!(&sound.lock().notes, &vec![0, 1, 2, 3]);
        assert_eq!(
            &sound.lock().tones,
            &vec![TONE_TRIANGLE, TONE_SQUARE, TONE_PULSE, TONE_NOISE]
        );
        assert_eq!(&sound.lock().volumes, &vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(
            &sound.lock().effects,
            &vec![
                EFFECT_NONE,
                EFFECT_SLIDE,
                EFFECT_VIBRATO,
                EFFECT_FADEOUT,
                EFFECT_HALF_FADEOUT,
                EFFECT_QUARTER_FADEOUT
            ]
        );
        assert_eq!(sound.lock().speed, 123);
    }

    #[test]
    fn test_sound_set_note() {
        let sound = Sound::new();
        sound
            .lock()
            .set_notes(" c 0 d # 1 r e 2 f 3 g 4 r a - 0 b 1 ");
        assert_eq!(&sound.lock().notes, &vec![0, 15, -1, 28, 41, 55, -1, 8, 23]);
    }

    #[test]
    fn test_sound_set_tone() {
        let sound = Sound::new();
        sound.lock().set_tones(" t s p n ");
        assert_eq!(
            &sound.lock().tones,
            &vec![TONE_TRIANGLE, TONE_SQUARE, TONE_PULSE, TONE_NOISE]
        );
    }

    #[test]
    fn test_sound_set_volume() {
        let sound = Sound::new();
        sound.lock().set_volumes(" 0 1 2 3 4 5 6 7 ");
        assert_eq!(&sound.lock().volumes, &vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_sound_set_effect() {
        let sound = Sound::new();
        sound.lock().set_effects(" n s v f h q");
        assert_eq!(
            &sound.lock().effects,
            &vec![
                EFFECT_NONE,
                EFFECT_SLIDE,
                EFFECT_VIBRATO,
                EFFECT_FADEOUT,
                EFFECT_HALF_FADEOUT,
                EFFECT_QUARTER_FADEOUT
            ]
        );
    }
}
