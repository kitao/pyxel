import
  unittest,
  pyxelcore/system,
  pyxelcore/settings,
  pyxelcore/keycodes

proc update = discard
proc draw = discard

suite "System":
  let system = newSystem(200, 200, "test", DEFAULT_PALETTE, 30, KEY_NONE)
  system.runApplication(update, draw)
