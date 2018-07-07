import math
import time
import glfw
from .renderer import Renderer
from .audioplayer import AudioPlayer
from .key import KEY_LEFT_BUTTON, KEY_MIDDLE_BUTTON, KEY_RIGHT_BUTTON

PERF_MEASURE_COUNT = 10


class App:
    def __init__(self, module, width, height, caption, scale, palette, fps,
                 border_width, border_color):
        self._module = module
        self._palette = palette[:]
        self._fps = fps
        self._border_width = border_width
        self._border_color = border_color
        self._next_update_time = 0
        self._one_frame_time = 1 / fps
        self._key_state = {}
        self._update = None
        self._draw = None

        self._perf_monitor_is_enabled = False
        self._perf_fps_count = 0
        self._perf_fps_start_time = 0
        self._perf_fps = 0
        self._perf_update_count = 0
        self._perf_update_total_time = 0
        self._perf_update_time = 0
        self._perf_draw_count = 0
        self._perf_draw_total_time = 0
        self._perf_draw_time = 0

        module.width = width
        module.height = height
        module.mouse_x = 0
        module.mouse_y = 0
        module.frame_count = 0

        # initialize window
        if not glfw.init():
            exit()

        window_width = width * scale + border_width
        window_height = height * scale + border_width
        self._window = glfw.create_window(window_width, window_height, caption,
                                          None, None)

        if not self._window:
            glfw.terminate()
            exit()

        glfw.make_context_current(self._window)
        glfw.set_window_size_limits(self._window, width, height,
                                    glfw.DONT_CARE, glfw.DONT_CARE)
        self._hidpi_scale = (glfw.get_framebuffer_size(self._window)[0] /
                             glfw.get_window_size(self._window)[0])
        self._update_viewport()

        glfw.set_key_callback(self._window, self._key_callback)
        glfw.set_cursor_pos_callback(self._window, self._cursor_pos_callback)
        glfw.set_mouse_button_callback(self._window,
                                       self._mouse_button_callback)

        # initialize renderer
        self._renderer = Renderer(width, height)

        # initialize audio player
        self._audio_player = AudioPlayer()

        # export module functions
        module.btn = self.btn
        module.btnp = self.btnp
        module.btnr = self.btnr
        module.run = self.run
        module.quit = self.quit
        module.bank = self._renderer.bank
        module.clip = self._renderer.clip
        module.pal = self._renderer.pal
        module.cls = self._renderer.cls
        module.pix = self._renderer.pix
        module.line = self._renderer.line
        module.rect = self._renderer.rect
        module.rectb = self._renderer.rectb
        module.circ = self._renderer.circ
        module.circb = self._renderer.circb
        module.blt = self._renderer.blt
        module.text = self._renderer.text
        module.play = self._audio_player.play

    def btn(self, key):
        return self._key_state.get(key, 0) > 0

    def btnp(self, key, hold=0, period=0):
        press_frame = self._key_state.get(key, 0)

        return (press_frame == self._module.frame_count
                or press_frame > 0 and period > 0 and
                (self._module.frame_count - press_frame - hold) % period == 0)

    def btnr(self, key):
        return self._key_state.get(key, 0) == -self.module._frame_count

    def run(self, update, draw):
        self._update = update
        self._draw = draw

        self._module.frame_count = 1
        self._next_update_time = self._perf_fps_start_time = time.time()

        with self._audio_player.output_stream:
            while not glfw.window_should_close(self._window):
                glfw.poll_events()

                self._measure_fps()
                self._update_viewport()
                self._update_frame()
                self._draw_frame()

                glfw.swap_buffers(self._window)

            glfw.terminate()

    def quit(self):
        glfw.set_window_should_close(self._window, True)

    def _key_callback(self, window, key, scancode, action, mods):
        if action == glfw.PRESS:
            self._key_state[key] = self._module.frame_count
        elif action == glfw.RELEASE:
            self._key_state[key] = -self._module.frame_count

    def _cursor_pos_callback(self, window, xpos, ypos):
        left = self._viewport_left
        top = self._viewport_top
        scale = self._viewport_scale

        self._module.mouse_x = int((xpos - left) / scale)
        self._module.mouse_y = int((ypos - top) / scale)

    def _mouse_button_callback(self, window, button, action, mods):
        if button == glfw.MOUSE_BUTTON_LEFT:
            button = KEY_LEFT_BUTTON
        elif button == glfw.MOUSE_BUTTON_MIDDLE:
            button = KEY_MIDDLE_BUTTON
        elif button == glfw.MOUSE_BUTTON_RIGHT:
            button = KEY_RIGHT_BUTTON
        else:
            return

        if action == glfw.PRESS:
            self._key_state[button] = self._module.frame_count
        elif action == glfw.RELEASE:
            self._key_state[button] = -self._module.frame_count

    def _update_viewport(self):
        win_width, win_height = glfw.get_window_size(self._window)
        scale_x = win_width // self._module.width
        scale_y = win_height // self._module.height
        scale = min(scale_x, scale_y)

        self._viewport_scale = scale
        self._viewport_width = self._module.width * scale
        self._viewport_height = self._module.height * scale
        self._viewport_left = (win_width - self._viewport_width) // 2
        self._viewport_top = (win_height - self._viewport_height) // 2
        self._viewport_bottom = (
            win_height - self._viewport_height - self._viewport_top)

    def _update_frame(self):
        # wait for update time
        while True:
            cur_time = time.time()
            wait_time = self._next_update_time - cur_time

            if wait_time > 0:
                time.sleep(wait_time)
            else:
                break

        update_count = math.floor(-wait_time / self._one_frame_time) + 1
        self._next_update_time += update_count * self._one_frame_time

        # update frame
        for _ in range(update_count):
            update_start_time = time.time()
            self._check_special_input()

            self._update()

            self._module.frame_count += 1
            self._measure_update_time(update_start_time)

    def _draw_frame(self):
        draw_start_time = time.time()

        self._draw()

        self._draw_perf_monitor()

        hs = self._hidpi_scale
        self._renderer.render(
            self._viewport_left * hs, self._viewport_bottom * hs,
            self._viewport_width * hs, self._viewport_height * hs,
            self._palette, self._border_color)

        self._measure_draw_time(draw_start_time)

    def _toggle_fullscreen(self):
        if glfw.get_window_monitor(self._window):  # fullscreen to window
            glfw.set_window_monitor(self._window, None, *self._window_info, 0)
        else:  # window to fullscreen
            info = [0] * 4
            info[0], info[1] = glfw.get_window_pos(self._window)
            info[2], info[3] = glfw.get_window_size(self._window)
            self._window_info = info

            monitor = glfw.get_primary_monitor()
            size = glfw.get_video_mode(monitor)[0]
            glfw.set_window_monitor(self._window, monitor, 0, 0, *size, 0)

    def _check_special_input(self):
        if self.btn(glfw.KEY_LEFT_ALT) or self.btn(glfw.KEY_RIGHT_ALT):
            if self.btnp(glfw.KEY_F):
                self._toggle_fullscreen()

            if self.btnp(glfw.KEY_P):
                self._perf_monitor_is_enabled = (
                    not self._perf_monitor_is_enabled)

        if self.btnp(glfw.KEY_ESCAPE):
            self.quit()

    def _measure_fps(self):
        cur_time = time.time()
        self._perf_fps_count += 1

        if self._perf_fps_count == PERF_MEASURE_COUNT:
            self._perf_fps = self._perf_fps_count / (
                cur_time - self._perf_fps_start_time)
            self._perf_fps_count = 0
            self._perf_fps_start_time = cur_time

    def _measure_update_time(self, update_start_time):
        self._perf_update_count += 1
        self._perf_update_total_time += time.time() - update_start_time

        if self._perf_update_count == PERF_MEASURE_COUNT:
            self._perf_update_time = (
                self._perf_update_total_time / self._perf_update_count) * 1000
            self._perf_update_total_time = 0
            self._perf_update_count = 0

    def _measure_draw_time(self, draw_start_time):
        self._perf_draw_count += 1
        self._perf_draw_total_time += time.time() - draw_start_time

        if self._perf_draw_count == PERF_MEASURE_COUNT:
            self._perf_draw_time = (
                self._perf_draw_total_time / self._perf_draw_count) * 1000
            self._perf_draw_total_time = 0
            self._perf_draw_count = 0

    def _draw_perf_monitor(self):
        if not self._perf_monitor_is_enabled:
            return

        fps = '{:.2f}'.format(self._perf_fps)
        update = '{:.2f}'.format(self._perf_update_time)
        draw = '{:.2f}'.format(self._perf_draw_time)

        self._renderer.text(1, 0, fps, 1)
        self._renderer.text(0, 0, fps, 9)
        self._renderer.text(1, 6, update, 1)
        self._renderer.text(0, 6, update, 9)
        self._renderer.text(1, 12, draw, 1)
        self._renderer.text(0, 12, draw, 9)
