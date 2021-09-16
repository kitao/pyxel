pub struct Profiler {
    measure_frame_count: u32,
    frame_count: u32,
    start_time: u32,
    total_time: u32,
    average_time: f64,
    average_fps: f64,
}

impl Profiler {
    pub fn new(measure_frame_count: u32) -> Profiler {
        assert!(measure_frame_count >= 1, "invalid measure frame count");

        Profiler {
            measure_frame_count,
            frame_count: 0,
            start_time: 0,
            total_time: 0,
            average_time: 0.0,
            average_fps: 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn average_time(&self) -> f64 {
        self.average_time
    }

    #[allow(dead_code)]
    pub fn average_fps(&self) -> f64 {
        self.average_fps
    }

    pub fn start(&mut self, tick_count: u32) {
        self.start_time = tick_count;
    }

    pub fn end(&mut self, tick_count: u32) {
        self.total_time += tick_count - self.start_time;
        self.frame_count += 1;

        if self.frame_count >= self.measure_frame_count {
            self.average_time = self.total_time as f64 / self.frame_count as f64;
            self.average_fps = 1000.0 / self.average_time;
            self.frame_count = 0;
            self.total_time = 0;
        }
    }
}
