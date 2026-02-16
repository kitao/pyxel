use crate::settings::{
    AUDIO_BUFFER_ADJUST_INTERVAL_MS, AUDIO_BUFFER_ADJUST_MARGIN_SAMPLES, AUDIO_SAMPLE_RATE,
};

pub struct AdaptiveBuffer {
    buffer: Vec<i16>,
    read_pos: usize,
    len: usize,
    refill_samples: Vec<i16>,

    target_fill: usize,
    min_target_fill: usize,
    max_target_fill: usize,

    callback_samples: usize,
    window_samples: usize,
    window_shortfall_samples: usize,
    window_slack_samples: usize,
    adjust_interval_samples: usize,
}

impl AdaptiveBuffer {
    pub fn new(callback_samples: usize, capacity_samples: usize) -> Self {
        assert!(callback_samples > 0);
        let capacity = capacity_samples.max(callback_samples);
        let adjust_interval_samples =
            (AUDIO_SAMPLE_RATE as usize * AUDIO_BUFFER_ADJUST_INTERVAL_MS as usize) / 1000;
        let min_target_fill = callback_samples.min(capacity);
        let max_target_fill = capacity.max(min_target_fill);
        let initial_target_fill = min_target_fill;

        Self {
            buffer: vec![0; capacity],
            read_pos: 0,
            len: 0,
            refill_samples: vec![0; callback_samples],

            target_fill: initial_target_fill,
            min_target_fill,
            max_target_fill,

            callback_samples,
            window_samples: 0,
            window_shortfall_samples: 0,
            window_slack_samples: 0,
            adjust_interval_samples: adjust_interval_samples.max(callback_samples),
        }
    }

    pub fn process<F: FnMut(&mut [i16])>(&mut self, out: &mut [i16], mut render: F) {
        let available_before = self.available();
        let copied = self.pop_into(out);
        let had_underrun = copied < out.len();
        if had_underrun {
            render(&mut out[copied..]);
        }

        let shortfall_samples = out.len() - copied;
        let slack_samples = available_before.saturating_sub(out.len());
        self.record_metrics(out.len(), shortfall_samples, slack_samples);
        self.adjust_target_if_needed();
        self.refill_to_target(out.len(), &mut render);
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

    fn record_metrics(&mut self, samples: usize, shortfall_samples: usize, slack_samples: usize) {
        self.window_samples = self.window_samples.saturating_add(samples);
        self.window_shortfall_samples = self
            .window_shortfall_samples
            .saturating_add(shortfall_samples);
        self.window_slack_samples = self.window_slack_samples.saturating_add(slack_samples);
    }

    fn adjust_target_if_needed(&mut self) {
        if self.window_samples < self.adjust_interval_samples {
            return;
        }

        let avg_shortfall = self
            .window_shortfall_samples
            .saturating_mul(self.callback_samples)
            .saturating_div(self.window_samples);
        let avg_slack = self
            .window_slack_samples
            .saturating_mul(self.callback_samples)
            .saturating_div(self.window_samples);
        let margin_samples = AUDIO_BUFFER_ADJUST_MARGIN_SAMPLES as usize;
        let desired_target = if avg_shortfall > 0 {
            self.target_fill
                .saturating_add(avg_shortfall)
                .saturating_add(margin_samples)
        } else {
            self.target_fill
                .saturating_sub(avg_slack.saturating_sub(margin_samples))
        };

        self.target_fill = desired_target.clamp(self.min_target_fill, self.max_target_fill);
        self.window_samples = 0;
        self.window_shortfall_samples = 0;
        self.window_slack_samples = 0;
    }

    fn refill_to_target<F: FnMut(&mut [i16])>(&mut self, callback_samples: usize, render: &mut F) {
        if callback_samples == 0 {
            return;
        }

        let available = self.available();
        if available >= self.target_fill {
            return;
        }

        let mut remaining = (self.target_fill - available).min(self.free_space());

        while remaining > 0 {
            let chunk_size = remaining.min(self.callback_samples);
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
