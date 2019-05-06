#ifndef PYXELCORE_RESOURCE_H_
#define PYXELCORE_RESOURCE_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Resource {
 public:
  bool SaveAsset(const char* filename);
  bool LoadAsset(const char* filename);
};

}  // namespace pyxelcore

#endif  // PYXELCORE_RESOURCE_H_
