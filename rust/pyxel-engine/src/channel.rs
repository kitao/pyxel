use std::collections::HashMap;

use blip_buf::BlipBuf;

use crate::mml_command::MmlCommand;
use crate::pyxel::TONES;
use crate::settings::{
    AUDIO_CLOCK_RATE, AUDIO_CONTROL_RATE, AUDIO_SAMPLE_RATE, DEFAULT_CHANNEL_GAIN,
    NOTE_INTERP_CLOCKS,
};
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
    glide_pending_params: Option<(Option<f32>, Option<u32>)>,
    last_midi_note: Option<f32>,
    envelope_slots: HashMap<u32, MmlCommand>,
    vibrato_slots: HashMap<u32, MmlCommand>,
    glide_slots: HashMap<u32, MmlCommand>,

    resume_sounds: Vec<SharedSound>,
    resume_should_loop: bool,

    pcm_position: usize,
}

pub type SharedChannel = shared_type!(Channel);

impl Channel {
    pub fn new() -> SharedChannel {
        new_shared_type!(Self {
            sounds: Vec::new(),
            gain: DEFAULT_CHANNEL_GAIN,
            detune: 0,

            voice: Voice::new(AUDIO_CLOCK_RATE, AUDIO_CONTROL_RATE, NOTE_INTERP_CLOCKS),
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
            glide_pending_params: None,
            last_midi_note: None,
            envelope_slots: HashMap::new(),
            vibrato_slots: HashMap::new(),
            glide_slots: HashMap::new(),

            resume_sounds: Vec::new(),
            resume_should_loop: false,

            pcm_position: 0,
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
    ) -> Result<(), String> {
        let sound = Sound::new();
        {
            let mut sound = sound.lock();
            sound.mml(code)?;
        }

        self.play_from_clock(
            vec![sound],
            Self::sec_to_clock(start_sec),
            should_loop,
            should_resume,
        );
        Ok(())
    }

    fn play_from_clock(
        &mut self,
        sounds: Vec<SharedSound>,
        start_clock: u32,
        should_loop: bool,
        should_resume: bool,
    ) {
        if sounds.is_empty()
            || sounds.iter().all(|sound| {
                let sound = sound.lock();
                sound.notes.is_empty() && sound.commands.is_empty() && sound.pcm.is_none()
            })
        {
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
        self.last_midi_note = None;
        self.pcm_position = 0;

        if self.current_sound_is_pcm() {
            self.voice.cancel_note();
        }

        if start_clock > 0 {
            if self.current_sound_is_pcm() {
                self.seek_pcm(start_clock);
            } else {
                self.process(None, start_clock);
            }
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
        if self.current_sound_is_pcm() {
            self.voice.process(blip_buf, 0, clock_count);
            return;
        }

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
                    let mut tone = tones.get(*tone as usize).unwrap_or(&tones[0]).lock();
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
                        if let (Some(semitone_offset), Some(duration_ticks)) =
                            (semitone_offset, duration_ticks)
                        {
                            self.glide_pending_params = None;
                            self.voice.glide.set(*semitone_offset, *duration_ticks);
                        } else {
                            self.glide_pending_params = Some((*semitone_offset, *duration_ticks));
                        }
                        self.voice.glide.enable();
                    } else {
                        self.glide_pending_params = None;
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

                    if let (Some(semitone_offset), Some(duration_ticks)) =
                        (semitone_offset, duration_ticks)
                    {
                        self.glide_pending_params = None;
                        self.voice.glide.set(*semitone_offset, *duration_ticks);
                    } else {
                        self.glide_pending_params = Some((*semitone_offset, *duration_ticks));
                    }
                    self.voice.glide.enable();
                }

                MmlCommand::Note {
                    midi_note,
                    duration_ticks,
                } => {
                    // Calculate note parameters
                    let midi_note = *midi_note as f32
                        + self.transpose_semitones
                        + self.detune_semitones
                        + self.detune as f32 / 100.0;

                    let velocity = self.tone_gain * self.gain * self.volume_level;

                    self.note_duration_clocks = self.clocks_per_tick * *duration_ticks;
                    let playback_clocks =
                        (self.note_duration_clocks as f32 * self.gate_ratio).round() as u32;

                    // Glide with auto parameters
                    if let Some((pending_offset, pending_ticks)) = self.glide_pending_params {
                        let resolved_offset = pending_offset
                            .unwrap_or(self.last_midi_note.unwrap_or(midi_note) - midi_note);
                        let resolved_ticks = pending_ticks.unwrap_or(*duration_ticks);
                        self.voice.glide.set(resolved_offset, resolved_ticks);
                    }

                    // Play note
                    self.voice.play_note(midi_note, velocity, playback_clocks);
                    self.last_midi_note = Some(midi_note);
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

    pub(crate) fn mix_pcm(&mut self, out: &mut [i16]) {
        if !self.is_playing || !self.current_sound_is_pcm() {
            return;
        }

        let clocks_per_sample = AUDIO_CLOCK_RATE / AUDIO_SAMPLE_RATE;
        let mut offset = 0usize;

        while offset < out.len() {
            let mut to_copy = 0usize;
            let mut end_reached = false;
            let mut should_advance = false;
            let gain = self.gain;

            {
                let sound = self.sounds[self.sound_index as usize].lock();
                let Some(pcm) = &sound.pcm else {
                    return;
                };
                let len = pcm.samples.len();

                if len == 0 || self.pcm_position >= len {
                    should_advance = true;
                } else {
                    let remaining = len - self.pcm_position;
                    to_copy = (out.len() - offset).min(remaining);
                    let samples = &pcm.samples;
                    for i in 0..to_copy {
                        let src = samples[self.pcm_position + i] as f32 * gain;
                        let mixed = out[offset + i] as f32 + src;
                        out[offset + i] = mixed.clamp(i16::MIN as f32, i16::MAX as f32) as i16;
                    }
                    end_reached = self.pcm_position + to_copy >= len;
                }
            }

            if should_advance {
                if !self.advance_pcm_sound() {
                    return;
                }
                continue;
            }

            self.pcm_position += to_copy;
            let clocks = clocks_per_sample * to_copy as u32;
            self.sound_elapsed_clocks += clocks;
            self.total_elapsed_clocks += clocks;
            offset += to_copy;

            if end_reached && !self.advance_pcm_sound() {
                return;
            }
        }
    }

    fn advance_pcm_sound(&mut self) -> bool {
        self.sound_index += 1;
        self.sound_elapsed_clocks = 0;
        self.pcm_position = 0;

        if self.sound_index >= self.sounds.len() as u32 {
            if self.should_loop {
                self.sound_index = 0;
                self.pcm_position = 0;
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

        self.is_playing
    }

    fn seek_pcm(&mut self, start_clock: u32) {
        let clocks_per_sample = AUDIO_CLOCK_RATE / AUDIO_SAMPLE_RATE;
        let sample_offset =
            (start_clock as u64 * AUDIO_SAMPLE_RATE as u64 / AUDIO_CLOCK_RATE as u64) as usize;
        let mut remaining = sample_offset;

        while remaining > 0 && (self.sound_index as usize) < self.sounds.len() {
            let len = {
                let sound = self.sounds[self.sound_index as usize].lock();
                match &sound.pcm {
                    Some(pcm) => pcm.samples.len(),
                    None => break,
                }
            };

            if remaining >= len {
                remaining -= len;
                self.sound_index += 1;
                self.sound_elapsed_clocks = 0;
                self.pcm_position = 0;
            } else {
                self.pcm_position = remaining;
                remaining = 0;
            }
        }

        self.sound_elapsed_clocks = (self.pcm_position as u32) * clocks_per_sample;
        self.total_elapsed_clocks += start_clock;

        if (self.sound_index as usize) >= self.sounds.len() {
            self.is_playing = false;
        }
    }

    fn current_sound_is_pcm(&self) -> bool {
        self.sounds
            .get(self.sound_index as usize)
            .is_some_and(|sound| sound.lock().pcm.is_some())
    }
}
