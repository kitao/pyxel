use std::collections::HashMap;

use blip_buf::BlipBuf;

use crate::mml_command::MmlCommand;
use crate::pyxel::TONES;
use crate::settings::{AUDIO_CLOCK_RATE, AUDIO_CONTROL_RATE, DEFAULT_CHANNEL_GAIN};
use crate::sound::{SharedSound, Sound};
use crate::tone::ToneMode;
use crate::voice::Voice;

pub type ChannelGain = f32;
pub type ChannelDetune = i32;

pub struct Channel {
    pub sounds: Vec<SharedSound>,
    pub gain: ChannelGain,
    pub detune: ChannelDetune,

    voice: Voice,
    is_playing: bool,
    should_loop: bool,
    should_resume: bool,
    sound_index: u32,
    note_duration_clocks: u32,
    sound_elapsed_clocks: u32,
    total_elapsed_clocks: u32,

    commands: Vec<MmlCommand>,
    command_index: u32,
    repeat_points: Vec<(u32, u32)>,
    clocks_per_tick: u32,
    gate_ratio: f32,
    tone_gain: f32,
    volume_level: f32,
    transpose_semitones: f32,
    detune_semitones: f32,
    envelope_slots: HashMap<u32, MmlCommand>,
    vibrato_slots: HashMap<u32, MmlCommand>,
    glide_slots: HashMap<u32, MmlCommand>,

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

            commands: Vec::new(),
            command_index: 0,
            repeat_points: Vec::new(),
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

    fn sec_to_clock(sec: Option<f32>) -> u32 {
        (sec.unwrap_or(0.0) * AUDIO_CLOCK_RATE as f32).round() as u32
    }

    pub fn play(
        &mut self,
        sounds: Vec<SharedSound>,
        start_sec: Option<f32>,
        should_loop: bool,
        should_resume: bool,
    ) {
        self.play_from_clock(
            sounds,
            Self::sec_to_clock(start_sec),
            should_loop,
            should_resume,
        );
    }

    pub fn play1(
        &mut self,
        sound: SharedSound,
        start_sec: Option<f32>,
        should_loop: bool,
        should_resume: bool,
    ) {
        self.play_from_clock(
            vec![sound],
            Self::sec_to_clock(start_sec),
            should_loop,
            should_resume,
        );
    }

    pub fn play_mml(
        &mut self,
        code: &str,
        start_sec: Option<f32>,
        should_loop: bool,
        should_resume: bool,
    ) {
        let sound = Sound::new();
        {
            let mut sound = sound.lock();
            sound.mml(code);
        }

        self.play_from_clock(
            vec![sound],
            Self::sec_to_clock(start_sec),
            should_loop,
            should_resume,
        );
    }

    fn play_from_clock(
        &mut self,
        sounds: Vec<SharedSound>,
        start_clock: u32,
        should_loop: bool,
        should_resume: bool,
    ) {
        if sounds.is_empty() {
            self.is_playing = false;
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
        self.command_index = 0;

        if start_clock > 0 {
            self.process(None, start_clock);
        }
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.voice.cancel_note();
    }

    pub fn play_pos(&mut self) -> Option<(u32, f32)> {
        if self.is_playing {
            let elapsed_sec = self.sound_elapsed_clocks as f32 / AUDIO_CLOCK_RATE as f32;
            Some((self.sound_index, elapsed_sec))
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
                self.command_index = 0;
                self.repeat_points.clear();

                {
                    let sound = self.sounds[self.sound_index as usize].lock();
                    self.commands = if sound.commands.is_empty() {
                        sound.to_commands()
                    } else {
                        sound.commands.clone()
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

        while self.command_index < self.commands.len() as u32 {
            let command = &self.commands[self.command_index as usize];
            self.command_index += 1;

            match command {
                MmlCommand::Tempo { clocks_per_tick } => {
                    self.clocks_per_tick = *clocks_per_tick;
                    self.voice.set_clocks_per_tick(*clocks_per_tick);
                }
                MmlCommand::Quantize { gate_ratio } => {
                    self.gate_ratio = *gate_ratio;
                }

                MmlCommand::Tone { tone } => {
                    let mut tone = tones[*tone as usize].lock();
                    match tone.mode {
                        ToneMode::Wavetable => self.voice.oscillator.set(tone.waveform()),
                        ToneMode::ShortPeriodNoise => self.voice.oscillator.set_noise(true),
                        ToneMode::LongPeriodNoise => self.voice.oscillator.set_noise(false),
                    }

                    self.tone_gain = tone.gain;
                }
                MmlCommand::Volume { level } => {
                    self.volume_level = *level;
                }

                MmlCommand::Transpose { semitone_offset } => {
                    self.transpose_semitones = *semitone_offset;
                }
                MmlCommand::Detune { semitone_offset } => {
                    self.detune_semitones = *semitone_offset;
                }

                MmlCommand::Envelope { slot } => {
                    if let Some(MmlCommand::EnvelopeSet {
                        initial_level,
                        segments,
                        ..
                    }) = self.envelope_slots.get(slot)
                    {
                        self.voice.envelope.set(*initial_level, segments);
                        self.voice.envelope.enable();
                    } else {
                        self.voice.envelope.disable();
                    }
                }
                command @ MmlCommand::EnvelopeSet {
                    slot,
                    initial_level,
                    segments,
                } => {
                    assert!(*slot > 0, "Envelope slot 0 is reserved for disable");

                    self.envelope_slots.insert(*slot, command.clone());
                    self.voice.envelope.set(*initial_level, segments);
                    self.voice.envelope.enable();
                }

                MmlCommand::Vibrato { slot } => {
                    if let Some(MmlCommand::VibratoSet {
                        delay_ticks,
                        period_ticks,
                        semitone_depth,
                        ..
                    }) = self.vibrato_slots.get(slot)
                    {
                        self.voice
                            .vibrato
                            .set(*delay_ticks, *period_ticks, *semitone_depth);
                        self.voice.vibrato.enable();
                    } else {
                        self.voice.vibrato.disable();
                    }
                }
                command @ MmlCommand::VibratoSet {
                    slot,
                    delay_ticks,
                    period_ticks,
                    semitone_depth,
                } => {
                    assert!(*slot > 0, "Vibrato slot 0 is reserved for disable");

                    self.vibrato_slots.insert(*slot, command.clone());
                    self.voice
                        .vibrato
                        .set(*delay_ticks, *period_ticks, *semitone_depth);
                    self.voice.vibrato.enable();
                }

                MmlCommand::Glide { slot } => {
                    if let Some(MmlCommand::GlideSet {
                        semitone_offset,
                        duration_ticks,
                        ..
                    }) = self.glide_slots.get(slot)
                    {
                        self.voice.glide.set(*semitone_offset, *duration_ticks);
                        self.voice.glide.enable();
                    } else {
                        self.voice.glide.disable();
                    }
                }
                command @ MmlCommand::GlideSet {
                    slot,
                    semitone_offset,
                    duration_ticks,
                } => {
                    assert!(*slot > 0, "Glide slot 0 is reserved for disable");

                    self.glide_slots.insert(*slot, command.clone());
                    self.voice.glide.set(*semitone_offset, *duration_ticks);
                    self.voice.glide.enable();
                }

                MmlCommand::Note {
                    midi_note,
                    duration_ticks,
                } => {
                    let midi_note = *midi_note as f32
                        + self.transpose_semitones
                        + self.detune_semitones
                        + self.detune as f32 / 100.0;

                    let velocity = self.tone_gain * self.gain * self.volume_level;

                    self.note_duration_clocks = self.clocks_per_tick * *duration_ticks;
                    let playback_clocks =
                        (self.note_duration_clocks as f32 * self.gate_ratio).round() as u32;

                    self.voice.play_note(midi_note, velocity, playback_clocks);
                    return;
                }
                MmlCommand::Rest { duration_ticks } => {
                    self.note_duration_clocks = self.clocks_per_tick * *duration_ticks;
                    return;
                }

                MmlCommand::RepeatStart => {
                    self.repeat_points.push((self.command_index, 0)); // Index after RepeatStart
                }
                MmlCommand::RepeatEnd { play_count } => {
                    if let Some((index, count)) = self.repeat_points.pop() {
                        if *play_count == 0 || count + 1 < *play_count {
                            self.repeat_points.push((index, count + 1));
                            self.command_index = index;
                        }
                    }
                }
            }
        }
    }
}
