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

  bool SaveAsset(const std::string& filename);
  bool LoadAsset(const std::string& filename);

 private:
  Graphics* graphics_;
  Audio* audio_;

  void DumpImage(std::stringstream& ss, int32_t image_index);
  void DumpTilemap(std::stringstream& ss, int32_t tilemap_index);
  void DumpSound(std::stringstream& ss, int32_t sound_index);
  void DumpMusic(std::stringstream& ss, int32_t music_index);

  void ParseVersion(std::stringstream& ss);
  void ParseImage(std::stringstream& ss, int32_t image_index);
  void ParseTilemap(std::stringstream& ss, int32_t tilemap_index);
  void ParseSound(std::stringstream& ss, int32_t sound_index);
  void ParseMusic(std::stringstream& ss, int32_t music_index);
};

}  // namespace pyxelcore

#endif  // PYXELCORE_RESOURCE_H_
