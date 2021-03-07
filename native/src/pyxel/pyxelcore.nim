import nimpy
import ../pyxel

proc greet(name: string): string {.exportpy.} =
  return "Hello, " & name & "!"

proc test_sdl {.exportpy.} =
  pyxel.testSdl()
