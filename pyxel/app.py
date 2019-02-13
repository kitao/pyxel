import datetime
import gzip
import inspect
import math
import os
import pickle
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
    AUDIO_MUSIC_COUNT,
    AUDIO_SOUND_COUNT,
    GIF_TRANSPARENCY_COLOR,
    GLFW_VERSION,
    KEY_0,
    KEY_1,
    KEY_2,
    KEY_3,
    KEY_ALT,
    KEY_CONTROL,
    KEY_LEFT_ALT,
    KEY_LEFT_CONTROL,
    KEY_LEFT_SHIFT,
    KEY_LEFT_SUPER,
    KEY_RIGHT_ALT,
    KEY_RIGHT_CONTROL,
    KEY_RIGHT_SHIFT,
    KEY_RIGHT_SUPER,
    KEY_SHIFT,
    KEY_SUPER,
    MOUSE_CURSOR_DATA,
    MOUSE_CURSOR_HEIGHT,
    MOUSE_CURSOR_IMAGE_X,
    MOUSE_CURSOR_IMAGE_Y,
    MOUSE_CURSOR_WIDTH,
    MOUSE_LEFT_BUTTON,
    MOUSE_MIDDLE_BUTTON,
    MOUSE_RIGHT_BUTTON,
    RENDERER_IMAGE_COUNT,
    RENDERER_TILEMAP_COUNT,
)
from .renderer import Renderer
from .utilities import (
    get_desktop_path,
    get_icon_image,
    get_palette,
    palettize_pil_image,
)


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

        global pyxel
        pyxel = module

        self._palette = palette[:]
        self._fps = fps
        self._border_width = border_width
        self._border_color = border_color
        self._next_update_time = 0
        self._one_frame_time = 1 / fps
        self._key_state = {}
        self._is_mouse_visible = False
        self._update = None
        self._draw = None
        self._capture_start = 0
        self._capture_count = 0
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

        # exports variables
        pyxel._app = self
        pyxel.width = width
        pyxel.height = height
        pyxel.mouse_x = 0
        pyxel.mouse_y = 0
        pyxel.frame_count = 0

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
        glfw.set_mouse_button_callback(self._window, self._mouse_button_callback)

        glfw.set_window_icon(self._window, 1, [get_icon_image()])
        glfw.set_input_mode(self._window, glfw.CURSOR, glfw.CURSOR_HIDDEN)

        # initialize renderer
        self._renderer = Renderer(width, height)

        # initialize audio player
        self._audio_player = AudioPlayer()

        # export module functions
        pyxel.btn = self.btn
        pyxel.btnp = self.btnp
        pyxel.btnr = self.btnr
        pyxel.mouse = self.mouse
        pyxel.run = self.run
        pyxel.run_with_profiler = self.run_with_profiler
        pyxel.quit = self.quit
        pyxel.save = self.save
        pyxel.load = self.load
        pyxel.image = self._renderer.image
        pyxel.tilemap = self._renderer.tilemap
        pyxel.clip = self._renderer.draw_command.clip
        pyxel.pal = self._renderer.draw_command.pal
        pyxel.cls = self._renderer.draw_command.cls
        pyxel.pix = self._renderer.draw_command.pix
        pyxel.line = self._renderer.draw_command.line
        pyxel.rect = self._renderer.draw_command.rect
        pyxel.rectb = self._renderer.draw_command.rectb
        pyxel.circ = self._renderer.draw_command.circ
        pyxel.circb = self._renderer.draw_command.circb
        pyxel.blt = self._renderer.draw_command.blt
        pyxel.bltm = self._renderer.draw_command.bltm
        pyxel.text = self._renderer.draw_command.text
        pyxel.sound = self._audio_player.sound
        pyxel.music = self._audio_player.music
        pyxel.play = self._audio_player.play
        pyxel.playm = self._audio_player.playm
        pyxel.stop = self._audio_player.stop

        # initialize mouse cursor
        pyxel.image(3, system=True).set(
            MOUSE_CURSOR_IMAGE_X, MOUSE_CURSOR_IMAGE_Y, MOUSE_CURSOR_DATA
        )

    def btn(self, key):
        press_frame = self._key_state.get(key, None)
        return (
            press_frame is not None
            and press_frame >= 0
            or press_frame == -(pyxel.frame_count + 2)
        )

    def btnp(self, key, hold=0, period=0):
        press_frame = self._key_state.get(key, None)

        if press_frame == pyxel.frame_count or press_frame == -(pyxel.frame_count + 2):
            return True

        if press_frame is None or press_frame < 0 or period <= 0:
            return False

        hold_frame = pyxel.frame_count - press_frame - hold
        return hold_frame >= 0 and hold_frame % period == 0

    def btnr(self, key):
        return self._key_state.get(key, None) == -(pyxel.frame_count + 1)

    def mouse(self, visible):
        self._is_mouse_visible = visible

    def run(self, update, draw):
        self._update = update
        self._draw = draw

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

    def run_with_profiler(self, update, draw):
        import cProfile
        import pstats

        profile = cProfile.Profile()
        profile.enable()
        profile.runcall(self.run, update, draw)
        profile.disable()

        stats = pstats.Stats(profile).strip_dirs().sort_stats("tottime")
        frame_count = pyxel.frame_count

        for key in stats.stats:
            cc, nc, tt, ct, callers = stats.stats[key]
            cc = round(cc / frame_count, 3)
            nc = round(nc / frame_count, 3)
            tt *= 1000 / frame_count
            ct *= 1000 / frame_count
            stats.stats[key] = (cc, nc, tt, ct, callers)

        stats.print_stats(30)

    def quit(self):
        glfw.set_window_should_close(self._window, True)

    @staticmethod
    def save(filename):
        data = {"version": pyxel.VERSION}

        image_list = [
            pyxel.image(i).data.dumps() for i in range(RENDERER_IMAGE_COUNT - 1)
        ]
        data["image"] = image_list

        tilemap_list = [
            (pyxel.tilemap(i).data.dumps(), pyxel.tilemap(i).refimg)
            for i in range(RENDERER_TILEMAP_COUNT)
        ]
        data["tilemap"] = tilemap_list

        sound_list = [pyxel.sound(i) for i in range(AUDIO_SOUND_COUNT - 1)]
        data["sound"] = sound_list

        music_list = [pyxel.music(i) for i in range(AUDIO_MUSIC_COUNT - 1)]
        data["music"] = music_list

        pickled_data = pickle.dumps(data)

        dirname = os.path.dirname(inspect.stack()[-1].filename)
        filename = os.path.join(dirname, filename)

        with gzip.open(filename, mode="wb") as fp:
            fp.write(pickled_data)

    @staticmethod
    def load(filename):
        dirname = os.path.dirname(inspect.stack()[-1].filename)
        filename = os.path.join(dirname, filename)

        with gzip.open(filename, mode="rb") as fp:
            pickled_data = fp.read()

        data = pickle.loads(pickled_data)

        # todo: version check

        image_list = data.get("image")
        if image_list:
            for i in range(RENDERER_IMAGE_COUNT - 1):
                pyxel.image(i).data[:, :] = pickle.loads(image_list[i])

        tilemap_list = data.get("tilemap")
        if tilemap_list:
            if type(tilemap_list[0]) is tuple:
                for i in range(RENDERER_TILEMAP_COUNT):
                    tilemap = pyxel.tilemap(i)
                    tilemap.data[:, :] = pickle.loads(tilemap_list[i][0])
                    tilemap.refimg = tilemap_list[i][1]
            else:  # todo: delete this block in the future
                for i in range(RENDERER_TILEMAP_COUNT):
                    pyxel.tilemap(i).data[:, :] = pickle.loads(tilemap_list[i])

        sound_list = data.get("sound")
        if sound_list:
            for i in range(AUDIO_SOUND_COUNT - 1):
                src = sound_list[i]
                dest = pyxel.sound(i)

                dest.note[:] = src.note
                dest.tone[:] = src.tone
                dest.volume[:] = src.volume
                dest.effect[:] = src.effect
                dest.speed = src.speed

        music_list = data.get("music")
        if music_list:
            for i in range(AUDIO_MUSIC_COUNT - 1):
                src = music_list[i]
                dest = pyxel.music(i)

                dest.ch0[:] = src.ch0
                dest.ch1[:] = src.ch1
                dest.ch2[:] = src.ch2
                dest.ch3[:] = src.ch3

    def _set_key_state(self, key, action):
        if action == glfw.PRESS:
            if self._key_state.get(key, -1) < 0:
                self._key_state[key] = pyxel.frame_count
        elif action == glfw.RELEASE:
            state = self._key_state.get(key, -1)
            if state == pyxel.frame_count:
                self._key_state[key] = -(pyxel.frame_count + 2)
            elif state >= 0:
                self._key_state[key] = -(pyxel.frame_count + 1)

    def _key_callback(self, window, key, scancode, action, mods):
        if action != glfw.PRESS and action != glfw.RELEASE:
            return

        self._set_key_state(key, action)
        state = self._key_state[key]

        if key == KEY_LEFT_SHIFT or key == KEY_RIGHT_SHIFT:
            self._key_state[KEY_SHIFT] = state
        elif key == KEY_LEFT_CONTROL or key == KEY_RIGHT_CONTROL:
            self._key_state[KEY_CONTROL] = state
        elif key == KEY_LEFT_ALT or key == KEY_RIGHT_ALT:
            self._key_state[KEY_ALT] = state
        elif key == KEY_LEFT_SUPER or key == KEY_RIGHT_SUPER:
            self._key_state[KEY_SUPER] = state

    def _mouse_button_callback(self, window, button, action, mods):
        if button == glfw.MOUSE_BUTTON_LEFT:
            button = MOUSE_LEFT_BUTTON
        elif button == glfw.MOUSE_BUTTON_MIDDLE:
            button = MOUSE_MIDDLE_BUTTON
        elif button == glfw.MOUSE_BUTTON_RIGHT:
            button = MOUSE_RIGHT_BUTTON
        else:
            return

        self._set_key_state(button, action)

    def _update_viewport(self):
        win_width, win_height = glfw.get_window_size(self._window)
        scale_x = win_width // pyxel.width
        scale_y = win_height // pyxel.height
        scale = min(scale_x, scale_y)

        self._viewport_scale = scale
        self._viewport_width = pyxel.width * scale
        self._viewport_height = pyxel.height * scale
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

        self._update_mouse_pos()
        self._update_gamepad()

        # update frame
        for _ in range(update_count):
            update_start_time = time.time()
            self._check_special_input()

            self._update()

            pyxel.frame_count += 1
            self._measure_update_time(update_start_time)

    def _update_mouse_pos(self):
        if self._viewport_scale > 0:
            x, y = glfw.get_cursor_pos(self._window)
            pyxel.mouse_x = int((x - self._viewport_left) / self._viewport_scale)
            pyxel.mouse_y = int((y - self._viewport_top) / self._viewport_scale)

    def _update_gamepad(self):
        for i in range(2):
            if i == 0:
                states, count = glfw.get_joystick_buttons(glfw.JOYSTICK_1)
                offset = pyxel.GAMEPAD_1_A
            else:
                states, count = glfw.get_joystick_buttons(glfw.JOYSTICK_2)
                offset = pyxel.GAMEPAD_2_A

            for j in range(count):
                action = states[j]
                button = offset + j

                self._set_key_state(button, action)

    def _draw_frame(self):
        draw_start_time = time.time()

        self._draw()

        self._draw_perf_monitor()
        self._draw_mouse_cursor()

        hs = self._hidpi_scale
        image = self._renderer.render(
            self._viewport_left * hs,
            self._viewport_bottom * hs,
            self._viewport_width * hs,
            self._viewport_height * hs,
            self._palette,
            self._border_color,
        )
        self._capture_images[self._capture_count % APP_GIF_CAPTURE_COUNT] = image
        self._capture_count += 1

        self._measure_draw_time(draw_start_time)

    def _toggle_fullscreen(self):
        if glfw.get_window_monitor(self._window):  # fullscreen to window
            glfw.set_window_monitor(
                self._window, None, *self._window_info, glfw.DONT_CARE
            )
        else:  # window to fullscreen
            info = [0] * 4
            info[0], info[1] = glfw.get_window_pos(self._window)
            info[2], info[3] = glfw.get_window_size(self._window)
            self._window_info = info

            monitor = glfw.get_primary_monitor()
            size = glfw.get_video_mode(monitor)[0]
            glfw.set_window_monitor(self._window, monitor, 0, 0, *size, glfw.DONT_CARE)

    def _check_special_input(self):
        if self.btn(KEY_ALT):
            if self.btnp(glfw.KEY_ENTER):
                self._toggle_fullscreen()

            if self.btnp(KEY_0):
                self._perf_monitor_is_enabled = not self._perf_monitor_is_enabled

            if self.btnp(KEY_1):
                self._save_capture_image()

            if self.btnp(KEY_2):
                self._capture_start = self._capture_count - 1

            if self.btnp(KEY_3):
                self._save_capture_animation()

        if self.btnp(glfw.KEY_ESCAPE):
            self.quit()

    def _save_capture_image(self):
        index = (self._capture_count - 1) % APP_GIF_CAPTURE_COUNT
        image = self._get_capture_image(index)
        image.save(self._get_capture_filename() + ".png", optimize=True)

    def _save_capture_animation(self):
        image_count = min(
            self._capture_count - self._capture_start, APP_GIF_CAPTURE_COUNT
        )

        if image_count <= 0:
            return

        start_index = (self._capture_count - image_count) % APP_GIF_CAPTURE_COUNT
        images = [self._get_capture_image(start_index)]

        for i in range(1, image_count):
            index = (start_index + i) % APP_GIF_CAPTURE_COUNT
            image = self._difference(
                self._get_capture_image(index - 1), self._get_capture_image(index)
            )
            images.append(image)

        color_index = self._get_color_palette_index(image, GIF_TRANSPARENCY_COLOR)

        images[0].save(
            self._get_capture_filename() + ".gif",
            save_all=True,
            append_images=images[1:],
            duration=self._one_frame_time * 1000,
            loop=0,
            optimize=False,
            transparency=color_index,
            disposal=1,
            palette=get_palette(fill=False),
        )

    @staticmethod
    def _get_color_palette_index(image, color):
        palette = image.getpalette()
        palette_colors = list(zip(palette[::3], palette[1::3], palette[2::3]))
        return palette_colors.index(color)

    @staticmethod
    def _difference(prev, curr):
        prev = np.asarray(prev.convert("RGBA"))
        curr = np.asarray(curr.convert("RGBA"))
        alpha = np.any(prev != curr, axis=-1, keepdims=True)
        new = alpha * curr
        red, green, blue, alpha = new.T
        trans_areas = alpha == 0
        new[..., :-1][trans_areas.T] = GIF_TRANSPARENCY_COLOR
        return palettize_pil_image(PIL.Image.fromarray(new))

    def _get_capture_image(self, index):
        image = PIL.Image.frombuffer(
            "RGB",
            (pyxel.width, pyxel.height),
            self._capture_images[index],
            "raw",
            "RGB",
            0,
            1,
        )

        image = palettize_pil_image(image)

        image = image.resize(
            (pyxel.width * APP_GIF_CAPTURE_SCALE, pyxel.height * APP_GIF_CAPTURE_SCALE)
        )

        return image

    @staticmethod
    def _get_capture_filename():
        return os.path.join(
            get_desktop_path(), datetime.datetime.now().strftime("pyxel-%y%m%d-%H%M%S")
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

    def _draw_mouse_cursor(self):
        if not self._is_mouse_visible:
            return

        pyxel.blt(
            pyxel.mouse_x,
            pyxel.mouse_y,
            3,
            MOUSE_CURSOR_IMAGE_X,
            MOUSE_CURSOR_IMAGE_Y,
            MOUSE_CURSOR_WIDTH,
            MOUSE_CURSOR_HEIGHT,
            1,
        )
