use crate::blip_buf::BlipBuf;
use crate::pyxel::TONES;
use crate::settings::{
    AUDIO_BIT_DEPTH, AUDIO_CLOCK_RATE, AUDIO_CONTROL_RATE, EFFECT_FADEOUT, EFFECT_HALF_FADEOUT,
    EFFECT_NONE, EFFECT_QUARTER_FADEOUT, EFFECT_SLIDE, EFFECT_VIBRATO, INITIAL_CHANNEL_GAIN,
    MAX_VOLUME, TONE_TRIANGLE,
};
use crate::sound::{SharedSound, Sound};
use crate::tone::Gain;
use crate::voice::Voice;
use crate::CLOCKS_PER_SPEED;

pub type Note = i16;
pub type ToneIndex = u16;
pub type Volume = u16;
pub type Effect = u16;
pub type Speed = f64;
pub type Detune = i32;

pub struct Channel {
    voice: Voice,
    is_first_note: bool,
    is_playing: bool,
    should_loop: bool,
    should_resume: bool,
    sound_index: u32,
    note_index: u32,
    clock_count: u32,
    resume_sounds: Vec<Sound>,
    resume_start_clock: u32,
    resume_should_loop: bool,
    pub sounds: Vec<Sound>,
    pub gain: Gain,
    pub detune: Detune,
    elapsed_clocks: u32,
    note_start_clock: u32,
    note_clocks: u32,
    prev_note: Option<f64>,
}

pub type SharedChannel = shared_type!(Channel);

impl Channel {
    pub fn new() -> SharedChannel {
        new_shared_type!(Self {
            voice: Voice::new(AUDIO_BIT_DEPTH, AUDIO_CLOCK_RATE, AUDIO_CONTROL_RATE),
            is_first_note: true,
            is_playing: false,
            should_loop: false,
            should_resume: false,
            sound_index: 0,
            note_index: 0,
            clock_count: 0,
            resume_sounds: Vec::new(),
            resume_start_clock: 0,
            resume_should_loop: false,
            sounds: Vec::new(),
            gain: INITIAL_CHANNEL_GAIN,
            detune: 0,
            elapsed_clocks: 0,
            note_start_clock: 0,
            note_clocks: 0,
            prev_note: None,
        })
    }

    pub fn play(
        &mut self,
        sounds: Vec<SharedSound>,
        start_tick: Option<f64>,
        should_loop: bool,
        should_resume: bool,
    ) {
        let sounds: Vec<Sound> = sounds.iter().map(|sound| sound.lock().clone()).collect();
        if sounds.is_empty() || sounds.iter().all(|sound| sound.notes.is_empty()) {
            return;
        }

        self.sounds = sounds;
        self.should_loop = should_loop;
        self.should_resume = self.is_playing && should_resume;
        self.sound_index = 0;
        self.note_index = 0;
        self.clock_count = (start_tick.unwrap_or(0.0) * CLOCKS_PER_SPEED as f64).round() as u32;

        if !should_resume {
            self.resume_sounds.clone_from(&self.sounds);
            self.resume_should_loop = should_loop;
            self.resume_start_clock = self.clock_count;
        }

        loop {
            let sound = &self.sounds[self.sound_index as usize];
            let sound_note_clocks = sound.note_clocks();
            let sound_clocks = sound.total_clocks();

            if self.clock_count < sound_clocks {
                self.note_index = self.clock_count / sound_note_clocks;
                self.clock_count %= sound_note_clocks;
                break;
            }

            self.clock_count -= sound_clocks;
            self.sound_index += 1;

            if self.sound_index >= self.sounds.len() as u32 {
                if should_loop {
                    self.sound_index = 0;
                } else {
                    return;
                }
            }
        }

        self.is_first_note = true;
        self.is_playing = true;
    }

    pub fn play1(
        &mut self,
        sound: SharedSound,
        start_tick: Option<f64>,
        should_loop: bool,
        should_resume: bool,
    ) {
        self.play(vec![sound], start_tick, should_loop, should_resume);
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.voice.stop();
        self.prev_note = None;
    }

    pub fn play_pos(&mut self) -> Option<(u32, u32)> {
        if self.is_playing {
            Some((self.sound_index, self.note_index))
        } else {
            None
        }
    }

    pub(crate) fn process(&mut self, blip_buf: &mut BlipBuf, clock_count: u32) {
        if !self.is_playing {
            return;
        }

        let mut processed_clocks = 0;
        let mut clock_count = clock_count;

        while clock_count > 0 {
            let note_tick_offset = self.elapsed_clocks - self.note_start_clock;

            if note_tick_offset == 0 {
                let sound = &self.sounds[self.sound_index as usize];
                self.note_clocks = sound.note_clocks(); // TBD

                let note = Self::circular_note(&sound.notes, self.note_index);
                let volume = Self::circular_volume(&sound.volumes, self.note_index);
                let tone_index = Self::circular_tone(&sound.tones, self.note_index);
                let effect = Self::circular_effect(&sound.effects, self.note_index);

                let tones = TONES.lock();
                let tone = tones[tone_index as usize].lock();

                match tone.noise {
                    crate::tone::Noise::ShortPeriod => {
                        self.voice.oscillator.set_noise(true, 32, tone.gain)
                    }
                    crate::tone::Noise::LongPeriod => {
                        self.voice.oscillator.set_noise(false, 93, tone.gain)
                    }
                    crate::tone::Noise::Off => {
                        let waveform: Vec<f64> = tone
                            .wavetable
                            .iter()
                            .map(|&v| v as f64 / 7.5 - 1.0)
                            .collect();
                        self.voice.oscillator.set(&waveform, tone.gain);
                    }
                }

                let level = volume as f64 / MAX_VOLUME as f64;
                if matches!(
                    effect,
                    EFFECT_FADEOUT | EFFECT_HALF_FADEOUT | EFFECT_QUARTER_FADEOUT
                ) {
                    let duration = match effect {
                        EFFECT_HALF_FADEOUT => self.note_clocks / 2,
                        EFFECT_QUARTER_FADEOUT => self.note_clocks / 4,
                        _ => self.note_clocks,
                    };
                    self.voice.envelope.set(level, &[(duration, 0.0)]);
                    self.voice.envelope.enable();
                } else {
                    self.voice.envelope.set_value(level);
                    self.voice.envelope.disable();
                }

                if effect == EFFECT_VIBRATO {
                    self.voice.vibrato.set(0, AUDIO_CLOCK_RATE / 5, 0.5);
                    self.voice.vibrato.enable();
                } else {
                    self.voice.vibrato.disable();
                }

                let midi_note = note as f64 + self.detune as f64 / 200.0;

                if effect == EFFECT_SLIDE {
                    if let Some(prev) = self.prev_note {
                        let semitone_offset = midi_note - prev;
                        self.voice.glide.set(semitone_offset, self.note_clocks);
                        self.voice.glide.enable();
                    } else {
                        self.voice.glide.disable();
                    }
                } else {
                    self.voice.glide.disable();
                }

                self.prev_note = Some(midi_note);

                self.voice.play(midi_note, self.note_clocks);
            }

            let clocks_to_note_end =
                self.note_clocks - (self.elapsed_clocks - self.note_start_clock);
            let clocks_to_process = clock_count.min(clocks_to_note_end);

            self.voice
                .process(blip_buf, processed_clocks, clocks_to_process);
            processed_clocks += clocks_to_process;

            self.elapsed_clocks += clocks_to_process;
            clock_count -= clocks_to_process;

            if self.elapsed_clocks - self.note_start_clock == self.note_clocks {
                self.note_index += 1;
                self.note_start_clock = self.elapsed_clocks;
                self.is_first_note = true;

                let sound = &self.sounds[self.sound_index as usize];
                if self.note_index >= sound.notes.len() as u32 {
                    self.sound_index += 1;
                    self.note_index = 0;

                    if self.sound_index >= self.sounds.len() as u32 {
                        if self.should_loop {
                            self.sound_index = 0;
                        } else {
                            self.stop();
                            if self.should_resume {
                                let sounds = self
                                    .resume_sounds
                                    .iter()
                                    .map(|s| new_shared_type!(s.clone()))
                                    .collect();
                                let resume_sec =
                                    (self.resume_start_clock + 1) as f64 / AUDIO_CLOCK_RATE as f64;
                                self.play(sounds, Some(resume_sec), self.resume_should_loop, false);
                            }
                            return;
                        }
                    }
                }
            }
        }
    }

    const fn circular_note(notes: &[Note], index: u32) -> Note {
        let len = notes.len();
        if len > 0 {
            notes[index as usize % len]
        } else {
            0
        }
    }

    const fn circular_tone(tones: &[ToneIndex], index: u32) -> ToneIndex {
        let len = tones.len();
        if len > 0 {
            tones[index as usize % len]
        } else {
            TONE_TRIANGLE
        }
    }

    const fn circular_volume(volumes: &[Volume], index: u32) -> Volume {
        let len = volumes.len();
        if len > 0 {
            volumes[index as usize % len]
        } else {
            MAX_VOLUME
        }
    }

    const fn circular_effect(effects: &[Effect], index: u32) -> Effect {
        let len = effects.len();
        if len > 0 {
            effects[index as usize % len]
        } else {
            EFFECT_NONE
        }
    }
}
