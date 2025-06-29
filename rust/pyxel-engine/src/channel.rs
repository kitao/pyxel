use std::collections::HashMap;

use crate::blip_buf::BlipBuf;
use crate::mml_command::MmlCommand;
use crate::pyxel::TONES;
use crate::settings::{
    AUDIO_CLOCK_RATE, AUDIO_CONTROL_RATE, DEFAULT_CHANNEL_GAIN, DEFAULT_CLOCKS_PER_TICK,
};
use crate::sound::{SharedSound, Sound};
use crate::tone::{Gain, Noise};
use crate::voice::Voice;

pub type Detune = i32;

pub struct Channel {
    pub sounds: Vec<SharedSound>,
    pub gain: Gain,
    pub detune: Detune,

    voice: Voice,
    is_playing: bool,
    should_loop: bool,
    should_resume: bool,
    sound_index: u32,
    note_duration_clocks: u32,
    sound_elapsed_clocks: u32,
    total_elapsed_clocks: u32,

    mml_commands: Vec<MmlCommand>,
    mml_command_index: u32,
    clocks_per_tick: u32,
    gate_ratio: f64,
    tone_gain: f64,
    volume_level: f64,
    transpose_semitones: f64,
    detune_semitones: f64,
    envelope_slots: HashMap<u8, MmlCommand>,
    vibrato_slots: HashMap<u8, MmlCommand>,
    glide_slots: HashMap<u8, MmlCommand>,

    resume_sounds: Vec<SharedSound>,
    resume_should_loop: bool,
}

pub type SharedChannel = shared_type!(Channel);

impl Channel {
    pub fn new() -> SharedChannel {
        new_shared_type!(Self {
            sounds: Vec::new(),
            gain: DEFAULT_CHANNEL_GAIN,
            detune: 0,

            voice: Voice::new(AUDIO_CLOCK_RATE, AUDIO_CONTROL_RATE),
            is_playing: false,
            should_loop: false,
            should_resume: false,
            sound_index: 0,
            note_duration_clocks: 0,
            sound_elapsed_clocks: 0,
            total_elapsed_clocks: 0,

            mml_commands: Vec::new(),
            mml_command_index: 0,
            clocks_per_tick: 0,
            gate_ratio: 1.0,
            tone_gain: 1.0,
            volume_level: 1.0,
            transpose_semitones: 0.0,
            detune_semitones: 0.0,
            envelope_slots: HashMap::new(),
            vibrato_slots: HashMap::new(),
            glide_slots: HashMap::new(),

            resume_sounds: Vec::new(),
            resume_should_loop: false,
        })
    }

    pub fn play(
        &mut self,
        sounds: Vec<SharedSound>,
        start_tick: Option<u32>,
        should_loop: bool,
        should_resume: bool,
    ) {
        self.play_from_tick(sounds, start_tick.unwrap_or(0), should_loop, should_resume);
    }

    pub fn play1(
        &mut self,
        sound: SharedSound,
        start_tick: Option<u32>,
        should_loop: bool,
        should_resume: bool,
    ) {
        self.play_from_tick(
            vec![sound],
            start_tick.unwrap_or(0),
            should_loop,
            should_resume,
        );
    }

    pub fn play_mml(
        &mut self,
        mml: &str,
        start_tick: Option<u32>,
        should_loop: bool,
        should_resume: bool,
    ) {
        let sound = Sound::new();
        {
            let mut sound = sound.lock();
            sound.set_mml(mml);
        }
        self.play1(sound, start_tick, should_loop, should_resume);
    }

    fn play_from_tick(
        &mut self,
        sounds: Vec<SharedSound>,
        start_tick: u32,
        should_loop: bool,
        should_resume: bool,
    ) {
        let start_clock = if start_tick == 0 {
            0
        } else {
            let mut start_tick = start_tick;
            let mut start_clock = 0;

            for sound in &sounds {
                let sound = sound.lock();
                let (clock, remaining_ticks) = sound.calc_clock_at_tick(start_tick);

                start_clock += clock;

                if let Some(remaining_ticks) = remaining_ticks {
                    start_tick = remaining_ticks;
                } else {
                    break;
                }
            }

            start_clock
        };

        self.play_from_clock(sounds, start_clock, should_loop, should_resume);
    }

    fn play_from_clock(
        &mut self,
        sounds: Vec<SharedSound>,
        start_clock: u32,
        should_loop: bool,
        should_resume: bool,
    ) {
        if sounds.is_empty() {
            return;
        }

        if !should_resume {
            self.total_elapsed_clocks = 0;
        } else if !self.should_resume {
            self.resume_sounds = self.sounds.clone();
            self.resume_should_loop = self.should_loop;
        }

        self.sounds = sounds;
        self.is_playing = true;
        self.should_loop = should_loop;
        self.should_resume = should_resume;
        self.sound_index = 0;
        self.note_duration_clocks = 0;
        self.sound_elapsed_clocks = 0;
        self.mml_command_index = 0;

        if start_clock > 0 {
            self.process(None, start_clock);
        }
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.voice.cancel_note();
    }

    pub fn play_pos(&mut self) -> Option<(u32, u32)> {
        if self.is_playing {
            let sound = self.sounds[self.sound_index as usize].lock();
            let (tick, _) = sound.calc_tick_at_clock(self.sound_elapsed_clocks);
            Some((self.sound_index, tick))
        } else {
            None
        }
    }

    pub(crate) fn process(&mut self, blip_buf: Option<&mut BlipBuf>, clock_count: u32) {
        let mut blip_buf = blip_buf;
        let start_clock_count = clock_count;
        let mut clock_count = clock_count;
        let mut clock_offset = 0;

        while clock_count > 0 {
            // Playback has ended
            if !self.is_playing {
                self.voice.process(blip_buf.as_deref_mut(), 0, clock_count);
                return;
            }

            // Sound head
            if self.sound_elapsed_clocks == 0 {
                self.note_duration_clocks = 0;
                self.mml_command_index = 0;
                self.clocks_per_tick = DEFAULT_CLOCKS_PER_TICK;

                {
                    let sound = self.sounds[self.sound_index as usize].lock();
                    self.mml_commands = if sound.mml_commands.is_empty() {
                        sound.generate_mml_commands()
                    } else {
                        sound.mml_commands.clone()
                    };
                }

                self.advance_command();
            }

            // Process clocks
            let process_clocks = clock_count.min(self.note_duration_clocks);
            self.voice
                .process(blip_buf.as_deref_mut(), clock_offset, process_clocks);

            clock_offset += process_clocks;
            clock_count -= process_clocks;

            self.note_duration_clocks -= process_clocks;
            self.sound_elapsed_clocks += process_clocks;
            self.total_elapsed_clocks += process_clocks;

            // End of note
            if self.note_duration_clocks == 0 {
                self.advance_command();
            }

            // End of sound
            if self.note_duration_clocks == 0 {
                self.sound_index += 1;
                self.sound_elapsed_clocks = 0;

                // End of sound list
                if self.sound_index >= self.sounds.len() as u32 {
                    if self.should_loop && clock_count < start_clock_count {
                        self.sound_index = 0;
                    } else if self.should_resume {
                        self.play_from_clock(
                            self.resume_sounds.clone(),
                            self.total_elapsed_clocks,
                            self.resume_should_loop,
                            false,
                        );
                    } else {
                        self.is_playing = false;
                    }
                }
            }
        }
    }

    fn advance_command(&mut self) {
        let tones = TONES.lock();

        while self.mml_command_index < self.mml_commands.len() as u32 {
            let mml_command = &self.mml_commands[self.mml_command_index as usize];
            self.mml_command_index += 1;

            match mml_command {
                MmlCommand::Tempo { clocks_per_tick } => {
                    self.clocks_per_tick = *clocks_per_tick;
                }
                MmlCommand::Quantize { gate_ratio } => {
                    self.gate_ratio = *gate_ratio;
                }

                MmlCommand::Tone { tone_index } => {
                    let mut tone = tones[*tone_index as usize].lock();
                    match tone.noise {
                        Noise::ShortPeriod => self.voice.oscillator.set_noise(true),
                        Noise::LongPeriod => self.voice.oscillator.set_noise(false),
                        Noise::Off => self.voice.oscillator.set(tone.waveform()),
                    }

                    self.tone_gain = tone.gain;
                }
                MmlCommand::Volume { level } => {
                    self.volume_level = *level;
                }

                MmlCommand::Transpose { semitone_offset } => {
                    self.transpose_semitones += *semitone_offset;
                }
                MmlCommand::Detune { semitone_offset } => {
                    self.detune_semitones += *semitone_offset;
                }

                MmlCommand::Envelope { slot } => {
                    if let Some(MmlCommand::EnvelopeSet {
                        level, segments, ..
                    }) = self.envelope_slots.get(slot)
                    {
                        self.voice.envelope.set(
                            *level,
                            &MmlCommand::convert_segments(segments, self.clocks_per_tick),
                        );
                        self.voice.envelope.enable();
                    } else {
                        self.voice.envelope.disable();
                    }
                }
                mml_command @ MmlCommand::EnvelopeSet {
                    slot,
                    level,
                    segments,
                } => {
                    assert!(*slot > 0, "Envelope slot 0 is reserved for disable");

                    self.envelope_slots.insert(*slot, mml_command.clone());
                    self.voice.envelope.set(
                        *level,
                        &MmlCommand::convert_segments(segments, self.clocks_per_tick),
                    );
                    self.voice.envelope.enable();
                }

                MmlCommand::Vibrato { slot } => {
                    if let Some(MmlCommand::VibratoSet {
                        delay_ticks,
                        period_clocks,
                        semitone_depth,
                        ..
                    }) = self.vibrato_slots.get(slot)
                    {
                        self.voice.vibrato.set(
                            MmlCommand::ticks_to_clocks(*delay_ticks, self.clocks_per_tick),
                            *period_clocks,
                            *semitone_depth,
                        );
                        self.voice.vibrato.enable();
                    } else {
                        self.voice.vibrato.disable();
                    }
                }
                mml_command @ MmlCommand::VibratoSet {
                    slot,
                    delay_ticks,
                    period_clocks,
                    semitone_depth,
                } => {
                    assert!(*slot > 0, "Vibrato slot 0 is reserved for disable");

                    self.vibrato_slots.insert(*slot, mml_command.clone());
                    self.voice.vibrato.set(
                        MmlCommand::ticks_to_clocks(*delay_ticks, self.clocks_per_tick),
                        *period_clocks,
                        *semitone_depth,
                    );
                    self.voice.vibrato.enable();
                }

                MmlCommand::Glide { slot } => {
                    if let Some(MmlCommand::GlideSet {
                        semitone_offset,
                        duration_ticks,
                        ..
                    }) = self.glide_slots.get(slot)
                    {
                        self.voice.glide.set(
                            *semitone_offset,
                            MmlCommand::ticks_to_clocks(*duration_ticks, self.clocks_per_tick),
                        );
                        self.voice.glide.enable();
                    } else {
                        self.voice.glide.disable();
                    }
                }
                mml_command @ MmlCommand::GlideSet {
                    slot,
                    semitone_offset,
                    duration_ticks,
                } => {
                    assert!(*slot > 0, "Glide slot 0 is reserved for disable");

                    self.glide_slots.insert(*slot, mml_command.clone());
                    self.voice.glide.set(
                        *semitone_offset,
                        MmlCommand::ticks_to_clocks(*duration_ticks, self.clocks_per_tick),
                    );
                    self.voice.glide.enable();
                }

                MmlCommand::Note {
                    midi_note,
                    duration_ticks,
                } => {
                    let midi_note = *midi_note as f64
                        + self.transpose_semitones
                        + self.detune_semitones
                        + MmlCommand::cents_to_semitones(self.detune);

                    let velocity = self.tone_gain * self.gain * self.volume_level;

                    self.note_duration_clocks =
                        MmlCommand::ticks_to_clocks(*duration_ticks, self.clocks_per_tick);
                    let playback_clocks =
                        (self.note_duration_clocks as f64 * self.gate_ratio).round() as u32;

                    self.voice.play_note(midi_note, velocity, playback_clocks);
                    return;
                }
                MmlCommand::Rest { duration_ticks } => {
                    self.note_duration_clocks =
                        MmlCommand::ticks_to_clocks(*duration_ticks, self.clocks_per_tick);
                    return;
                }
            }
        }
    }
}
