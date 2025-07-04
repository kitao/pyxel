use crate::audio::Audio;
use crate::blip_buf::BlipBuf;
use crate::mml_command::MmlCommand;
use crate::mml_parser::{calc_commands_sec, parse_mml};
use crate::old_mml_parser::parse_old_mml;
use crate::pyxel::{CHANNELS, TONES};
use crate::settings::{
    AUDIO_CLOCK_RATE, AUDIO_SAMPLE_RATE, DEFAULT_SOUND_SPEED, EFFECT_FADEOUT, EFFECT_HALF_FADEOUT,
    EFFECT_NONE, EFFECT_QUARTER_FADEOUT, EFFECT_SLIDE, EFFECT_VIBRATO, MAX_VOLUME, TONE_NOISE,
    TONE_PULSE, TONE_SQUARE, TONE_TRIANGLE, VIBRATO_DEPTH_CENTS, VIBRATO_PERIOD_TICKS,
};
use crate::tone::Noise;
use crate::utils::simplify_string;
use crate::SOUND_TICKS_PER_SECOND;

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

    pub fn mml(&mut self, code: &str) {
        self.commands = parse_mml(code);
    }

    pub fn mml0(&mut self) {
        self.commands.clear();
    }

    pub fn old_mml(&mut self, code: &str) {
        self.commands = parse_old_mml(code);
    }

    pub fn save(&self, filename: &str, duration_sec: f32, use_ffmpeg: Option<bool>) {
        assert!(duration_sec > 0.0);

        let num_samples = (duration_sec * AUDIO_SAMPLE_RATE as f32).round() as u32;
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
        Audio::save_samples(filename, &samples, use_ffmpeg.unwrap_or(false));
        channels.iter().for_each(|channel| channel.lock().stop());
    }

    pub fn total_sec(&self) -> Option<f32> {
        if self.commands.is_empty() {
            Some(self.notes.len() as f32 * self.speed as f32 / SOUND_TICKS_PER_SECOND as f32)
        } else {
            calc_commands_sec(&self.commands)
        }
    }

    pub(crate) fn to_commands(&self) -> Vec<MmlCommand> {
        let mut commands = Vec::new();
        let mut prev_note = 0;
        let tones = TONES.lock();

        // TODO: Remove redundant commands

        commands.push(MmlCommand::Tempo {
            clocks_per_tick: AUDIO_CLOCK_RATE / SOUND_TICKS_PER_SECOND,
        });
        commands.push(MmlCommand::Quantize { gate_ratio: 1.0 });
        commands.push(MmlCommand::Transpose {
            semitone_offset: 0.0,
        });
        commands.push(MmlCommand::Detune {
            semitone_offset: 0.0,
        });
        commands.push(MmlCommand::Envelope { slot: 0 });
        commands.push(MmlCommand::Vibrato { slot: 0 });
        commands.push(MmlCommand::Glide { slot: 0 });

        for (i, note) in self.notes.iter().enumerate() {
            // Rest
            if *note < 0 {
                commands.push(MmlCommand::Rest {
                    duration_ticks: self.speed,
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
                level: volume as f32 / MAX_VOLUME as f32,
            });

            // Tone
            commands.push(MmlCommand::Tone {
                tone_index: tone_index as u32,
            });

            // Fade out
            if effect == EFFECT_FADEOUT {
                commands.push(MmlCommand::EnvelopeSet {
                    slot: 1,
                    initial_level: 1.0,
                    segments: vec![(self.speed, 0.0)],
                });
            } else if effect == EFFECT_HALF_FADEOUT {
                let ticks2 = (self.speed as f32 / 2.0).round() as u32;
                let ticks1 = self.speed - ticks2;
                commands.push(MmlCommand::EnvelopeSet {
                    slot: 1,
                    initial_level: 1.0,
                    segments: vec![(ticks1, 1.0), (ticks2, 0.0)],
                });
            } else if effect == EFFECT_QUARTER_FADEOUT {
                let ticks2 = (self.speed as f32 / 4.0).round() as u32;
                let ticks1 = self.speed - ticks2;
                commands.push(MmlCommand::EnvelopeSet {
                    slot: 1,
                    initial_level: 1.0,
                    segments: vec![(ticks1, 1.0), (ticks2, 0.0)],
                });
            } else {
                commands.push(MmlCommand::Envelope { slot: 0 });
            }

            // Vibrato
            if effect == EFFECT_VIBRATO {
                commands.push(MmlCommand::VibratoSet {
                    slot: 1,
                    delay_ticks: 0,
                    period_ticks: VIBRATO_PERIOD_TICKS,
                    semitone_depth: VIBRATO_DEPTH_CENTS as f32 / 100.0,
                });
            } else {
                commands.push(MmlCommand::Vibrato { slot: 0 });
            }

            // Slide
            if effect == EFFECT_SLIDE {
                commands.push(MmlCommand::GlideSet {
                    slot: 1,
                    semitone_offset: (prev_note - *note) as f32,
                    duration_ticks: self.speed,
                });
            } else {
                commands.push(MmlCommand::Glide { slot: 0 });
            }

            // Note
            let tone = tones[tone_index as usize].lock();
            let midi_note = *note + if tone.noise == Noise::Off { 36 } else { 60 };
            commands.push(MmlCommand::Note {
                midi_note: midi_note as u32,
                duration_ticks: self.speed,
            });

            prev_note = *note;
        }

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
