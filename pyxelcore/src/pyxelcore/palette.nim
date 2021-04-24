import sequtils

type
  Palette* = ref object
    colorCount: int
    replaceTable: seq[int]
    displayColors: seq[int]

proc newPalette*(colorCount: int): Palette =
  doAssert(colorCount >= 0, "Invalid color count")

  new(result)
  result.colorCount = colorCount
  result.replaceTable = toSeq(0 ..< colorCount)
  result.displayColors = newSeq[int](colorCount)

proc colorCount*(self: Palette): int {.inline.} = self.colorCount

proc `[]`*(self: Palette, index: int): int {.inline.} =
  doAssert(index >= 0 and index < self.colorCount, "Invalid index")

  result = self.replaceTable[index]

proc `[]=`*(self: Palette, index, value: int) {.inline.} =
  doAssert(index >= 0 and index < self.colorCount, "Invalid index")
  doAssert(value >= 0 and value < self.colorCount, "Invalid value")

  self.replaceTable[index] = value

proc reset*(self: Palette) =
  for i in 0 ..< self.colorCount:
    self.replaceTable[i] = i

proc getDisplayColor*(self: Palette, color: int): int =
  doAssert(color >= 0 and color < self.colorCount, "Invalid color")

  result = self.displayColors[color]

proc setDisplayColor*(self: Palette, color: int, rgb: int) =
  doAssert(color >= 0 and color < self.colorCount, "Invalid color")
  doAssert(rgb >= 0x000000 and rgb <= 0xffffff, "Invalid rgb")

  self.displayColors[color] = rgb

proc setDisplayColors*(self: Palette, rgbs: openArray[int]) =
  doAssert(rgbs.len == self.colorCount, "Invalid lengh array")

  for i, rgb in rgbs:
    self.setDisplayColor(i, rgb)
