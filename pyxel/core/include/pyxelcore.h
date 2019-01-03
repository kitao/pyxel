#ifndef PYXELCORE_H_
#define PYXELCORE_H_

#ifdef WIN32
#define DLLEXPORT __declspec(dllexport)
#else
#define DLLEXPORT
#endif

DLLEXPORT int test(int width, int height);

#endif // PYXELCORE_H_
