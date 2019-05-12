#include "pyxelcore/sound.h"

namespace pyxelcore {

std::map<char, int> NOTE_TABLE{{'c', 0}, {'d', 2}, {'e', 4}, {'f', 5},
                               {'g', 7}, {'a', 9}, {'b', 11}};

std::map<char, int> TONE_TABLE{{'t', TONE_TRIANGLE},
                               {'s', TONE_SQUARE},
                               {'p', TONE_PULSE},
                               {'n', TONE_NOISE}};

std::map<char, int> EFFECT_TABLE{{'n', EFFECT_NONE},
                                 {'s', EFFECT_SLIDE},
                                 {'v', EFFECT_VIBRATO},
                                 {'f', EFFECT_FADEOUT}};

Sound::Sound() {
  note_length_ = 0;
  tone_length_ = 0;
  volume_length_ = 0;
  effect_length_ = 0;
  speed_ = INITIAL_SOUND_SPEED;
}

void Sound::Set(const char* note,
                const char* tone,
                const char* volume,
                const char* effect,
                int32_t speed) {
  SetNote(note);
  SetTone(tone);
  SetVolume(volume);
  SetEffect(effect);
  Speed(speed);
}

void Sound::SetNote(const char* note) {
  std::string data = FormatData(note);

  note_length_ = 0;

  for (int32_t i = 0; i < data.length();) {
    char c = data[i++];
    int32_t param;

    if (c >= 'a' && c <= 'g') {
      param = NOTE_TABLE[c];
      c = i < data.length() ? data[i++] : 0;

      if (c == '#' || c == '-') {
        param += c == '#' ? 1 : -1;
        c = i < data.length() ? data[i++] : 0;
      }

      if (c >= '0' && c <= '4') {
        param += (c - '0') * 12;
      } else {
        char buf[256];
        snprintf(buf, sizeof(buf), "invalid sound note '%c'", c);
        PRINT_ERROR(buf);
        printf("index %d\n", i);

        return;
      }
    } else if (c == 'r') {
      param = -1;
    } else {
      char buf[256];
      snprintf(buf, sizeof(buf), "invalid sound note '%c'", c);
      PRINT_ERROR(buf);

      return;
    }

    if (note_length_ < MAX_SOUND_LENGTH) {
      note_[note_length_] = param;
      note_length_++;
    } else {
      PRINT_ERROR("too long sound note");
    }
  }
}

void Sound::SetTone(const char* tone) {
  std::string data = FormatData(tone);

  tone_length_ = 0;

  for (int32_t i = 0; i < data.length(); i++) {
    char c = data[i];
    int32_t param;

    if (c == 't' || c == 's' || c == 'p' || c == 'n') {
      param = TONE_TABLE[c];
    } else {
      char buf[256];
      snprintf(buf, sizeof(buf), "invalid sound tone '%c'", c);
      PRINT_ERROR(buf);

      return;
    }

    if (tone_length_ < MAX_SOUND_LENGTH) {
      tone_[tone_length_] = param;
      tone_length_++;
    } else {
      PRINT_ERROR("too long sound tone");
    }
  }
}

void Sound::SetVolume(const char* volume) {
  std::string data = FormatData(volume);

  volume_length_ = 0;

  for (int32_t i = 0; i < data.length(); i++) {
    char c = data[i];
    int32_t param;

    if (c >= '0' && c <= '7') {
      param = c - '0';
    } else {
      char buf[256];
      snprintf(buf, sizeof(buf), "invalid sound volume '%c'", c);
      PRINT_ERROR(buf);

      return;
    }

    if (volume_length_ < MAX_SOUND_LENGTH) {
      volume_[volume_length_] = param;
      volume_length_++;
    } else {
      PRINT_ERROR("too long sound volume");
    }
  }
}

void Sound::SetEffect(const char* effect) {
  std::string data = FormatData(effect);

  effect_length_ = 0;

  for (int32_t i = 0; i < data.length(); i++) {
    char c = data[i];
    int32_t param;

    if (c == 'n' || c == 's' || c == 'v' || c == 'f') {
      param = EFFECT_TABLE[c];
    } else {
      char buf[256];
      snprintf(buf, sizeof(buf), "invalid sound effect '%c'", c);
      PRINT_ERROR(buf);

      return;
    }

    if (effect_length_ < MAX_SOUND_LENGTH) {
      effect_[effect_length_] = param;
      effect_length_++;
    } else {
      PRINT_ERROR("too long sound effect");
    }
  }
}

void Sound::ReplaceAll(std::string& str,
                       const std::string& from,
                       const std::string& to) {
  std::string::size_type pos = str.find(from);

  while ((pos = str.find(from, pos)) != std::string::npos) {
    str.replace(pos, from.length(), to);
    pos += to.length();
  }
}

std::string Sound::FormatData(const char* str) {
  std::string s = std::string(str);

  ReplaceAll(s, " ", "");
  ReplaceAll(s, "\n", "");
  ReplaceAll(s, "\t", "");
  std::transform(s.begin(), s.end(), s.begin(), ::tolower);

  return s;
}

}  // namespace pyxelcore