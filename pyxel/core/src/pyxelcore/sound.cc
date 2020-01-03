#include "pyxelcore/sound.h"

namespace pyxelcore {

std::map<char, int> NOTE_TABLE = {
    {'c', 0}, {'d', 2}, {'e', 4}, {'f', 5}, {'g', 7}, {'a', 9}, {'b', 11},
};

std::map<char, int> TONE_TABLE = {
    {'t', TONE_TRIANGLE},
    {'s', TONE_SQUARE},
    {'p', TONE_PULSE},
    {'n', TONE_NOISE},
};

std::map<char, int> EFFECT_TABLE = {
    {'n', EFFECT_NONE},
    {'s', EFFECT_SLIDE},
    {'v', EFFECT_VIBRATO},
    {'f', EFFECT_FADEOUT},
};

Sound::Sound() {
  speed_ = INITIAL_SOUND_SPEED;
}

void Sound::Set(const std::string& note,
                const std::string& tone,
                const std::string& volume,
                const std::string& effect,
                int32_t speed) {
  SetNote(note);
  SetTone(tone);
  SetVolume(volume);
  SetEffect(effect);
  Speed(speed);
}

void Sound::SetNote(const std::string& note) {
  std::string data = FormatData(note);

  note_.resize(0);

  for (int32_t i = 0; i < data.size();) {
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
        std::string s = {c};
        PYXEL_ERROR("invalid sound note '" + s + "'");
      }
    } else if (c == 'r') {
      param = -1;
    } else {
      std::string s = {c};
      PYXEL_ERROR("invalid sound note '" + s + "'");
    }

    note_.push_back(param);
  }
}

void Sound::SetTone(const std::string& tone) {
  std::string data = FormatData(tone);

  tone_.resize(0);

  for (int32_t i = 0; i < data.length(); i++) {
    char c = data[i];
    int32_t param;

    if (c == 't' || c == 's' || c == 'p' || c == 'n') {
      param = TONE_TABLE[c];
    } else {
      std::string s = {c};
      PYXEL_ERROR("invalid sound tone '" + s + "'");
    }

    tone_.push_back(param);
  }
}

void Sound::SetVolume(const std::string& volume) {
  std::string data = FormatData(volume);

  volume_.resize(0);

  for (int32_t i = 0; i < data.length(); i++) {
    char c = data[i];
    int32_t param;

    if (c >= '0' && c <= '7') {
      param = c - '0';
    } else {
      std::string s = {c};
      PYXEL_ERROR("invalid sound volume '" + s + "'");
    }

    volume_.push_back(param);
  }
}

void Sound::SetEffect(const std::string& effect) {
  std::string data = FormatData(effect);

  effect_.resize(0);

  for (int32_t i = 0; i < data.length(); i++) {
    char c = data[i];
    int32_t param;

    if (c == 'n' || c == 's' || c == 'v' || c == 'f') {
      param = EFFECT_TABLE[c];
    } else {
      std::string s = {c};
      PYXEL_ERROR("invalid sound effect '" + s + "'");
    }

    effect_.push_back(param);
  }
}

std::string Sound::FormatData(const std::string& str) {
  std::string res = std::string(str);

  for (char c : WHITESPACE) {
    res = ReplaceAll(res, " ", "");
  }

  std::transform(res.begin(), res.end(), res.begin(), ::tolower);

  return res;
}

}  // namespace pyxelcore