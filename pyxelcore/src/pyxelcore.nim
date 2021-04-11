type Pyxel = ref object
  width, height: int
  frameCount: int
  fps: int

let pyxel* = new(Pyxel)

include pyxelcore/system
