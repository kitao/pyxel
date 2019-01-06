#!/bin/bash
#
# Advance Preparation
#
# Mac:
#   brew install python3 sdl2 sdl2_image mingw-w64
#
# Linux:
#   sudo apt install python3 python3-pip libsdl2-dev libsdl2-image-dev
#

SCRIPT_DIR=$(cd $(dirname $0);pwd)
DOWNLOAD_DIR=$SCRIPT_DIR/download
SDL2_INCDIR=$SCRIPT_DIR/include/SDL2
SDL2_LIBDIR=$SCRIPT_DIR/lib
SDL2_BINDIR=$SCRIPT_DIR/bin

MINGW_SDL2_URL="https://www.libsdl.org/release/SDL2-devel-2.0.9-mingw.tar.gz"
MINGW_SDL2_IMAGE_URL="https://www.libsdl.org/projects/SDL_image/release/SDL2_image-devel-2.0.4-mingw.tar.gz"

rm -rf $SDL2_INCDIR $SDL2_LIBDIR $SDL_BINDIR $DOWNLOAD_DIR
mkdir -p $SDL2_INCDIR $SDL2_LIBDIR/win{32,64} $SDL2_BINDIR/win{32,64} $DOWNLOAD_DIR

cd $DOWNLOAD_DIR
curl -L $MINGW_SDL2_URL -o SDL2.tar.gz
tar xzf SDL2.tar.gz
cd SDL2-*
cp i686-w64-mingw32/include/SDL2/*.h $SDL2_INCDIR
cp i686-w64-mingw32/lib/libSDL2{.dll,main}.a $SDL2_LIBDIR/win32
cp i686-w64-mingw32/bin/SDL2.dll $SDL2_BINDIR/win32
cp x86_64-w64-mingw32/lib/libSDL2{.dll,main}.a $SDL2_LIBDIR/win64
cp x86_64-w64-mingw32/bin/SDL2.dll $SDL2_BINDIR/win64

cd $DOWNLOAD_DIR
curl -L $MINGW_SDL2_IMAGE_URL -o SDL2_image.tar.gz
tar xzf SDL2_image.tar.gz
cd SDL2_image-*
cp i686-w64-mingw32/include/SDL2/*.h $SDL2_INCDIR
cp i686-w64-mingw32/lib/libSDL2_image.dll.a $SDL2_LIBDIR/win32
cp i686-w64-mingw32/bin/*.dll $SDL2_BINDIR/win32
cp x86_64-w64-mingw32/lib/libSDL2_image.dll.a $SDL2_LIBDIR/win64
cp x86_64-w64-mingw32/bin/*.dll $SDL2_BINDIR/win64

rm -rf $DOWNLOAD_DIR
