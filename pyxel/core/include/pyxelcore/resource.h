#ifndef PYXELCORE_RESOURCE_H_
#define PYXELCORE_RESOURCE_H_

namespace pyxelcore {

class Resource {
public:
  Resource();
  ~Resource();

  void Save(char *filename);
  void Load(char *filename);

private:
};

} // namespace pyxelcore

#endif // PYXELCORE_RESOURCE_H_
