#include "pyxelcore/audio.h"

#include "pyxelcore/music.h"
#include "pyxelcore/sound.h"

namespace pyxelcore {

Audio::Audio() {
  SDL_AudioSpec audio_spec;
  audio_spec.freq = AUDIO_SAMPLE_RATE;
  audio_spec.format = AUDIO_S16LSB;
  audio_spec.channels = 0;
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
  uint16_t* frame_data = reinterpret_cast<uint16_t*>(stream);
  int32_t frame_count = len / sizeof(uint16_t);

  for (int32_t i = 0; i < frame_count; i++) {
    frame_data[i] = 0;
  }
}

void Audio::PlaySound(int32_t channel, int32_t sound_index, bool loop) {
  //
}

void Audio::PlaySound(int32_t channel,
                      int32_t* sound_index,
                      int32_t sound_index_count,
                      bool loop) {
  //
}

void Audio::PlayMusic(int32_t music_index, bool loop) {
  //
}

void Audio::StopPlaying(int32_t channel) {
  //
}

/*
  class AudioPlayer:
      def __init__(self):
          try:
              self._output_stream = sd.OutputStream(
                  samplerate=AUDIO_SAMPLE_RATE,
                  blocksize=AUDIO_BLOCK_SIZE,
                  channels=1,
                  dtype="int16",
                  callback=self._output_stream_callback,
              )
          except sd.PortAudioError:
              self._output_stream = None

          self._channel_list = [Channel() for _ in range(AUDIO_CHANNEL_COUNT)]
          self._sound_list = [Sound() for _ in range(AUDIO_SOUND_COUNT)]
          self._music_list = [Music() for _ in range(AUDIO_MUSIC_COUNT)]

      @property
      def output_stream(self):
          return self._output_stream

      def sound(self, snd, *, system=False):
          if not system and snd == AUDIO_SOUND_COUNT - 1:
              raise ValueError("sound bank {} is reserved for
  system".format(snd))

          return self._sound_list[snd]

      def music(self, msc):
          return self._music_list[msc]

      def play(self, ch, snd, *, loop=False):
          if isinstance(snd, list):
              sound_list = [self._sound_list[s] for s in snd]
          else:
              sound_list = [self._sound_list[snd]]

          self._channel_list[ch].play(sound_list, loop)

      def playm(self, msc, *, loop=False):
          music = self._music_list[msc]

          if music.ch0:
              self.play(0, music.ch0, loop=loop)

          if music.ch1:
              self.play(1, music.ch1, loop=loop)

          if music.ch2:
              self.play(2, music.ch2, loop=loop)

          if music.ch3:
              self.play(3, music.ch3, loop=loop)

      def stop(self, ch=None):
          if ch is None:
              for i in range(AUDIO_CHANNEL_COUNT):
                  self._channel_list[i].stop()
          else:
              self._channel_list[ch].stop()

      def _output_stream_callback(self, outdata, frames, time, status):
          for i in range(frames):
              output = 0
              for channel in self._channel_list:
                  output += channel.output()
              outdata[i] = output
*/

}  // namespace pyxelcore
