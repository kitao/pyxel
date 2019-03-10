#ifndef PYXELCORE_INPUT_H_
#define PYXELCORE_INPUT_H_

namespace pyxelcore {

class Input {
public:
  Input();
  ~Input();

  int mouse_x_getter() { return mouse_x_; }
  int mouse_y_getter() { return mouse_y_; }

  int btn(int key);
  int btnp(int key, int hold, int period);
  int btnr(int key);
  void mouse(int visible);

private:
  int mouse_x_;
  int mouse_y_;
};

} // namespace pyxelcore

#endif // PYXELCORE_INPUT_H_
