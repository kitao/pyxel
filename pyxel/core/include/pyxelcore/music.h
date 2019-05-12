#ifndef PYXELCORE_MUSIC_H_
#define PYXELCORE_MUSIC_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Music {
 public:
  Music();
  ~Music();

  int32_t* Ch0() { return ch0_; }
  int32_t Ch0Length() const { return ch0_length_; }
  void Ch0Length(int32_t length);

  int32_t* Ch1() { return ch1_; }
  int32_t Ch1Length() const { return ch1_length_; }
  void Ch1Length(int32_t length);

  int32_t* Ch2() { return ch2_; }
  int32_t Ch2Length() const { return ch2_length_; }
  void Ch2Length(int32_t length);

  int32_t* Ch3() { return ch3_; }
  int32_t Ch3Length() const { return ch3_length_; }
  void Ch3Length(int32_t length);

  void Set(const int32_t* ch0,
           int32_t ch0_length,
           const int32_t* ch1,
           int32_t ch1_length,
           const int32_t* ch2,
           int32_t ch2_length,
           const int32_t* ch3,
           int32_t ch3_length);

 private:
  int32_t ch0_[MAX_MUSIC_LENGTH];
  int32_t ch0_length_;
  int32_t ch1_[MAX_MUSIC_LENGTH];
  int32_t ch1_length_;
  int32_t ch2_[MAX_MUSIC_LENGTH];
  int32_t ch2_length_;
  int32_t ch3_[MAX_MUSIC_LENGTH];
  int32_t ch3_length_;
};

inline void Music::Ch0Length(int32_t length) {
  if (length < 0 || length >= MAX_MUSIC_LENGTH) {
    PRINT_ERROR("invalid channel length");
    return;
  }

  ch0_length_ = length;
}

inline void Music::Ch1Length(int32_t length) {
  if (length < 0 || length >= MAX_MUSIC_LENGTH) {
    PRINT_ERROR("invalid channel length");
    return;
  }

  ch1_length_ = length;
}

inline void Music::Ch2Length(int32_t length) {
  if (length < 0 || length >= MAX_MUSIC_LENGTH) {
    PRINT_ERROR("invalid channel length");
    return;
  }

  ch2_length_ = length;
}

inline void Music::Ch3Length(int32_t length) {
  if (length < 0 || length >= MAX_MUSIC_LENGTH) {
    PRINT_ERROR("invalid channel length");
    return;
  }

  ch3_length_ = length;
}

}  // namespace pyxelcore

#endif  // PYXELCORE_MUSIC_H_
