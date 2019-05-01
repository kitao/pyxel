#ifndef PYXELCORE_RESOURCE_H_
#define PYXELCORE_RESOURCE_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Resource {
 public:
  void LoadAsset(const char* filename);
  void SaveAsset(const char* filename);
};

}  // namespace pyxelcore

#endif  // PYXELCORE_RESOURCE_H_
