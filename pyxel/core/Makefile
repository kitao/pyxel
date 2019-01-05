INCDIR = include
LIBDIR = lib
SRCDIR = src
CFLAGS = $(SDL2_CFLAGS) -I$(INCDIR) -O3
LDFLAGS = $(SDL2_LDFLAGS) -L$(LIBDIR) -shared
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
SDL2_LDFLAGS = `sdl2-config --libs`
TARGET = $(LIBNAME)_darwin_amd64.dylib
endif

# Windows 32bit
ifeq ($(PF),win32)
CC = i686-w64-mingw32-gcc
SDL2_CFLAGS = $(MINGW_SDL2_CONFIG_CFLAGS)
SDL2_LDFLAGS = `echo $(MINGW_SDL2_CONFIG_STATIC_LIBS) | sed 's|-L/usr/lib -lmingw32 -lSDL2main -lSDL2|-L/usr/local/Cellar/mingw-w64/*/toolchain-i686/i686-w64-mingw32/lib -lmingw32 -lSDL2main_windows_386 -lSDL2_windows_386|'`
TARGET = $(LIBNAME)_windows_386.dll
endif

# Windows 64bit
ifeq ($(PF),win64)
CC = x86_64-w64-mingw32-gcc
SDL2_CFLAGS = $(MINGW_SDL2_CONFIG_CFLAGS)
SDL2_LDFLAGS = `echo $(MINGW_SDL2_CONFIG_STATIC_LIBS) | sed 's|-L/usr/lib -lmingw32 -lSDL2main -lSDL2|-L/usr/local/Cellar/mingw-w64/*/toolchain-x86_64/x86_64-w64-mingw32/lib -lmingw32 -lSDL2main_windows_amd64 -lSDL2_windows_amd64|'`
TARGET = $(LIBNAME)_windows_amd64.dll
endif

# Linux 64bit
ifeq ($(PF),linux)
CC = gcc
SDL2_CFLAGS = `sdl2-config --cflags`
SDL2_LDFLAGS = `sdl2-config --libs`
TARGET = $(LIBNAME)_linux_amd64.so
endif


.PHONY: all clean distclean test

all: $(TARGET)

clean:
	rm -rf $(OBJS) $(TARGET)

distclean:
	rm -rf $(OBJS) *.dylib *.dll *.so include/SDL2 lib

test:
	python3 __init__.py

$(TARGET): $(OBJS)
	$(CC) -o $@ $^ $(LDFLAGS)

.c.o:
	$(CC) -c $< -o $@ $(CFLAGS)
