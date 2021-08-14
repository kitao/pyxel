use array_macro::array;
use blip_buf::BlipBuf;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::channel::Channel;
use crate::music::Music;
use crate::platform::{AudioCallback, Platform};
use crate::settings::{
    CHANNEL_COUNT, CLOCK_RATE, MUSIC_COUNT, SAMPLE_COUNT, SAMPLE_RATE, SOUND_COUNT,
    TICK_CLOCK_COUNT,
};
use crate::sound::Sound;
use crate::Pyxel;

pub struct AudioCore {
    blip_buf: BlipBuf,
    channels: [Arc<Mutex<Channel>>; CHANNEL_COUNT as usize],
}

pub struct Audio {
    audio_core: Arc<Mutex<AudioCore>>,
    channels: [Arc<Mutex<Channel>>; CHANNEL_COUNT as usize],
    sounds: [Rc<RefCell<Sound>>; SOUND_COUNT as usize],
    musics: [Rc<RefCell<Music>>; MUSIC_COUNT as usize],
}

impl Audio {
    pub fn new<T: Platform>(platform: &mut T) -> Audio {
        let mut blip_buf = BlipBuf::new(SAMPLE_COUNT);
        let channels = array![_ => Arc::new(Mutex::new(Channel::new())); CHANNEL_COUNT as usize];
        let sounds = array![_ => Rc::new(RefCell::new(Sound::new())); SOUND_COUNT as usize];
        let musics = array![_ => Rc::new(RefCell::new(Music::new())); MUSIC_COUNT as usize];

        blip_buf.set_rates(CLOCK_RATE as f64, SAMPLE_RATE as f64);

        let audio_core = Arc::new(Mutex::new(AudioCore {
            blip_buf: blip_buf,
            channels: channels.clone(),
        }));

        let audio = Audio {
            audio_core: audio_core.clone(),
            channels: channels,
            sounds: sounds,
            musics: musics,
        };

        platform.start_audio(SAMPLE_RATE, SAMPLE_COUNT, audio_core);

        audio
    }
}

impl AudioCallback for AudioCore {
    fn update(&mut self, out: &mut [i16]) {
        let mut samples = self.blip_buf.read_samples(out, false);

        while samples < out.len() {
            for channel in &mut self.channels {
                channel.lock().unwrap().update(&mut self.blip_buf);
            }

            self.blip_buf.end_frame(TICK_CLOCK_COUNT);

            samples += self.blip_buf.read_samples(&mut out[samples..], false);
        }
    }
}

impl Pyxel {
    pub fn channel(&self, channel_no: u32) -> Arc<Mutex<Channel>> {
        self.audio.channels[channel_no as usize].clone()
    }

    pub fn sound(&self, sound_no: u32) -> Rc<RefCell<Sound>> {
        self.audio.sounds[sound_no as usize].clone()
    }

    pub fn music(&self, music_no: u32) -> Rc<RefCell<Music>> {
        self.audio.musics[music_no as usize].clone()
    }

    pub fn play(&mut self, channel: u32, sequence: &[u32], is_looping: bool) {
        let sounds = sequence
            .iter()
            .map(|sound_no| self.audio.sounds[*sound_no as usize].borrow().clone())
            .collect();

        self.audio.audio_core.lock().unwrap().channels[channel as usize]
            .lock()
            .unwrap()
            .play(sounds, is_looping);
    }

    pub fn playm(&mut self, music_no: u32, looping: bool) {
        let music = self.audio.musics[music_no as usize].clone();

        for i in 0..CHANNEL_COUNT {
            self.play(i, &music.borrow().sequences[i as usize], looping);
        }
    }

    pub fn stop(&mut self, channel: u32) {
        self.audio.audio_core.lock().unwrap().channels[channel as usize]
            .lock()
            .unwrap()
            .stop();
    }

    pub fn stop_(&mut self) {
        for i in 0..CHANNEL_COUNT {
            self.stop(i);
        }
    }
}
