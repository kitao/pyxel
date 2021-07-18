pub struct Profiler {
    measure_frame_count: u32,
    frame_count: u32,
    start_time: u32,
    end_time: u32,
    total_time: u32,
    average_time: f64,
    average_fps: f64,
}

impl Profiler {
    pub fn new(measure_frame_count: u32) -> Profiler {
        assert!(measure_frame_count >= 1, "invalid measure frame count");

        Profiler {
            measure_frame_count: measure_frame_count,
            frame_count: 0,
            start_time: 0,
            end_time: 0,
            total_time: 0,
            average_time: 0.0,
            average_fps: 0.0,
        }
    }

    pub fn average_time(&self) -> f64 {
        self.average_time
    }

    pub fn averate_fps(&self) -> f64 {
        self.averate_fps()
    }

    pub fn start(&mut self, ticks: u32) {
        self.start_time = ticks;
    }

    pub fn end(&mut self, ticks: u32) {
        self.total_time += ticks - self.start_time;
        self.frame_count += 1;

        if self.frame_count >= self.measure_frame_count {
            self.average_time = self.total_time as f64 / self.frame_count as f64;
            self.average_fps = 1000.0 / self.average_time;

            self.frame_count = 0;
            self.total_time = 0;
        }
    }
}
