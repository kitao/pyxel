#include "pyxelcore/audio.h"

#include "pyxelcore/music.h"
#include "pyxelcore/sound.h"

namespace pyxelcore {

Audio::Audio() {
  SDL_AudioSpec audio_spec;
  audio_spec.freq = AUDIO_SAMPLE_RATE;
  audio_spec.format = AUDIO_S16LSB;
  audio_spec.channels = 1;
  audio_spec.samples = AUDIO_BLOCK_SIZE;
  audio_spec.callback = callback;
  audio_spec.userdata = this;

  audio_device_id_ = SDL_OpenAudioDevice(NULL, 0, &audio_spec, NULL, 0);
  if (audio_device_id_ == 0) {
    PYXEL_ERROR("failed to initialize SDL Audio");
  }

  sound_bank_ = new Sound*[TOTAL_SOUND_BANK_COUNT];
  for (int32_t i = 0; i < TOTAL_SOUND_BANK_COUNT; i++) {
    sound_bank_[i] = new Sound();
  }

  music_bank_ = new Music*[MUSIC_BANK_COUNT];
  for (int32_t i = 0; i < MUSIC_BANK_COUNT; i++) {
    music_bank_[i] = new Music();
  }

  SDL_PauseAudioDevice(audio_device_id_, 0);
}

Audio::~Audio() {
  SDL_CloseAudioDevice(audio_device_id_);

  for (int32_t i = 0; i < TOTAL_SOUND_BANK_COUNT; i++) {
    delete sound_bank_[i];
  }
  delete[] sound_bank_;

  for (int32_t i = 0; i < MUSIC_BANK_COUNT; i++) {
    delete music_bank_[i];
  }
  delete[] music_bank_;
}

void Audio::callback(void* userdata, uint8_t* stream, int len) {
  Audio* audio = reinterpret_cast<Audio*>(userdata);
  int16_t* frame_data = reinterpret_cast<int16_t*>(stream);
  int32_t frame_count = len / sizeof(uint16_t);

  for (int32_t i = 0; i < frame_count; i++) {
    int16_t output = 0;

    for (int32_t i = 0; i < MUSIC_CHANNEL_COUNT; i++) {
      output += audio->channel_[i].Output();
    }

    frame_data[i] = output;
  }
}

void Audio::PlaySound(int32_t channel, int32_t sound_index, bool loop) {
  SDL_LockAudioDevice(audio_device_id_);

  if (channel < 0 || channel >= MUSIC_CHANNEL_COUNT) {
    PYXEL_ERROR("invalid channel");
  }

  if (sound_index < 0 || sound_index >= TOTAL_SOUND_BANK_COUNT) {
    PYXEL_ERROR("invalid sound index");
  }

  const SoundList sound_list = {sound_bank_[sound_index]};
  channel_[channel].PlaySound(sound_list, loop);

  SDL_UnlockAudioDevice(audio_device_id_);
}

void Audio::PlaySound(int32_t channel,
                      const SoundIndexList& sound_index_list,
                      bool loop) {
  SDL_LockAudioDevice(audio_device_id_);

  if (channel < 0 || channel >= MUSIC_CHANNEL_COUNT) {
    PYXEL_ERROR("invalid channel");
  }

  SoundList sound_list;

  for (int32_t sound_index : sound_index_list) {
    if (sound_index < 0 || sound_index >= TOTAL_SOUND_BANK_COUNT) {
      PYXEL_ERROR("invalid sound index");
    }

    sound_list.push_back(sound_bank_[sound_index]);
  }

  channel_[channel].PlaySound(sound_list, loop);

  SDL_UnlockAudioDevice(audio_device_id_);
}

void Audio::PlayMusic(int32_t music_index, bool loop) {
  SDL_LockAudioDevice(audio_device_id_);

  if (music_index < 0 || music_index >= MUSIC_BANK_COUNT) {
    PYXEL_ERROR("invalid music index");
  }

  Music* music = music_bank_[music_index];

  PlaySound(0, music->Channel0(), loop);
  PlaySound(1, music->Channel1(), loop);
  PlaySound(2, music->Channel2(), loop);
  PlaySound(3, music->Channel3(), loop);

  SDL_UnlockAudioDevice(audio_device_id_);
}

void Audio::StopPlaying(int32_t channel) {
  SDL_LockAudioDevice(audio_device_id_);

  if (channel != -1 && (channel < 0 || channel >= MUSIC_CHANNEL_COUNT)) {
    PYXEL_ERROR("invalid channel");
  }

  if (channel == -1) {
    for (int32_t i = 0; i < MUSIC_CHANNEL_COUNT; i++) {
      channel_[i].StopPlaying();
    }
  } else {
    channel_[channel].StopPlaying();
  }

  SDL_UnlockAudioDevice(audio_device_id_);
}

}  // namespace pyxelcore
