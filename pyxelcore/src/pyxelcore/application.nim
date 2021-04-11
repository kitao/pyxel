import sdl2

type
  Application = ref object
    window: WindowsPtr
    render: RendererPtr

proc newApplication(title: string, width, height: int): Application =
  if not sdl2.wasInit(INIT_EVERYTHING):
    sdl2.init(INIT_EVERYTHING)

  new(result)

  result.window = createWindow(
    title = title,
    x = SDL_WINDOWPOS_CENTERED,
    y = SDL_WINDOWPOS_CENTERED,
    w = width,
    h = height,
    flags = SDL_WINDOW_HIDDEN
  )

  result.render = createRenderer(
    window = window,
    index = -1,
    flags = Renderer_Accelerated or Renderer_PresentVsync
  )


proc terminate(self: Application) =
  destroy(self.render)
  destroy(self.window)

#[
proc run(self: Application) =
  var
    evt = sdl2.defaultEvent
    runGame = true

  while runGame:
    while pollEvent(evt):
      if evt.kind == QuitEvent:
        runGame = false
        break

      render.setDrawColor(0, 0, 0, 255)
      render.clear()
      render.present()
]#
