use std::sync::{Arc, Mutex};

use blip_buf::BlipBuf;

use crate::oscillator::{Effect, Oscillator, Tone};
use crate::sound::Sound;

pub struct Channel {
    oscillator: Oscillator,
    sounds: Vec<Arc<Mutex<Sound>>>,
    /*
     bool is_playing_;
     bool is_loop_;
     SoundList sound_list_;
     int32_t play_index_;

     int32_t time_;
     int32_t one_note_time_;
     int32_t total_note_time_;

     int32_t tone_;
     int32_t note_;
     float pitch_;
     int32_t volume_;
     int32_t effect_;

     int32_t effect_time_;
     float effect_pitch_;
     int32_t effect_volume_;

     void PlaySound();
     void Update();
     void NextSound();
     float NoteToPitch(float note);
     float Lfo(int32_t time);
    */
}

impl Channel {
    pub fn new(sample_rate: u32) -> Channel {
        assert!(sample_rate > 0);

        let oscillator = Oscillator::new();

        Channel {
            oscillator: oscillator,
            sounds: Vec::new(),
        }

        /*
        Channel::Channel() {
          is_playing_ = false;
          is_loop_ = false;
          play_index_ = 0;

          time_ = 0;
          one_note_time_ = 0;

          tone_ = 0;
          note_ = 0;
          pitch_ = 0.0f;
          volume_ = 0;
          effect_ = 0;

          effect_time_ = 0;
          effect_pitch_ = 0.0f;
          effect_volume_ = 0;
        }
        */
    }

    #[inline]
    pub fn update(&mut self, blip_buf: &mut BlipBuf) {
        /*
        if (!is_playing_) {
        return;
        }

        if (total_note_time_ == 0) {
        NextSound();
        return;
        }

        // forward note
        if (time_ % one_note_time_ == 0) {
        Sound* sound = sound_list_[play_index_];
        int32_t pos = time_ / one_note_time_;
        note_ = sound->Note()[pos];
        volume_ = (sound->Volume().empty()
                        ? 7
                        : sound->Volume()[pos % sound->Volume().size()]) *
                    AUDIO_ONE_VOLUME;

        if (note_ >= 0 && volume_ > 0) {
            float last_pitch = pitch_;
            tone_ = sound->Tone().empty() ? TONE_TRIANGLE
                                        : sound->Tone()[pos % sound->Tone().size()];
            pitch_ = NoteToPitch(note_);
            effect_ = sound->Effect().empty()
                        ? EFFECT_NONE
                        : sound->Effect()[pos % sound->Effect().size()];

            oscillator_.SetTone(tone_);
            oscillator_.SetPeriod(AUDIO_SAMPLE_RATE / pitch_);
            oscillator_.SetVolume(volume_);

        } else {
            oscillator_.Stop();
        }
        }

        // play note
        if (note_ >= 0 && volume_ > 0) {
        float a;
        int32_t pitch;

        }

        time_++;

        if (time_ == total_note_time_) {
        NextSound();
        }

        */
        self.oscillator
            .play(440.0, 1, Tone::Triangle, 1.0, Effect::None);

        self.oscillator.update(blip_buf);
    }

    #[inline]
    pub fn play_pos() -> i32 {
        //return is_playing_ ? play_index_ * 100 + time_ / one_note_time_ : -1;
        0
    }

    #[inline]
    pub fn play_sound(sound: Vec<Arc<Sound>>, is_loop: bool) {
        /*
        if (sound_list.empty()) {
        return;
        }

        is_playing_ = true;
        is_loop_ = loop;
        sound_list_ = sound_list;
        play_index_ = 0;

        PlaySound();
        */
    }

    #[inline]
    fn play_sound_() {
        /*
        Sound* sound = sound_list_[play_index_];

        time_ = 0;
        one_note_time_ = sound->Speed() * AUDIO_ONE_SPEED;
        total_note_time_ = one_note_time_ * sound->Note().size();
        */
    }

    #[inline]
    pub fn stop_playing() {
        /*
        is_playing_ = false;
        pitch_ = 0.0f;
        oscillator_.Stop();
        */
    }

    /*
    void Channel::NextSound() {
        play_index_ += 1;

        if (play_index_ < sound_list_.size()) {
        PlaySound();
        } else if (is_loop_) {
        play_index_ = 0;
        PlaySound();
        } else {
        StopPlaying();
        }
    }
    */

    /*
    #[inline]
    fn note_to_period(&self, note: f64) -> u32 {
        let freq = 440.0 * 2.0_f64.powf((note - 33.0) as f64 / 12.0);
        (self.sample_rate as f64 / freq).round() as u32
    }
    */
}
