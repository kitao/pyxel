use std::sync::{Arc, Mutex};

use blip_buf::BlipBuf;

use crate::oscillator::Oscillator;
use crate::sound::Sound;

pub struct Channel {
    sample_rate: u32,
    start_offset: u32,
    sound_period: u32,
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
            sample_rate: sample_rate,
            start_offset: 0,
            sound_period: 0,
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
    pub fn update(&mut self, blip_buf: &mut BlipBuf, end_time: u32) {
        if self.start_offset >= end_time {
            self.oscillator.update(blip_buf, 0, end_time);
            self.start_offset -= end_time;
            return;
        }

        let mut cur_time = 0;

        if self.start_offset > 0 {
            self.oscillator.update(blip_buf, 0, self.start_offset);
            cur_time = self.start_offset;
        }

        loop {
            // play

            let next_time = cur_time + self.sound_period;

            if next_time >= end_time {
                self.oscillator.update(blip_buf, cur_time, end_time);
                self.start_offset = next_time - end_time;
                return;
            }

            self.oscillator.update(blip_buf, cur_time, next_time);

            cur_time = next_time;
        }

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

        /*for i in 0..out.len() as usize {
            let time = ((CLOCK_RATE / SAMPLE_RATE as f64) * i as f64) as u32;

            for channel in &mut self.channels {
                let last_output = channel.output();
                channel.update();
                blip_buf.add_delta(time, channel.output() as i32 - last_output as i32)
            }
        }*/

        /*
        int period = (int) (clock_rate / w->frequency / 2 + 0.5);
        int volume = (int) (w->volume * 65536 / 2 + 0.5);
        for ( ; w->time < clocks; w->time += period )
        {
            int delta = w->phase * volume - w->amp;
            w->amp += delta;
            blip_add_delta( blip, w->time, delta );
            w->phase = -w->phase;
        }
        w->time -= clocks;
        */
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
