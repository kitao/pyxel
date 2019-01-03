SRCDIR = src
INCDIR = include
CFLAGS = -O3 -I$(INCDIR) $(SDL2_CFLAGS)
LDFLAGS = -shared $(SDL2_LDFLAGS)
LIBNAME = libpyxelcore

SDL2_URL = "https://github.com/veandco/go-sdl2-libs/archive/acefe9bbab163b8e6f1687599a4cd3052983c9a1.zip"
SDL2_INCDIR = include/SDL2
SDL2_LIBDIR = lib/SDL2

SRCS = $(wildcard $(SRCDIR)/*.c)
OBJS = $(SRCS:.c=.o)

ifeq ($(PLATFORM),)
ifeq ($(shell uname),Darwin)
PLATFORM = mac
endif
endif

ifeq ($(PLATFORM),mac)
CC = gcc
SDL2_CFLAGS = -D_THREAD_SAFE
SDL2_LDFLAGS = -L$(SDL2_LIBDIR) -lSDL2_darwin_amd64 -lm -liconv -Wl,-framework,CoreAudio -Wl,-framework,AudioToolbox -Wl,-framework,ForceFeedback -lobjc -Wl,-framework,CoreVideo -Wl,-framework,Cocoa -Wl,-framework,Carbon -Wl,-framework,IOKit -Wl,-weak_framework,QuartzCore -Wl,-weak_framework,Metal
TARGET = $(LIBNAME)_darwin_amd64.dylib
endif

ifeq ($(PLATFORM),win32)
TARGET = $(LIBNAME)_windows_386.dll
endif

ifeq ($(PLATFORM),win64)
TARGET = $(LIBNAME)_windows_amd64.dll
endif

ifeq ($(PLATFORM),linux32)
CC = gcc
TARGET = $(LIBNAME)_linux_386.so
endif

ifeq ($(PLATFORM),linux64)
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
	rm -rf $(OBJS) *.so *.dll *.dylib

distclean: clean
	rm -rf $(SDL2_INCDIR) lib download .depends

test:
	python3 __init__.py

$(TARGET): $(OBJS)
	$(CC) -o $@ $^ $(LDFLAGS)

.c.o:
	$(CC) $(CFLAGS) -c $< -o $@
