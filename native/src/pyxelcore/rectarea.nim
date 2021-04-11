type
  RectArea* = object
    left, top: int
    width, height: int

proc initRectAreaFromPos*(x1, y1, x2, y2: int): RectArea {.inline.} =
  if x1 < x2:
    result.left = x1
    result.width = x2 - x1 + 1
  else:
    result.left = x2
    result.width = x1 - x2 + 1

  if y1 < y2:
    result.top = y1
    result.height = y2 - y1 + 1
  else:
    result.top = y2
    result.height = y1 - y2 + 1

proc initRectAreaFromSize*(left, top, width, height: int): RectArea {.inline.} =
  doAssert(width >= 0 and height >= 0, "Invalide size")

  result.left = left
  result.top = top
  result.width = width
  result.height = height

proc left*(self: RectArea): int {.inline.} = self.left
proc top*(self: RectArea): int {.inline.} = self.top
proc right*(self: RectArea): int {.inline.} = self.left + self.width - 1
proc bottom*(self: RectArea): int {.inline.} = self.top + self.height - 1
proc width*(self: RectArea): int {.inline.} = self.width
proc height*(self: RectArea): int {.inline.} = self.height

proc isEmpty*(self: RectArea): bool {.inline.} =
  self.width <= 0 or self.height <= 0

proc contains*(self: RectArea, x, y: int): bool {.inline.} =
  x >= self.left and x < self.left + self.width and
  y >= self.top and y < self.top + self.height

proc intersects*(self: RectArea, rect: RectArea): RectArea {.inline.} =
  let left = max(self.left, rect.left)
  let top = max(self.top, rect.top)
  let right = min(self.right, rect.right)
  let bottom = min(self.bottom, rect.bottom)
  let width = right - left + 1
  let height = bottom - top + 1

  if width > 0 and height > 0:
    result = RectArea(left: left, top: top, width: width, height: height)
  else:
    result = RectArea(left: 0, top: 0, width: 0, height: 0)
