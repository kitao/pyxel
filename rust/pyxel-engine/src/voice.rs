use crate::blip_buf::BlipBuf;
use crate::settings::CLOCK_RATE;

const WAVEFORM_TABLE_LENGTH: usize = 32;
const WAVEFORM_TABLE_MAX_VALUE: u8 = 15;

pub fn seconds_to_clocks(seconds: f64) -> u32 {
    (seconds * CLOCK_RATE as f64).round() as u32
}

pub struct Waveform {
    table: [f64; WAVEFORM_TABLE_LENGTH],
    phase: usize,
    lfsr: u16,
    tap_bit: u8,
    amplitude: f64,
    cached_note: f64,
    cached_phase_duration: u32,
}

impl Waveform {
    pub fn new(table: [u8; WAVEFORM_TABLE_LENGTH]) -> Self {
        let half_max = WAVEFORM_TABLE_MAX_VALUE as f64 / 2.0;
        let table = table.map(|v| (v as f64 - half_max) / half_max);

        Self {
            table,
            phase: 0,
            lfsr: 0,
            tap_bit: 0,
            amplitude: table[0],
            cached_note: f64::NAN,
            cached_phase_duration: 0,
        }
    }

    pub fn new_noise(short_period: bool) -> Self {
        Self {
            table: [0.0; WAVEFORM_TABLE_LENGTH],
            phase: 0,
            lfsr: if short_period { 0x0201 } else { 0x7001 },
            tap_bit: if short_period { 6 } else { 1 },
            amplitude: -1.0,
            cached_note: f64::MAX,
            cached_phase_duration: 0,
        }
        // The LFSR originally starts with 0x0001, but to avoid leading zeros, it is advanced first.
        // For short-period noise, it's shifted 15 times (half of the 32-sample period), resulting in 0x0201.
        // For long-period noise, it's shifted 45 times (half of the 93-sample period), resulting in 0x7001.
    }

    pub fn amplitude(&self) -> f64 {
        self.amplitude
    }

    pub fn phase_duration(&mut self, note: f64) -> u32 {
        if note == self.cached_note {
            return self.cached_phase_duration;
        }

        let semitone_offset = (note - 69.0) / 12.0;
        let pitch = 440.0 * semitone_offset.exp2();
        let period = CLOCK_RATE as f64 / pitch;
        let phase_duration = (period / WAVEFORM_TABLE_LENGTH as f64).round() as u32;

        self.cached_note = note;
        self.cached_phase_duration = phase_duration;

        phase_duration
    }

    pub fn next_phase(&mut self) {
        if self.tap_bit == 0 {
            self.phase = (self.phase + 1) % WAVEFORM_TABLE_LENGTH;
            self.amplitude = self.table[self.phase];
        } else {
            let feedback = (self.lfsr ^ (self.lfsr >> self.tap_bit)) & 1;
            self.lfsr = ((self.lfsr >> 1) | (feedback << 14)) & 0x7FFF;
            self.amplitude = if (self.lfsr & 1) == 0 { 1.0 } else { -1.0 };
        }
    }
}

pub struct Envelope {
    attack_end: u32,
    decay_end: u32,
    release_duration: u32,
    attack_step: f64,
    sustain_level: f64,
    decay_step: f64,
    release_step: f64,
}

impl Envelope {
    pub fn new(attack: f64, decay: f64, sustain: f64, release: f64) -> Self {
        let attack = seconds_to_clocks(attack);
        let decay = seconds_to_clocks(decay);
        let release = seconds_to_clocks(release);
        let attack_step = if attack > 0 { 1.0 / attack as f64 } else { 0.0 };
        let decay_step = if decay > 0 {
            (1.0 - sustain) / decay as f64
        } else {
            0.0
        };
        let release_step = if release > 0 {
            sustain / release as f64
        } else {
            0.0
        };

        Self {
            attack_end: attack,
            decay_end: attack + decay,
            release_duration: release,
            attack_step,
            sustain_level: sustain,
            decay_step,
            release_step,
        }
    }

    pub fn amplitude(&self, clock: u32, note_duration: u32) -> f64 {
        if clock < self.attack_end {
            clock as f64 * self.attack_step
        } else if clock < self.decay_end {
            1.0 - (clock - self.attack_end) as f64 * self.decay_step
        } else if clock < note_duration {
            self.sustain_level
        } else if clock < note_duration + self.release_duration {
            self.sustain_level - (clock - note_duration) as f64 * self.release_step
        } else {
            0.0
        }
    }
}

pub struct Vibrato {
    delay: u32,
    period: u32,
    inv_period: f64,
    depth: f64,
}

impl Vibrato {
    pub fn new(delay: f64, speed: f64, depth: f64) -> Self {
        let delay = seconds_to_clocks(delay);
        let period = seconds_to_clocks(1.0 / speed);
        let inv_period = 1.0 / period as f64;
        Self {
            delay,
            period,
            inv_period,
            depth,
        }
    }

    pub fn pitch_offset(&self, clock: u32) -> f64 {
        if clock < self.delay {
            0.0
        } else {
            let phase = (clock - self.delay) as f64 * self.inv_period;
            let modulation = 1.0 - 4.0 * ((phase + 0.25).fract() - 0.5).abs();
            modulation * self.depth
        }
    }
}

pub struct Portament {
    pitch_step: f64,
    duration: u32,
}

impl Portament {
    pub fn new(pitch_offset: f64, time: f64) -> Self {
        let duration = seconds_to_clocks(time);
        let pitch_step = if duration > 0 {
            pitch_offset / duration as f64
        } else {
            0.0
        };
        Self {
            pitch_step,
            duration,
        }
    }

    pub fn pitch_offset(&self, clock: u32) -> f64 {
        if clock < self.duration {
            self.pitch_step * (self.duration - clock) as f64
        } else {
            0.0
        }
    }
}

pub struct NewOscillator {
    envelope: Envelope,
    waveform: Waveform,
    vibrato: Option<Vibrato>,
    portament: Option<Portament>,
    pitch: f64,
    duration_clocks: u32,
    elapsed_clocks: u32,
    note_on_clock: u32,
    active: bool,
    last_sample: f64, // 前回の出力サンプル値を保持
}

impl NewOscillator {
    pub fn new() -> Self {
        Self {
            waveform: Waveform::new([0; WAVEFORM_TABLE_LENGTH]),
            envelope: Envelope::new(0.0, 0.0, 1.0, 0.0),
            vibrato: None,
            portament: None,
            pitch: 0.0,
            duration_clocks: 0,
            elapsed_clocks: 0,
            note_on_clock: 0,
            active: false,
            last_sample: 0.0,
        }
    }

    pub fn set_waveform(&mut self, waveform: [u8; WAVEFORM_TABLE_LENGTH]) {
        self.waveform = Waveform::new(waveform);
    }

    pub fn set_noise(&mut self, short_period: bool) {
        self.waveform = Waveform::new_noise(short_period);
    }

    pub fn set_envelope(&mut self, attack: f64, decay: f64, sustain: f64, release: f64) {
        self.envelope = Envelope::new(attack, decay, sustain, release);
    }

    pub fn start_vibrato(&mut self, delay: f64, speed: f64, depth: f64) {
        self.vibrato = Some(Vibrato::new(delay, speed, depth));
    }

    pub fn stop_vibrato(&mut self) {
        self.vibrato = None;
    }

    pub fn start_portament(&mut self, offset: f64, time: f64) {
        self.portament = Some(Portament::new(offset, time));
    }

    pub fn stop_portament(&mut self) {
        self.portament = None;
    }

    pub fn note_on(&mut self, pitch: f64, duration: u32) {
        // TODO
    }

    pub fn process(&mut self, blip_buf: &mut BlipBuf, clocks: u32, start_clock: u32) {
        if !self.active {
            return;
        }

        let table_length = WAVEFORM_TABLE_LENGTH as u32;
        let mut current_clock = start_clock;
        let mut previous_sample = self.last_sample; // 前回のサンプル値からスタート

        while current_clock < start_clock + clocks {
            if self.elapsed_clocks >= self.duration_clocks {
                self.active = false;
                break;
            }

            // Wavetable の現在の位相から残りのステップ数を計算
            let phase_pos = self.waveform.phase as u32 % table_length;
            let steps_remaining = table_length - phase_pos;

            // 処理するステップ数（分割呼び出しでも正しい出力を保証）
            let steps = steps_remaining.min((start_clock + clocks) - current_clock);

            // 1周期分の開始時に音量とピッチ変調を計算
            let amplitude = self
                .envelope
                .level(self.elapsed_clocks, self.duration_clocks);
            let mut pitch_offset = 0.0;

            if let Some(portament) = &self.portament {
                pitch_offset += portament.pitch_offset(self.elapsed_clocks);
            }
            if let Some(vibrato) = &self.vibrato {
                pitch_offset += vibrato.pitch_offset(self.elapsed_clocks);
            }

            let final_pitch = self.pitch + pitch_offset;

            // steps 回分の処理を実行（位相を明示的に管理）
            for _ in 0..steps {
                let current_sample = self.waveform.sample() * amplitude;
                let delta = ((current_sample - previous_sample) * 32767.0) as i32;

                // 差分を BlipBuf に渡す
                blip_buf.add_delta(current_clock as u64, delta);

                previous_sample = current_sample; // サンプル値を更新
                current_clock += 1;
                self.elapsed_clocks += 1;

                if self.elapsed_clocks >= self.duration_clocks {
                    self.active = false;
                    break;
                }

                // 位相を明示的に進める
                self.waveform.next_phase();
            }
        }

        // 次回の process 呼び出しに備えてサンプル値を保存
        self.last_sample = previous_sample;
    }
}
