use crate::oscillator::{Effect, Tone};
use crate::settings::{DEFAULT_SOUND_SPEED, MAX_SOUND_VOLUME};

pub type Note = i32;
pub type Volume = u32;
pub type Speed = u32;

#[derive(Clone)]
pub struct Sound {
  notes: Vec<Note>,
  tones: Vec<Tone>,
  volumes: Vec<Volume>,
  effects: Vec<Effect>,
  speed: Speed,
}

impl Sound {
  pub fn new() -> Sound {
    Sound {
      notes: Vec::new(),
      tones: Vec::new(),
      volumes: Vec::new(),
      effects: Vec::new(),
      speed: DEFAULT_SOUND_SPEED,
    }
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.notes.len()
  }

  #[inline]
  pub fn note(&self, index: u32) -> Note {
    self.notes[index as usize]
  }

  /*pub fn notes_mut(&mut self) -> &mut Vec<Note> {
    &mut self.notes
  }*/

  #[inline]
  pub fn tone(&self, index: u32) -> Tone {
    let len = self.tones.len();

    if len > 0 {
      self.tones[index as usize % len]
    } else {
      Tone::Triangle
    }
  }

  /*#[inline]
  pub fn tones_mut(&mut self) -> &mut Vec<Tone> {
    &mut self.tones
  }*/

  #[inline]
  pub fn volume(&self, index: u32) -> Volume {
    let len = self.volumes.len();

    if len > 0 {
      self.volumes[index as usize % len]
    } else {
      MAX_SOUND_VOLUME
    }
  }

  /*#[inline]
  pub fn volumes_mut(&mut self) -> &mut Vec<Volume> {
    &mut self.volumes
  }*/

  #[inline]
  pub fn effect(&self, index: u32) -> Effect {
    let len = self.effects.len();

    if len > 0 {
      self.effects[index as usize % len]
    } else {
      Effect::None
    }
  }

  /*#[inline]
  pub fn effects_mut(&mut self) -> &mut Vec<Effect> {
    &mut self.effects
  }*/

  #[inline]
  pub fn speed(&self) -> Speed {
    self.speed
  }

  /*#[inline]
  pub fn set_speed(&mut self, speed: Speed) {
    self.speed = speed;
  }*/

  /*
  void Set(const std::string& note,
           const std::string& tone,
           const std::string& volume,
           const std::string& effect,
           int32_t speed);
  void SetNote(const std::string& note);
  void SetTone(const std::string& tone);
  void SetVolume(const std::string& volume);
  void SetEffect(const std::string& effect);
  static std::string FormatData(const std::string& str);
  */

  /*
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
  */
}
