#ifndef PYXELCORE_RESOURCE_H_
#define PYXELCORE_RESOURCE_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Graphics;
class Audio;
class Image;
class Tilemap;
class Sound;
class Music;

class Resource {
 public:
  Resource(Graphics* graphics, Audio* audio);

  void SaveAsset(const std::string& filename);
  void LoadAsset(const std::string& filename,
                 bool image = true,
                 bool tilemap = true,
                 bool sound = true,
                 bool music = true);

 private:
  Graphics* graphics_;
  Audio* audio_;

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
};

inline std::string Resource::GetVersionName() {
  return std::string(RESOURCE_ARCHIVE_DIRNAME) + "version";
}

inline std::string Resource::GetImageName(int32_t image_index) {
  return std::string(RESOURCE_ARCHIVE_DIRNAME) + "image" +
         std::to_string(image_index);
}

inline std::string Resource::GetTilemapName(int32_t tilemap_index) {
  return std::string(RESOURCE_ARCHIVE_DIRNAME) + "tilemap" +
         std::to_string(tilemap_index);
}

inline std::string Resource::GetSoundName(int32_t sound_index) {
  return std::string(RESOURCE_ARCHIVE_DIRNAME) + "sound" +
         (sound_index < 10 ? "0" : "") + std::to_string(sound_index);
}

inline std::string Resource::GetMusicName(int32_t music_index) {
  return std::string(RESOURCE_ARCHIVE_DIRNAME) + "music" +
         std::to_string(music_index);
}

}  // namespace pyxelcore

#endif  // PYXELCORE_RESOURCE_H_
