#
# Preparation
#
when defined(windows):
  const
    OsBinDir = "/windows"
    PyLibExt = ".pyd"
elif defined(macosx):
  const
    OsBinDir = "/macos"
    PyLibExt = ".so"
elif defined(linux):
  const
    OsBinDir = "/linux"
    PyLibExt = ".so"
else:
  raise newException(OSError, "Unsupported OS")

#
# Package
#
version       = "0.1.0"
author        = "Takashi Kitao"
description   = "A retro game engine for Python and Nim"
license       = "MIT"
srcDir        = "src"
binDir        = "../pyxel/bin" & OsBinDir

namedBin["pyxelcore/extension"] = "pyxelextension" & PyLibExt
namedBin["pyxelcore/player"] = "pyxelplayer"
namedBin["pyxelcore/editor"] = "pyxeleditor"
namedBin["pyxelcore/packager"] = "pyxelpackager"

#
# Dependencies
#
requires "nim >= 1.4.4"
requires "nimpy"
requires "sdl2"
