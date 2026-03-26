pub struct Profiler {
    num_measure_frames: u32,
    num_measured_frames: u32,
    start_time: u32,
    total_time: u32,
    average_time: f32,
    average_fps: f32,
}

impl Profiler {
    pub fn new(num_measure_frames: u32) -> Self {
        assert!(num_measure_frames >= 1, "invalid measure frame count");
        Self {
            num_measure_frames,
            num_measured_frames: 0,
            start_time: 0,
            total_time: 0,
            average_time: 0.0,
            average_fps: 0.0,
        }
    }

    pub const fn average_time(&self) -> f32 {
        self.average_time
    }

    pub const fn average_fps(&self) -> f32 {
        self.average_fps
    }

    pub fn start(&mut self, tick_count: u32) {
        self.start_time = tick_count;
    }

    pub fn end(&mut self, tick_count: u32) {
        self.total_time += tick_count - self.start_time;
        self.num_measured_frames += 1;

        if self.num_measured_frames >= self.num_measure_frames {
            self.average_time = self.total_time as f32 / self.num_measured_frames as f32;
            self.average_fps = 1000.0 / self.average_time;
            self.num_measured_frames = 0;
            self.total_time = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = Profiler::new(5);
        assert_eq!(p.average_time(), 0.0);
        assert_eq!(p.average_fps(), 0.0);
    }

    #[test]
    #[should_panic(expected = "invalid measure frame count")]
    fn test_new_zero_frames_panics() {
        Profiler::new(0);
    }

    #[test]
    fn test_single_measurement_cycle() {
        // 1-frame profiler: delta = 110 - 100 = 10ms → fps = 1000/10 = 100
        let mut p = Profiler::new(1);
        p.start(100);
        p.end(110);
        assert_eq!(p.average_time(), 10.0);
        assert_eq!(p.average_fps(), 100.0);
    }

    #[test]
    fn test_multi_frame_average() {
        // 3-frame profiler: averages should only update after 3rd end()
        let mut p = Profiler::new(3);

        // Frame 1: delta = 10
        p.start(0);
        p.end(10);
        assert_eq!(p.average_time(), 0.0, "should not update before 3 frames");

        // Frame 2: delta = 20
        p.start(100);
        p.end(120);
        assert_eq!(p.average_time(), 0.0, "should not update before 3 frames");

        // Frame 3: delta = 30, total = 60, average = 20
        p.start(200);
        p.end(230);
        assert_eq!(p.average_time(), 20.0);
        assert_eq!(p.average_fps(), 1000.0 / 20.0);
    }

    #[test]
    fn test_consecutive_cycles() {
        let mut p = Profiler::new(2);

        // Cycle 1: deltas 10, 20 → average = 15
        p.start(0);
        p.end(10);
        p.start(100);
        p.end(120);
        assert_eq!(p.average_time(), 15.0);

        // Cycle 2: deltas 5, 5 → average = 5 (previous cycle fully replaced)
        p.start(200);
        p.end(205);
        assert_eq!(p.average_time(), 15.0, "mid-cycle retains previous average");

        p.start(300);
        p.end(305);
        assert_eq!(p.average_time(), 5.0);
        assert_eq!(p.average_fps(), 200.0);
    }

    #[test]
    fn test_zero_time_frame() {
        // start and end at the same tick → 0ms frame time
        let mut p = Profiler::new(1);
        p.start(100);
        p.end(100);
        assert_eq!(p.average_time(), 0.0);
        // FPS is inf (1000/0), but we just verify it doesn't panic
        assert!(p.average_fps().is_infinite());
    }

    #[test]
    fn test_tick_count_wraparound() {
        // u32 subtraction wraps around correctly
        let mut p = Profiler::new(1);
        p.start(u32::MAX - 5);
        p.end(u32::MAX); // delta = 5
        assert_eq!(p.average_time(), 5.0);
    }
}
