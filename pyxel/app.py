import math
import time
import pygame
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

        self._quit = False
        self._key_state = {}
        self._mouse_x = 0
        self._mouse_y = 0

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

        # initialize window
        pygame.init()
        pygame.display.set_mode(self._get_window_size(),
                                pygame.OPENGL | pygame.DOUBLEBUF)

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

    @property
    def frame_count(self):
        return self._frame_count

    @property
    def mouse_x(self):
        return self._mouse_x

    @property
    def mouse_y(self):
        return self._mouse_y

    def btn(self, key):
        return self._key_state.get(key, 0) > 0

    def btnp(self, key, hold=0, period=0):
        press_frame = self._key_state.get(key, 0)

        return (press_frame == self._frame_count
                or press_frame > 0 and period > 0 and
                (self._frame_count - press_frame - hold) % period == 0)

    def run(self):
        while True:
            self._update()
            self._draw()

            if self._quit:
                break

        pygame.quit()

    def update(self):
        pass

    def draw(self):
        pass

    def _get_window_size(self):
        return (self._width * self._pixel_scale + self._border_width,
                self._height * self._pixel_scale + self._border_width)

    def _set_pixel_scale(self, pixel_scale):
        self._pixel_scale = max(pixel_scale, 1)
        self._window.set_size(*self._get_window_size())

    def _update(self):
        time.sleep(0.001)

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

            self._frame_count += 1
            self._process_event()
            self._control()

            self.update()

            self._last_updated_time += self._one_frame_time

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

    def _process_event(self):
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                self._quit = True

            elif event.type == pygame.KEYDOWN:
                self._key_state[event.key] = self._frame_count

            elif event.type == pygame.KEYUP:
                self._key_state[event.key] = -self._frame_count

            elif event.type == pygame.MOUSEBUTTONDOWN:
                self._key_state[0x10000 + event.button] = self._frame_count

            elif event.type == pygame.MOUSEBUTTONUP:
                self._key_state[0x10000 + event.button] = -self._frame_count

            elif event.type == pygame.MOUSEMOTION:
                self._mouse_x = event.pos[0] // self._pixel_scale
                self._mouse_y = event.pos[1] // self._pixel_scale

    def _control(self):
        if self.btn(pygame.K_LALT) or self.btn(pygame.K_RALT):
            if self.btnp(pygame.K_UP):
                self._set_pixel_scale(self._pixel_scale + 1)

            if self.btnp(pygame.K_DOWN):
                self._set_pixel_scale(self._pixel_scale - 1)

            if self.btnp(pygame.K_RETURN):
                pygame.display.set_mode(
                    self._get_window_size(),
                    pygame.OPENGL | pygame.DOUBLEBUF | pygame.FULLSCREEN)
                self._renderer = Renderer(self._width, self._height)

            if self.btnp(pygame.K_p):
                self._perf_monitor = not self._perf_monitor

        if self.btnp(pygame.K_ESCAPE):
            self._quit = True

    def _draw(self):
        self.draw()

        surface = pygame.display.get_surface()
        surface_width, surface_height = surface.get_size()
        scale_x = surface_width // self._width
        scale_y = surface_height // self._height
        scale = min(scale_x, scale_y)

        width = self._width * scale
        height = self._height * scale
        left = (surface_width - width) // 2
        bottom = (surface_height - height) // 2

        self._renderer.render(left, bottom, width, height, self._palette,
                              self._clear_color)

        pygame.display.flip()
