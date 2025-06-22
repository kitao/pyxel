use std::collections::HashMap;

use crate::blip_buf::BlipBuf;
use crate::mml_command::MmlCommand;
use crate::pyxel::{SOUNDS, TONES};
use crate::settings::{
    AUDIO_CLOCK_RATE, AUDIO_CONTROL_RATE, DEFAULT_CHANNEL_GAIN, DEFAULT_MML_CPT, TONE_TRIANGLE,
};
use crate::sound::SharedSound;
use crate::tone::{Gain, Noise};
use crate::voice::Voice;

pub type Detune = i32;

pub struct Channel {
    voice: Voice,
    is_playing: bool,
    should_loop: bool,
    should_resume: bool,
    sound_index: u32,
    resume_sounds: Vec<SharedSound>,
    resume_start_clock: u32,
    resume_should_loop: bool,

    pub sounds: Vec<SharedSound>,
    pub gain: Gain,
    pub detune: Detune,

    skip_clocks: u32,
    elapsed_clocks: u32,

    commands: Vec<MmlCommand>,
    command_index: u32,
    cpt: u32,
    envelope_slots: HashMap<u8, MmlCommand>,
    vibrato_slots: HashMap<u8, MmlCommand>,
    glide_slots: HashMap<u8, MmlCommand>,
    transpose_semitones: f64,
    detune_semitones: f64,
    volume_level: f64,
    gate_ratio: f64,
    is_sound_head: bool,
    duration_clocks: u32,
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

            resume_sounds: Vec::new(),
            resume_start_clock: 0,
            resume_should_loop: false,

            sounds: Vec::new(),
            gain: DEFAULT_CHANNEL_GAIN,
            detune: 0,

            skip_clocks: 0,
            elapsed_clocks: 0,

            commands: Vec::new(),
            command_index: 0,
            cpt: 0,
            envelope_slots: HashMap::new(),
            vibrato_slots: HashMap::new(),
            glide_slots: HashMap::new(),
            transpose_semitones: 0.0,
            detune_semitones: 0.0,
            volume_level: 1.0,
            gate_ratio: 1.0,
            is_sound_head: false,
            duration_clocks: 0,
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

    fn play_from_tick(
        &mut self,
        sounds: Vec<SharedSound>,
        start_tick: u32,
        should_loop: bool,
        should_resume: bool,
    ) {
        let mut start_tick = start_tick;
        let mut start_clock = 0;

        for sound in &sounds {
            let sound = sound.lock();
            let (clock, remaining_ticks) = sound.calc_clock_at_tick(start_tick);

            start_clock += clock;

            if let Some(remaining_ticks) = remaining_ticks {
                start_tick = remaining_ticks;
            } else {
                start_clock = clock;
                break;
            }
        }

        self.play_from_clock(sounds, start_clock, should_loop, should_resume);
    }

    fn play_from_clock(
        &mut self,
        sounds: Vec<SharedSound>,
        start_clock: u32,
        should_loop: bool,
        should_resume: bool,
    ) {
        if sounds.is_empty() || sounds.iter().all(|sound| sound.lock().notes.is_empty()) {
            return;
        }

        if !should_resume {
            self.resume_sounds = self.sounds.clone();
            self.resume_should_loop = should_loop;
            self.resume_start_clock = self.elapsed_clocks;
        }

        // TODO
        self.sounds = sounds;
        self.should_loop = should_loop;
        self.sound_index = 0;
        self.skip_clocks = start_clock;
        self.elapsed_clocks = 0;
        self.is_sound_head = true;
        self.is_playing = true;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.voice.cancel_note();
    }

    pub fn play_pos(&mut self) -> Option<(u32, u32)> {
        if self.is_playing {
            let sound = self.sounds[self.sound_index as usize].lock();
            let (tick, _) = sound.calc_tick_at_clock(self.elapsed_clocks);

            Some((self.sound_index, tick))
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

        while self.skip_clocks > 0 || clock_count > 0 {
            if self.is_sound_head {
                self.is_sound_head = false;
                self.duration_clocks = 0;
                self.command_index = 0;
                self.cpt = DEFAULT_MML_CPT;

                let sounds = SOUNDS.lock();
                let sound = &sounds[self.sound_index as usize].lock();

                if sound.commands.is_empty() {
                    self.commands = sound.to_commands();
                } else {
                    self.commands = sound.commands.clone();
                }
            }

            if self.duration_clocks == 0 {
                self.advance_command();
                if self.duration_clocks > self.skip_clocks {
                    self.voice.advance_note(self.skip_clocks);
                }

                if self.command_index >= self.commands.len() as u32 {
                    self.is_sound_head = true;
                    self.sound_index += 1;

                    if self.sound_index >= self.sounds.len() as u32 {
                        if self.should_loop {
                            self.sound_index = 0;
                        } else {
                            self.stop();

                            if self.should_resume {
                                self.play_from_clock(
                                    self.sounds.clone(),
                                    self.resume_start_clock + self.elapsed_clocks,
                                    self.resume_should_loop,
                                    false,
                                );

                                if self.is_playing {
                                    continue;
                                }
                            }

                            if self.skip_clocks == 0 {
                                self.voice.process(blip_buf, 0, clock_count);
                            }
                            return;
                        }
                    }
                }
            }

            if self.skip_clocks > 0 {
                let process_clocks = self.skip_clocks.min(self.duration_clocks);

                clock_offset += process_clocks;
                self.skip_clocks -= process_clocks;

                self.duration_clocks -= process_clocks;
                self.elapsed_clocks += process_clocks;
            }

            if self.skip_clocks == 0 {
                let process_clocks = clock_count.min(self.duration_clocks);
                self.voice.process(blip_buf, clock_offset, process_clocks);

                clock_offset += process_clocks;
                clock_count -= process_clocks;

                self.duration_clocks -= process_clocks;
                self.elapsed_clocks += process_clocks;
            }
        }
    }

    fn advance_command(&mut self) {
        let tones = TONES.lock();

        for i in self.command_index as usize..self.commands.len() {
            match &self.commands[i] {
                MmlCommand::Tempo { bpm } => {
                    self.cpt = MmlCommand::bpm_to_cpt(*bpm);
                }
                MmlCommand::Quantize { gate_1_8 } => {
                    self.gate_ratio = MmlCommand::gate_to_ratio(*gate_1_8);
                }

                MmlCommand::Tone { tone_index } => {
                    let mut tone = tones[*tone_index as usize].lock();
                    match tone.noise {
                        Noise::ShortPeriod => self.voice.oscillator.set_noise(true),
                        Noise::LongPeriod => self.voice.oscillator.set_noise(false),
                        Noise::Off => self.voice.oscillator.set(tone.waveform()),
                    }
                }
                MmlCommand::Volume { volume_0_15 } => {
                    self.volume_level = MmlCommand::volume_to_level(*volume_0_15);
                }

                MmlCommand::Transpose { key_offset } => {
                    self.transpose_semitones += *key_offset as f64;
                }
                MmlCommand::Detune { offset_cents } => {
                    self.detune_semitones += MmlCommand::cents_to_semitones(*offset_cents);
                }

                MmlCommand::Envelope { slot } => {
                    if let Some(MmlCommand::EnvelopeSet {
                        volume_0_15,
                        segments,
                        ..
                    }) = self.envelope_slots.get(&slot)
                    {
                        self.voice.envelope.set(
                            MmlCommand::volume_to_level(*volume_0_15),
                            &MmlCommand::convert_segments(segments, self.cpt),
                        );
                        self.voice.envelope.enable();
                    } else {
                        self.voice.envelope.disable();
                    }
                }
                command @ MmlCommand::EnvelopeSet {
                    slot,
                    volume_0_15,
                    segments,
                } => {
                    assert!(*slot > 0, "Envelope slot 0 is reserved for disable");

                    self.envelope_slots.insert(*slot, command.clone());
                    self.voice.envelope.set(
                        MmlCommand::volume_to_level(*volume_0_15),
                        &MmlCommand::convert_segments(&segments, self.cpt),
                    );
                    self.voice.envelope.enable();
                }

                MmlCommand::Vibrato { slot } => {
                    if let Some(MmlCommand::VibratoSet {
                        delay_ticks,
                        frequency_chz,
                        depth_cents,
                        ..
                    }) = self.vibrato_slots.get(&slot)
                    {
                        self.voice.vibrato.set(
                            MmlCommand::ticks_to_clocks(*delay_ticks, self.cpt),
                            *frequency_chz as u32,
                            MmlCommand::cents_to_semitones(*depth_cents),
                        );
                        self.voice.vibrato.enable();
                    } else {
                        self.voice.vibrato.disable();
                    }
                }
                command @ MmlCommand::VibratoSet {
                    slot,
                    delay_ticks,
                    frequency_chz,
                    depth_cents,
                } => {
                    assert!(*slot > 0, "Vibrato slot 0 is reserved for disable");

                    self.vibrato_slots.insert(*slot, command.clone());
                    self.voice.vibrato.set(
                        MmlCommand::ticks_to_clocks(*delay_ticks, self.cpt),
                        *frequency_chz as u32,
                        MmlCommand::cents_to_semitones(*depth_cents),
                    );
                    self.voice.vibrato.enable();
                }

                MmlCommand::Glide { slot } => {
                    if let Some(MmlCommand::GlideSet {
                        offset_cents,
                        duration_ticks,
                        ..
                    }) = self.glide_slots.get(&slot)
                    {
                        self.voice.glide.set(
                            MmlCommand::cents_to_semitones(*offset_cents),
                            MmlCommand::ticks_to_clocks(*duration_ticks, self.cpt),
                        );
                        self.voice.glide.enable();
                    } else {
                        self.voice.glide.disable();
                    }
                }
                command @ MmlCommand::GlideSet {
                    slot,
                    offset_cents,
                    duration_ticks,
                } => {
                    assert!(*slot > 0, "Glide slot 0 is reserved for disable");

                    self.glide_slots.insert(*slot, command.clone());
                    self.voice.glide.set(
                        MmlCommand::cents_to_semitones(*offset_cents),
                        MmlCommand::ticks_to_clocks(*duration_ticks, self.cpt),
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

                    let tone = tones[TONE_TRIANGLE as usize].lock();
                    let velocity = tone.gain * self.gain * self.volume_level;

                    self.duration_clocks = MmlCommand::ticks_to_clocks(*duration_ticks, self.cpt);

                    let playback_clocks =
                        (self.duration_clocks as f64 * self.gate_ratio).round() as u32;
                    self.voice.play_note(midi_note, velocity, playback_clocks);

                    return;
                }
                MmlCommand::Rest { duration_ticks } => {
                    self.duration_clocks = MmlCommand::ticks_to_clocks(*duration_ticks, self.cpt);
                    return;
                }
            }
        }
    }
}
