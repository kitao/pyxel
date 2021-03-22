import unittest

import core/rectarea

suite "RectArea":
  test "default constructor":
    let rect = RectArea()
    check(rect.left == 0 and rect.top == 0)
    check(rect.right == -1 and rect.bottom == -1)
    check(rect.width == 0 and rect.height == 0)

  test "initRectAreaFromPos and properties":
    let rect1 = initRectAreaFromPos(0, 0, 0, 0)
    check(rect1.left == 0 and rect1.top == 0)
    check(rect1.right == 0 and rect1.bottom == 0)
    check(rect1.width == 1 and rect1.height == 1)

    let rect2 = initRectAreaFromPos(1, 2, 30, 40)
    check(rect2.left == 1 and rect2.top == 2)
    check(rect2.right == 30 and rect2.bottom == 40)
    check(rect2.width == 30 and rect2.height == 39)

    var rect3 = initRectAreaFromPos(10, 20, 3, 4)
    check(rect3.left == 3 and rect3.top == 4)
    check(rect3.right == 10 and rect3.bottom == 20)
    check(rect3.width == 8 and rect3.height == 17)

  test "initRectAreaFromSize":
    let rect1 = initRectAreaFromSize(1, 2, 3, 4)
    check(rect1.left == 1 and rect1.top == 2)
    check(rect1.right == 3 and rect1.bottom == 5)
    check(rect1.width == 3 and rect1.height == 4)

    let rect2 = initRectAreaFromSize(10, 20, 0, 40)
    check(rect2.left == 10 and rect2.top == 20)
    check(rect2.right == 9 and rect2.bottom == 59)
    check(rect2.width == 0 and rect2.height == 40)

    let rect3 = initRectAreaFromSize(100, 200, 300, 0)
    check(rect3.left == 100 and rect3.top == 200)
    check(rect3.right == 399 and rect3.bottom == 199)
    check(rect3.width == 300 and rect3.height == 0)

    expect AssertionDefect:
      discard initRectAreaFromSize(1, 2, -1, 4)

    expect AssertionDefect:
      discard initRectAreaFromSize(1, 2, 3, -1)

  test "isEmpty":
    let rect1 = initRectAreaFromSize(1, 2, 3, 4)
    check(not rect1.isEmpty)

    let rect2 = initRectAreaFromSize(1, 2, 0, 4)
    check(rect2.isEmpty)

    let rect3 = initRectAreaFromSize(1, 2, 3, 0)
    check(rect3.isEmpty)

  test "contains":
    let rect1 = initRectAreaFromPos(1, 2, 3, 4)
    check(rect1.contains(1, 2))
    check(rect1.contains(3, 4))
    check(not rect1.contains(0, 2))
    check(not rect1.contains(1, 1))
    check(not rect1.contains(4, 4))
    check(not rect1.contains(3, 5))

    let rect2 = initRectAreaFromSize(1, 2, 0, 4)
    check(not rect2.contains(1, 2))
    check(not rect2.contains(1, 4))

    let rect3 = initRectAreaFromSize(1, 2, 3, 0)
    check(not rect3.contains(1, 2))
    check(not rect3.contains(3, 2))

  test "intersects":
    let rect1 = initRectAreaFromSize(10, 20, 30, 40)
    let rect2 = initRectAreaFromSize(11, 22, 300, 400)
    let rect3 = initRectAreaFromSize(5, 6, 10, 20)
    let rect4 = initRectAreaFromSize(1, 2, 3, 4)
    let rect5 = initRectAreaFromSize(0, 0, 0, 0)

    check(rect1.intersects(rect2) == initRectAreaFromPos(11, 22, 39, 59))
    check(rect1.intersects(rect3) == initRectAreaFromPos(10, 20, 14, 25))
    check(rect1.intersects(rect4).isEmpty)
    check(rect1.intersects(rect4).isEmpty)
    check(rect1.intersects(rect5).isEmpty)
