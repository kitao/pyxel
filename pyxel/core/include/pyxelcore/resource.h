#ifndef PYXELCORE_RESOURCE_H_
#define PYXELCORE_RESOURCE_H_

#include <cstdint>

namespace pyxelcore {

class Resource {
 public:
  Resource();
  ~Resource();

  void save(const char* filename);
  void load(const char* filename);

 private:
};

}  // namespace pyxelcore

#endif  // PYXELCORE_RESOURCE_H_
