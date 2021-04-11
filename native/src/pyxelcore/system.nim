proc width(self: Pyxel): int = self.width

proc height(self: Pyxel): int = self.height

proc frameCount*(self: Pyxel): int = self.frameCount

proc dropfile(self: Pyxel): string = "test"

proc caption(self: Pyxel): string = "test"

proc `caption=`(self: Pyxel, caption: string) =
  self.caption = caption

proc init(self: Pyxel, width, height: int, caption: string, scale: int,
    palette: int, fps, quit_key: int, fullscreen: bool) =
  self.fps = fps
  self.frameCount = 0

proc run(self: Pyxel, update: proc, draw: proc) =
  discard

proc quit(self: Pyxel) =
  discard

proc flip(self: Pyxel) =
  discard

proc show(self: Pyxel) =
  discard
