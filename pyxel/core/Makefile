INCDIR = include
SRCDIR = src
LIBDIR = lib
BINDIR = bin
CFLAGS = -I$(INCDIR) $(SDL2_CFLAGS) -O3
LDFLAGS = $(SDL2_LDFLAGS) -shared
LIBNAME = libpyxelcore

SRCS = $(wildcard $(SRCDIR)/*.cpp)
OBJS = $(SRCS:.cpp=.o)

ifeq ($(PF),)
ifeq ($(shell uname),Darwin)
PF = macos
endif
ifeq ($(shell uname),Linux)
PF = linux
endif
endif

ifeq ($(PF),macos)
CC = g++
SDL2_CFLAGS = `sdl2-config --cflags`
SDL2_LDFLAGS = `sdl2-config --libs` -lSDL2_image
TARGET = $(BINDIR)/macos/$(LIBNAME).dylib
endif

ifeq ($(PF),win32)
CC = i686-w64-mingw32-g++
SDL2_CFLAGS = -Dmain=SDL_main -DPYXEL_DLL
SDL2_LDFLAGS = -L$(LIBDIR)/win32 -lmingw32 -lSDL2main -lSDL2 -lSDL2_image -mwindows -static-libgcc -static-libstdc++
TARGET = $(BINDIR)/win32/$(LIBNAME).dll
endif

ifeq ($(PF),win64)
CC = x86_64-w64-mingw32-g++
SDL2_CFLAGS = -Dmain=SDL_main -DPYXEL_DLL
SDL2_LDFLAGS = -L$(LIBDIR)/win64 -lmingw32 -lSDL2main -lSDL2 -lSDL2_image -mwindows -static-libgcc -static-libstdc++
TARGET = $(BINDIR)/win64/$(LIBNAME).dll
endif

ifeq ($(PF),linux)
CC = g++
SDL2_CFLAGS = `sdl2-config --cflags`
SDL2_LDFLAGS = `sdl2-config --libs` -lSDL2_image -static-libgcc -static-libstdc++
TARGET = $(BINDIR)/linux/$(LIBNAME).so
endif

.PHONY: all clean distclean test

all: $(TARGET)

clean:
	rm -rf $(OBJS) $(TARGET)

distclean:
	rm -rf $(OBJS) *.dylib *.dll *.so include/SDL2 lib bin

test:
	python3 __init__.py

$(TARGET): $(OBJS)
	mkdir -p $(shell dirname $(TARGET))
	$(CC) -o $@ $^ $(LDFLAGS)

.cpp.o:
	$(CC) -c $< -o $@ $(CFLAGS)
