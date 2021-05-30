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
    musics: Vec<Music>,
    /*
    SDL_AudioDeviceID audio_device_id_;
    Sound** sound_bank_;
    Music** music_bank_;
    Channel channel_[MUSIC_CHANNEL_COUNT];
    */
}

impl AudioCallback for Audio {
    fn audio_callback(&mut self, out: &mut [f32]) {
        let blip_buf = &self.blip_buf;

        for x in out.iter_mut() {
            *x = 0.0;
        }
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
            channels.push(Channel::new());
        }

        for _ in 0..SOUND_COUNT {
            sounds.push(Arc::new(Mutex::new(Sound::new())));
        }

        for _ in 0..MUSIC_COUNT {
            musics.push(Music::new());
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
    pub fn sound(&self, no: usize) -> Arc<Mutex<Sound>> {
        self.sounds[no].clone()
    }

    #[inline]
    pub fn music(&self, no: usize) -> &Music {
        &self.musics[no]
    }

    /*
    Sound* GetSoundBank(int32_t sound_index, bool system = false) const;
    Music* GetMusicBank(int32_t music_index) const;
    int32_t GetPlayPos(int32_t channel) const;
    void PlaySound(int32_t channel, int32_t sound_index, bool loop = false);
    void PlaySound(int32_t channel,
    const SoundIndexList& sound_index_list,
    bool loop = false);
    void PlayMusic(int32_t music_index, bool loop = false);
    void StopPlaying(int32_t channel = -1);
    */
}
