#include "pyxelcore.h"
#include "pyxelcore/app.h"
#include <cstdio>

static pyxelcore::App *app = NULL;

int Width_Getter() { return app ? app->Width_Getter() : 0; }

int Height_Getter() { return app ? app->Height_Getter() : 0; }

int FrameCount_Getter() { return app ? app->FrameCount_Getter() : 0; }

void Init(int width, int height, char *caption, int scale, int *palette,
          int fps, int border_width, int border_color) {
  app = new pyxelcore::App(width, height, caption, scale, palette, fps,
                           border_width, border_color);
}

void Run(void (*update)(), void (*draw)()) {
  if (app) {
    app->Run(update, draw);
  }
}

void Quit() {}
