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

  if (SDL_OpenAudio(&audio_spec, NULL) < 0) {
    PRINT_ERROR("failed to initialize SDL Audio");
    exit(1);
  }

  sound_bank_ = new Sound*[SOUND_BANK_COUNT];
  for (int32_t i = 0; i < SOUND_BANK_COUNT; i++) {
    sound_bank_[i] = new Sound();
  }

  music_bank_ = new Music*[MUSIC_BANK_COUNT];
  for (int32_t i = 0; i < MUSIC_BANK_COUNT; i++) {
    music_bank_[i] = new Music();
  }

  SDL_PauseAudio(0);
}

Audio::~Audio() {
  for (int32_t i = 0; i < SOUND_BANK_COUNT; i++) {
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
  if (channel < 0 || channel >= MUSIC_CHANNEL_COUNT) {
    PRINT_ERROR("invalid channel");
    return;
  }

  if (sound_index < 0 || sound_index >= SOUND_BANK_COUNT) {
    PRINT_ERROR("invalid sound index");
    return;
  }

  Sound* sound = sound_bank_[sound_index];
  channel_[channel].PlaySound(&sound, 1, loop);
}

void Audio::PlaySound(int32_t channel,
                      int32_t* sound_index,
                      int32_t sound_length,
                      bool loop) {
  if (channel < 0 || channel >= MUSIC_CHANNEL_COUNT) {
    PRINT_ERROR("invalid channel");
    return;
  }

  if (sound_length < 0 || sound_length >= MAX_MUSIC_LENGTH) {
    PRINT_ERROR("invalid sound length");
    return;
  }

  Sound* sound[sound_length];

  for (int32_t i = 0; i < sound_length; i++) {
    int32_t index = sound_index[i];

    if (index < 0 || index >= SOUND_BANK_COUNT) {
      PRINT_ERROR("invalid sound index");
      return;
    }

    sound[i] = sound_bank_[index];
  }

  channel_[channel].PlaySound(sound, sound_length, loop);
}

void Audio::PlayMusic(int32_t music_index, bool loop) {
  if (music_index < 0 || music_index >= MUSIC_CHANNEL_COUNT) {
    PRINT_ERROR("invalid music index");
    return;
  }

  Music* music = music_bank_[music_index];

  for (int32_t i = 0; i < MUSIC_CHANNEL_COUNT; i++) {
    if (music->SoundLength(i) > 0) {
      PlaySound(i, music->Sound(i), music->SoundLength(i), loop);
    }
  }
}

void Audio::StopPlaying(int32_t channel) {
  if (channel != -1 && (channel < 0 || channel >= MUSIC_CHANNEL_COUNT)) {
    PRINT_ERROR("invalide channel");
    return;
  }

  if (channel == -1) {
    for (int32_t i = 0; i < MUSIC_CHANNEL_COUNT; i++) {
      channel_[i].StopPlaying();
    }
  } else {
    channel_[channel].StopPlaying();
  }
}

int32_t Audio::GetPlayPos(int32_t channel) const {
  if (channel < 0 || channel >= MUSIC_CHANNEL_COUNT) {
    PRINT_ERROR("invalid channel");
    return 0;
  }

  return channel_[channel].PlayPos();
}

}  // namespace pyxelcore
