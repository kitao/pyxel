import
  sequtils,
  sugar,
  rectarea,
  palette

type
  GraphicsBuffer*[T] = ref object
    width: int
    height: int
    colorCount: int
    data: seq[T]
    dataRowIndices: seq[int]
    selfRect: RectArea
    clipRect: RectArea
    palette: Palette[T]

proc newGraphicsBuffer*[T](width, height, colorCount: int,
                           palette: Palette[T]): GraphicsBuffer[T] =
  doAssert(width > 0 and height > 0, "Invalid size")
  doAssert(colorCount > 0, "Invalid colorCount")
  doAssert(palette == nil or palette.colorCount == colorCount, "Invalid palette")

  new(result)
  result.width = width
  result.height = height
  result.colorCount = colorCount
  result.data = newSeq[T](width * height)
  result.dataRowIndices = toSeq(0 ..< height).map(x => x * width)
  result.selfRect = initRectAreaFromSize(0, 0, width, height)
  result.clilpRect = result.selfRect
  result.palette = palette

proc width*(self: GraphicsBuffer): int {.inline.} = self.width
proc height*(self: GraphicsBuffer): int {.inline.} = self.height
proc colorCount*(self: GraphicsBuffer): int {.inline.} = self.colorCount
proc palette*(self: GraphicsBuffer): Palette {.inline.} = self.palette

proc getDataIndex(self: GraphicsBuffer, x, y: int): int {.inline.} =
  self.dataRowIndices[y] + x

proc getDrawColor(self: GraphicsBuffer, color: int): self.T {.inline.} =
  doAssert(color >= 0 and color <= self.colorCount)

  if self.palette != nil:
    result = self.T(self.palette[color])
  else:
    result = self.T(color)

proc getClippingArea*(self: GraphicsBuffer): (int, int, int, int) =
  (self.clipRect.left, self.clipRect.top,
   self.clipRect.width, self.clipRect.height)

proc setClippingArea*(self: GraphicsBuffer, x, y, width, height: int) =
  self.clipRect = initRectAreaFromSize(x, y, width, height).intersects(self.selfRect)

proc clear*(self: GraphicsBuffer, color: int) =
  let col = self.getDrawColor(color)

  for i in 0 ..< self.data.len:
    self.data[i] = col

proc getPixel(self: GraphicsBuffer, x, y: int): self.T =
  if self.selfRect.contains(x, y):
    result = self.data[self.getDataIndex(x, y)]
  else:
    result = 0

proc setPixel(self: GraphicsBuffer, x, y, color: int) =
  if self.clipRect.containts(x, y):
    self.data[self.getDataIndex(x, y)] = self.getDrawColor(color)

proc drawLine(self: GraphicsBuffer, x1, y1, x2, y2, color: int) =
  let col = self.getDrawColor(color)

  discard

proc drawRectangle(self: GraphicsBuffer, x, y, width, height, color: int) =
  let col = self.getDrawColor(color)

  discard

proc drawRectangleBorder(self: GraphicsBuffer, x, y, width, height, color: int) =
  let col = self.getDrawColor(color)

  discard

proc drawCircle(self: GraphicsBuffer, x, y, radius, color: int) =
  let col = self.getDrawColor(color)

  discard

proc drawCircleBorder(self: GraphicsBuffer, x, y, r, color: int) =
  let col = self.getDrawColor(color)

  discard

proc drawTriangle(self: GraphicsBuffer, x1, y1, x2, y2, x3, y3, color: int) =
  let col = self.getDrawColor(color)

  discard

proc drawTriangleBorder(self: GraphicsBuffer, x1, y1, x2, y2, x3, y3, color: int) =
  let col = self.getDrawColor(color)

  discard

proc drawGraphicsBuffer(self: GraphicsBuffer, x, y: int, gbuf: GraphicsBuffer,
                        u, v, width, height, colorKey: int = -1) =
  discard

#proc bltm(self: Canvas, x, y: int, img: Canvas, u, v, w, h, colkey: int) =
#  discard

#proc text(self: Canvas, x, y: int, s: string, col: int) =
#  discard


#[
  struct CopyArea {
    int32_t u;
    int32_t v;
    int32_t x;
    int32_t y;
    int32_t width;
    int32_t height;

    bool IsEmpty() { return width == 0 || height == 0; }
  };

inline Rectangle::CopyArea Rectangle::GetCopyArea(int32_t x,
  int32_t y,
  const Rectangle& src,
  int32_t u,
  int32_t v,
  int32_t width,
  int32_t height,
  bool flip_x,
  bool flip_y) const {
int32_t left_cut = Max(src.left_ - u, left_ - x, 0);
int32_t right_cut =
  Max(u + width - 1 - src.right_, x + width - 1 - right_, 0);
int32_t top_cut = Max(src.top_ - v, top_ - y, 0);
int32_t bottom_cut =
  Max(v + height - 1 - src.bottom_, y + height - 1 - bottom_, 0);

CopyArea copy_area = {
  u + (flip_x ? right_cut : left_cut),
  v + (flip_y ? bottom_cut : top_cut),
  x + left_cut,
  y + top_cut,
  Max(width - left_cut - right_cut, 0),
  Max(height - top_cut - bottom_cut, 0),
};

return copy_area;
}

]#
