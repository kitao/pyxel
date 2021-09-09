#ifndef PYXELCORE_SOUND_H_
#define PYXELCORE_SOUND_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Sound {
 public:
  Sound();

  SoundData& Note() { return note_; }
  SoundData& Tone() { return tone_; }
  SoundData& Volume() { return volume_; }
  SoundData& Effect() { return effect_; }
  int32_t Speed() const { return speed_; }
  void Speed(int32_t speed);

  void Set(const std::string& note,
           const std::string& tone,
           const std::string& volume,
           const std::string& effect,
           int32_t speed);
  void SetNote(const std::string& note);
  void SetTone(const std::string& tone);
  void SetVolume(const std::string& volume);
  void SetEffect(const std::string& effect);

 private:
  SoundData note_;
  SoundData tone_;
  SoundData volume_;
  SoundData effect_;
  int32_t speed_;

  static std::string FormatData(const std::string& str);
};

inline void Sound::Speed(int32_t speed) {
  if (speed < 1) {
    PYXEL_ERROR("invalid speed");
  }

  speed_ = speed;
}

}  // namespace pyxelcore

#endif  // PYXELCORE_SOUND_H_
