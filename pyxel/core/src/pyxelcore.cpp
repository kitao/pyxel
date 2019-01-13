#include "pyxelcore.h"
#include <cstdio>

int Width() { return 0; }

int Height() { return 0; }

int FrameCount() { return 0; }

void Init(int width, int height, char *caption, int scale, int *palette,
          int fps, int border_width, int border_color) {
  printf("width: %d\n", width);
  printf("%d\n", palette[0]);
  printf("%d\n", palette[1]);
  printf("%s\n", caption);
}

void Run(void (*update)(), void (*draw)()) {}

void Quit() {}
