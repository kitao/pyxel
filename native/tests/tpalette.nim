import
  unittest,
  core/palette

suite "Palette":
  test "types":
    check(Rgb is uint32)

  test "properties":
    let pal = newPalette[uint8](3)

    check(pal.colorCount == 3)

  test "[] and []=":
    let pal = newPalette[uint8](3)

    for i in 0 ..< pal.colorCount:
      check(pal[i] == i)

    for i in 0 ..< pal.colorCount:
      pal[i] = 2
      check(pal[i] == 2)

  test "reset":
    let pal = newPalette[uint8](3)

    for i in 0 ..< pal.colorCount:
      pal[i] = 0

    pal.reset()

    for i in 0 ..< pal.colorCount:
      check(pal[i] == i)

  test "getDisplayColor":
    let pal = newPalette[uint8](3)

    for i in 0 ..< pal.colorCount:
      check(pal.getDisplayColor(i) == 0)

    expect AssertionDefect:
      discard pal.getDisplayColor(-1)

    expect AssertionDefect:
      discard pal.getDisplayColor(3)

  test "setDisplayColor":
    let pal = newPalette[uint8](3)

    check(pal.getDisplayColor(0) == 0)
    pal.setDisplayColor(0, 0x112233)
    check(pal.getDisplayColor(0) == 0x112233)

    check(pal.getDisplayColor(2) == 0)
    pal.setDisplayColor(2, 0x445566)
    check(pal.getDisplayColor(2) == 0x445566)

    expect AssertionDefect:
      pal.setDisplayColor(-1, 0)

    expect AssertionDefect:
      pal.setDisplayColor(3, 0)

  test "setDisplayColors":
    let pal = newPalette[uint8](3)

    pal.setDisplayColors([0x111111, 0x222222, 0x333333])
    check(pal.getDisplayColor(0) == 0x111111)
    check(pal.getDisplayColor(1) == 0x222222)
    check(pal.getDisplayColor(2) == 0x333333)
