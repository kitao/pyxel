SRCDIR = src
INCDIR = include
CFLAGS = -O3 -I$(INCDIR) $(SDL2_CFLAGS)
LDFLAGS = -shared $(SDL2_LDFLAGS)
LIBNAME = libpyxelcore

SDL2_URL = "https://github.com/veandco/go-sdl2-libs/archive/acefe9bbab163b8e6f1687599a4cd3052983c9a1.zip"
SDL2_INCDIR = include/SDL2
SDL2_LIBDIR = lib/SDL2

MINGW_VERSION = 5.0.4_1

SRCS = $(wildcard $(SRCDIR)/*.c)
OBJS = $(SRCS:.c=.o)

ifeq ($(PF),)
ifeq ($(shell uname),Darwin)
PF = mac
endif
endif

# macOS 64bit
ifeq ($(PF),mac)
CC = gcc
SDL2_CFLAGS = -D_THREAD_SAFE
SDL2_LDFLAGS = -L$(SDL2_LIBDIR) -lSDL2_darwin_amd64 -lm -liconv -Wl,-framework,CoreAudio -Wl,-framework,AudioToolbox -Wl,-framework,ForceFeedback -lobjc -Wl,-framework,CoreVideo -Wl,-framework,Cocoa -Wl,-framework,Carbon -Wl,-framework,IOKit -Wl,-weak_framework,QuartzCore -Wl,-weak_framework,Metal
TARGET = $(LIBNAME)_darwin_amd64.dylib
endif

# Windows 32bit
ifeq ($(PF),win32)
CC = i686-w64-mingw32-gcc
SDL2_CFLAGS = -Dmain=SDL_main
SDL2_LDFLAGS = -L$(SDL2_LIBDIR) -L/usr/local/Cellar/mingw-w64/$(MINGW_VERSION)/toolchain-i686/i686-w64-mingw32/lib -lmingw32 -lSDL2main_windows_386 -lSDL2_windows_386 -mwindows -Wl,--no-undefined -lm -ldinput8 -ldxguid -ldxerr8 -luser32 -lgdi32 -lwinmm -limm32 -lole32 -loleaut32 -lshell32 -lsetupapi -lversion -luuid -static-libgcc
TARGET = $(LIBNAME)_windows_i386.dll
endif

# Windows 64bit
ifeq ($(PF),win64)
CC = x86_64-w64-mingw32-gcc
SDL2_CFLAGS = -Dmain=SDL_main
SDL2_LDFLAGS = -L$(SDL2_LIBDIR) -L/usr/local/Cellar/mingw-w64/$(MINGW_VERSION)/toolchain-x86_64/x86_64-w64-mingw32/lib -lmingw32 -lSDL2main_windows_amd64 -lSDL2_windows_amd64 -Wl,--no-undefined -lm -ldinput8 -ldxguid -ldxerr8 -luser32 -lgdi32 -lwinmm -limm32 -lole32 -loleaut32 -lshell32 -lsetupapi -lversion -luuid -static-libgcc
TARGET = $(LIBNAME)_windows_amd64.dll
endif

# Linux 32bit
ifeq ($(PF),linux32)
CC = gcc
TARGET = $(LIBNAME)_linux_i386.so
endif

# Linux 64bit
ifeq ($(PF),linux64)
CC = gcc
SDL2_CFLAGS = -D_REENTRANT
SDL2_LDFLAGS = -L$(SDL2_LIBDIR) -L/usr/lib/x86_64-linux-gnu -lSDL2_linux_amd64 -Wl,--no-undefined -lm -ldl -lasound -lm -ldl -lpthread -lpulse-simple -lpulse -lsndio -lX11 -lXext -lXcursor -lXinerama -lXi -lXrandr -lXss -lXxf86vm -lwayland-egl -lwayland-client -lwayland-cursor -lxkbcommon -lpthread -lrt
TARGET = $(LIBNAME)_linux_amd64.so
endif

.PHONY: all depends clean distclean test

all: depends $(TARGET)

depends: .depends

.depends:
	rm -rf download
	mkdir -p download
	curl -L $(SDL2_URL) -o download/data.zip
	unzip download/data.zip -d download
	mkdir -p $(SDL2_INCDIR) $(SDL2_LIBDIR)
	cp download/*/*.a $(SDL2_LIBDIR)
	cp download/*/include/SDL2/* $(SDL2_INCDIR)
	rm -rf download
	touch .depends

clean:
	rm -rf $(OBJS) $(TARGET)

distclean: clean
	rm -rf $(SDL2_INCDIR) lib download .depends *.so *.dll *.dylib

test:
	python3 __init__.py

$(TARGET): $(OBJS)
	$(CC) -o $@ $^ $(LDFLAGS)

.c.o:
	$(CC) $(CFLAGS) -c $< -o $@
