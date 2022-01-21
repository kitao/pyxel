use std::cmp::max;

use blip_buf::BlipBuf;

use crate::oscillator::Oscillator;
use crate::settings::{
    EFFECT_NONE, MAX_EFFECT, MAX_NOTE, MAX_TONE, MAX_VOLUME, NUM_CHANNELS, TONE_TRIANGLE,
};
use crate::sound::{SharedSound, Sound};
use crate::types::{Effect, Note, Tone, Volume};

pub struct Channel {
    oscillator: Oscillator,
    sounds: Vec<Sound>,
    is_playing: bool,
    should_loop: bool,
    sound_index: u32,
    note_index: u32,
    tick_count: u32,
    pub gain: u8,
}

pub type SharedChannel = shared_type!(Channel);

impl Channel {
    pub fn new() -> SharedChannel {
        new_shared_type!(Self {
            oscillator: Oscillator::new(),
            sounds: Vec::new(),
            is_playing: false,
            should_loop: false,
            sound_index: 0,
            note_index: 0,
            tick_count: 0,
            gain: u8::MAX / NUM_CHANNELS as u8,
        })
    }

    pub fn play_pos(&mut self) -> Option<(u32, u32)> {
        if self.is_playing {
            Some((self.sound_index, self.note_index))
        } else {
            None
        }
    }

    pub fn play(&mut self, sounds: Vec<SharedSound>, start_tick: Option<u32>, should_loop: bool) {
        let sounds: Vec<Sound> = sounds.iter().map(|sound| sound.lock().clone()).collect();
        if sounds.is_empty() || sounds.iter().all(|sound| sound.notes.is_empty()) {
            return;
        }
        self.sounds = sounds;
        self.is_playing = true;
        self.should_loop = should_loop;
        self.sound_index = 0;
        self.note_index = 0;
        self.tick_count = start_tick.unwrap_or(0);
        loop {
            let sound = &self.sounds[self.sound_index as usize];
            let sound_ticks = sound.notes.len() as u32 * sound.speed;
            if self.tick_count < sound_ticks {
                self.note_index = self.tick_count / sound.speed;
                self.tick_count %= sound.speed;
                break;
            }
            self.tick_count -= sound_ticks;
            self.sound_index += 1;
            if self.sound_index >= self.sounds.len() as u32 {
                if should_loop {
                    self.sound_index = 0;
                } else {
                    return;
                }
            }
        }
    }

    pub fn play1(&mut self, sound: SharedSound, start_tick: Option<u32>, should_loop: bool) {
        self.play(vec![sound], start_tick, should_loop);
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.should_loop = false;
        self.sound_index = 0;
        self.note_index = 0;
        self.oscillator.stop();
    }

    pub(crate) fn update(&mut self, blip_buf: &mut BlipBuf) {
        if !self.is_playing {
            return;
        }
        let mut sound = &self.sounds[self.sound_index as usize];
        let speed = max(sound.speed, 1);
        if self.tick_count % speed == 0 {
            if self.tick_count > 0 {
                self.note_index += 1;
            }
            while self.note_index >= sound.notes.len() as u32 {
                self.sound_index += 1;
                self.note_index = 0;
                if self.sound_index >= self.sounds.len() as u32 {
                    if self.should_loop {
                        self.sound_index = 0;
                    } else {
                        self.stop();
                        return;
                    }
                }
                sound = &self.sounds[self.sound_index as usize];
            }

            let note = Self::circular_note(&sound.notes, self.note_index);
            assert!(note <= MAX_NOTE, "invalid sound note {}", note);
            let volume = Self::circular_volume(&sound.volumes, self.note_index);
            assert!(volume <= MAX_VOLUME, "invalid sound volume {}", volume);
            let tone = Self::circular_tone(&sound.tones, self.note_index);
            assert!(tone <= MAX_TONE, "invalid sound tone {}", tone);
            let effect = Self::circular_effect(&sound.effects, self.note_index);
            assert!(effect <= MAX_EFFECT, "invalid sound effect {}", effect);
            let speed = max(sound.speed, 1);

            if note >= 0 && volume > 0 {
                self.oscillator.play(
                    note as f64,
                    tone,
                    (self.gain as f64 * volume as f64) / (u8::MAX as f64 * MAX_VOLUME as f64),
                    effect,
                    speed,
                );
            }
        }
        self.oscillator.update(blip_buf);
        self.tick_count += 1;
    }

    fn circular_note(notes: &[Note], index: u32) -> Note {
        let len = notes.len();
        if len > 0 {
            notes[index as usize % len]
        } else {
            0
        }
    }

    fn circular_tone(tones: &[Tone], index: u32) -> Tone {
        let len = tones.len();
        if len > 0 {
            tones[index as usize % len]
        } else {
            TONE_TRIANGLE
        }
    }

    fn circular_volume(volumes: &[Volume], index: u32) -> Volume {
        let len = volumes.len();
        if len > 0 {
            volumes[index as usize % len]
        } else {
            MAX_VOLUME
        }
    }

    fn circular_effect(effects: &[Effect], index: u32) -> Effect {
        let len = effects.len();
        if len > 0 {
            effects[index as usize % len]
        } else {
            EFFECT_NONE
        }
    }
}
