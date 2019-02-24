#ifndef PYXELCORE_INPUT_H_
#define PYXELCORE_INPUT_H_

namespace pyxelcore {

class Input {
public:
  Input();
  ~Input();

  int MouseX();
  int MouseY();

  int Btn(int key);
  int Btnp(int key, int hold, int period);
  int Btnr(int key);
  void Mouse(int visible);

private:
};

} // namespace pyxelcore

#endif // PYXELCORE_INPUT_H_
