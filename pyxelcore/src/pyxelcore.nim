import pyxelcore/system

type
  Pyxel = ref object
    system: System

let pyxel* = new(Pyxel)

#
# System
#
proc width*(self: Pyxel): int {.inline.} = self.system.screenWidth
proc height*(self: Pyxel): int {.inline.} = self.system.screenHeight
proc frameCount*(self: Pyxel): int {.inline.} = self.system.frameCount
proc dropfile(self: Pyxel): seq[string] {.inline.} = self.system.dropFiles
proc caption(self: Pyxel): string {.inline.} = self.system.windowCaption
proc `caption=`(self: Pyxel, caption: string) {.inline.} =
  self.system.windowCaption = caption

#
# Resource
#

#
# Input
#

#
# Graphics
#

#
# Audio
#


#[
include pyxelcore/resource
include pyxelcore/input
include pyxelcore/graphics
include pyxelcore/audio
include pyxelcore/image
include pyxelcore/tilemap
include pyxelcore/sound
include pyxelcore/music
]#
