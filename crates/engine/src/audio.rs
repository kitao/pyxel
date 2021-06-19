use std::sync::{Arc, Mutex};

use blip_buf::BlipBuf;

use crate::channel::Channel;
use crate::music::Music;
use crate::platform::{AudioCallback, Platform};
use crate::settings::{
    CHANNEL_COUNT, CLOCK_RATE, MUSIC_COUNT, SAMPLE_COUNT, SAMPLE_RATE, SOUND_COUNT,
};
use crate::sound::Sound;

pub struct Audio {
    blip_buf: BlipBuf,
    channels: Vec<Channel>,
    sounds: Vec<Arc<Mutex<Sound>>>,
    musics: Vec<Arc<Mutex<Music>>>,
}

impl AudioCallback for Audio {
    fn audio_callback(&mut self, out: &mut [i16]) {
        let blip_buf = &mut self.blip_buf;
        let clocks = blip_buf.clocks_needed(out.len() as u32);

        for channel in &mut self.channels {
            channel.update(blip_buf, clocks);
        }

        blip_buf.end_frame(clocks);
        blip_buf.read_samples(out, false);
    }
}

impl Audio {
    pub fn new<T: Platform>(platform: &mut T) -> Arc<Mutex<Audio>> {
        let mut blip_buf = BlipBuf::new(SAMPLE_COUNT);
        let mut channels = Vec::new();
        let mut sounds = Vec::new();
        let mut musics = Vec::new();

        blip_buf.set_rates(CLOCK_RATE, SAMPLE_RATE as f64);

        for _ in 0..CHANNEL_COUNT {
            channels.push(Channel::new(SAMPLE_RATE));
        }

        for _ in 0..SOUND_COUNT {
            sounds.push(Arc::new(Mutex::new(Sound::new())));
        }

        for _ in 0..MUSIC_COUNT {
            musics.push(Arc::new(Mutex::new(Music::new())));
        }

        let audio = Arc::new(Mutex::new(Audio {
            blip_buf: blip_buf,
            channels: channels,
            sounds: sounds,
            musics: musics,
        }));

        platform.init_audio(SAMPLE_RATE, SAMPLE_COUNT, audio.clone());

        audio
    }

    #[inline]
    pub fn sound(&self, no: u32) -> Arc<Mutex<Sound>> {
        self.sounds[no as usize].clone()
    }

    #[inline]
    pub fn music(&self, no: u32) -> Arc<Mutex<Music>> {
        self.musics[no as usize].clone()
    }

    /*
    int32_t GetPlayPos(int32_t channel) const;
    void PlaySound(int32_t channel, int32_t sound_index, bool loop = false);
    void PlaySound(int32_t channel,
    const SoundIndexList& sound_index_list,
    bool loop = false);
    void PlayMusic(int32_t music_index, bool loop = false);
    void StopPlaying(int32_t channel = -1);
    */
}
