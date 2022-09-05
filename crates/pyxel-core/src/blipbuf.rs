/*
    Copied from blippers by Antoine Büsch
    https://github.com/abusch/blippers
*/

#![allow(dead_code)]

pub const BLIP_MAX_FRAME: i32 = 4000;
pub const BLIP_MAX_RATIO: i32 = 1 << 20;
const PRE_SHIFT: i32 = 32;
const TIME_BITS: i32 = PRE_SHIFT + 20;
const TIME_UNIT: u64 = 1 << TIME_BITS as u64;
const BASS_SHIFT: i32 = 9;
const END_FRAME_EXTRA: i32 = 2;
const HALF_WIDTH: i32 = 8;
const BUF_EXTRA: i32 = HALF_WIDTH * 2 + END_FRAME_EXTRA;
const PHASE_BITS: i32 = 5;
const PHASE_COUNT: i32 = 1 << PHASE_BITS;
const DELTA_BITS: i32 = 15;
const DELTA_UNIT: i32 = 1 << DELTA_BITS;
const FRAC_BITS: i32 = TIME_BITS - PRE_SHIFT;

pub struct BlipBuf {
    factor: u64,
    offset: u64,
    avail: i32,
    integrator: i32,
    buf: Vec<i32>,
}

impl BlipBuf {
    pub fn new(size: usize) -> Self {
        let buf = vec![0_i32; size + BUF_EXTRA as usize];
        let factor = TIME_UNIT / BLIP_MAX_RATIO as u64;
        let mut m = Self {
            factor,
            offset: factor / 2,
            avail: 0,
            integrator: 0,
            buf,
        };
        m.clear();

        m
    }

    pub fn set_rates(&mut self, clock_rate: f64, sample_rate: f64) {
        let factor = TIME_UNIT as f64 * sample_rate / clock_rate;
        // round up
        self.factor = factor.ceil() as u64;
    }

    pub fn clear(&mut self) {
        self.offset = self.factor / 2;
        self.avail = 0;
        self.integrator = 0;
        self.buf.fill(0);
    }

    pub fn add_delta(&mut self, time: u64, delta: i32) {
        let fixed = (time * self.factor + self.offset) >> PRE_SHIFT;
        let start = (self.avail + (fixed >> FRAC_BITS) as i32) as usize;
        let out = &mut self.buf[start..];

        let phase_shift = FRAC_BITS - PHASE_BITS;
        let phase = ((fixed >> phase_shift) as i32 & (PHASE_COUNT - 1)) as usize;
        let phase_rev = PHASE_COUNT as usize - phase as usize;

        let interp = (fixed >> (phase_shift - DELTA_BITS)) as i32 & (DELTA_UNIT - 1);
        let delta2 = (delta * interp as i32) >> DELTA_BITS;
        let delta = delta - delta2;

        out[0] = out[0]
            .wrapping_add(BL_STEP[phase][0] as i32 * delta + BL_STEP[phase + 1][0] as i32 * delta2);
        out[1] = out[1]
            .wrapping_add(BL_STEP[phase][1] as i32 * delta + BL_STEP[phase + 1][1] as i32 * delta2);
        out[2] = out[2]
            .wrapping_add(BL_STEP[phase][2] as i32 * delta + BL_STEP[phase + 1][2] as i32 * delta2);
        out[3] = out[3]
            .wrapping_add(BL_STEP[phase][3] as i32 * delta + BL_STEP[phase + 1][3] as i32 * delta2);
        out[4] = out[4]
            .wrapping_add(BL_STEP[phase][4] as i32 * delta + BL_STEP[phase + 1][4] as i32 * delta2);
        out[5] = out[5]
            .wrapping_add(BL_STEP[phase][5] as i32 * delta + BL_STEP[phase + 1][5] as i32 * delta2);
        out[6] = out[6]
            .wrapping_add(BL_STEP[phase][6] as i32 * delta + BL_STEP[phase + 1][6] as i32 * delta2);
        out[7] = out[7]
            .wrapping_add(BL_STEP[phase][7] as i32 * delta + BL_STEP[phase + 1][7] as i32 * delta2);

        out[8] = out[8].wrapping_add(
            BL_STEP[phase_rev][7] as i32 * delta + BL_STEP[phase_rev - 1][7] as i32 * delta2,
        );
        out[9] = out[9].wrapping_add(
            BL_STEP[phase_rev][6] as i32 * delta + BL_STEP[phase_rev - 1][6] as i32 * delta2,
        );
        out[10] = out[10].wrapping_add(
            BL_STEP[phase_rev][5] as i32 * delta + BL_STEP[phase_rev - 1][5] as i32 * delta2,
        );
        out[11] = out[11].wrapping_add(
            BL_STEP[phase_rev][4] as i32 * delta + BL_STEP[phase_rev - 1][4] as i32 * delta2,
        );
        out[12] = out[12].wrapping_add(
            BL_STEP[phase_rev][3] as i32 * delta + BL_STEP[phase_rev - 1][3] as i32 * delta2,
        );
        out[13] = out[13].wrapping_add(
            BL_STEP[phase_rev][2] as i32 * delta + BL_STEP[phase_rev - 1][2] as i32 * delta2,
        );
        out[14] = out[14].wrapping_add(
            BL_STEP[phase_rev][1] as i32 * delta + BL_STEP[phase_rev - 1][1] as i32 * delta2,
        );
        out[15] = out[15].wrapping_add(
            BL_STEP[phase_rev][0] as i32 * delta + BL_STEP[phase_rev - 1][0] as i32 * delta2,
        );
    }

    pub fn add_delta_fast(&mut self, time: u64, delta: i32) {
        let fixed = ((time * self.factor + self.offset) >> PRE_SHIFT) as u64;
        let out = &mut self.buf[(self.avail as usize + (fixed >> FRAC_BITS) as usize)..];

        let interp = (fixed >> (FRAC_BITS - DELTA_BITS) & (DELTA_UNIT - 1) as u64) as i32;
        let delta2 = delta * interp;

        out[7] += delta * DELTA_UNIT - delta2;
        out[8] += delta2;
    }

    pub fn clocks_needed(&self, samples: i32) -> i32 {
        // Fail if the buffer can't hold that many more samples
        assert!(self.avail + samples <= self.buf.capacity() as i32);

        let needed = samples as u64 * TIME_UNIT;
        if needed < self.offset as u64 {
            0
        } else {
            ((needed - self.offset + self.factor - 1) / self.factor) as i32
        }
    }

    pub const fn samples_avail(&self) -> i32 {
        self.avail
    }

    pub fn read_samples(&mut self, buf: &mut [i16], stereo: bool) -> usize {
        let count = if buf.len() > self.avail as usize {
            self.avail as usize
        } else {
            buf.len()
        };

        let step = if stereo { 2 } else { 1 };
        // let in = self.buf;
        let mut sum = self.integrator;
        let mut out = 0;
        for i in 0..count {
            // Eliminate fraction
            let mut s = sum >> DELTA_BITS;
            sum = sum.wrapping_add(self.buf[i]);
            s = s.clamp(i16::MIN as i32, i16::MAX as i32);
            buf[out] = s as i16;
            out += step;
            sum = sum.wrapping_sub(s << (DELTA_BITS - BASS_SHIFT));
        }
        self.integrator = sum;
        self.remove_samples(count);
        count
    }

    pub fn end_frame(&mut self, t: u64) {
        let off = t * self.factor + self.offset;
        self.avail += (off >> TIME_BITS) as i32;
        self.offset = off & (TIME_UNIT - 1);

        assert!(self.avail <= self.buf.capacity() as i32);
    }

    fn remove_samples(&mut self, count: usize) {
        let remain = (self.avail + BUF_EXTRA - count as i32) as usize;
        self.avail -= count as i32;

        self.buf.copy_within(count..(count + remain), 0);
        self.buf[remain..(remain + count as usize)].fill(0);
    }
}

const BL_STEP: [[i16; HALF_WIDTH as usize]; PHASE_COUNT as usize + 1] = [
    [43, -115, 350, -488, 1136, -914, 5861, 21022],
    [44, -118, 348, -473, 1076, -799, 5274, 21001],
    [45, -121, 344, -454, 1011, -677, 4706, 20936],
    [46, -122, 336, -431, 942, -549, 4156, 20829],
    [47, -123, 327, -404, 868, -418, 3629, 20679],
    [47, -122, 316, -375, 792, -285, 3124, 20488],
    [47, -120, 303, -344, 714, -151, 2644, 20256],
    [46, -117, 289, -310, 634, -17, 2188, 19985],
    [46, -114, 273, -275, 553, 117, 1758, 19675],
    [44, -108, 255, -237, 471, 247, 1356, 19327],
    [43, -103, 237, -199, 390, 373, 981, 18944],
    [42, -98, 218, -160, 310, 495, 633, 18527],
    [40, -91, 198, -121, 231, 611, 314, 18078],
    [38, -84, 178, -81, 153, 722, 22, 17599],
    [36, -76, 157, -43, 80, 824, -241, 17092],
    [34, -68, 135, -3, 8, 919, -476, 16558],
    [32, -61, 115, 34, -60, 1006, -683, 16001],
    [29, -52, 94, 70, -123, 1083, -862, 15422],
    [27, -44, 73, 106, -184, 1152, -1015, 14824],
    [25, -36, 53, 139, -239, 1211, -1142, 14210],
    [22, -27, 34, 170, -290, 1261, -1244, 13582],
    [20, -20, 16, 199, -335, 1301, -1322, 12942],
    [18, -12, -3, 226, -375, 1331, -1376, 12293],
    [15, -4, -19, 250, -410, 1351, -1408, 11638],
    [13, 3, -35, 272, -439, 1361, -1419, 10979],
    [11, 9, -49, 292, -464, 1362, -1410, 10319],
    [9, 16, -63, 309, -483, 1354, -1383, 9660],
    [7, 22, -75, 322, -496, 1337, -1339, 9005],
    [6, 26, -85, 333, -504, 1312, -1280, 8355],
    [4, 31, -94, 341, -507, 1278, -1205, 7713],
    [3, 35, -102, 347, -506, 1238, -1119, 7082],
    [1, 40, -110, 350, -499, 1190, -1021, 6464],
    [0, 43, -115, 350, -488, 1136, -914, 5861],
];

#[cfg(test)]
mod tests {
    use crate::blipbuf::{BlipBuf, BLIP_MAX_FRAME, BLIP_MAX_RATIO};

    const OVERSAMPLE: i32 = BLIP_MAX_RATIO;
    const BLIP_SIZE: i32 = BLIP_MAX_FRAME / 2;

    fn blip() -> BlipBuf {
        BlipBuf::new(BLIP_SIZE as usize)
    }

    #[test]
    fn test_empty_on_creation() {
        let b = blip();
        assert!(b.samples_avail() == 0);
    }

    #[test]
    fn default_ratio() {
        let b = blip();
        assert_eq!(b.clocks_needed(1), BLIP_MAX_RATIO);
    }

    #[test]
    fn end_frame_sample_avail() {
        let mut b = blip();
        b.end_frame(OVERSAMPLE as u64);
        assert_eq!(b.samples_avail(), 1);

        b.end_frame(OVERSAMPLE as u64 * 2);
        assert_eq!(b.samples_avail(), 3);
    }

    #[test]
    fn end_frame_sample_avail_fractional() {
        let mut b = blip();

        b.end_frame(OVERSAMPLE as u64 * 2 - 1);
        assert_eq!(b.samples_avail(), 1);

        b.end_frame(1);
        assert_eq!(b.samples_avail(), 2);
    }

    #[ignore = "FIXME"]
    #[test]
    #[should_panic]
    fn end_frame_limits() {
        let mut b = blip();

        b.end_frame(0);
        assert_eq!(b.samples_avail(), 0);

        b.end_frame((BLIP_SIZE * OVERSAMPLE + OVERSAMPLE - 1) as u64);
        // should panic
        b.end_frame(1);
    }

    #[test]
    fn read_samples() {
        let mut b = blip();
        let mut buf = [-1, -1];

        b.end_frame((3 * OVERSAMPLE + OVERSAMPLE - 1) as u64);
        assert_eq!(b.read_samples(&mut buf[..], false), 2);
        assert_eq!(buf[0], 0);
        assert_eq!(buf[1], 0);

        assert_eq!(b.samples_avail(), 1);
        assert_eq!(b.clocks_needed(1), 1);
    }

    #[test]
    fn read_samples_stereo() {
        let mut b = blip();
        let mut buf = [-1, -1, -1];

        b.end_frame((2 * OVERSAMPLE) as u64);
        assert_eq!(b.read_samples(&mut buf[..], true), 2);
        assert_eq!(buf[0], 0);
        assert_eq!(buf[1], -1);
        assert_eq!(buf[2], 0);
    }

    #[test]
    fn set_rates() {
        let mut b = blip();

        b.set_rates(2.0, 2.0);
        assert_eq!(b.clocks_needed(10), 10);
        b.set_rates(2.0, 4.0);
        assert_eq!(b.clocks_needed(10), 5);
        b.set_rates(4.0, 2.0);
        assert_eq!(b.clocks_needed(10), 20);
    }

    #[test]
    fn set_rates_round_up() {
        let mut b = blip();

        for r in 1..10000 {
            b.set_rates(r as f64, 1.0);
            assert!(b.clocks_needed(1) <= r);
        }
    }

    #[test]
    fn set_rates_accuracy() {
        let mut b = blip();
        let max_error = 100;

        for r in (BLIP_SIZE / 2)..BLIP_SIZE {
            let mut c = r / 2;
            while c < 8000000 {
                b.set_rates(c as f64, r as f64);
                let error = b.clocks_needed(r) - c;
                assert!(error.abs() < (c / max_error));

                c += c / 32;
            }
        }
    }

    #[test]
    fn set_rates_high_accuracy() {
        let mut b = blip();
        b.set_rates(1000000.0, BLIP_SIZE as f64);

        if b.clocks_needed(BLIP_SIZE) != 1000000 {
            eprintln!("Skipping because 64bits int isn't available");
            return;
        }

        for r in (BLIP_SIZE / 2)..BLIP_SIZE {
            let mut c = r / 2;
            while c < 200000000 {
                b.set_rates(c as f64, r as f64);
                assert_eq!(b.clocks_needed(r), c);

                c += c / 32;
            }
        }
    }

    #[test]
    fn set_rates_long_term_accuracy() {
        let mut b = blip();
        b.set_rates(1000000.0, BLIP_SIZE as f64);

        if b.clocks_needed(BLIP_SIZE) != 1000000 {
            eprintln!("Skipping because 64bits int isn't available");
            return;
        }

        // Generates secs seconds and ensures that exactly secs*sample_rate samples
        // are generated
        let clock_rate = 1789773.0;
        let sample_rate = 44100.0;
        let secs = 1000_f64;
        b.set_rates(clock_rate, sample_rate);

        const BUF_SIZE: i32 = BLIP_SIZE / 2;
        let clock_size = b.clocks_needed(BUF_SIZE) - 1;
        let mut total_samples = 0_f64;

        let mut remain = clock_rate * secs;
        loop {
            let n = if remain < clock_size as f64 {
                remain as i32
            } else {
                clock_size
            };
            if n == 0 {
                break;
            }

            b.end_frame(n as u64);
            let mut buf = [0; BUF_SIZE as usize];
            total_samples += b.read_samples(&mut buf[..], false) as f64;

            remain -= n as f64;
        }

        assert_eq!(total_samples, sample_rate * secs);
    }
}
