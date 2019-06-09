#include "pyxelcore/resource.h"

#include "pyxelcore/audio.h"
#include "pyxelcore/graphics.h"
#include "pyxelcore/image.h"
#include "pyxelcore/music.h"
#include "pyxelcore/sound.h"
#include "pyxelcore/tilemap.h"

#include "miniz-cpp/zip_file.hpp"

#define PARSE_CHANNEL(ss, music, channel)                          \
  do {                                                             \
    std::string line;                                              \
                                                                   \
    std::getline(ss, line);                                        \
    line = Trim(line);                                             \
                                                                   \
    SoundIndexList& data = music->channel();                       \
    data.clear();                                                  \
                                                                   \
    if (line != "none") {                                          \
      for (int32_t i = 0; i < line.size() / 2; i++) {              \
        int32_t v = std::stoi(line.substr(i * 2, 2), nullptr, 16); \
                                                                   \
        data.push_back(v);                                         \
      }                                                            \
    }                                                              \
  } while (false)

#define PARSE_SOUND(ss, sound, property) \
  do {                                   \
    std::string line;                    \
    std::getline(ss, line);              \
    line = Trim(line);                   \
                                         \
    SoundData& data = sound->property(); \
    data.clear();                        \
                                         \
    if (line != "none") {                \
      for (char c : line) {              \
        data.push_back(c - '0');         \
      }                                  \
    }                                    \
  } while (false)

namespace pyxelcore {

class ParseError {};

Resource::Resource(Graphics* graphics, Audio* audio) {
  graphics_ = graphics;
  audio_ = audio;
}

bool Resource::SaveAsset(const std::string& filename) {
  std::stringstream ss;

  ss << "__pyxel__" << std::endl;
  ss << VERSION << std::endl;

  for (int32_t i = 0; i < IMAGE_BANK_FOR_SYSTEM; i++) {
    DumpImage(ss, i);
  }

  for (int32_t i = 0; i < TILEMAP_BANK_COUNT; i++) {
    DumpTilemap(ss, i);
  }

  for (int32_t i = 0; i < SOUND_BANK_FOR_SYSTEM; i++) {
    DumpSound(ss, i);
  }

  for (int32_t i = 0; i < MUSIC_BANK_COUNT; i++) {
    DumpMusic(ss, i);
  }

  try {
    miniz_cpp::zip_file file;
    file.writestr(RESOURCE_ARCHIVE_NAME, ss.str());
    file.save(filename);
  } catch (...) {
    PRINT_ERROR("cannot save file '" + filename + "'");
    return false;
  }

  return true;
}

bool Resource::LoadAsset(const std::string& filename) {
  try {
    miniz_cpp::zip_file file(filename);
    std::stringstream ss(file.read(RESOURCE_ARCHIVE_NAME));

    std::string line;
    std::getline(ss, line);
    line = Trim(line);

    if (line == "__pyxel__") {
      ParseVersion(ss);
    } else {
      throw ParseError();
    }

    while (!ss.eof()) {
      std::getline(ss, line);
      line = Trim(line);

      if (line.find("__") == 0) {
        if (line.find("__image_") == 0) {
          int32_t image_index = std::atoi(line.substr(8, 1).c_str());
          ParseImage(ss, image_index);
        } else if (line.find("__tilemap_") == 0) {
          int32_t tilemap_index = std::atoi(line.substr(10, 1).c_str());
          ParseTilemap(ss, tilemap_index);
        } else if (line.find("__sound_") == 0) {
          int32_t sound_index = std::atoi(line.substr(8, 2).c_str());
          ParseSound(ss, sound_index);
        } else if (line.find("__music_") == 0) {
          int32_t music_index = std::atoi(line.substr(8, 1).c_str());
          ParseMusic(ss, music_index);
        }
      }
    }
  } catch (...) {
    PRINT_ERROR("invalid resource file");
    return false;
  }

  return true;
}

void Resource::DumpImage(std::stringstream& ss, int32_t image_index) {
  Image* image = graphics_->GetImageBank(image_index);
  int32_t** data = image->Data();

  bool is_editted = false;

  for (int32_t i = 0; i < image->Height(); i++) {
    for (int32_t j = 0; j < image->Width(); j++) {
      if (data[i][j] != 0) {
        is_editted = true;
        break;
      }
    }

    if (is_editted) {
      break;
    }
  }

  if (!is_editted) {
    return;
  }

  ss << std::endl;
  ss << "__image_" << image_index << "__" << std::endl;

  ss << std::hex;

  for (int32_t i = 0; i < image->Height(); i++) {
    for (int32_t j = 0; j < image->Width(); j++) {
      ss << data[i][j];
    }

    ss << std::endl;
  }

  ss << std::dec;
}

void Resource::DumpTilemap(std::stringstream& ss, int32_t tilemap_index) {
  Tilemap* tilemap = graphics_->GetTilemapBank(tilemap_index);
  int32_t** data = tilemap->Data();

  bool is_editted = false;

  for (int32_t i = 0; i < tilemap->Height(); i++) {
    for (int32_t j = 0; j < tilemap->Width(); j++) {
      if (data[i][j] != 0) {
        is_editted = true;
        break;
      }
    }

    if (is_editted) {
      break;
    }
  }

  if (!is_editted) {
    return;
  }

  ss << std::endl;
  ss << "__tilemap_" << tilemap_index << "__" << std::endl;

  ss << std::hex;

  for (int32_t i = 0; i < tilemap->Height(); i++) {
    for (int32_t j = 0; j < tilemap->Width(); j++) {
      ss << std::setw(3) << std::setfill('0') << data[i][j];
    }

    ss << std::endl;
  }

  ss << std::dec;
}

void Resource::DumpSound(std::stringstream& ss, int32_t sound_index) {
  Sound* sound = audio_->GetSoundBank(sound_index);

  if (sound->Note().size() == 0 && sound->Tone().size() == 0 &&
      sound->Volume().size() == 0 && sound->Effect().size() == 0) {
    return;
  }

  ss << std::endl;
  ss << "__sound_" << std::setw(2) << std::setfill('0') << sound_index << "__"
     << std::endl;

  ss << std::hex;

  if (sound->Note().size() > 0) {
    for (int32_t v : sound->Note()) {
      if (v < 0) {
        v = 0xff;
      }

      ss << std::setw(2) << std::setfill('0') << v;
    }
    ss << std::endl;
  } else {
    ss << "none" << std::endl;
  }

  if (sound->Tone().size() > 0) {
    for (int32_t v : sound->Tone()) {
      ss << v;
    }
    ss << std::endl;
  } else {
    ss << "none" << std::endl;
  }

  if (sound->Volume().size() > 0) {
    for (int32_t v : sound->Volume()) {
      ss << v;
    }
    ss << std::endl;
  } else {
    ss << "none" << std::endl;
  }

  if (sound->Effect().size() > 0) {
    for (int32_t v : sound->Effect()) {
      ss << v;
    }
    ss << std::endl;
  } else {
    ss << "none" << std::endl;
  }

  ss << std::dec << sound->Speed() << std::endl;
}

void Resource::DumpMusic(std::stringstream& ss, int32_t music_index) {
  Music* music = audio_->GetMusicBank(music_index);

  if (music->Channel0().size() == 0 && music->Channel1().size() == 0 &&
      music->Channel2().size() == 0 and music->Channel3().size() == 0) {
    return;
  }

  ss << std::endl;
  ss << "__music_" << music_index << "__" << std::endl;

  ss << std::hex;

  if (music->Channel0().size() > 0) {
    for (int32_t v : music->Channel0()) {
      ss << std::setw(2) << std::setfill('0') << v;
    }
    ss << std::endl;
  } else {
    ss << "none" << std::endl;
  }

  if (music->Channel1().size() > 0) {
    for (int32_t v : music->Channel1()) {
      ss << std::setw(2) << std::setfill('0') << v;
    }
    ss << std::endl;
  } else {
    ss << "none" << std::endl;
  }

  if (music->Channel2().size() > 0) {
    for (int32_t v : music->Channel2()) {
      ss << std::setw(2) << std::setfill('0') << v;
    }
    ss << std::endl;
  } else {
    ss << "none" << std::endl;
  }

  if (music->Channel3().size() > 0) {
    for (int32_t v : music->Channel3()) {
      ss << std::setw(2) << std::setfill('0') << v;
    }
    ss << std::endl;
  } else {
    ss << "none" << std::endl;
  }

  ss << std::dec;
}

void Resource::ParseVersion(std::stringstream& ss) {
  //
}

void Resource::ParseImage(std::stringstream& ss, int32_t image_index) {
  Image* image = graphics_->GetImageBank(image_index);
  int32_t** data = image->Data();

  for (int32_t i = 0; i < image->Height(); i++) {
    std::string line;
    std::getline(ss, line);
    line = Trim(line);

    if (line.size() != image->Width()) {
      throw ParseError();
    }

    for (int32_t j = 0; j < image->Width(); j++) {
      data[i][j] = std::stoi(line.substr(j, 1), nullptr, 16);
    }
  }
}

void Resource::ParseTilemap(std::stringstream& ss, int32_t tilemap_index) {
  Tilemap* tilemap = graphics_->GetTilemapBank(tilemap_index);
  int32_t** data = tilemap->Data();

  for (int32_t i = 0; i < tilemap->Height(); i++) {
    std::string line;
    std::getline(ss, line);
    line = Trim(line);

    if (line.size() != tilemap->Width() * 3) {
      throw ParseError();
    }

    for (int32_t j = 0; j < tilemap->Width(); j++) {
      data[i][j] = std::stoi(line.substr(j * 3, 3), nullptr, 16);
    }
  }
}

void Resource::ParseSound(std::stringstream& ss, int32_t sound_index) {
  Sound* sound = audio_->GetSoundBank(sound_index);

  {
    std::string line;
    std::getline(ss, line);
    line = Trim(line);

    SoundData& note = sound->Note();
    note.clear();

    if (line != "none") {
      for (int32_t i = 0; i < line.size() / 2; i++) {
        int32_t v = std::stoi(line.substr(i * 2, 2), nullptr, 16);

        if (v == 0xff) {
          v = -1;
        }

        note.push_back(v);
      }
    }
  }

  PARSE_SOUND(ss, sound, Tone);
  PARSE_SOUND(ss, sound, Volume);
  PARSE_SOUND(ss, sound, Effect);

  {
    std::string line;
    std::getline(ss, line);
    line = Trim(line);
    sound->Speed(std::stoi(line));
  }
}

void Resource::ParseMusic(std::stringstream& ss, int32_t music_index) {
  Music* music = audio_->GetMusicBank(music_index);

  PARSE_CHANNEL(ss, music, Channel0);
  PARSE_CHANNEL(ss, music, Channel1);
  PARSE_CHANNEL(ss, music, Channel2);
  PARSE_CHANNEL(ss, music, Channel3);
}

}  // namespace pyxelcore
