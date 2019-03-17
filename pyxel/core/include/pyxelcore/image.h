#ifndef PYXELCORE_IMAGE_H_
#define PYXELCORE_IMAGE_H_

namespace pyxelcore {

class Image {
public:
  Image(int width, int height);
  ~Image();

  int width() { return width_; }
  int height() { return height_; }
  int *data() { return data_; }

  int get(int x, int y);
  void set(int x, int y, int data);
  void set(int x, int y, int *data, int data_width, int data_height);
  void load(int x, int y, char *filename);
  void copy(int x, int y, int img, int u, int v, int w, int h);

private:
  int width_;
  int height_;
  int *data_;
};

} // namespace pyxelcore

#endif // PYXELCORE_IMAGE_H_
