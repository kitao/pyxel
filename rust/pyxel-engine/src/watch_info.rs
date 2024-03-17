use std::env::var;
use std::fs::{read_to_string, write};

use crate::settings::WATCH_INFO_FILE_ENVVAR;

pub struct WatchInfo {
    watch_info_file: Option<String>,
    window_x: i32,
    window_y: i32,
    window_width: u32,
    window_height: u32,
}

impl WatchInfo {
    pub fn new() -> Self {
        let watch_info_file = var(WATCH_INFO_FILE_ENVVAR).ok();
        if let Some(ref watch_info_file) = watch_info_file {
            let watch_info = read_to_string(watch_info_file).unwrap();
            let watch_info: Vec<&str> = watch_info.split(' ').collect();
            if watch_info.len() == 4 {
                let window_x = watch_info[0].parse::<i32>().unwrap();
                let window_y = watch_info[1].parse::<i32>().unwrap();
                let window_width = watch_info[2].parse::<u32>().unwrap();
                let window_height = watch_info[3].parse::<u32>().unwrap();
                pyxel_platform::set_window_pos(window_x, window_y);
                pyxel_platform::set_window_size(window_width, window_height);
            }
        }
        Self {
            watch_info_file,
            window_x: i32::MIN,
            window_y: i32::MIN,
            window_width: u32::MAX,
            window_height: u32::MAX,
        }
    }

    pub fn update(&mut self) {
        if self.watch_info_file.is_none() || pyxel_platform::is_fullscreen() {
            return;
        }
        let (window_x, window_y) = pyxel_platform::window_pos();
        let (window_width, window_height) = pyxel_platform::window_size();
        if window_x != self.window_x
            || window_y != self.window_y
            || window_width != self.window_width
            || window_height != self.window_height
        {
            self.window_x = window_x;
            self.window_y = window_y;
            self.window_width = window_width;
            self.window_height = window_height;
            let watch_info_file = self.watch_info_file.as_ref().unwrap();
            let watch_info = format!("{window_x} {window_y} {window_width} {window_height}");
            write(watch_info_file, watch_info).unwrap();
        }
    }
}
