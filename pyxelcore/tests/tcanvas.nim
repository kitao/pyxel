import
  unittest,
  pyxelcore/graphicsbuffer

#[
suite "GraphicsBuffer":
  setup:
    var gbuf1 = newGraphicsBuffer[uint8](10, 20, 3, true)
    var gbuf2 = newGraphicsBuffer[uint16](30, 40, 5000, false)

  test "newGraphicsBuffer":
    expect AssertionDefect:
      discard newGraphicsBuffer[uint8](0, 1, 2, true)

    expect AssertionDefect:
      discard newGraphicsBuffer[uint8](1, 0, 2, true)

    expect AssertionDefect:
      discard newGraphicsBuffer[uint8](1, 2, 0, true)

  test "properties":
    assert(gbuf1.width == 10 and gbuf1.height == 20)
    assert(gbuf1.colorCount == 3)
    assert(gbuf1.hasPalette)

    assert(gbuf2.width == 30 and gbuf2.height == 40)
    assert(gbuf2.colorCount == 5000)
    assert(not gbuf2.hasPalette)

  test "getPalette and setPalette":
    discard
]#
