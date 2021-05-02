import
  sdl2,
  sdl2/image,

  settings

type
  System* = ref object
    screenWidth: int
    screenHeight: int
    windowCaption: string
    colorPalette: array[COLOR_COUNT, int]
    targetFps: int
    quitKey: int

    frameCount: int
    dropFiles: seq[string]
    isLoopRunning: bool
    isQuitRequested: bool
    isUpdateSuspended: bool
    isPerformanceMonitorOn: bool

    sdlWindow: WindowPtr
    sdlRenderer: RendererPtr

proc newSystem*(screenWidth, screenHeight: int, windowCaption: string,
    colorPalette: array[COLOR_COUNT, int], targetFps, quitKey: int): System =
  new(result)

  result.screenWidth = screenWidth
  result.screenHeight = screenHeight
  result.windowCaption = windowCaption
  result.colorPalette = colorPalette
  result.targetFps = targetFps
  result.quitKey = quitKey

  result.frameCount = 0
  result.dropFiles = @[]

  if sdl2.wasInit(INIT_EVERYTHING) != 0:
    if sdl2.init(INIT_VIDEO or INIT_AUDIO or INIT_GAMECONTROLLER):
      discard
    # sdl2.init(INIT_EVERYTHING)

  if sdl2.init(IMG_INIT_PNG):
    discard

  doAssert(screenWidth > 0 and screenHeight <= MAX_SCREEN_SIZE and
      screenHeight > 0 and screenHeight <= MAX_SCREEN_SIZE, "invalid screen size")
  doAssert(targetFps > 0, "invalid target fps")

  result.isLoopRunning = false
  result.isQuitRequested = false
  result.isUpdateSuspended = false
  result.isPerformanceMonitorOn = false

  result.sdlWindow = createWindow(
    title = cstring(windowCaption),
    x = SDL_WINDOWPOS_CENTERED,
    y = SDL_WINDOWPOS_CENTERED,
    w = cint(screenWidth),
    h = cint(screenHeight),
    flags = 0 # SDL_WINDOW_HIDDEN
  )

  result.sdlRenderer = createRenderer(
    window = result.sdlWindow,
    index = -1,
    flags = Renderer_Accelerated or Renderer_PresentVsync
  )

proc screenWidth*(self: System): int {.inline.} = self.screenWidth

proc screenHeight*(self: System): int {.inline.} = self.screenHeight

proc frameCount*(self: System): int {.inline.} = self.frameCount

proc dropFiles*(self: System): seq[string] {.inline.} = self.dropFiles

proc windowCaption*(self: System): string {.inline.} = self.windowCaption

proc `windowCaption=`*(self: System, windowCaption: string) {.inline.} =
  self.windowCaption = windowCaption

proc runApplication*(self: System, update: proc, draw: proc) =
  var
    event = sdl2.defaultEvent
    running = true

  while running:
    while bool(pollEvent(event)):
      if event.kind == QuitEvent:
        running = false
        break

      self.sdlRenderer.setDrawColor(0, 0, 0, 255)
      self.sdlRenderer.clear()
      self.sdlRenderer.present()

proc quitApplication*(self: System) =
  discard

proc flipScreen*(self: System) =
  discard

proc showScreen*(self: System) =
  discard
