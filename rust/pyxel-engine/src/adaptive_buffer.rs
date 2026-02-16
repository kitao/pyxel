use std::time::Instant;

use crate::settings::{
    AUDIO_BUFFER_ADJUST_SAMPLES, AUDIO_BUFFER_DOWN_SAMPLES, AUDIO_BUFFER_UNDERRUN_SAMPLES,
    AUDIO_BUFFER_UP_SAMPLES, AUDIO_MAX_BUFFER_SAMPLES, AUDIO_MIN_BUFFER_SAMPLES, AUDIO_SAMPLE_RATE,
};

const CALLBACK_UNDERRUN_TOLERANCE_SAMPLES: u128 = 44; // 44 / 22050 * 1000 = 2.0ms

pub struct AdaptiveBuffer {
    buffer: Vec<i16>,
    read_pos: usize,
    len: usize,
    refill_samples: Vec<i16>,

    target_fill: usize,
    min_target_fill: usize,
    max_target_fill: usize,

    last_callback_start_time: Option<Instant>,
    last_callback_samples: Option<usize>,
    window_samples: usize,
    window_underrun_samples: usize,
    adjust_interval_samples: usize,
    underrun_threshold_samples: usize,
    up_samples: usize,
    down_samples: usize,
}

impl AdaptiveBuffer {
    pub fn new() -> Self {
        let initial_callback_samples = AUDIO_MIN_BUFFER_SAMPLES as usize;
        let capacity = (AUDIO_MAX_BUFFER_SAMPLES as usize).max(initial_callback_samples);
        let adjust_interval_samples =
            (AUDIO_BUFFER_ADJUST_SAMPLES as usize).max(initial_callback_samples);
        let min_target_fill = initial_callback_samples;
        let max_target_fill = capacity;

        Self {
            buffer: vec![0; capacity],
            read_pos: 0,
            len: 0,
            refill_samples: vec![0; initial_callback_samples],

            target_fill: min_target_fill,
            min_target_fill,
            max_target_fill,

            last_callback_start_time: None,
            last_callback_samples: None,
            window_samples: 0,
            window_underrun_samples: 0,
            adjust_interval_samples,
            underrun_threshold_samples: AUDIO_BUFFER_UNDERRUN_SAMPLES as usize,
            up_samples: AUDIO_BUFFER_UP_SAMPLES as usize,
            down_samples: AUDIO_BUFFER_DOWN_SAMPLES as usize,
        }
    }

    pub fn process<F: FnMut(&mut [i16])>(
        &mut self,
        out: &mut [i16],
        callback_start_time: Instant,
        mut render: F,
    ) {
        if out.is_empty() {
            self.last_callback_start_time = Some(callback_start_time);
            self.last_callback_samples = Some(0);
            return;
        }

        let callback_samples = out.len();
        if self.refill_samples.len() < callback_samples {
            self.refill_samples.resize(callback_samples, 0);
        }

        let min_target_fill = callback_samples.min(self.max_target_fill);
        if self.min_target_fill != min_target_fill {
            self.min_target_fill = min_target_fill;
            self.target_fill = self.target_fill.max(self.min_target_fill);
        }

        let adjust_interval_samples = (AUDIO_BUFFER_ADJUST_SAMPLES as usize).max(callback_samples);
        if self.adjust_interval_samples != adjust_interval_samples {
            self.adjust_interval_samples = adjust_interval_samples;
            self.window_samples = 0;
            self.window_underrun_samples = 0;
        }

        let callback_underrun_samples =
            self.estimate_callback_underrun_samples(callback_start_time, callback_samples);
        let copied = self.pop_into(out);
        if copied < out.len() {
            render(&mut out[copied..]);
        }

        let buffer_underrun_samples = out.len().saturating_sub(copied);
        let underrun_samples = callback_underrun_samples.max(buffer_underrun_samples);
        self.record_metrics(out.len(), underrun_samples);
        self.adjust_target_if_needed();
        self.refill_to_target(callback_samples, &mut render);
    }

    fn available(&self) -> usize {
        self.len
    }

    fn free_space(&self) -> usize {
        self.buffer.len() - self.len
    }

    fn pop_into(&mut self, out: &mut [i16]) -> usize {
        let to_pop = out.len().min(self.len);
        if to_pop == 0 {
            return 0;
        }

        let first = to_pop.min(self.buffer.len() - self.read_pos);
        out[..first].copy_from_slice(&self.buffer[self.read_pos..self.read_pos + first]);

        let second = to_pop - first;
        if second > 0 {
            out[first..first + second].copy_from_slice(&self.buffer[..second]);
        }

        self.read_pos = (self.read_pos + to_pop) % self.buffer.len();
        self.len -= to_pop;
        to_pop
    }

    fn push_from_refill(&mut self, refill_len: usize) -> usize {
        let to_push = refill_len.min(self.free_space());
        if to_push == 0 {
            return 0;
        }

        let write_pos = (self.read_pos + self.len) % self.buffer.len();
        let first = to_push.min(self.buffer.len() - write_pos);
        self.buffer[write_pos..write_pos + first].copy_from_slice(&self.refill_samples[..first]);

        let second = to_push - first;
        if second > 0 {
            self.buffer[..second].copy_from_slice(&self.refill_samples[first..first + second]);
        }

        self.len += to_push;
        to_push
    }

    fn record_metrics(&mut self, samples: usize, underrun_samples: usize) {
        self.window_samples = self.window_samples.saturating_add(samples);
        self.window_underrun_samples = self
            .window_underrun_samples
            .saturating_add(underrun_samples);
    }

    fn adjust_target_if_needed(&mut self) {
        if self.window_samples < self.adjust_interval_samples {
            return;
        }

        let desired_target = if self.window_underrun_samples >= self.underrun_threshold_samples {
            self.target_fill.saturating_add(self.up_samples)
        } else if self.window_underrun_samples == 0 {
            self.target_fill.saturating_sub(self.down_samples)
        } else {
            self.target_fill
        };

        self.target_fill = desired_target.clamp(self.min_target_fill, self.max_target_fill);
        self.window_samples = 0;
        self.window_underrun_samples = 0;
    }

    fn estimate_callback_underrun_samples(
        &mut self,
        callback_start_time: Instant,
        callback_samples: usize,
    ) -> usize {
        let Some(last_callback_start_time) = self.last_callback_start_time else {
            self.last_callback_start_time = Some(callback_start_time);
            self.last_callback_samples = Some(callback_samples);
            return 0;
        };
        let Some(last_callback_samples) = self.last_callback_samples else {
            self.last_callback_start_time = Some(callback_start_time);
            self.last_callback_samples = Some(callback_samples);
            return 0;
        };
        self.last_callback_start_time = Some(callback_start_time);
        self.last_callback_samples = Some(callback_samples);

        if last_callback_samples == 0 {
            return 0;
        }

        let elapsed_us = callback_start_time
            .duration_since(last_callback_start_time)
            .as_micros();
        let expected_us = (last_callback_samples as u128)
            .saturating_mul(1_000_000)
            .saturating_div(AUDIO_SAMPLE_RATE as u128);
        let late_us = elapsed_us.saturating_sub(expected_us);
        if late_us == 0 {
            return 0;
        }

        let late_samples = late_us
            .saturating_mul(AUDIO_SAMPLE_RATE as u128)
            .saturating_add(999_999)
            .saturating_div(1_000_000);
        let effective_late_samples =
            late_samples.saturating_sub(CALLBACK_UNDERRUN_TOLERANCE_SAMPLES);
        effective_late_samples.min(usize::MAX as u128) as usize
    }

    fn refill_to_target<F: FnMut(&mut [i16])>(&mut self, callback_samples: usize, render: &mut F) {
        let available = self.available();
        if available >= self.target_fill {
            return;
        }

        let mut remaining = (self.target_fill - available).min(self.free_space());

        while remaining > 0 {
            let chunk_size = remaining.min(callback_samples);
            {
                let refill_chunk = &mut self.refill_samples[..chunk_size];
                render(refill_chunk);
            }

            let pushed = self.push_from_refill(chunk_size);
            if pushed == 0 {
                break;
            }

            remaining -= pushed;
            if pushed < chunk_size {
                break;
            }
        }
    }
}
