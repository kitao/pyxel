#include "pyxelcore/resource.h"

#include "miniz-cpp/zip_file.hpp"
#include "picojason/picojson.h"

namespace pyxelcore {

bool Resource::SaveAsset(const char* filename) {
  // TODO

  miniz_cpp::zip_file file;

  file.writestr("file1.txt", "this is file 1");
  file.writestr("file2.txt", "this is file 2");
  file.writestr("file3.txt", "this is file 3");
  file.writestr("file4.txt", "this is file 4");
  file.writestr("file5.txt", "this is file 5");

  // file.save(argv[1]);

  return true;
}

bool Resource::LoadAsset(const char* filename) {
  // TODO

  miniz_cpp::zip_file file(filename);
  file.printdir();

  return true;
}

}  // namespace pyxelcore
