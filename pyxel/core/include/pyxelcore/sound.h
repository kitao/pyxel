#ifndef PYXELCORE_SOUND_H_
#define PYXELCORE_SOUND_H_

#include "pyxelcore/common.h"

#include <string>

namespace pyxelcore {

class Sound {
 public:
  Sound();
  ~Sound();

  int32_t* Note() { return note_; }
  int32_t NoteLength() const { return note_length_; }
  void NoteLength(int32_t length);

  int32_t* Tone() { return tone_; }
  int32_t ToneLength() const { return tone_length_; }
  void ToneLength(int32_t length);

  int32_t* Volume() { return tone_; }
  int32_t VolumeLength() const { return volume_length_; }
  void VolumeLength(int32_t length);

  int32_t* Effect() { return effect_; }
  int32_t EffectLength() const { return effect_length_; }
  void EffectLength(int32_t length);

  int32_t Speed() const { return speed_; }
  void Speed(int32_t speed);

  void Set(const char* note,
           const char* tone,
           const char* volume,
           const char* effect,
           int32_t speed);
  void SetNote(const char* note);
  void SetTone(const char* tone);
  void SetVolume(const char* volume);
  void SetEffect(const char* effect);

 private:
  int32_t note_[MAX_SOUND_LENGTH];
  int32_t note_length_;
  int32_t tone_[MAX_SOUND_LENGTH];
  int32_t tone_length_;
  int32_t volume_[MAX_SOUND_LENGTH];
  int32_t volume_length_;
  int32_t effect_[MAX_SOUND_LENGTH];
  int32_t effect_length_;
  int32_t speed_;

  static std::string FormatString(const std::string& str);
  static std::string ReplaceString(const std::string& str,
                                   const std::string& from,
                                   const std::string& to);
};

inline void Sound::NoteLength(int32_t length) {
  if (length < 0 || length >= MAX_SOUND_LENGTH) {
    PRINT_ERROR("invalid note length");
    return;
  }

  note_length_ = length;
}

inline void Sound::ToneLength(int32_t length) {
  if (length < 0 || length >= MAX_SOUND_LENGTH) {
    PRINT_ERROR("invalid tone length");
    return;
  }

  tone_length_ = length;
}

inline void Sound::VolumeLength(int32_t length) {
  if (length < 0 || length >= MAX_SOUND_LENGTH) {
    PRINT_ERROR("invalid volume length");
    return;
  }

  volume_length_ = length;
}

inline void Sound::EffectLength(int32_t length) {
  if (length < 0 || length >= MAX_SOUND_LENGTH) {
    PRINT_ERROR("invalid effect length");
    return;
  }

  effect_length_ = length;
}

inline void Sound::Speed(int32_t speed) {
  if (speed < 1) {
    PRINT_ERROR("invalid speed");
    return;
  }

  speed_ = speed;
}

}  // namespace pyxelcore

#endif  // PYXELCORE_SOUND_H_
