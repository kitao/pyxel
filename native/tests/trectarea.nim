import unittest

import options, core/rectarea

suite "RectArea":
  test "initRectAreaFromPos and properties":
    let rect1 = initRectAreaFromPos(0, 0, 0, 0)
    assert(rect1.left == 0 and rect1.top == 0)
    assert(rect1.right == 0 and rect1.bottom == 0)
    assert(rect1.width == 1 and rect1.height == 1)

    let rect2 = initRectAreaFromPos(1, 2, 30, 40)
    assert(rect2.left == 1 and rect2.top == 2)
    assert(rect2.right == 30 and rect2.bottom == 40)
    assert(rect2.width == 30 and rect2.height == 39)

    var rect3 = initRectAreaFromPos(10, 20, 3, 4)
    assert(rect3.left == 3 and rect3.top == 4)
    assert(rect3.right == 10 and rect3.bottom == 20)
    assert(rect3.width == 8 and rect3.height == 17)

  test "initRectAreaFromSize":
    let rect1 = initRectAreaFromSize(1, 2, 3, 4)
    assert(rect1.left == 1 and rect1.top == 2)
    assert(rect1.right == 3 and rect1.bottom == 5)
    assert(rect1.width == 3 and rect1.height == 4)

    expect AssertionDefect:
      discard initRectAreaFromSize(1, 2, 0, 4)

    expect AssertionDefect:
      discard initRectAreaFromSize(1, 2, 3, 0)

  test "contains":
    let rect = initRectAreaFromPos(1, 2, 3, 4)
    assert(rect.contains(1, 2))
    assert(rect.contains(3, 4))
    assert(not rect.contains(0, 2))
    assert(not rect.contains(1, 1))
    assert(not rect.contains(4, 4))
    assert(not rect.contains(3, 5))

  test "intersects":
    let rect1 = initRectAreaFromSize(10, 20, 30, 40)
    let rect2 = initRectAreaFromSize(11, 22, 300, 400)
    let rect3 = initRectAreaFromSize(5, 6, 10, 20)
    let rect4 = initRectAreaFromSize(1, 2, 3, 4)

    assert(rect1.intersects(rect2).get() == initRectAreaFromPos(11, 22, 39, 59))
    assert(rect1.intersects(rect3).get() == initRectAreaFromPos(10, 20, 14, 25))
    assert(rect1.intersects(rect4).isNone)
