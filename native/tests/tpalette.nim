import unittest

import palette

suite "Palette":
  setup:
    var palette = newPalette[3, uint8]()

  test "Constants":
    check(Rgb24 is uint32)
    check(palette.ColorCount == 3)

    var a: palette.ColorType
    check(a is uint8)

  test "getDisplayColor":
    for i in 0 ..< palette.ColorCount:
      check(palette.getDisplayColor(i) == 0)

    expect AssertionDefect:
      discard palette.getDisplayColor(-1)

    expect AssertionDefect:
      discard palette.getDisplayColor(3)

  test "setDisplayColor":
    check(palette.getDisplayColor(0) == 0)
    palette.setDisplayColor(0, 0x112233)
    check(palette.getDisplayColor(0) == 0x112233)

    check(palette.getDisplayColor(2) == 0)
    palette.setDisplayColor(2, 0x445566)
    check(palette.getDisplayColor(2) == 0x445566)

    expect AssertionDefect:
      palette.setDisplayColor(-1, 0)

    expect AssertionDefect:
      palette.setDisplayColor(3, 0)

  test "setDisplayColors":
    palette.setDisplayColors([0x111111, 0x222222, 0x333333])
    check(palette.getDisplayColor(0) == 0x111111)
    check(palette.getDisplayColor(1) == 0x222222)
    check(palette.getDisplayColor(2) == 0x333333)

  test "getReplaceColor":
    for i in 0 ..< palette.ColorCount:
      check(palette.getReplaceColor(i) == i)

    expect AssertionDefect:
      discard palette.getReplaceColor(-1)

    expect AssertionDefect:
      discard palette.getReplaceColor(8)

  test "setReplaceColor":
    check(palette.getReplaceColor(0) == 0)
    palette.setReplaceColor(0, 2)
    check(palette.getReplaceColor(0) == 2)

    check(palette.getReplaceColor(2) == 2)
    palette.setReplaceColor(2, 0)
    check(palette.getReplaceColor(2) == 0)

    expect AssertionDefect:
      palette.setReplaceColor(-1, 0)

    expect AssertionDefect:
      palette.setReplaceColor(3, 0)

    expect AssertionDefect:
      palette.setReplaceColor(0, -1)

    expect AssertionDefect:
      palette.setReplaceColor(0, 3)

  test "resetReplaceColor":
    for i in 0 ..< palette.ColorCount:
      palette.setReplaceColor(i, 0)
      check(palette.getReplaceColor(i) == 0)

    palette.resetReplaceColor()

    for i in 0 ..< palette.ColorCount:
      check(palette.getReplaceColor(i) == i)
