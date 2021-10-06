INCDIR = include
SRCDIR = src
LIBDIR = lib
BINDIR = bin
CFLAGS = -I$(INCDIR) $(SDL2_CFLAGS) -std=gnu++17 -fPIC -O3
LDFLAGS = $(SDL2_LDFLAGS) -shared
LIBNAME = libpyxelcore

SRCS = $(shell find $(SRCDIR) -type f -name '*.cc')
OBJS = $(SRCS:.cc=.o)

ifeq ($(PF),)
ifeq ($(shell uname),Darwin)
PF = macos
endif
ifeq ($(shell uname),Linux)
PF = linux
endif
ifneq ($(findstring MINGW32,$(shell uname)),)
PF = win32
endif
ifneq ($(findstring MINGW64,$(shell uname)),)
PF = win64
endif
endif

ifeq ($(PF),macos)
CC = g++-11
SDL2_CFLAGS = `sdl2-config --cflags`
SDL2_LDFLAGS = `sdl2-config --libs` -lSDL2_image
TARGET = $(BINDIR)/macos/$(LIBNAME).dylib
endif

ifeq ($(PF),win32)
CC = i686-w64-mingw32-g++
SDL2_CFLAGS = -I$(INCDIR)/win32/SDL2 -Dmain=SDL_main -DPYXEL_DLL
SDL2_LDFLAGS = -static -lstdc++ -lgcc -lwinpthread -L$(LIBDIR)/win32 -Wl,-Bdynamic -lSDL2main -lSDL2 -lSDL2_image -mwindows
TARGET = $(BINDIR)/win32/$(LIBNAME).dll
endif

ifeq ($(PF),win64)
CC = x86_64-w64-mingw32-g++
SDL2_CFLAGS = -I$(INCDIR)/win64/SDL2 -Dmain=SDL_main -DPYXEL_DLL
SDL2_LDFLAGS = -static -lstdc++ -lgcc -lwinpthread -L$(LIBDIR)/win64 -Wl,-Bdynamic -lSDL2main -lSDL2 -lSDL2_image -mwindows
TARGET = $(BINDIR)/win64/$(LIBNAME).dll
endif

ifeq ($(PF),linux)
CC = g++
SDL2_CFLAGS = `sdl2-config --cflags`
SDL2_LDFLAGS = `sdl2-config --libs` -lSDL2_image -static-libgcc -static-libstdc++
TARGET = $(BINDIR)/linux/$(LIBNAME).so
endif

.PHONY: all format clean distclean test

all: $(TARGET)

format:
	find include/pyxelcore.h include/pyxelcore src -name "*.h" -or -name "*.cc" | xargs clang-format -i
	isort ../..
	black ../..

clean:
	rm -rf $(OBJS) $(TARGET)

distclean:
	rm -rf $(OBJS) *.dylib *.dll *.so include/SDL2 lib bin

test:
	python3 ../examples/01_hello_pyxel.py
	python3 ../examples/02_jump_game.py
	python3 ../examples/03_draw_api.py
	python3 ../examples/04_sound_api.py
	python3 ../examples/05_color_palette.py
	python3 ../examples/06_click_game.py
	python3 ../examples/07_snake.py
	python3 ../examples/08_triangle_api.py
	python3 ../examples/09_shooter.py
	python3 ../editor/__init__.py

$(TARGET): $(OBJS)
	mkdir -p $(shell dirname $(TARGET))
	$(CC) -o $@ $^ $(LDFLAGS)

.cc.o:
	$(CC) -c $< -o $@ $(CFLAGS)
