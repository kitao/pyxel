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
