type
  Rgb* = uint32

  Palette*[ColorCount: static[uint], ColorType] = ref object
    displayColor: array[ColorCount, Rgb]
    replaceColor: array[ColorCount, ColorType]

proc newPalette*[ColorCount, ColorType](): Palette[ColorCount, ColorType] =
  new(result)
  result.resetReplaceColor()

proc getDisplayColor*(palette: Palette, color: int): Rgb =
  assert(color >= 0 and color < palette.ColorCount, "Invalid color")
  result = palette.displayColor[color]

proc setDisplayColor*(palette: var Palette, color: int, rgb: Rgb) =
  assert(color >= 0 and color < palette.ColorCount, "Invalid color")
  palette.displayColor[color] = rgb

proc setDisplayColor*(palette: var Palette, color: ref array[palette.ColorCount, Rgb]) =
  for i, c in color:
    palette.displayColor[i] = c

proc getReplaceColor*(palette: Palette, color: int): palette.ColorType =
  assert(color >= 0 and color < palette.ColorCount, "Invalid color")
  result = palette.replaceColor[color]

proc setReplaceColor*(palette: Palette, srcColor: int, dstColor: int) =
  assert(srcColor >= 0 and srcColor < palette.ColorCount, "Invalid src color")
  assert(dstColor >= 0 and dstColor < palette.ColorCount, "Invalid dst color")
  palette.replaceColor[srcColor] = palette.ColorType(dstColor)

proc resetReplaceColor*(palette: var Palette) =
  for i in 0 ..< palette.ColorCount:
    palette.replaceColor[i] = palette.ColorType(i)
