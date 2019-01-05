#!/bin/bash
#
# Advance Preparation
#
# Mac:
#   brew install python3 sdl2 mingw-w64
#
# Linux:
#   sudo apt install python3 python3-pip libsdl2-dev
#

SCRIPT_DIR=$(cd $(dirname $0);pwd)
DOWNLOAD_DIR=$SCRIPT_DIR/download
SDL2_INCDIR=$SCRIPT_DIR/include/SDL2
SDL2_LIBDIR=$SCRIPT_DIR/lib

MINGW_SDL2_URL="https://www.libsdl.org/release/SDL2-devel-2.0.9-mingw.tar.gz"

rm -rf $SDL2_INCDIR $SDL2_LIBDIR $DOWNLOAD_DIR
mkdir -p $SDL2_INCDIR $SDL2_LIBDIR $DOWNLOAD_DIR

cd $DOWNLOAD_DIR
curl -L $MINGW_SDL2_URL -o SDL2.tar.gz
tar xzf SDL2.tar.gz

cd SDL2-*
cp i686-w64-mingw32/include/SDL2/*.h $SDL2_INCDIR
cp i686-w64-mingw32/lib/libSDL2.a $SDL2_LIBDIR/libSDL2_windows_386.a
cp i686-w64-mingw32/lib/libSDL2main.a $SDL2_LIBDIR/libSDL2main_windows_386.a
cp x86_64-w64-mingw32/lib/libSDL2.a $SDL2_LIBDIR/libSDL2_windows_amd64.a
cp x86_64-w64-mingw32/lib/libSDL2main.a $SDL2_LIBDIR/libSDL2main_windows_amd64.a

rm -rf $DOWNLOAD_DIR
