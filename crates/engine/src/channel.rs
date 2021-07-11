use blip_buf::BlipBuf;

use crate::oscillator::Oscillator;
use crate::settings::MAX_SOUND_VOLUME;
use crate::sound::Sound;

pub struct Channel {
    oscillator: Oscillator,
    sounds: Vec<Sound>,
    is_playing: bool,
    is_looping: bool,
    sound_index: u32,
    note_index: u32,
    tick_count: u32,
}

impl Channel {
    pub fn new() -> Channel {
        Channel {
            oscillator: Oscillator::new(),
            sounds: Vec::new(),
            is_playing: false,
            is_looping: false,
            sound_index: 0,
            note_index: 0,
            tick_count: 0,
        }
    }

    pub fn update(&mut self, blip_buf: &mut BlipBuf) {
        if !self.is_playing {
            return;
        }

        let sound = &self.sounds[self.sound_index as usize];

        if self.tick_count % sound.speed() as u32 == 0 {
            if self.note_index >= sound.notes().len() as u32 {
                self.sound_index += 1;
                self.note_index = 0;

                if self.sound_index >= self.sounds.len() as u32 {
                    if self.is_looping {
                        self.sound_index = 0;
                    } else {
                        self.stop();
                        return;
                    }
                }
            }

            let sound = &self.sounds[self.sound_index as usize];
            let note = sound.note(self.note_index);
            let volume = sound.volume(self.note_index);

            if note >= 0 && volume > 0 {
                self.oscillator.play(
                    note as f64,
                    sound.tone(self.note_index),
                    volume as f64 / MAX_SOUND_VOLUME as f64,
                    sound.effect(self.note_index),
                    sound.speed() as u32,
                );
            }

            self.note_index += 1;
        }

        self.oscillator.update(blip_buf);
        self.tick_count += 1;
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn is_looping(&self) -> bool {
        self.is_looping
    }

    pub fn sound_index(&self) -> u32 {
        self.sound_index
    }

    pub fn note_index(&self) -> u32 {
        self.note_index
    }

    pub fn play(&mut self, sounds: Vec<Sound>, is_looping: bool) {
        self.sounds = sounds;
        self.is_playing = true;
        self.is_looping = is_looping;
        self.sound_index = 0;
        self.note_index = 0;
        self.tick_count = 0;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.is_looping = false;
        self.sound_index = 0;
        self.note_index = 0;

        self.oscillator.stop();
    }
}
