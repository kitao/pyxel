type Pyxel = ref object
  # System
  width, height: int
  frameCount: int
  fps: int

  # Resource

let pyxel* = new(Pyxel)

include pyxelcore/system
include pyxelcore/resource
include pyxelcore/input
include pyxelcore/graphics
include pyxelcore/audio
include pyxelcore/image
include pyxelcore/tilemap
include pyxelcore/sound
include pyxelcore/music
