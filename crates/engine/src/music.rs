use crate::settings::CHANNEL_COUNT;

pub struct Music {
    channels: Vec<Vec<u32>>,
}

impl Music {
    pub fn new() -> Music {
        let mut channels = Vec::new();

        for _ in 0..CHANNEL_COUNT {
            channels.push(Vec::new());
        }

        Music { channels: channels }
    }
}

impl Music {
    pub fn channel_mut(&mut self, index: u32) -> &mut Vec<u32> {
        &mut self.channels[index as usize]
    }

    pub fn set(&mut self, index: u32, sounds: &Vec<u32>) {
        self.channels[index as usize] = sounds.clone();
    }
}
