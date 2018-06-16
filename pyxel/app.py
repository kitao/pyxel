import math
import time
import pyglet
from .renderer import Renderer

PALETTE = [
    0x000000, 0x1d2b53, 0x7e2553, 0x008751, 0xab5236, 0x5f574f, 0xc2c3c7,
    0xfff1e8, 0xff004d, 0xffa300, 0xffec27, 0x00e436, 0x29adff, 0x83769c,
    0xff77a8, 0xffccaa
]

BG_COLOR = 0x101018
BORDER_WIDTH = 0
FPS = 30


class Window(pyglet.window.Window):
    def __init__(self, app):
        window_width = app._width * app._scale + app._border_width
        window_height = app._height * app._scale + app._border_width
        super().__init__(window_width, window_height)

        self.app = app
        self.renderer = Renderer(app._width, app._height)
        self.one_frame_time = 1 / app._fps
        self.last_updated_time = time.time() - self.one_frame_time

        app.bank = self.renderer.bank
        app.clip = self.renderer.clip
        app.pal = self.renderer.pal
        app.cls = self.renderer.cls
        app.pix = self.renderer.pix
        app.line = self.renderer.line
        app.rect = self.renderer.rect
        app.rectb = self.renderer.rectb
        app.circ = self.renderer.circ
        app.circb = self.renderer.circb
        app.blt = self.renderer.blt
        app.text = self.renderer.text

        pyglet.clock.set_fps_limit(self.app._fps)
        pyglet.clock.schedule(self.update)

    def update(self, _):
        elapsed_time = time.time() - self.last_updated_time
        update_count = math.floor(elapsed_time / self.one_frame_time)

        for _ in range(update_count):
            self.app.update()
            self.last_updated_time += self.one_frame_time

    def on_draw(self):
        window_width, window_height = self.get_viewport_size()
        scale_x = window_width // self.renderer.width
        scale_y = window_height // self.renderer.height
        scale = min(scale_x, scale_y)
        width = self.renderer.width * scale
        height = self.renderer.height * scale
        left = (window_width - width) // 2
        bottom = (window_height - height) // 2

        self.renderer.render(left, bottom, width, height, self.app._palette,
                             self.app._bg_color)

    def on_key_press(self, key, modifiers):
        self.app.key_press(key, modifiers)

    def on_text(self, text):
        self.app.text_input(text)


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
        self._window = Window(self)

    @property
    def scale(self):
        return self._scale

    @scale.setter
    def scale(self, scale):
        self._scale = max(scale, 1)
        window_width = self._width * self._scale + self._border_width
        window_height = self._height * self._scale + self._border_width
        self._window.set_size(window_width, window_height)

    @property
    def fullscreen(self):
        return self._window.fullscreen

    @fullscreen.setter
    def fullscreen(self, fullscreen):
        self._window.set_fullscreen(fullscreen)

    def update(self):
        pass

    def key_press(self, key, mod):
        pass

    def text_input(self, text):
        pass

    @staticmethod
    def run():
        pyglet.app.run()
