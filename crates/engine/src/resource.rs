use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

use crate::capturer::Capturer;
use crate::image::Image;
use crate::settings::RESOURCE_ARCHIVE_DIRNAME;
use crate::Pyxel;

pub struct Resource {
    capturer: Capturer,
}

impl Resource {
    pub fn new(width: u32, height: u32, capture_frame_count: u32) -> Resource {
        Resource {
            capturer: Capturer::new(width, height, capture_frame_count),
        }
    }

    fn version_name() -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "version"
    }

    fn image_name(image_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "image" + &image_no.to_string()
    }

    fn tilemap_name(tilemap_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "tilemap" + &tilemap_no.to_string()
    }

    fn sound_name(sound_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "sound" + &format!("{:02}", sound_no)
    }

    fn music_name(music_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "music" + &music_no.to_string()
    }

    /*
    void Resource::ClearImage(int32_t image_index) {
      Image* image = graphics_->GetImageBank(image_index);
      int32_t** data = image->Data();

      for (int32_t i = 0; i < image->Height(); i++) {
        for (int32_t j = 0; j < image->Width(); j++) {
          data[i][j] = 0;
        }
      }
    }

    void Resource::ClearTilemap(int32_t tilemap_index) {
      Tilemap* tilemap = graphics_->GetTilemapBank(tilemap_index);
      int32_t** data = tilemap->Data();

      for (int32_t i = 0; i < tilemap->Height(); i++) {
        for (int32_t j = 0; j < tilemap->Width(); j++) {
          data[i][j] = 0;
        }
      }
    }

    void Resource::ClearSound(int32_t sound_index) {
      Sound* sound = audio_->GetSoundBank(sound_index);

      sound->Note().clear();
      sound->Tone().clear();
      sound->Volume().clear();
      sound->Effect().clear();
    }

    void Resource::ClearMusic(int32_t music_index) {
      Music* music = audio_->GetMusicBank(music_index);

      music->Channel0().clear();
      music->Channel1().clear();
      music->Channel2().clear();
      music->Channel3().clear();
    }
    */

    /*
    void Resource::ParseImage(int32_t image_index, const std::string& str) {
      Image* image = graphics_->GetImageBank(image_index);
      int32_t** data = image->Data();
      std::stringstream ss(str);

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

    void Resource::ParseTilemap(int32_t tilemap_index, const std::string& str) {
      Tilemap* tilemap = graphics_->GetTilemapBank(tilemap_index);
      int32_t** data = tilemap->Data();
      std::stringstream ss(str);

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

      try {  // for backward compatibility
        std::string line;
        std::getline(ss, line);
        line = Trim(line);

        tilemap->ImageIndex(std::stoi(line));
      } catch (...) {
        tilemap->ImageIndex(0);
      }
    }

    void Resource::ParseSound(int32_t sound_index, const std::string& str) {
      Sound* sound = audio_->GetSoundBank(sound_index);
      std::stringstream ss(str);

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

    void Resource::ParseMusic(int32_t music_index, const std::string& str) {
      Music* music = audio_->GetMusicBank(music_index);
      std::stringstream ss(str);

      PARSE_CHANNEL(ss, music, Channel0);
      PARSE_CHANNEL(ss, music, Channel1);
      PARSE_CHANNEL(ss, music, Channel2);
      PARSE_CHANNEL(ss, music, Channel3);
    }

    #define PARSE_CHANNEL(ss, music, channel)                          \
      do {                                                             \
        SoundIndexList& data = music->channel();                       \
        data.clear();                                                  \
                                                                       \
        std::string line = GetTrimmedLine(ss);                         \
                                                                       \
        if (line != "none") {                                          \
          for (int32_t i = 0; i < line.size() / 2; i++) {              \
            int32_t v = std::stoi(line.substr(i * 2, 2), nullptr, 16); \
                                                                       \
            data.push_back(v);                                         \
          }                                                            \
        }                                                              \
      } while (false)

    #define PARSE_SOUND(ss, sound, property)   \
      do {                                     \
        SoundData& data = sound->property();   \
        data.clear();                          \
                                               \
        std::string line = GetTrimmedLine(ss); \
                                               \
        if (line != "none") {                  \
          for (char c : line) {                \
            data.push_back(c - '0');           \
          }                                    \
        }                                      \
      } while (false)

    */

    /*
    std::string Resource::DumpImage(int32_t image_index) const {
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
        return "";
      }

      std::stringstream ss;

      ss << std::hex;

      for (int32_t i = 0; i < image->Height(); i++) {
        for (int32_t j = 0; j < image->Width(); j++) {
          ss << data[i][j];
        }

        ss << std::endl;
      }

      return ss.str();
    }

    std::string Resource::DumpTilemap(int32_t tilemap_index) const {
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
        return "";
      }

      std::stringstream ss;

      ss << std::hex;

      for (int32_t i = 0; i < tilemap->Height(); i++) {
        for (int32_t j = 0; j < tilemap->Width(); j++) {
          ss << std::setw(3) << std::setfill('0') << data[i][j];
        }

        ss << std::endl;
      }

      ss << std::dec << tilemap->ImageIndex() << std::endl;

      return ss.str();
    }

    std::string Resource::DumpSound(int32_t sound_index) const {
      Sound* sound = audio_->GetSoundBank(sound_index);

      if (sound->Note().size() == 0 && sound->Tone().size() == 0 &&
          sound->Volume().size() == 0 && sound->Effect().size() == 0) {
        return "";
      }

      std::stringstream ss;

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

      return ss.str();
    }

    std::string Resource::DumpMusic(int32_t music_index) const {
      Music* music = audio_->GetMusicBank(music_index);

      if (music->Channel0().size() == 0 && music->Channel1().size() == 0 &&
          music->Channel2().size() == 0 && music->Channel3().size() == 0) {
        return "";
      }

      std::stringstream ss;

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

      return ss.str();
    }
    */

    pub fn capture_screen(&mut self, screen: &Image, frame_count: u32) {
        self.capturer.capture_screen(screen, frame_count);
    }
}

impl Pyxel {
    pub fn load(&mut self, filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
        let mut archive = ZipArchive::new(File::open(&Path::new(filename)).unwrap()).unwrap();

        //
        // version
        //
        {
            let mut file = archive.by_name(&Resource::version_name()).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            println!("{}", contents);

            /*
              int32_t lib_ver = 0;
              int32_t res_ver = 0;

              {
                std::string name = GetVersionName();

                if (file.has_file(name)) {
                  std::stringstream ss(file.read(name));
                  std::string line = GetTrimmedLine(ss);

                  std::vector<std::string> lib_ver_str = Split(VERSION, '.');
                  std::vector<std::string> res_ver_str = Split(line, '.');
                  const int32_t ver_size = lib_ver_str.size();

                  if (res_ver_str.size() != ver_size) {
                    throw ParseError();
                  }

                  for (int32_t i = 0; i < ver_size; ++i) {
                    lib_ver =
                        lib_ver * 100 +
                        (i < lib_ver_str.size() ? std::atoi(lib_ver_str[i].c_str()) : 0);
                    res_ver =
                        res_ver * 100 +
                        (i < res_ver_str.size() ? std::atoi(res_ver_str[i].c_str()) : 0);
                  }

                  if (res_ver > lib_ver) {
                    PYXEL_ERROR("unsupported resource file version '" + line + "'");
                  }
                } else {
                  throw ParseError();
                }
              }
            */
        }

        //
        // Image
        //
        if image {
            /*
            for (int32_t i = 0; i < USER_IMAGE_BANK_COUNT; i++) {
            std::string name = GetImageName(i);

            if (file.has_file(name)) {
                ParseImage(i, file.read(name));
            } else {
                ClearImage(i);
            }
            */
        }

        //
        // Tilemap
        //
        if tilemap {
            /*
            for (int32_t i = 0; i < TILEMAP_BANK_COUNT; i++) {
            std::string name = GetTilemapName(i);

            if (file.has_file(name)) {
                ParseTilemap(i, file.read(name));
            } else {
                ClearTilemap(i);
            }
            */
        }

        //
        // Sound
        //
        if sound {
            /*
            for (int32_t i = 0; i < USER_SOUND_BANK_COUNT; i++) {
            std::string name = GetSoundName(i);

            if (file.has_file(name)) {
                ParseSound(i, file.read(name));
            } else {
                ClearSound(i);
            }
            */
        }

        //
        // Music
        //
        if music {
            /*
            for (int32_t i = 0; i < MUSIC_BANK_COUNT; i++) {
              std::string name = GetMusicName(i);

              if (file.has_file(name)) {
                ParseMusic(i, file.read(name));
              } else {
                ClearMusic(i);
              }
            }
            */
        }
    }

    pub fn save(&mut self, filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
        // TODO
        let _ = (filename, image, tilemap, sound, music); // dummy

        /*
        std::ofstream ofs(std::filesystem::u8path(filename), std::ios::binary);

        if (ofs.fail()) {
          PYXEL_ERROR("cannot save file '" + filename + "'");
        }

        miniz_cpp::zip_file file;

        file.writestr(GetVersionName(), VERSION + '\n');

        for (int32_t i = 0; i < USER_IMAGE_BANK_COUNT; i++) {
          std::string str = DumpImage(i);

          if (str.size() > 0) {
            file.writestr(GetImageName(i), str);
          }
        }

        for (int32_t i = 0; i < TILEMAP_BANK_COUNT; i++) {
          std::string str = DumpTilemap(i);

          if (str.size() > 0) {
            file.writestr(GetTilemapName(i), str);
          }
        }

        for (int32_t i = 0; i < USER_SOUND_BANK_COUNT; i++) {
          std::string str = DumpSound(i);

          if (str.size() > 0) {
            file.writestr(GetSoundName(i), str);
          }
        }

        for (int32_t i = 0; i < MUSIC_BANK_COUNT; i++) {
          std::string str = DumpMusic(i);

          if (str.size() > 0) {
            file.writestr(GetMusicName(i), str);
          }
        }

        file.save(ofs);
        ofs.close();
        */
    }

    pub fn screenshot(&mut self) {
        self.resource.capturer.screenshot(&self.colors);
    }

    pub fn reset_capture(&mut self) {
        self.resource.capturer.reset_capture();
    }

    pub fn screencast(&mut self) {
        self.resource.capturer.screencast(&self.colors);
    }
}
