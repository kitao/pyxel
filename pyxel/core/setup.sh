#!/bin/bash
#
# Preparation
#
# Mac:
#   brew install sdl2 sdl2_image mingw-w64
#
# Linux:
#   sudo apt install libsdl2-dev libsdl2-image_dev
#

SCRIPT_DIR=$(cd $(dirname $0);pwd)
DOWNLOAD_DIR=$SCRIPT_DIR/temp
SDL2_INCDIR=$SCRIPT_DIR/include/SDL2
SDL2_LIBDIR=$SCRIPT_DIR/lib

MINGW_SDL2_URL="https://www.libsdl.org/release/SDL2-devel-2.0.9-mingw.tar.gz"
MINGW_SDL2_IMAGE_URL="https://www.libsdl.org/projects/SDL_image/release/SDL2_image-devel-2.0.4-mingw.tar.gz"
DARWIN_LIBS_URL="https://github.com/veandco/go-sdl2-libs/raw/master/lib{SDL2_image,jpeg,png,z}_darwin_amd64.a"
WINDOWS_LIBS_URL="https://github.com/veandco/go-sdl2-libs/raw/master/lib{SDL2_image,jpeg,png,z}_windows_{386,amd64}.a"

if [ $(uname) != "Darwin" ]; then
  echo "this script works only on Mac"
  exit 1
fi

rm -rf $SDL2_INCDIR $SDL2_LIBDIR $DOWNLOAD_DIR
mkdir -p $SDL2_INCDIR $SDL2_LIBDIR $DOWNLOAD_DIR

echo -n "installing SDL2 and SDL2_image for Mac ... "
brew reinstall sdl2 sdl2_image
cp /usr/local/lib/libSDL2.a $SDL2_LIBDIR/libSDL2_darwin_amd64.a
cp /usr/local/lib/libSDL2_image.a $SDL2_LIBDIR/libSDL2_image_darwin_amd64.a
echo "done"

echo -n "download SDL2 for Windows ... "
cd $DOWNLOAD_DIR
curl -s -L $MINGW_SDL2_URL -o SDL2.tar.gz
tar xzf SDL2.tar.gz
cd SDL2-*
cp i686-w64-mingw32/include/SDL2/*.h $SDL2_INCDIR
cp i686-w64-mingw32/lib/libSDL2.a $SDL2_LIBDIR/libSDL2_windows_386.a
cp i686-w64-mingw32/lib/libSDL2main.a $SDL2_LIBDIR/libSDL2main_windows_386.a
cp x86_64-w64-mingw32/lib/libSDL2.a $SDL2_LIBDIR/libSDL2_windows_amd64.a
cp x86_64-w64-mingw32/lib/libSDL2main.a $SDL2_LIBDIR/libSDL2main_windows_amd64.a
echo "done"

echo -n "download SDL2_image for Windows ... "
cd $DOWNLOAD_DIR
curl -s -L $MINGW_SDL2_IMAGE_URL -o SDL2_image.tar.gz
tar xzf SDL2_image.tar.gz
cd SDL2_image-*
cp i686-w64-mingw32/include/SDL2/*.h $SDL2_INCDIR
cp i686-w64-mingw32/lib/libSDL2_image.a $SDL2_LIBDIR/libSDL2_image_windows_386.a
cp x86_64-w64-mingw32/lib/libSDL2_image.a $SDL2_LIBDIR/libSDL2_image_windows_amd64.a
echo "done"

echo -n "downloading dependent libraries for Mac and Windows ... "
cd $SDL2_LIBDIR
curl -s -L -O $DARWIN_LIBS_URL
curl -s -L -O $WINDOWS_LIBS_URL
echo "done"

rm -rf $DOWNLOAD_DIR
