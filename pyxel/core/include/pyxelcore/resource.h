#ifndef PYXELCORE_RESOURCE_H_
#define PYXELCORE_RESOURCE_H_

#include <cstdint>

namespace pyxelcore {

class Resource {
 public:
  Resource();
  ~Resource();

  void Save(const char* filename);
  void Load(const char* filename);

 private:
};

}  // namespace pyxelcore

#endif  // PYXELCORE_RESOURCE_H_
