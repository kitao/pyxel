import math
import time
import pyglet
from pyglet.window import Window
from pyglet.window import key as pyglet_key
from pyglet.window import mouse as pyglet_mouse
from .renderer import Renderer

PALETTE = [
    0x000000, 0x1d2b53, 0x7e2553, 0x008751, 0xab5236, 0x5f574f, 0xc2c3c7,
    0xfff1e8, 0xff004d, 0xffa300, 0xffec27, 0x00e436, 0x29adff, 0x83769c,
    0xff77a8, 0xffccaa
]

PIXEL_SCALE = 4
BORDER_WIDTH = 0
CLEAR_COLOR = 0x101018
FPS = 30

PERF_SAMPLE_COUNT = 10

pyglet_key.MOUSE_LEFT = 0x20001
pyglet_key.MOUSE_MIDDLE = 0x20002
pyglet_key.MOUSE_RIGHT = 0x20003


class App:
    def __init__(self,
                 width,
                 height,
                 *,
                 pixel_scale=PIXEL_SCALE,
                 border_width=BORDER_WIDTH,
                 clear_color=CLEAR_COLOR,
                 palette=PALETTE,
                 fps=FPS):
        self._width = width
        self._height = height
        self._pixel_scale = pixel_scale
        self._border_width = border_width
        self._clear_color = clear_color
        self._palette = palette[:]
        self._fps = fps
        self._frame_count = 0
        self._one_frame_time = 1 / fps
        self._last_updated_time = time.time() - self._one_frame_time
        self._perf_update_time = 0
        self._perf_update_count = 0
        self._perf_fps_time = time.time()
        self._perf_fps_count = 0
        self._perf_monitor = False
        self._cur_fps = 0
        self._cur_update_time = 0
        self._mouse_x = 0
        self._mouse_y = 0

        # initialize window
        self._window = Window(*self._get_window_size())
        self._window.on_draw = self._on_draw
        self._window.on_key_press = self._on_key_press
        self._window.on_mouse_motion = self._on_mouse_motion
        self._window.on_mouse_press = self._on_mouse_press
        self._window.on_mouse_release = self._on_mouse_release

        self._key_state = pyglet_key.KeyStateHandler()
        self._key_hold_time = {}
        self._window.push_handlers(self._key_state)

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
        pyglet.clock.schedule(self._on_update)

    def btn(self, key):
        return self._key_hold_time.get(key, 0) > 0

    def btnp(self, key, hold=0, period=0):
        t = self._key_hold_time.get(key, 0) - (hold + 1)
        return t == 0 or t > 0 and period > 0 and t % period == 0

    @property
    def frame_count(self):
        return self._frame_count

    @property
    def mouse_x(self):
        return self._mouse_x

    @property
    def mouse_y(self):
        return self._mouse_y

    @staticmethod
    def run():
        pyglet.app.run()

    def update(self):
        pass

    def _get_window_size(self):
        return (self._width * self._pixel_scale + self._border_width,
                self._height * self._pixel_scale + self._border_width)

    def _set_pixel_scale(self, pixel_scale):
        self._pixel_scale = max(pixel_scale, 1)
        self._window.set_size(*self._get_window_size())

    def _update_key_state(self):
        for k, v in self._key_state.items():
            if not v:
                self._key_hold_time[k] = 0
            elif k in self._key_hold_time:
                self._key_hold_time[k] += 1
            else:
                self._key_hold_time[k] = 1

    def _on_update(self, dt):
        cur_time = time.time()
        elapsed_time = cur_time - self._last_updated_time
        update_count = math.floor(elapsed_time / self._one_frame_time)

        # measure fps
        if update_count > 0:
            self._perf_fps_count += 1

            if self._perf_fps_count >= PERF_SAMPLE_COUNT:
                self._cur_fps = round(
                    self._perf_fps_count / (cur_time - self._perf_fps_time), 2)
                self._perf_fps_count = 0
                self._perf_fps_time = cur_time

        # update frame
        for _ in range(update_count):
            start_time = time.time()

            self._update_key_state()
            self.update()
            self._last_updated_time += self._one_frame_time
            self._frame_count += 1

            # measure update time
            self._perf_update_count += 1
            self._perf_update_time += time.time() - start_time

            if self._perf_update_count >= PERF_SAMPLE_COUNT:
                self._cur_update_time = round(
                    self._perf_update_time / self._perf_update_count * 1000, 2)
                self._perf_update_time = 0
                self._perf_update_count = 0

            if self._perf_monitor:
                fps = 'fps:{}'.format(self._cur_fps)
                update = 'update:{}'.format(self._cur_update_time)

                self.text(1, 0, fps, 1)
                self.text(0, 0, fps, 9)
                self.text(1, 6, update, 1)
                self.text(0, 6, update, 9)

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
                              self._clear_color)

    def _on_key_press(self, key, modifiers):
        if modifiers & pyglet_key.MOD_ALT or modifiers & pyglet_key.MOD_OPTION:
            if key == pyglet_key.UP:
                self._set_pixel_scale(self._pixel_scale + 1)

            if key == pyglet_key.DOWN:
                self._set_pixel_scale(self._pixel_scale - 1)

            if key == pyglet_key.ENTER:
                self._window.set_fullscreen(not self._window.fullscreen)

            if key == pyglet_key.P:
                self._perf_monitor = not self._perf_monitor

        if key == pyglet_key.ESCAPE:
            exit()

    def _on_mouse_motion(self, x, y, dx, dy):
        self._mouse_x = x // self._pixel_scale
        self._mouse_y = self._height - y // self._pixel_scale - 1

    def _on_mouse_press(self, x, y, button, modifiers):
        if button & pyglet_mouse.LEFT:
            self._key_state[pyglet_key.MOUSE_LEFT] = True

        if button & pyglet_mouse.MIDDLE:
            self._key_state[pyglet_key.MOUSE_MIDDLE] = True

        if button & pyglet_mouse.RIGHT:
            self._key_state[pyglet_key.MOUSE_RIGHT] = True

    def _on_mouse_release(self, x, y, button, modifiers):
        if button & pyglet_mouse.LEFT:
            self._key_state[pyglet_key.MOUSE_LEFT] = False

        if button & pyglet_mouse.MIDDLE:
            self._key_state[pyglet_key.MOUSE_MIDDLE] = False

        if button & pyglet_mouse.RIGHT:
            self._key_state[pyglet_key.MOUSE_RIGHT] = False
