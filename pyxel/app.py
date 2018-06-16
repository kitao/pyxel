import math
import time
import pyglet
from pyglet.window import Window, key
from .renderer import Renderer

PALETTE = [
    0x000000, 0x1d2b53, 0x7e2553, 0x008751, 0xab5236, 0x5f574f, 0xc2c3c7,
    0xfff1e8, 0xff004d, 0xffa300, 0xffec27, 0x00e436, 0x29adff, 0x83769c,
    0xff77a8, 0xffccaa
]

BG_COLOR = 0x101018
BORDER_WIDTH = 0
FPS = 30


class App:
    def __init__(self,
                 width,
                 height,
                 scale,
                 *,
                 palette=PALETTE,
                 bg_color=BG_COLOR,
                 border_width=BORDER_WIDTH,
                 fps=FPS):
        self._width = width
        self._height = height
        self._scale = scale
        self._palette = palette[:]
        self._bg_color = bg_color
        self._border_width = border_width
        self._fps = fps
        self._one_frame_time = 1 / fps
        self._last_updated_time = time.time() - self._one_frame_time

        self.mouse_x = 0
        self.mouse_y = 0

        # initialize window
        self._window = Window(*self._window_size())
        self._window.on_key_press = self._on_key_press
        self._window.on_mouse_motion = self._on_mouse_motion
        self._window.on_draw = self._on_draw

        # initialize renderer
        self._renderer = Renderer(width, height)
        self.bank = self._renderer.bank
        self.clip = self._renderer.clip
        self.pal = self._renderer.pal
        self.cls = self._renderer.cls
        self.pix = self._renderer.pix
        self.line = self._renderer.line
        self.rect = self._renderer.rect
        self.rectb = self._renderer.rectb
        self.circ = self._renderer.circ
        self.circb = self._renderer.circb
        self.blt = self._renderer.blt
        self.text = self._renderer.text

        # start updating regulary
        pyglet.clock.set_fps_limit(fps)
        pyglet.clock.schedule(self._on_update)

    def _window_size(self):
        return (self._width * self._scale + self._border_width,
                self._height * self._scale + self._border_width)

    def _set_scale(self, scale):
        self._scale = max(scale, 1)
        self._window.set_size(*self._window_size())

    def _on_update(self, dt):
        elapsed_time = time.time() - self._last_updated_time
        update_count = math.floor(elapsed_time / self._one_frame_time)

        for _ in range(update_count):
            self.update()
            self._last_updated_time += self._one_frame_time

    def _on_draw(self):
        viewport_width, viewport_height = self._window.get_viewport_size()
        scale_x = viewport_width // self._width
        scale_y = viewport_height // self._height
        scale = min(scale_x, scale_y)

        width = self._width * scale
        height = self._height * scale
        left = (viewport_width - width) // 2
        bottom = (viewport_height - height) // 2

        self._renderer.render(left, bottom, width, height, self._palette,
                              self._bg_color)

    def _on_key_press(self, key_, modifiers):
        alt_or_opt = (modifiers & key.MOD_ALT or modifiers & key.MOD_OPTION)

        if key_ == key.UP and alt_or_opt:
            self._set_scale(self._scale + 1)

        if key_ == key.DOWN and alt_or_opt:
            self._set_scale(self._scale - 1)

        if key_ == key.ENTER and alt_or_opt:
            self.fullscreen = not self.fullscreen

        if key_ == key.ESCAPE:
            exit()

        self.key_press(key_, modifiers)

    def _on_mouse_motion(self, x, y, dx, dy):
        self.mouse_x = x // self._scale
        self.mouse_y = self._height - y // self._scale - 1

    def update(self):
        pass

    def key_press(self, key, mod):
        pass

    @staticmethod
    def run():
        pyglet.app.run()
