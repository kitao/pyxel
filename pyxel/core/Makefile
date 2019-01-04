INCDIR = include
LIBDIR = lib
SRCDIR = src
CFLAGS = -O3 -I$(INCDIR) $(SDL2_CFLAGS)
LDFLAGS = -shared -L$(LIBDIR) $(SDL2_LDFLAGS)
LIBNAME = libpyxelcore

MINGW_SDL2_CONFIG_CFLAGS=-I/usr/include/SDL2 -Dmain=SDL_main
MINGW_SDL2_CONFIG_STATIC_LIBS=-L/usr/lib -lmingw32 -lSDL2main -lSDL2 -mwindows -Wl,--no-undefined -lm -ldinput8 -ldxguid -ldxerr8 -luser32 -lgdi32 -lwinmm -limm32 -lole32 -loleaut32 -lshell32 -lsetupapi -lversion -luuid -static-libgcc

SRCS = $(wildcard $(SRCDIR)/*.c)
OBJS = $(SRCS:.c=.o)

ifeq ($(PF),)
ifeq ($(shell uname),Darwin)
PF = mac
endif
ifeq ($(shell uname),Linux)
PF = linux
endif
endif

# macOS 64bit
ifeq ($(PF),mac)
CC = gcc
SDL2_CFLAGS = `sdl2-config --cflags`
SDL2_LDFLAGS = `sdl2-config --static-libs|sed 's/-lSDL2/-lSDL2_image_darwin_amd64 -lSDL2_darwin_amd64/'`
TARGET = $(LIBNAME)_darwin_amd64.dylib
endif

# Linux 64bit
ifeq ($(PF),linux)
CC = gcc
SDL2_CFLAGS = `sdl2-config --cflags`
SDL2_LDFLAGS = `sdl2-config --libs|sed 's/-lSDL2/-lSDL2_image -lSDL2/'`
TARGET = $(LIBNAME)_linux_amd64.so
endif

# Windows 32bit
ifeq ($(PF),win32)
CC = i686-w64-mingw32-gcc
SDL2_CFLAGS = $(MINGW_SDL2_CONFIG_CFLAGS)
SDL2_LDFLAGS = `echo $(MINGW_SDL2_CONFIG_STATIC_LIBS)|sed 's|-L/usr/lib -lmingw32 -lSDL2main -lSDL2|-L/usr/local/Cellar/mingw-w64/*/toolchain-i686/i686-w64-mingw32/lib -lmingw32 -lSDL2_image_windows_386 -lSDL2main_windows_386 -lSDL2_windows_386|'`
TARGET = $(LIBNAME)_windows_386.dll
endif

# Windows 64bit
ifeq ($(PF),win64)
CC = x86_64-w64-mingw32-gcc
SDL2_CFLAGS = $(MINGW_SDL2_CONFIG_CFLAGS)
SDL2_LDFLAGS = `echo $(MINGW_SDL2_CONFIG_STATIC_LIBS)|sed 's|-L/usr/lib -lmingw32 -lSDL2main -lSDL2|-L/usr/local/Cellar/mingw-w64/*/toolchain-x86_64/x86_64-w64-mingw32/lib -lmingw32 -lSDL2_image_windows_amd64 -lSDL2main_windows_amd64 -lSDL2_windows_amd64|'`
TARGET = $(LIBNAME)_windows_amd64.dll
endif

.PHONY: all clean distclean test

all: $(TARGET)

clean:
	rm -rf $(OBJS) $(TARGET)

distclean:
	rm -rf $(OBJS) *.so *.dll *.dylib

test:
	python3 __init__.py

$(TARGET): $(OBJS)
	$(CC) -o $@ $^ $(LDFLAGS)

.c.o:
	$(CC) -c $< -o $@ $(CFLAGS)
