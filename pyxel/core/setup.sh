#!/bin/bash
#
# Advance Preparation
#
# Mac:
#   brew install python3 sdl2 sdl2_image clang-format gifsicle
#   (reboot terminal)
#   pip3 install -U black flake8 isort mypy twine
#
# Linux:
#   sudo apt update
#   sudo apt dist-upgrade
#   sudo apt autoremove
#   sudo apt install python3 python3-pip libsdl2-dev libsdl2-image-dev
#
# Windows:
#   choco upgrade all -y
#   choco install -y msys2
#
#   [32bit]
#   pacman -Syu
#   pacman -S make mingw-w64-i686-toolchain
#
#   [64bit]
#   pacman -Syu
#   pacman -S make mingw-w64-x86_64-toolchain
#

SCRIPT_DIR=$(cd $(dirname $0);pwd)
DOWNLOAD_DIR=$SCRIPT_DIR/download
INCDIR=$SCRIPT_DIR/include/SDL2
LIBDIR=$SCRIPT_DIR/lib
BINDIR=$SCRIPT_DIR/bin

MINGW_SDL2_URL="https://www.libsdl.org/release/SDL2-devel-2.0.12-mingw.tar.gz"
MINGW_SDL2_IMAGE_URL="https://www.libsdl.org/projects/SDL_image/release/SDL2_image-devel-2.0.5-mingw.tar.gz"

GIFSICLE_WIN32_URL="https://eternallybored.org/misc/gifsicle/releases/gifsicle-1.92-win32.zip"
GIFSICLE_WIN64_URL="https://eternallybored.org/misc/gifsicle/releases/gifsicle-1.92-win64.zip"

rm -rf $INCDIR $LIBDIR $SDL_BINDIR $DOWNLOAD_DIR
mkdir -p $INCDIR $LIBDIR/win{32,64} $BINDIR/win{32,64} $DOWNLOAD_DIR

cd $DOWNLOAD_DIR
curl -L $MINGW_SDL2_URL -o SDL2.tar.gz
tar xzf SDL2.tar.gz
cd SDL2-*
cp i686-w64-mingw32/include/SDL2/*.h $INCDIR
cp i686-w64-mingw32/lib/libSDL2{.dll,main}.a $LIBDIR/win32
cp i686-w64-mingw32/bin/SDL2.dll $BINDIR/win32
cp x86_64-w64-mingw32/lib/libSDL2{.dll,main}.a $LIBDIR/win64
cp x86_64-w64-mingw32/bin/SDL2.dll $BINDIR/win64

cd $DOWNLOAD_DIR
curl -L $MINGW_SDL2_IMAGE_URL -o SDL2_image.tar.gz
tar xzf SDL2_image.tar.gz
cd SDL2_image-*
cp i686-w64-mingw32/include/SDL2/*.h $INCDIR
cp i686-w64-mingw32/lib/libSDL2_image.dll.a $LIBDIR/win32
cp i686-w64-mingw32/bin/*.dll $BINDIR/win32
cp x86_64-w64-mingw32/lib/libSDL2_image.dll.a $LIBDIR/win64
cp x86_64-w64-mingw32/bin/*.dll $BINDIR/win64

cd $DOWNLOAD_DIR
curl -L $GIFSICLE_WIN32_URL -o gifsicle-win32.zip
curl -L $GIFSICLE_WIN64_URL -o gifsicle-win64.zip
unzip -q gifsicle-win32.zip -d gifsicle-win32
unzip -q gifsicle-win64.zip -d gifsicle-win64
cp gifsicle-win32/gifsicle-*/gifsicle.exe $BINDIR/win32
cp gifsicle-win64/gifsicle-*/gifsicle.exe $BINDIR/win64

rm -rf $DOWNLOAD_DIR
