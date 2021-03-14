import unittest

import palette

suite "Palette":
  setup:
    var palette = newPalette[8, uint8]()

  test "ColorCount":
    check(palette.ColorCount == 8)

  test "ColorType":
    var color: palette.ColorType
    check(color is uint8)

  test "getDisplayColor":
    for i in 0 ..< palette.ColorCount:
      check(palette.getDisplayColor(i) == 0)

    expect AssertionDefect:
      discard palette.getDisplayColor(-1)

    expect AssertionDefect:
      discard palette.getDisplayColor(8)

  test "setDisplayColor":
    check(palette.getDisplayColor(0) == 0)
    palette.setDisplayColor(0, 0x112233)
    check(palette.getDisplayColor(0) == 0x112233)

    check(palette.getDisplayColor(7) == 0)
    palette.setDisplayColor(7, 0x445566)
    check(palette.getDisplayColor(7) == 0x445566)

    expect AssertionDefect:
      palette.setDisplayColor(-1, 0)

    expect AssertionDefect:
      palette.setDisplayColor(8, 0)

  test "getReplaceColor":
    for i in 0 ..< palette.ColorCount:
      check(palette.getReplaceColor(i) == uint8(i))

    expect AssertionDefect:
      discard palette.getReplaceColor(-1)

    expect AssertionDefect:
      discard palette.getReplaceColor(8)

  test "setReplaceColor":
    check(palette.getReplaceColor(0) == 0)
    palette.setReplaceColor(0, 1)
    check(palette.getReplaceColor(0) == 1)

    check(palette.getReplaceColor(7) == 7)
    palette.setReplaceColor(7, 2)
    check(palette.getReplaceColor(7) == 2)

    expect AssertionDefect:
      palette.setReplaceColor(-1, 0)

    expect AssertionDefect:
      palette.setReplaceColor(8, 0)

    expect AssertionDefect:
      palette.setReplaceColor(0, -1)

    expect AssertionDefect:
      palette.setReplaceColor(0, 8)

  test "resetReplaceColor":
    for i in 0 ..< palette.ColorCount:
      palette.setReplaceColor(i, 0)
      check(palette.getReplaceColor(i) == 0)

    palette.resetReplaceColor()

    for i in 0 ..< palette.ColorCount:
      check(palette.getReplaceColor(i) == uint8(i))
