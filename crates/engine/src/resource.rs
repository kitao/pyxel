use crate::canvas::Canvas;
use crate::image::Image;
use crate::settings::CAPTURE_SCALE;
use crate::Pyxel;

struct FrameInfo {
    frame_image: Image,
    frame_count: u32,
}

pub struct Resource {
    max_frame_count: u32,
    frame_info: Vec<FrameInfo>,
    start_frame_index: u32,
    cur_frame_index: u32,
    cur_frame_count: u32,
}

impl Resource {
    pub fn new(width: u32, height: u32, max_frame_count: u32) -> Resource {
        let mut frame_info = Vec::new();
        for _ in 0..max_frame_count {
            frame_info.push(FrameInfo {
                frame_image: Image::without_arc_mutex(width, height),
                frame_count: 0,
            })
        }

        Resource {
            max_frame_count: max_frame_count,
            frame_info: frame_info,
            start_frame_index: 0,
            cur_frame_index: 0,
            cur_frame_count: 0,
        }
    }

    pub fn capture_screen(&mut self, screen: &Image, frame_count: u32) {
        if self.max_frame_count == 0 {
            return;
        }

        self.cur_frame_index = (self.cur_frame_index + 1) % self.max_frame_count;
        self.cur_frame_count += 1;

        self.frame_info[self.cur_frame_index as usize]
            .frame_image
            .blt(
                0,
                0,
                screen,
                0,
                0,
                screen.width() as i32,
                screen.height() as i32,
                None,
            );
        self.frame_info[self.cur_frame_index as usize].frame_count = frame_count;

        if self.cur_frame_count > self.max_frame_count {
            self.start_frame_index = (self.start_frame_index + 1) % self.max_frame_count;
            self.cur_frame_count = self.max_frame_count;
        }
    }

    /*
    void ClearImage(int32_t image_index);
    void ClearTilemap(int32_t tilemap_index);
    void ClearSound(int32_t sound_index);
    void ClearMusic(int32_t music_index);

    std::string DumpImage(int32_t image_index) const;
    std::string DumpTilemap(int32_t tilemap_index) const;
    std::string DumpSound(int32_t sound_index) const;
    std::string DumpMusic(int32_t music_index) const;

    void ParseImage(int32_t image_index, const std::string& str);
    void ParseTilemap(int32_t tilemap_index, const std::string& str);
    void ParseSound(int32_t sound_index, const std::string& str);
    void ParseMusic(int32_t music_index, const std::string& str);

    static std::string GetVersionName();
    static std::string GetImageName(int32_t image_index);
    static std::string GetTilemapName(int32_t tilemap_index);
    static std::string GetSoundName(int32_t sound_index);
    static std::string GetMusicName(int32_t music_index);
    */
}

impl Pyxel {
    pub fn load(&mut self, filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
        // TODO
        let _ = (filename, image, tilemap, sound, music); // dummy

        /*
        std::ifstream ifs(std::filesystem::u8path(filename), std::ios::binary);

        if (ifs.fail()) {
          PYXEL_ERROR("cannot open file '" + filename + "'");
        }

        miniz_cpp::zip_file file;
        file.load(ifs);

        ifs.close();

        try {
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

          if (image) {
            for (int32_t i = 0; i < USER_IMAGE_BANK_COUNT; i++) {
              std::string name = GetImageName(i);

              if (file.has_file(name)) {
                ParseImage(i, file.read(name));
              } else {
                ClearImage(i);
              }
            }
          }

          if (tilemap) {
            for (int32_t i = 0; i < TILEMAP_BANK_COUNT; i++) {
              std::string name = GetTilemapName(i);

              if (file.has_file(name)) {
                ParseTilemap(i, file.read(name));
              } else {
                ClearTilemap(i);
              }
            }
          }

          if (sound) {
            for (int32_t i = 0; i < USER_SOUND_BANK_COUNT; i++) {
              std::string name = GetSoundName(i);

              if (file.has_file(name)) {
                ParseSound(i, file.read(name));
              } else {
                ClearSound(i);
              }
            }
          }

          if (music) {
            for (int32_t i = 0; i < MUSIC_BANK_COUNT; i++) {
              std::string name = GetMusicName(i);

              if (file.has_file(name)) {
                ParseMusic(i, file.read(name));
              } else {
                ClearMusic(i);
              }
            }
          }
        } catch (...) {
          PYXEL_ERROR("invalid resource file");
        }
              */
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

    pub fn save_png(&mut self) {
        // TODO
        /*
        SDL_Surface* surface = SDL_CreateRGBSurfaceWithFormat(
            0, width_ * SCREEN_CAPTURE_SCALE, height_ * SCREEN_CAPTURE_SCALE, 32,
            SDL_PIXELFORMAT_RGB888);

        SDL_LockSurface(surface);

        int32_t** src_data = captured_images_[cur_frame_]->Data();
        int32_t* dst_data = reinterpret_cast<int32_t*>(surface->pixels);

        int32_t scaled_width = width_ * SCREEN_CAPTURE_SCALE;
        int32_t scaled_height = height_ * SCREEN_CAPTURE_SCALE;

        for (int32_t i = 0; i < scaled_height; i++) {
          for (int32_t j = 0; j < scaled_width; j++) {
            int32_t index = scaled_width * i + j;
            int32_t color =
                src_data[i / SCREEN_CAPTURE_SCALE][j / SCREEN_CAPTURE_SCALE];

            dst_data[index] = palette_color_[color];
          }
        }

        SDL_UnlockSurface(surface);
        IMG_SavePNG(surface, (GetBaseName() + ".png").c_str());
        SDL_FreeSurface(surface);
        */
    }

    pub fn reset_gif(&mut self) {
        if self.resource.max_frame_count == 0 {
            return;
        }

        self.resource.start_frame_index =
            (self.resource.cur_frame_index + 1) % self.resource.max_frame_count;
        self.resource.cur_frame_count = 0;
    }

    pub fn save_gif(&mut self) {
        if self.resource.max_frame_count == 0 || self.resource.cur_frame_count == 0 {
            return;
        }

        // TODO
        /*
        std::string filename = GetBaseName() + ".gif";
        GifWriter* gif_writer =
            new GifWriter(filename, width_, height_, palette_color_);

        for (int32_t i = 0; i < frame_count_; i++) {
          int32_t index = (start_frame_ + i) % SCREEN_CAPTURE_COUNT;

          gif_writer->AddFrame(captured_images_[index],
                               captured_frames_[index] * 100.0f / fps_ + 0.5f);
        }

        gif_writer->EndFrame();
        delete gif_writer;

        // try to optimize the generated GIF file with Gifsicle
        int32_t res = system(("gifsicle -b -O3 -Okeep-empty " + filename).c_str());

        ResetScreenCapture();
        */
    }

    /*
    std::string Recorder::GetBaseName() const {
    #ifdef WIN32
      std::string desktop_path = getenv("USERPROFILE");
      desktop_path += "\\Desktop\\";
    #else
      std::string desktop_path = getenv("HOME");
      desktop_path += "/Desktop/";
    #endif

      char basename[30];
      time_t t = std::time(nullptr);
      std::strftime(basename, sizeof(basename), "pyxel-%y%m%d-%H%M%S",
                    std::localtime(&t));

      return desktop_path + basename;
    }
    */
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
*/
