use std::sync::{Arc, Mutex};

use blip_buf::BlipBuf;

use crate::channel::Channel;
use crate::music::Music;
use crate::platform::{AudioCallback, Platform};
use crate::settings::{
    CHANNEL_COUNT, CLOCK_RATE, MUSIC_COUNT, SAMPLE_COUNT, SAMPLE_RATE, SOUND_COUNT,
    TICK_CLOCK_COUNT,
};
use crate::sound::Sound;

pub struct Audio {
    blip_buf: BlipBuf,
    channels: Vec<Channel>,
    sounds: Vec<Sound>,
    musics: Vec<Music>,
}

impl AudioCallback for Audio {
    fn audio_callback(&mut self, out: &mut [i16]) {
        let mut samples = self.blip_buf.read_samples(out, false);

        while samples < out.len() {
            for channel in &mut self.channels {
                channel.update(&mut self.blip_buf);
            }

            self.blip_buf.end_frame(TICK_CLOCK_COUNT);

            samples += self.blip_buf.read_samples(&mut out[samples..], false);
        }
    }
}

impl Audio {
    pub fn new<T: Platform>(platform: &mut T) -> Arc<Mutex<Audio>> {
        let mut blip_buf = BlipBuf::new(SAMPLE_COUNT);
        let channels = (0..CHANNEL_COUNT).map(|_| Channel::new()).collect();
        let sounds = (0..SOUND_COUNT).map(|_| Sound::new()).collect();
        let musics = (0..MUSIC_COUNT).map(|_| Music::new()).collect();

        blip_buf.set_rates(CLOCK_RATE as f64, SAMPLE_RATE as f64);

        let audio = Arc::new(Mutex::new(Audio {
            blip_buf: blip_buf,
            channels: channels,
            sounds: sounds,
            musics: musics,
        }));

        platform.start_audio(SAMPLE_RATE, SAMPLE_COUNT, audio.clone());

        audio
    }

    pub fn sound(&self, sound_no: u32) -> &Sound {
        &self.sounds[sound_no as usize]
    }

    pub fn music(&self, music_no: u32) -> &Music {
        &self.musics[music_no as usize]
    }

    pub fn play_sound(&mut self, channel_no: u32, sound_nos: &[u32], is_looping: bool) {
        let mut sounds: Vec<Sound> = Vec::new();

        for sound_no in sound_nos {
            sounds.push(self.sounds[*sound_no as usize].clone());
        }

        self.channels[channel_no as usize].play(sounds, is_looping);
    }

    pub fn play_music(&mut self, music_no: u32, is_looping: bool) {
        for i in 0..MUSIC_COUNT {
            let sequence = &self.musics[music_no as usize].sequence(i).clone();
            self.play_sound(i, sequence, is_looping);
        }
    }

    pub fn stop(&mut self, channel_no: u32) {
        self.channels[channel_no as usize].stop();
    }

    /*pub fn play_pos(&self) -> Option<(u32, u32)> {
        //
    }*/
}
