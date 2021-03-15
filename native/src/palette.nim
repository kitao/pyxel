type
  Rgb24* = uint32

  Palette*[ColorCount: static[int], ColorType] = ref object
    displayColors: array[ColorCount, Rgb24]
    replaceColors: array[ColorCount, ColorType]

proc newPalette*[ColorCount, ColorType](): Palette[ColorCount, ColorType] =
  new(result)
  result.resetReplaceColor()

proc getDisplayColor*(self: Palette, color: int): Rgb24 =
  assert(color >= 0 and color < self.ColorCount, "Invalid color")
  result = self.displayColors[color]

proc setDisplayColor*(self: var Palette, color: int, rgb: Rgb24) =
  assert(color >= 0 and color < self.ColorCount, "Invalid color")
  self.displayColors[color] = rgb

proc setDisplayColors*[ColorCount: static[int], ColorType](self: var Palette[
    ColorCount, ColorType], colors: array[ColorCount, int]) =
  for i, color in colors:
    self.displayColors[i] = Rgb24(color)

proc getReplaceColor*(self: Palette, color: int): int =
  assert(color >= 0 and color < self.ColorCount, "Invalid color")
  result = int(self.replaceColors[color])

proc setReplaceColor*(self: Palette, srcColor: int, dstColor: int) =
  assert(srcColor >= 0 and srcColor < self.ColorCount, "Invalid src color")
  assert(dstColor >= 0 and dstColor < self.ColorCount, "Invalid dst color")
  self.replaceColors[srcColor] = self.ColorType(dstColor)

proc resetReplaceColor*(self: var Palette) =
  for i in 0 ..< self.ColorCount:
    self.replaceColors[i] = self.ColorType(i)
