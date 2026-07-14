use std::sync::Arc;

use blip_buf::BlipBuf;

use crate::audio::Audio;
use crate::channel::Channel;
use crate::mml_command::MmlCommand;
use crate::mml_parser::{parse_mml, total_duration_sec};
use crate::old_mml_parser::parse_old_mml;
use crate::pcm_decoder::{load_pcm, PcmData};
use crate::pyxel;
use crate::settings::{
    AUDIO_CLOCK_RATE, AUDIO_SAMPLE_RATE, DEFAULT_SOUND_SPEED, EFFECT_FADEOUT, EFFECT_HALF_FADEOUT,
    EFFECT_NONE, EFFECT_QUARTER_FADEOUT, EFFECT_SLIDE, EFFECT_VIBRATO, MAX_VOLUME,
    SOUND_TICKS_PER_SECOND, TONE_NOISE, TONE_PULSE, TONE_SQUARE, TONE_TRIANGLE,
    VIBRATO_DEPTH_CENTS, VIBRATO_PERIOD_TICKS,
};
use crate::tone::ToneMode;
use crate::utils::simplify_string;

pub type SoundNote = i8;
pub type SoundTone = u8;
pub type SoundVolume = u8;
pub type SoundEffect = u8;
pub type SoundSpeed = u16;

#[derive(Clone, PartialEq)]
struct LegacyCommandSource {
    notes: Vec<SoundNote>,
    tones: Vec<SoundTone>,
    volumes: Vec<SoundVolume>,
    effects: Vec<SoundEffect>,
    speed: SoundSpeed,
    tone_modes: Vec<ToneMode>,
}

#[derive(Clone)]
enum CommandCacheSource {
    Legacy(LegacyCommandSource),
    Mml(u64),
}

#[derive(Clone)]
struct CommandCache {
    source: CommandCacheSource,
    commands: Arc<[MmlCommand]>,
}

#[derive(Clone)]
pub struct Sound {
    pub notes: Vec<SoundNote>,
    pub tones: Vec<SoundTone>,
    pub volumes: Vec<SoundVolume>,
    pub effects: Vec<SoundEffect>,
    pub speed: SoundSpeed,

    pub(crate) commands: Vec<MmlCommand>,
    pub(crate) pcm: Option<PcmData>,
    command_revision: u64,
    command_cache: Option<CommandCache>,
}

define_audio_type!(RcSound, Sound);

impl Sound {
    pub fn new() -> RcSound {
        new_audio_type!(Self {
            notes: Vec::new(),
            tones: Vec::new(),
            volumes: Vec::new(),
            effects: Vec::new(),
            speed: DEFAULT_SOUND_SPEED,

            commands: Vec::new(),
            pcm: None,
            command_revision: 0,
            command_cache: None,
        })
    }

    // Configuration

    pub fn set(
        &mut self,
        note_str: &str,
        tone_str: &str,
        volume_str: &str,
        effect_str: &str,
        speed: SoundSpeed,
    ) -> Result<(), String> {
        Self::validate_speed(speed)?;
        self.set_notes(note_str)?;
        self.set_tones(tone_str)?;
        self.set_volumes(volume_str)?;
        self.set_effects(effect_str)?;
        self.speed = speed;
        Ok(())
    }

    pub fn validate_speed(speed: SoundSpeed) -> Result<(), String> {
        if speed == 0 {
            return Err("speed must be greater than 0".to_string());
        }
        Ok(())
    }

    // Parse note symbols into semitone offsets.
    pub fn set_notes(&mut self, note_str: &str) -> Result<(), String> {
        let note_str = simplify_string(note_str);
        let mut chars = note_str.chars();
        self.notes.clear();

        while let Some(c) = chars.next() {
            let mut note: SoundNote;
            if ('a'..='g').contains(&c) {
                note = match c {
                    'c' => 0,
                    'd' => 2,
                    'e' => 4,
                    'f' => 5,
                    'g' => 7,
                    'a' => 9,
                    'b' => 11,
                    _ => return Err(format!("Invalid sound note '{c}'")),
                };

                let mut c = chars.next().unwrap_or('\0');
                if c == '#' {
                    note += 1;
                    c = chars.next().unwrap_or('\0');
                } else if c == '-' {
                    note -= 1;
                    c = chars.next().unwrap_or('\0');
                }

                if ('0'..='4').contains(&c) {
                    note += (c.to_digit(10).unwrap() as SoundNote) * 12;
                } else {
                    return Err(format!("Invalid sound note '{c}'"));
                }
            } else if c == 'r' {
                note = -1;
            } else {
                return Err(format!("Invalid sound note '{c}'"));
            }
            self.notes.push(note);
        }
        Ok(())
    }

    pub fn set_tones(&mut self, tone_str: &str) -> Result<(), String> {
        self.tones.clear();
        for c in simplify_string(tone_str).chars() {
            let tone = match c {
                't' => TONE_TRIANGLE,
                's' => TONE_SQUARE,
                'p' => TONE_PULSE,
                'n' => TONE_NOISE,
                '0'..='9' => c.to_digit(10).unwrap() as SoundTone,
                _ => return Err(format!("Invalid sound tone '{c}'")),
            };
            self.tones.push(tone);
        }
        Ok(())
    }

    pub fn set_volumes(&mut self, volume_str: &str) -> Result<(), String> {
        self.volumes.clear();
        for c in simplify_string(volume_str).chars() {
            if ('0'..='7').contains(&c) {
                self.volumes.push(c.to_digit(10).unwrap() as SoundVolume);
            } else {
                return Err(format!("Invalid sound volume '{c}'"));
            }
        }
        Ok(())
    }

    pub fn set_effects(&mut self, effect_str: &str) -> Result<(), String> {
        self.effects.clear();
        for c in simplify_string(effect_str).chars() {
            let effect = match c {
                'n' => EFFECT_NONE,
                's' => EFFECT_SLIDE,
                'v' => EFFECT_VIBRATO,
                'f' => EFFECT_FADEOUT,
                'h' => EFFECT_HALF_FADEOUT,
                'q' => EFFECT_QUARTER_FADEOUT,
                _ => return Err(format!("Invalid sound effect '{c}'")),
            };
            self.effects.push(effect);
        }
        Ok(())
    }

    // MML & PCM

    pub fn set_mml(&mut self, code: &str) -> Result<(), String> {
        self.clear_pcm();
        self.commands = parse_mml(code)?;
        self.command_revision = self.command_revision.wrapping_add(1);
        Ok(())
    }

    pub fn clear_mml(&mut self) {
        self.commands.clear();
        self.command_revision = self.command_revision.wrapping_add(1);
    }

    pub fn old_mml(&mut self, code: &str) -> Result<(), String> {
        self.clear_pcm();
        self.commands = parse_old_mml(code)?;
        self.command_revision = self.command_revision.wrapping_add(1);
        Ok(())
    }

    pub fn load_pcm(&mut self, filename: &str) -> Result<(), String> {
        self.clear_mml();

        let pcm = load_pcm(filename, AUDIO_SAMPLE_RATE)?;
        self.pcm = Some(pcm);
        Ok(())
    }

    pub fn clear_pcm(&mut self) {
        self.pcm = None;
    }

    // Export and duration

    #[cfg(pyxel_core)]
    pub fn save(
        &self,
        filename: &str,
        duration_sec: f32,
        use_ffmpeg: Option<bool>,
    ) -> Result<(), String> {
        let num_samples = Audio::duration_samples(duration_sec)?;

        let render_sound = new_audio_type!(self.clone());
        let render_channel = Channel::new();
        audio_mut!(render_channel).play(vec![render_sound], None, true, false)?;

        let mut samples = vec![0; num_samples as usize];
        let mut blip_buf = BlipBuf::new(num_samples);
        blip_buf.set_rates(AUDIO_CLOCK_RATE as f64, AUDIO_SAMPLE_RATE as f64);

        Audio::render_samples(&[render_channel], &mut blip_buf, &mut samples);
        Audio::save_samples(filename, &samples, use_ffmpeg.unwrap_or(false))
    }

    pub fn total_seconds(&self) -> Option<f32> {
        if let Some(pcm) = &self.pcm {
            Some(pcm.samples.len() as f32 / AUDIO_SAMPLE_RATE as f32)
        } else if self.commands.is_empty() {
            Some(self.notes.len() as f32 * self.speed as f32 / SOUND_TICKS_PER_SECOND as f32)
        } else {
            total_duration_sec(&self.commands)
        }
    }

    // Command emission

    pub(crate) fn to_commands(&self) -> Vec<MmlCommand> {
        let mut commands = Vec::new();
        self.emit_commands(&mut commands);
        commands
    }

    pub(crate) fn command_snapshot(&mut self) -> Arc<[MmlCommand]> {
        if self.commands.is_empty() {
            if let Some(CommandCache {
                source: CommandCacheSource::Legacy(source),
                commands,
            }) = &self.command_cache
            {
                if self.matches_legacy_source(source) {
                    return commands.clone();
                }
            }

            let mut commands = Vec::new();
            self.emit_commands(&mut commands);
            let commands: Arc<[MmlCommand]> = Arc::from(commands);
            self.command_cache = Some(CommandCache {
                source: CommandCacheSource::Legacy(self.legacy_source()),
                commands: commands.clone(),
            });
            commands
        } else {
            if let Some(CommandCache {
                source: CommandCacheSource::Mml(revision),
                commands,
            }) = &self.command_cache
            {
                if *revision == self.command_revision {
                    return commands.clone();
                }
            }

            let commands: Arc<[MmlCommand]> = Arc::from(self.commands.clone());
            self.command_cache = Some(CommandCache {
                source: CommandCacheSource::Mml(self.command_revision),
                commands: commands.clone(),
            });
            commands
        }
    }

    pub(crate) fn requires_tone(&mut self) -> bool {
        self.pcm.is_none()
            && self
                .command_snapshot()
                .iter()
                .any(|command| matches!(command, MmlCommand::Note { .. }))
    }

    fn matches_legacy_source(&self, source: &LegacyCommandSource) -> bool {
        if self.notes != source.notes
            || self.tones != source.tones
            || self.volumes != source.volumes
            || self.effects != source.effects
            || self.speed != source.speed
        {
            return false;
        }

        let tones = pyxel::tones();
        tones.len() == source.tone_modes.len()
            && tones
                .iter()
                .zip(&source.tone_modes)
                .all(|(tone, mode)| audio_ref!(tone).mode == *mode)
    }

    fn legacy_source(&self) -> LegacyCommandSource {
        let tones = pyxel::tones();
        LegacyCommandSource {
            notes: self.notes.clone(),
            tones: self.tones.clone(),
            volumes: self.volumes.clone(),
            effects: self.effects.clone(),
            speed: self.speed,
            tone_modes: tones.iter().map(|tone| audio_ref!(tone).mode).collect(),
        }
    }

    pub(crate) fn emit_commands(&self, commands: &mut Vec<MmlCommand>) {
        commands.clear();
        self.emit_fixed_params(commands);
        self.emit_envelope_slots(commands);
        self.emit_vibrato_slot(commands);
        self.emit_glide_slot(commands);
        self.emit_notes(commands);
    }

    fn emit_fixed_params(&self, commands: &mut Vec<MmlCommand>) {
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
    }

    fn emit_envelope_slots(&self, commands: &mut Vec<MmlCommand>) {
        if self.effects.contains(&EFFECT_FADEOUT) {
            commands.push(MmlCommand::EnvelopeSet {
                slot: 1,
                initial_level: 1.0,
                segments: vec![(self.speed as u32, 0.0)].into(),
            });
        }

        if self.effects.contains(&EFFECT_HALF_FADEOUT) {
            let fade_ticks = (self.speed as f32 / 2.0).round() as u32;
            let hold_ticks = self.speed as u32 - fade_ticks;
            commands.push(MmlCommand::EnvelopeSet {
                slot: 2,
                initial_level: 1.0,
                segments: vec![(hold_ticks, 1.0), (fade_ticks, 0.0)].into(),
            });
        }

        if self.effects.contains(&EFFECT_QUARTER_FADEOUT) {
            let fade_ticks = (self.speed as f32 / 4.0).round() as u32;
            let hold_ticks = self.speed as u32 - fade_ticks;
            commands.push(MmlCommand::EnvelopeSet {
                slot: 3,
                initial_level: 1.0,
                segments: vec![(hold_ticks, 1.0), (fade_ticks, 0.0)].into(),
            });
        }
    }

    fn emit_vibrato_slot(&self, commands: &mut Vec<MmlCommand>) {
        if self.effects.contains(&EFFECT_VIBRATO) {
            commands.push(MmlCommand::VibratoSet {
                slot: 1,
                delay_ticks: 0,
                period_ticks: VIBRATO_PERIOD_TICKS,
                semitone_depth: VIBRATO_DEPTH_CENTS as f32 / 100.0,
            });
        } else {
            commands.push(MmlCommand::Vibrato { slot: 0 });
        }
    }

    fn emit_glide_slot(&self, commands: &mut Vec<MmlCommand>) {
        if self.effects.contains(&EFFECT_SLIDE) {
            commands.push(MmlCommand::GlideSet {
                slot: 1,
                semitone_offset: None,
                duration_ticks: None,
            });
        } else {
            commands.push(MmlCommand::Glide { slot: 0 });
        }
    }

    fn emit_notes(&self, commands: &mut Vec<MmlCommand>) {
        let tones = pyxel::tones();
        let duration_ticks = self.speed as u32;

        let mut last_tone: Option<SoundTone> = None;
        let mut last_volume: Option<SoundVolume> = None;
        let mut last_fadeout: Option<SoundEffect> = None;
        let mut last_vibrato: Option<SoundEffect> = None;
        let mut last_slide: Option<SoundEffect> = None;

        for (i, &note) in self.notes.iter().enumerate() {
            if note < 0 {
                commands.push(MmlCommand::Rest { duration_ticks });
                continue;
            }

            let tone = self.cycled_or(i, &self.tones, TONE_TRIANGLE);
            let volume = self.cycled_or(i, &self.volumes, MAX_VOLUME);
            let effect = self.cycled_or(i, &self.effects, EFFECT_NONE);

            if last_tone != Some(tone) {
                last_tone = Some(tone);
                commands.push(MmlCommand::Tone { tone });
            }

            if last_volume != Some(volume) {
                last_volume = Some(volume);
                commands.push(MmlCommand::Volume {
                    level: volume as f32 / MAX_VOLUME as f32,
                });
            }

            if last_fadeout != Some(effect) {
                last_fadeout = Some(effect);
                let slot = match effect {
                    EFFECT_FADEOUT => 1,
                    EFFECT_HALF_FADEOUT => 2,
                    EFFECT_QUARTER_FADEOUT => 3,
                    _ => 0,
                };
                commands.push(MmlCommand::Envelope { slot });
            }

            if last_vibrato != Some(effect) {
                last_vibrato = Some(effect);
                commands.push(MmlCommand::Vibrato {
                    slot: u32::from(effect == EFFECT_VIBRATO),
                });
            }

            if last_slide != Some(effect) {
                last_slide = Some(effect);
                commands.push(MmlCommand::Glide {
                    slot: u32::from(effect == EFFECT_SLIDE),
                });
            }

            let base_note =
                tones
                    .get(tone as usize)
                    .or_else(|| tones.first())
                    .map_or(36_u32, |tone| {
                        if audio_ref!(tone).mode == ToneMode::Wavetable {
                            36
                        } else {
                            60
                        }
                    });
            commands.push(MmlCommand::Note {
                midi_note: note as u32 + base_note,
                duration_ticks,
            });
        }
    }

    fn cycled_or<T: Copy>(&self, index: usize, values: &[T], default: T) -> T {
        if values.is_empty() {
            default
        } else {
            values[index % values.len()]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_snapshot_is_reused_until_legacy_sound_changes() {
        let sound = Sound::new();
        let mut sound = audio_mut!(sound);
        sound.set("c2e2", "00", "77", "nn", 6).unwrap();

        let first = sound.command_snapshot();
        let second = sound.command_snapshot();
        assert!(std::sync::Arc::ptr_eq(&first, &second));

        sound.notes[0] = 31;
        let changed = sound.command_snapshot();
        assert!(!std::sync::Arc::ptr_eq(&first, &changed));
    }

    #[test]
    fn command_snapshot_is_reused_until_mml_changes() {
        let sound = Sound::new();
        let mut sound = audio_mut!(sound);
        sound.set_mml("T120 O4 C").unwrap();

        let first = sound.command_snapshot();
        let second = sound.command_snapshot();
        assert!(std::sync::Arc::ptr_eq(&first, &second));

        sound.set_mml("T120 O4 D").unwrap();
        let changed = sound.command_snapshot();
        assert!(!std::sync::Arc::ptr_eq(&first, &changed));
    }

    #[test]
    fn command_snapshot_preserves_maximum_legacy_note() {
        let sound = Sound::new();
        let mut sound = audio_mut!(sound);
        sound.notes = vec![SoundNote::MAX];

        let commands = sound.command_snapshot();
        let midi_note = commands.iter().find_map(|command| match command {
            MmlCommand::Note { midi_note, .. } => Some(*midi_note),
            _ => None,
        });

        assert_eq!(midi_note, Some(163));
    }
}
