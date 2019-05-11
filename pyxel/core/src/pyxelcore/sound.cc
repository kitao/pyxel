#include "pyxelcore/sound.h"

namespace pyxelcore {

Sound::Sound() {
  note_length_ = 0;
  tone_length_ = 0;
  volume_length_ = 0;
  effect_length_ = 0;
  speed_ = INITIAL_SOUND_SPEED;
}

Sound::~Sound() {}

void Sound::Set(const char* note,
                const char* tone,
                const char* volume,
                const char* effect,
                int32_t speed) {
  SetNote(note);
  SetNote(tone);
  SetVolume(volume);
  SetEffect(effect);
  Speed(speed);
}

void Sound::SetNote(const char* note) {
  std::string data(note);
  data = FormatString(data);

  note_length_ = 0;

  int32_t index = 0;

  for (int32_t i = 0; i < data.length(); i++) {
    char c = data[i];
    //
  }

  /*
    def set_note(self, data):
        while data:
            c = data[0]
            data = data[1:]

            param = SOUND_NOTE_TABLE.get(c, None)

            if param is not None:
                c = data[0]
                data = data[1:]

                if c == "#" or c == "-":
                    param += c == "#" and 1 or -1

                    c = data[0]
                    data = data[1:]

                if "0" <= c <= "4":
                    param += int(c) * 12
                else:
                    raise ValueError("invalid sound note")
            elif c == "r":
                param = -1
            else:
                raise ValueError("invalid sound note")

            param_list.append(param)

        self._note[:] = param_list
  */
}

void Sound::SetTone(const char* tone) {
  //
}

void Sound::SetVolume(const char* volume) {
  //
}

void Sound::SetEffect(const char* effect) {
  //
}

std::string Sound::FormatString(const std::string& str) {
  std::string s = str;

  s = ReplaceString(s, " ", "");
  s = ReplaceString(s, "\n", "");
  s = ReplaceString(s, "\t", "");
  std::transform(s.begin(), s.end(), s.begin(), ::tolower);

  return s;
}

std::string Sound::ReplaceString(const std::string& str,
                                 const std::string& from,
                                 const std::string& to) {
  std::string s = str;
  std::string::size_type pos;

  for (pos = s.find(from, pos); pos != std::string::npos; pos += to.length()) {
    s.replace(pos, from.length(), to);
  }

  return s;
}

/*
class Sound:
    def set_note(self, data):
        param_list = []
        data = data.replace(" ", "").replace("\n", "").replace("\t", "").lower()

        while data:
            c = data[0]
            data = data[1:]

            param = SOUND_NOTE_TABLE.get(c, None)

            if param is not None:
                c = data[0]
                data = data[1:]

                if c == "#" or c == "-":
                    param += c == "#" and 1 or -1

                    c = data[0]
                    data = data[1:]

                if "0" <= c <= "4":
                    param += int(c) * 12
                else:
                    raise ValueError("invalid sound note")
            elif c == "r":
                param = -1
            else:
                raise ValueError("invalid sound note")

            param_list.append(param)

        self._note[:] = param_list

    def set_tone(self, data):
        param_list = []
        data = data.replace(" ", "").lower()

        while data:
            c = data[0]
            data = data[1:]

            param = SOUND_TONE_TABLE.get(c, None)

            if param is None:
                raise ValueError("invalid sound tone")

            param_list.append(param)

        self._tone[:] = param_list

    def set_volume(self, data):
        param_list = []
        data = data.replace(" ", "").lower()

        while data:
            c = data[0]
            data = data[1:]

            if "0" <= c <= "7":
                param = int(c)
            else:
                raise ValueError("invalid sound volume")

            param_list.append(param)

        self._volume[:] = param_list

    def set_effect(self, data):
        param_list = []
        data = data.replace(" ", "").lower()

        while data:
            c = data[0]
            data = data[1:]

            param = SOUND_EFFECT_TABLE.get(c, None)

            if param is None:
                raise ValueError("invalid sound effect")

            param_list.append(param)

        self._effect[:] = param_list
*/

}  // namespace pyxelcore