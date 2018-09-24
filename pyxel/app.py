import datetime
import gzip
import inspect
import math
import os
import pickle
import platform
import subprocess
import time

import glfw
import numpy as np
import PIL.Image

from .audio_player import AudioPlayer
from .constants import (
    APP_GIF_CAPTURE_COUNT,
    APP_GIF_CAPTURE_SCALE,
    APP_MEASURE_FRAME_COUNT,
    APP_SCREEN_MAX_SIZE,
    APP_SCREEN_SCALE_CUTDOWN,
    APP_SCREEN_SCALE_MINIMUM,
    DEFAULT_PALETTE,
    GLFW_VERSION,
    ICON_DATA,
    KEY_0,
    KEY_1,
    KEY_2,
    KEY_3,
    KEY_ALT,
    KEY_CONTROL,
    KEY_LEFT_ALT,
    KEY_LEFT_BUTTON,
    KEY_LEFT_CONTROL,
    KEY_LEFT_SHIFT,
    KEY_LEFT_SUPER,
    KEY_MIDDLE_BUTTON,
    KEY_RIGHT_ALT,
    KEY_RIGHT_BUTTON,
    KEY_RIGHT_CONTROL,
    KEY_RIGHT_SHIFT,
    KEY_RIGHT_SUPER,
    KEY_SHIFT,
    KEY_SUPER,
)
from .renderer import Renderer


class App:
    def __init__(
        self,
        module,
        width,
        height,
        caption,
        scale,
        palette,
        fps,
        border_width,
        border_color,
    ):
        if glfw.get_version() < tuple(map(int, GLFW_VERSION.split("."))):
            raise RuntimeError("glfw version is lower than {}".format(GLFW_VERSION))

        if width > APP_SCREEN_MAX_SIZE or height > APP_SCREEN_MAX_SIZE:
            raise ValueError(
                "screen size is larger than {}x{}".format(
                    APP_SCREEN_MAX_SIZE, APP_SCREEN_MAX_SIZE
                )
            )

        self._module = module
        self._palette = palette[:]
        self._pil_palette = self._get_pil_palette(palette)
        self._fps = fps
        self._border_width = border_width
        self._border_color = border_color
        self._next_update_time = 0
        self._one_frame_time = 1 / fps
        self._key_state = {}
        self._update = None
        self._draw = None
        self._capture_start = 0
        self._capture_index = 0
        self._capture_images = [None] * APP_GIF_CAPTURE_COUNT

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

        monitor = glfw.get_primary_monitor()
        display_width, display_height = glfw.get_video_mode(monitor)[0]

        if scale == 0:
            scale = max(
                min(
                    (display_width // width) - APP_SCREEN_SCALE_CUTDOWN,
                    (display_height // height) - APP_SCREEN_SCALE_CUTDOWN,
                ),
                APP_SCREEN_SCALE_MINIMUM,
            )

        window_width = width * scale + border_width
        window_height = height * scale + border_width
        self._window = glfw.create_window(
            window_width, window_height, caption, None, None
        )

        if not self._window:
            glfw.terminate()
            exit()

        glfw.set_window_pos(
            self._window,
            (display_width - window_width) // 2,
            (display_height - window_height) // 2,
        )

        glfw.make_context_current(self._window)
        glfw.set_window_size_limits(
            self._window, width, height, glfw.DONT_CARE, glfw.DONT_CARE
        )
        self._hidpi_scale = (
            glfw.get_framebuffer_size(self._window)[0]
            / glfw.get_window_size(self._window)[0]
        )
        self._update_viewport()

        glfw.set_key_callback(self._window, self._key_callback)
        glfw.set_cursor_pos_callback(self._window, self._cursor_pos_callback)
        glfw.set_mouse_button_callback(self._window, self._mouse_button_callback)

        glfw.set_window_icon(self._window, 1, [self._get_icon_image()])

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
        module.save = self.save
        module.load = self.load
        module.image = self._renderer.image
        module.clip = self._renderer.draw_command.clip
        module.pal = self._renderer.draw_command.pal
        module.cls = self._renderer.draw_command.cls
        module.pix = self._renderer.draw_command.pix
        module.line = self._renderer.draw_command.line
        module.rect = self._renderer.draw_command.rect
        module.rectb = self._renderer.draw_command.rectb
        module.circ = self._renderer.draw_command.circ
        module.circb = self._renderer.draw_command.circb
        module.blt = self._renderer.draw_command.blt
        module.text = self._renderer.draw_command.text
        module.sound = self._audio_player.sound
        module.play = self._audio_player.play
        module.stop = self._audio_player.stop

    def btn(self, key):
        press_frame = self._key_state.get(key, 0)
        return press_frame > 0 or press_frame == -self._module.frame_count - 1

    def btnp(self, key, hold=0, period=0):
        press_frame = self._key_state.get(key, 0)
        hold_frame = self._module.frame_count - press_frame - hold
        return (
            press_frame == self._module.frame_count
            or press_frame == -self._module.frame_count - 1
            or press_frame > 0
            and period > 0
            and hold_frame >= 0
            and hold_frame % period == 0
        )

    def btnr(self, key):
        return self._key_state.get(key, 0) == -self._module.frame_count

    def run(self, update, draw):
        self._update = update
        self._draw = draw

        self._module.frame_count = 1
        self._next_update_time = self._perf_fps_start_time = time.time()

        def main_loop():
            while not glfw.window_should_close(self._window):
                glfw.poll_events()

                self._measure_fps()
                self._update_viewport()
                self._update_frame()
                self._draw_frame()

                glfw.swap_buffers(self._window)
            glfw.terminate()

        if self._audio_player.output_stream:
            with self._audio_player.output_stream:
                main_loop()
        else:
            main_loop()

    def quit(self):
        glfw.set_window_should_close(self._window, True)

    def save(self, filename):
        data = {"version": self._module.VERSION}

        image = [self._module.image(i).data.dumps() for i in range(3)]
        data["image"] = image

        pickled_data = pickle.dumps(data)

        with gzip.open(filename, mode="wb") as fp:
            fp.write(pickled_data)

    def load(self, filename):
        with gzip.open(filename, mode="rb") as fp:
            pickled_data = fp.read()

        data = pickle.loads(pickled_data)

        image = data["image"]
        for i in range(3):
            self._module.image(i).data[:, :] = np.loads(image[i])

    def palettize_pil_image(self, pil_image):
        im = pil_image.im.convert("P", 0, self._pil_palette.im)
        return pil_image._new(im)

    @staticmethod
    def _get_icon_image():
        width = len(ICON_DATA[0])
        height = len(ICON_DATA)
        color_list = list(map(lambda x: int(x, 16), "".join(ICON_DATA)))

        image = []
        for color in color_list:
            rgb = DEFAULT_PALETTE[color]
            image.append((rgb >> 16) & 0xff)
            image.append((rgb >> 8) & 0xff)
            image.append(rgb & 0xff)

        icon = PIL.Image.frombuffer(
            "RGB", (width, height), bytes(image), "raw", "RGB", 0, 1
        ).convert("RGBA")

        pixels = icon.load()
        for x in range(width):
            for y in range(height):
                r, g, b, a = pixels[x, y]
                if (r, g, b) == (0, 0, 0):
                    pixels[x, y] = (0, 0, 0, 0)

        return icon

    def _key_callback(self, window, key, scancode, action, mods):
        if action == glfw.PRESS:
            state = self._module.frame_count
        elif action == glfw.RELEASE:
            if self._key_state[key] == self._module.frame_count:
                state = -self._module.frame_count - 1
            else:
                state = -self._module.frame_count
        else:
            return

        self._key_state[key] = state

        if key == KEY_LEFT_SHIFT or key == KEY_RIGHT_SHIFT:
            self._key_state[KEY_SHIFT] = state
        elif key == KEY_LEFT_CONTROL or key == KEY_RIGHT_CONTROL:
            self._key_state[KEY_CONTROL] = state
        elif key == KEY_LEFT_ALT or key == KEY_RIGHT_ALT:
            self._key_state[KEY_ALT] = state
        elif key == KEY_LEFT_SUPER or key == KEY_RIGHT_SUPER:
            self._key_state[KEY_SUPER] = state

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
            if self._key_state[button] == self._module.frame_count:
                self._key_state[button] = -self._module.frame_count - 1
            else:
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
        self._viewport_bottom = win_height - self._viewport_height - self._viewport_top

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
        image = self._renderer.render(
            self._viewport_left * hs,
            self._viewport_bottom * hs,
            self._viewport_width * hs,
            self._viewport_height * hs,
            self._palette,
            self._border_color,
        )
        self._capture_images[self._capture_index % APP_GIF_CAPTURE_COUNT] = image
        self._capture_index += 1

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
        if self.btn(KEY_ALT):
            if self.btnp(glfw.KEY_ENTER):
                self._toggle_fullscreen()

            if self.btnp(KEY_0):
                self._perf_monitor_is_enabled = not self._perf_monitor_is_enabled

            if self.btnp(KEY_1):
                self._save_capture_image()

            if self.btnp(KEY_2):
                self._capture_start = self._capture_index - 1

            if self.btnp(KEY_3):
                self._save_capture_animation()

        if self.btnp(glfw.KEY_ESCAPE):
            self.quit()

    def _save_capture_image(self):
        index = (self._capture_index - 1) % APP_GIF_CAPTURE_COUNT
        image = self._get_capture_image(index)
        image.save(self._get_capture_filename() + ".png")

    def _save_capture_animation(self):
        image_count = min(
            self._capture_index - self._capture_start, APP_GIF_CAPTURE_COUNT
        )

        if image_count <= 0:
            return

        start_index = self._capture_index - image_count
        images = []

        for i in range(image_count):
            index = (start_index + i) % APP_GIF_CAPTURE_COUNT
            images.append(self._get_capture_image(index))

        images[0].save(
            self._get_capture_filename() + ".gif",
            save_all=True,
            append_images=images[1:],
            duration=self._one_frame_time * 1000,
            loop=0,
            optimize=True,
        )

    def _get_capture_image(self, index):
        image = PIL.Image.frombuffer(
            "RGB",
            (self._module.width, self._module.height),
            self._capture_images[index],
            "raw",
            "RGB",
            0,
            1,
        )

        image = self.palettize_pil_image(image)

        image = image.resize(
            (
                self._module.width * APP_GIF_CAPTURE_SCALE,
                self._module.height * APP_GIF_CAPTURE_SCALE,
            )
        )

        return image

    @staticmethod
    def _get_pil_palette(palette):
        rgb_palette = []

        for color in palette:
            r = (color >> 16) & 0xff
            g = (color >> 8) & 0xff
            b = color & 0xff
            rgb_palette.extend((r, g, b))

        rgb_palette += [0] * 240 * 3

        pil_palette = PIL.Image.new("P", (1, 1), 0)
        pil_palette.putpalette(rgb_palette)

        return pil_palette

    @staticmethod
    def _get_capture_filename():
        plat = platform.system()

        if plat == "Windows":
            path = os.path.join(os.path.join(os.environ["USERPROFILE"]), "Desktop")
        elif plat == "Darwin":
            path = os.path.join(os.path.join(os.path.expanduser("~")), "Desktop")
        else:
            path = os.path.join(os.path.join(os.path.expanduser("~")), "Desktop")
            if not os.path.exists(path):
                try:
                    path = (
                        subprocess.check_output(["xdg-user-dir DESKTOP"], shell=True)
                        .decode("utf-8")
                        .split("\n")[0]
                    )
                    if not os.path.exists(path):
                        raise OSError
                except (subprocess.CalledProcessError, OSError):
                    path = os.path.expanduser("~")

        return os.path.join(
            path, datetime.datetime.now().strftime("pyxel-%y%m%d-%H%M%S")
        )

    def _measure_fps(self):
        cur_time = time.time()
        self._perf_fps_count += 1

        if self._perf_fps_count == APP_MEASURE_FRAME_COUNT:
            self._perf_fps = self._perf_fps_count / (
                cur_time - self._perf_fps_start_time
            )
            self._perf_fps_count = 0
            self._perf_fps_start_time = cur_time

    def _measure_update_time(self, update_start_time):
        self._perf_update_count += 1
        self._perf_update_total_time += time.time() - update_start_time

        if self._perf_update_count == APP_MEASURE_FRAME_COUNT:
            self._perf_update_time = (
                self._perf_update_total_time / self._perf_update_count
            ) * 1000
            self._perf_update_total_time = 0
            self._perf_update_count = 0

    def _measure_draw_time(self, draw_start_time):
        self._perf_draw_count += 1
        self._perf_draw_total_time += time.time() - draw_start_time

        if self._perf_draw_count == APP_MEASURE_FRAME_COUNT:
            self._perf_draw_time = (
                self._perf_draw_total_time / self._perf_draw_count
            ) * 1000
            self._perf_draw_total_time = 0
            self._perf_draw_count = 0

    def _draw_perf_monitor(self):
        if not self._perf_monitor_is_enabled:
            return

        fps = "{:.2f}".format(self._perf_fps)
        update = "{:.2f}".format(self._perf_update_time)
        draw = "{:.2f}".format(self._perf_draw_time)

        text = self._renderer.draw_command.text
        text(1, 0, fps, 1)
        text(0, 0, fps, 9)
        text(1, 6, update, 1)
        text(0, 6, update, 9)
        text(1, 12, draw, 1)
        text(0, 12, draw, 9)
