#ifndef PYXELCORE_UTILITIES_H_
#define PYXELCORE_UTILITIES_H_

#include <cstdint>

namespace pyxelcore {

void RaiseError(const char* message);

int32_t GetConstantNumber(const char* name);
const char* GetConstantString(const char* name);

}  // namespace pyxelcore

#endif  // PYXELCORE_UTILITIES_H_
