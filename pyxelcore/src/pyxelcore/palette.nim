import
  sequtils,
  sugar

type
  Rgb* = uint32

  Palette*[T] = ref object
    colorCount: int
    replaceTable: seq[T]
    displayColors: seq[Rgb]

proc newPalette*[T](colorCount: int): Palette[T] =
  doAssert(colorCount >= 0, "Invalid colorCount")

  new(result)
  result.colorCount = colorCount
  result.replaceTable = toSeq(0 ..< colorCount).map(x => T(x))
  result.displayColors = newSeq[uint32](colorCount)

proc colorCount*(self: Palette): int {.inline.} = self.colorCount

proc `[]`*(self: Palette, index: int): int {.inline.} =
  assert(index >= 0 and index < self.colorCount, "Invalid index")

  result = int(self.replaceTable[index])

proc `[]=`*(self: Palette, index: int, value: int) {.inline.} =
  assert(index >= 0 and index < self.colorCount, "Invalid index")
  assert(value >= 0 and value < self.colorCount, "Invalid value")

  self.replaceTable[index] = self.T(value)

proc reset*(self: Palette) =
  for i in 0 ..< self.colorCount:
    self.replaceTable[i] = self.T(i)

proc getDisplayColor*(self: Palette, color: int): Rgb =
  assert(color >= 0 and color < self.colorCount, "Invalid color")

  result = self.displayColors[color]

proc setDisplayColor*(self: Palette, color: int, rgb: Rgb) =
  assert(color >= 0 and color < self.colorCount, "Invalid color")

  self.displayColors[color] = rgb

proc setDisplayColors*(self: Palette, rgbs: openArray[int]) =
  doAssert(rgbs.len == self.colorCount, "Invalid lengh array")

  for i, rgb in rgbs:
    self.displayColors[i] = Rgb(rgb)
