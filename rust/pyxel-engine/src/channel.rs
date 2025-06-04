use crate::blip_buf::BlipBuf;
use crate::pyxel::TONES;
use crate::settings::{
    AUDIO_CLOCK_RATE, AUDIO_CONTROL_RATE, EFFECT_FADEOUT, EFFECT_HALF_FADEOUT, EFFECT_NONE,
    EFFECT_QUARTER_FADEOUT, EFFECT_SLIDE, EFFECT_VIBRATO, INITIAL_CHANNEL_GAIN, MAX_VOLUME,
    TONE_TRIANGLE, VIBRATO_PERIOD_CLOCKS, VIBRATO_SEMITONE_DEPTH,
};
use crate::sound::{SharedSound, Sound};
use crate::tone::{Gain, Noise};
use crate::voice::Voice;
use crate::CLOCKS_PER_SPEED;

pub type Note = i16;
pub type ToneIndex = u16;
pub type Volume = u16;
pub type Effect = u16;
pub type Speed = u32;
pub type Detune = i32;

pub struct Channel {
    voice: Voice,
    is_playing: bool,
    should_loop: bool,
    should_resume: bool,
    sound_index: u32,
    note_index: u32,
    clock_count: u32,
    resume_sounds: Vec<Sound>,
    resume_clock_offset: u32,
    resume_should_loop: bool,
    pub sounds: Vec<Sound>,
    pub gain: Gain,
    pub detune: Detune,

    is_first_note: bool,
    duration_clocks: u32,
    prev_midi_note: Option<f64>,
    playback_clocks: u32,
}

pub type SharedChannel = shared_type!(Channel);

impl Channel {
    pub fn new() -> SharedChannel {
        new_shared_type!(Self {
            voice: Voice::new(AUDIO_CLOCK_RATE, AUDIO_CONTROL_RATE),
            is_playing: false,
            should_loop: false,
            should_resume: false,
            sound_index: 0,
            note_index: 0,
            clock_count: 0,
            resume_sounds: Vec::new(),
            resume_clock_offset: 0,
            resume_should_loop: false,
            sounds: Vec::new(),
            gain: INITIAL_CHANNEL_GAIN,
            detune: 0,

            is_first_note: false,
            duration_clocks: 0,
            prev_midi_note: None,
            playback_clocks: 0,
        })
    }

    pub fn play(
        &mut self,
        sounds: Vec<SharedSound>,
        start_tick: Option<f64>,
        should_loop: bool,
        should_resume: bool,
    ) {
        let start_clock = (start_tick.unwrap_or(0.0) * CLOCKS_PER_SPEED as f64).round() as u32;
        self.play_from_clock(sounds, start_clock, should_loop, should_resume);
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

    fn play_from_clock(
        &mut self,
        sounds: Vec<SharedSound>,
        start_clock: u32,
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
        self.clock_count = start_clock;

        if !should_resume {
            self.resume_sounds.clone_from(&self.sounds);
            self.resume_should_loop = should_loop;
            self.resume_clock_offset = self.playback_clocks;
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

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.voice.cancel_note();
        self.prev_midi_note = None;
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
            self.voice.process(blip_buf, 0, clock_count);
            return;
        }

        let mut clock_offset = 0;
        let mut clock_count = clock_count;

        while clock_count > 0 {
            // Advance to the next note
            if !self.is_first_note && self.duration_clocks == 0 {
                self.note_index += 1;

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
                                self.play_from_clock(
                                    sounds,
                                    self.resume_clock_offset + self.playback_clocks,
                                    self.resume_should_loop,
                                    false,
                                );

                                if self.is_playing {
                                    continue;
                                }
                            }

                            self.voice.process(blip_buf, 0, clock_count);
                            return;
                        }
                    }
                }
            }

            // Start playing the current note
            if self.is_first_note || self.duration_clocks == 0 {
                let sound = &self.sounds[self.sound_index as usize];
                let note_clocks = sound.note_clocks();
                self.duration_clocks = note_clocks;
                let note = sound.notes[self.note_index as usize];

                if note >= 0 {
                    let volume = Self::circular_volume(&sound.volumes, self.note_index);
                    let tone_index = Self::circular_tone(&sound.tones, self.note_index);
                    let effect = Self::circular_effect(&sound.effects, self.note_index);

                    let tones = TONES.lock();
                    let mut tone = tones[tone_index as usize].lock();

                    match tone.noise {
                        Noise::ShortPeriod => self.voice.oscillator.set_noise(true),
                        Noise::LongPeriod => self.voice.oscillator.set_noise(false),
                        Noise::Off => self.voice.oscillator.set(tone.waveform()),
                    }

                    if effect == EFFECT_FADEOUT {
                        self.voice.envelope.set(1.0, &[(note_clocks, 0.0)]);
                        self.voice.envelope.enable();
                    } else if effect == EFFECT_HALF_FADEOUT {
                        self.voice
                            .envelope
                            .set(1.0, &[(note_clocks / 2, 1.0), (note_clocks / 2, 0.0)]);
                        self.voice.envelope.enable();
                    } else if effect == EFFECT_QUARTER_FADEOUT {
                        self.voice
                            .envelope
                            .set(1.0, &[(note_clocks * 3 / 4, 1.0), (note_clocks / 4, 0.0)]);
                        self.voice.envelope.enable();
                    } else {
                        self.voice.envelope.disable();
                    }

                    if effect == EFFECT_VIBRATO {
                        self.voice
                            .vibrato
                            .set(0, VIBRATO_PERIOD_CLOCKS, VIBRATO_SEMITONE_DEPTH);
                        self.voice.vibrato.enable();
                    } else {
                        self.voice.vibrato.disable();
                    }

                    let note_offset = if tone.noise == Noise::Off { 36 } else { 60 };
                    let midi_note = (note + note_offset) as f64 + self.detune as f64 / 200.0;

                    if effect == EFFECT_SLIDE {
                        if let Some(prev_midi_note) = self.prev_midi_note {
                            let semitone_offset = prev_midi_note - midi_note;
                            self.voice.glide.set(semitone_offset, note_clocks);
                            self.voice.glide.enable();
                        } else {
                            self.voice.glide.disable();
                        }
                    } else {
                        self.voice.glide.disable();
                    }

                    self.prev_midi_note = Some(midi_note);

                    self.voice.play_note(
                        midi_note,
                        self.gain * tone.gain * volume as f64 / MAX_VOLUME as f64,
                        note_clocks,
                    );
                }
            }

            self.is_first_note = false;

            let process_clocks = clock_count.min(self.duration_clocks);
            self.voice.process(blip_buf, clock_offset, process_clocks);

            clock_offset += process_clocks;
            clock_count -= process_clocks;

            self.duration_clocks -= process_clocks;
            self.playback_clocks += process_clocks;
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
