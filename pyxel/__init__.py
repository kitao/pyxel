from typing import Any, Callable, List
from . import core  # type: ignore

#
# constants
#

VERSION: str = core.get_constant_string("VERSION")

DEFAULT_CAPTION: str = core.get_constant_string("DEFAULT_CAPTION")
DEFAULT_SCALE: int = core.get_constant_number("DEFAULT_SCALE")
DEFAULT_PALETTE: List[int] = [
    core.get_constant_number("DEFAULT_PALETTE_00"),
    core.get_constant_number("DEFAULT_PALETTE_01"),
    core.get_constant_number("DEFAULT_PALETTE_02"),
    core.get_constant_number("DEFAULT_PALETTE_03"),
    core.get_constant_number("DEFAULT_PALETTE_04"),
    core.get_constant_number("DEFAULT_PALETTE_05"),
    core.get_constant_number("DEFAULT_PALETTE_06"),
    core.get_constant_number("DEFAULT_PALETTE_07"),
    core.get_constant_number("DEFAULT_PALETTE_08"),
    core.get_constant_number("DEFAULT_PALETTE_09"),
    core.get_constant_number("DEFAULT_PALETTE_10"),
    core.get_constant_number("DEFAULT_PALETTE_11"),
    core.get_constant_number("DEFAULT_PALETTE_12"),
    core.get_constant_number("DEFAULT_PALETTE_13"),
    core.get_constant_number("DEFAULT_PALETTE_14"),
    core.get_constant_number("DEFAULT_PALETTE_15"),
]
DEFAULT_FPS: int = core.get_constant_number("DEFAULT_FPS")
DEFAULT_BORDER_WIDTH: int = core.get_constant_number("DEFAULT_BORDER_WIDTH")
DEFAULT_BORDER_COLOR: int = core.get_constant_number("DEFAULT_BORDER_COLOR")

KEY_SPACE: int = core.get_constant_number("KEY_SPACE")
KEY_APOSTROPHE: int = core.get_constant_number("KEY_APOSTROPHE")
KEY_COMMA: int = core.get_constant_number("KEY_COMMA")
KEY_MINUS: int = core.get_constant_number("KEY_MINUS")
KEY_PERIOD: int = core.get_constant_number("KEY_PERIOD")
KEY_SLASH: int = core.get_constant_number("KEY_SLASH")
KEY_0: int = core.get_constant_number("KEY_0")
KEY_1: int = core.get_constant_number("KEY_1")
KEY_2: int = core.get_constant_number("KEY_2")
KEY_3: int = core.get_constant_number("KEY_3")
KEY_4: int = core.get_constant_number("KEY_4")
KEY_5: int = core.get_constant_number("KEY_5")
KEY_6: int = core.get_constant_number("KEY_6")
KEY_7: int = core.get_constant_number("KEY_7")
KEY_8: int = core.get_constant_number("KEY_8")
KEY_9: int = core.get_constant_number("KEY_9")
KEY_SEMICOLON: int = core.get_constant_number("KEY_SEMICOLON")
KEY_EQUAL: int = core.get_constant_number("KEY_EQUAL")
KEY_A: int = core.get_constant_number("KEY_A")
KEY_B: int = core.get_constant_number("KEY_B")
KEY_C: int = core.get_constant_number("KEY_C")
KEY_D: int = core.get_constant_number("KEY_D")
KEY_E: int = core.get_constant_number("KEY_E")
KEY_F: int = core.get_constant_number("KEY_F")
KEY_G: int = core.get_constant_number("KEY_G")
KEY_H: int = core.get_constant_number("KEY_H")
KEY_I: int = core.get_constant_number("KEY_I")
KEY_J: int = core.get_constant_number("KEY_J")
KEY_K: int = core.get_constant_number("KEY_K")
KEY_L: int = core.get_constant_number("KEY_L")
KEY_M: int = core.get_constant_number("KEY_M")
KEY_N: int = core.get_constant_number("KEY_N")
KEY_O: int = core.get_constant_number("KEY_O")
KEY_P: int = core.get_constant_number("KEY_P")
KEY_Q: int = core.get_constant_number("KEY_Q")
KEY_R: int = core.get_constant_number("KEY_R")
KEY_S: int = core.get_constant_number("KEY_S")
KEY_T: int = core.get_constant_number("KEY_T")
KEY_U: int = core.get_constant_number("KEY_U")
KEY_V: int = core.get_constant_number("KEY_V")
KEY_W: int = core.get_constant_number("KEY_W")
KEY_X: int = core.get_constant_number("KEY_X")
KEY_Y: int = core.get_constant_number("KEY_Y")
KEY_Z: int = core.get_constant_number("KEY_Z")
KEY_LEFT_BRACKET: int = core.get_constant_number("KEY_LEFT_BRACKET")
KEY_BACKSLASH: int = core.get_constant_number("KEY_BACKSLASH")
KEY_RIGHT_BRACKET: int = core.get_constant_number("KEY_RIGHT_BRACKET")
KEY_GRAVE_ACCENT: int = core.get_constant_number("KEY_GRAVE_ACCENT")
KEY_ESCAPE: int = core.get_constant_number("KEY_ESCAPE")
KEY_ENTER: int = core.get_constant_number("KEY_ENTER")
KEY_TAB: int = core.get_constant_number("KEY_TAB")
KEY_BACKSPACE: int = core.get_constant_number("KEY_BACKSPACE")
KEY_INSERT: int = core.get_constant_number("KEY_INSERT")
KEY_DELETE: int = core.get_constant_number("KEY_DELETE")
KEY_RIGHT: int = core.get_constant_number("KEY_RIGHT")
KEY_LEFT: int = core.get_constant_number("KEY_LEFT")
KEY_DOWN: int = core.get_constant_number("KEY_DOWN")
KEY_UP: int = core.get_constant_number("KEY_UP")
KEY_PAGE_UP: int = core.get_constant_number("KEY_PAGE_UP")
KEY_PAGE_DOWN: int = core.get_constant_number("KEY_PAGE_DOWN")
KEY_HOME: int = core.get_constant_number("KEY_HOME")
KEY_END: int = core.get_constant_number("KEY_END")
KEY_CAPS_LOCK: int = core.get_constant_number("KEY_CAPS_LOCK")
KEY_SCROLL_LOCK: int = core.get_constant_number("KEY_SCROLL_LOCK")
KEY_NUM_LOCK: int = core.get_constant_number("KEY_NUM_LOCK")
KEY_PRINT_SCREEN: int = core.get_constant_number("KEY_PRINT_SCREEN")
KEY_PAUSE: int = core.get_constant_number("KEY_PAUSE")
KEY_F1: int = core.get_constant_number("KEY_F1")
KEY_F2: int = core.get_constant_number("KEY_F2")
KEY_F3: int = core.get_constant_number("KEY_F3")
KEY_F4: int = core.get_constant_number("KEY_F4")
KEY_F5: int = core.get_constant_number("KEY_F5")
KEY_F6: int = core.get_constant_number("KEY_F6")
KEY_F7: int = core.get_constant_number("KEY_F7")
KEY_F8: int = core.get_constant_number("KEY_F8")
KEY_F9: int = core.get_constant_number("KEY_F9")
KEY_F10: int = core.get_constant_number("KEY_F10")
KEY_F11: int = core.get_constant_number("KEY_F11")
KEY_F12: int = core.get_constant_number("KEY_F12")
KEY_KP_0: int = core.get_constant_number("KEY_KP_0")
KEY_KP_1: int = core.get_constant_number("KEY_KP_1")
KEY_KP_2: int = core.get_constant_number("KEY_KP_2")
KEY_KP_3: int = core.get_constant_number("KEY_KP_3")
KEY_KP_4: int = core.get_constant_number("KEY_KP_4")
KEY_KP_5: int = core.get_constant_number("KEY_KP_5")
KEY_KP_6: int = core.get_constant_number("KEY_KP_6")
KEY_KP_7: int = core.get_constant_number("KEY_KP_7")
KEY_KP_8: int = core.get_constant_number("KEY_KP_8")
KEY_KP_9: int = core.get_constant_number("KEY_KP_9")
KEY_KP_DECIMAL: int = core.get_constant_number("KEY_KP_DECIMAL")
KEY_KP_DIVIDE: int = core.get_constant_number("KEY_KP_DIVIDE")
KEY_KP_MULTIPLY: int = core.get_constant_number("KEY_KP_MULTIPLY")
KEY_KP_SUBTRACT: int = core.get_constant_number("KEY_KP_SUBTRACT")
KEY_KP_ADD: int = core.get_constant_number("KEY_KP_ADD")
KEY_KP_ENTER: int = core.get_constant_number("KEY_KP_ENTER")
KEY_KP_EQUAL: int = core.get_constant_number("KEY_KP_EQUAL")
KEY_LEFT_SHIFT: int = core.get_constant_number("KEY_LEFT_SHIFT")
KEY_LEFT_CONTROL: int = core.get_constant_number("KEY_LEFT_CONTROL")
KEY_LEFT_ALT: int = core.get_constant_number("KEY_LEFT_ALT")
KEY_LEFT_SUPER: int = core.get_constant_number("KEY_LEFT_SUPER")
KEY_RIGHT_SHIFT: int = core.get_constant_number("KEY_RIGHT_SHIFT")
KEY_RIGHT_CONTROL: int = core.get_constant_number("KEY_RIGHT_CONTROL")
KEY_RIGHT_ALT: int = core.get_constant_number("KEY_RIGHT_ALT")
KEY_RIGHT_SUPER: int = core.get_constant_number("KEY_RIGHT_SUPER")
KEY_MENU: int = core.get_constant_number("KEY_MENU")
KEY_SHIFT: int = core.get_constant_number("KEY_SHIFT")
KEY_CONTROL: int = core.get_constant_number("KEY_CONTROL")
KEY_ALT: int = core.get_constant_number("KEY_ALT")
KEY_SUPER: int = core.get_constant_number("KEY_SUPER")
MOUSE_LEFT_BUTTON: int = core.get_constant_number("MOUSE_LEFT_BUTTON")
MOUSE_MIDDLE_BUTTON: int = core.get_constant_number("MOUSE_MIDDLE_BUTTON")
MOUSE_RIGHT_BUTTON: int = core.get_constant_number("MOUSE_RIGHT_BUTTON")
GAMEPAD_1_A: int = core.get_constant_number("GAMEPAD_1_A")
GAMEPAD_1_B: int = core.get_constant_number("GAMEPAD_1_B")
GAMEPAD_1_X: int = core.get_constant_number("GAMEPAD_1_X")
GAMEPAD_1_Y: int = core.get_constant_number("GAMEPAD_1_Y")
GAMEPAD_1_LEFT_SHOULDER: int = core.get_constant_number("GAMEPAD_1_LEFT_SHOULDER")
GAMEPAD_1_RIGHT_SHOULDER: int = core.get_constant_number("GAMEPAD_1_RIGHT_SHOULDER")
GAMEPAD_1_SELECT: int = core.get_constant_number("GAMEPAD_1_SELECT")
GAMEPAD_1_START: int = core.get_constant_number("GAMEPAD_1_START")
GAMEPAD_1_UP: int = core.get_constant_number("GAMEPAD_1_UP")
GAMEPAD_1_RIGHT: int = core.get_constant_number("GAMEPAD_1_RIGHT")
GAMEPAD_1_DOWN: int = core.get_constant_number("GAMEPAD_1_DOWN")
GAMEPAD_1_LEFT: int = core.get_constant_number("GAMEPAD_1_LEFT")
GAMEPAD_2_A: int = core.get_constant_number("GAMEPAD_2_A")
GAMEPAD_2_B: int = core.get_constant_number("GAMEPAD_2_B")
GAMEPAD_2_X: int = core.get_constant_number("GAMEPAD_2_X")
GAMEPAD_2_Y: int = core.get_constant_number("GAMEPAD_2_Y")
GAMEPAD_2_LEFT_SHOULDER: int = core.get_constant_number("GAMEPAD_2_LEFT_SHOULDER")
GAMEPAD_2_RIGHT_SHOULDER: int = core.get_constant_number("GAMEPAD_2_RIGHT_SHOULDER")
GAMEPAD_2_SELECT: int = core.get_constant_number("GAMEPAD_2_SELECT")
GAMEPAD_2_START: int = core.get_constant_number("GAMEPAD_2_START")
GAMEPAD_2_UP: int = core.get_constant_number("GAMEPAD_2_UP")
GAMEPAD_2_RIGHT: int = core.get_constant_number("GAMEPAD_2_RIGHT")
GAMEPAD_2_DOWN: int = core.get_constant_number("GAMEPAD_2_DOWN")
GAMEPAD_2_LEFT: int = core.get_constant_number("GAMEPAD_2_LEFT")


#
# Image class
#


class Image:
    width: int = 0
    height: int = 0
    data: Any = None

    def get(self, x: int, y: int) -> int:
        pass

    def set(self, x: int, y: int, data: Any) -> None:
        pass

    def load(self, x: int, y: int, filename: str) -> None:
        pass

    def copy(self, x: int, y: int, img: int, u: int, v: int, w: int, h: int) -> None:
        pass


#
# Tilemap class
#


class Tilemap:
    width: int = 0
    height: int = 0
    data: Any = None

    def get(self, x: int, y: int) -> int:
        pass

    def set(self, x: int, y: int, data: Any) -> None:
        pass

    def copy(self, x: int, y: int, tm: int, u: int, v: int, w: int, h: int) -> None:
        pass


#
# Sound class
#


class Sound:
    note: List[int] = []
    tone: List[int] = []
    volume: List[int] = []
    effect: List[int] = []
    speed: int = 0

    def set(self, note: str, tone: str, volume: str, effect: str, speed: int) -> None:
        pass

    def set_note(self, data: str) -> None:
        pass

    def set_tone(self, data: str) -> None:
        pass

    def set_volume(self, data: str) -> None:
        pass

    def set_effect(self, data: str) -> None:
        pass


#
# Music class
#


class Music:
    ch0: List[int] = []
    ch1: List[int] = []
    ch2: List[int] = []
    ch3: List[int] = []

    def set(self, ch0, ch1, ch2, ch3) -> None:
        pass

    def set_ch0(self, data) -> None:
        pass

    def set_ch1(self, data) -> None:
        pass

    def set_ch2(self, data) -> None:
        pass

    def set_ch3(self, data) -> None:
        pass


#
# System
#

width: int = 0
height: int = 0
frame_count: int = 0


def init(
    width: int,
    height: int,
    *,
    caption: str = DEFAULT_CAPTION,
    scale: int = DEFAULT_SCALE,
    palette: List[int] = DEFAULT_PALETTE,
    fps: int = DEFAULT_FPS,
    border_width: int = DEFAULT_BORDER_WIDTH,
    border_color: int = DEFAULT_BORDER_COLOR
) -> None:
    import sys
    from .app import App  # type: ignore

    module = sys.modules[__name__]

    App(module, width, height, caption, scale, palette, fps, border_width, border_color)


def run(update: Callable[[], None], draw: Callable[[], None]) -> None:
    pass


def run_with_profiler(update: Callable[[], None], draw: Callable[[], None]) -> None:
    pass


def quit() -> None:
    pass


#
# Resource
#


def save(filename: str) -> None:
    pass


def load(filename: str) -> None:
    pass


#
# Input
#

mouse_x: int = 0
mouse_y: int = 0


def btn(key: int) -> bool:
    pass


def btnp(key: int, hold: int = 0, period: int = 0) -> bool:
    pass


def btnr(key: int) -> bool:
    pass


def mouse(visible: bool) -> None:
    pass


#
# Graphics
#


def image(img: int, *, system: bool = False) -> Image:
    pass


def tilemap(tm: int) -> Tilemap:
    pass


def clip(x1: int = None, y1: int = None, x2: int = None, y2: int = None) -> None:
    pass


def pal(col1: int = None, col2: int = None) -> None:
    pass


def cls(col: int) -> None:
    pass


def pix(x: int, y: int, col: int) -> None:
    pass


def line(x1: int, y1: int, x2: int, y2: int, col: int) -> None:
    pass


def rect(x1: int, y1: int, x2: int, y2: int, col: int) -> None:
    pass


def rectb(x1: int, y1: int, x2: int, y2: int, col: int) -> None:
    pass


def circ(x: int, y: int, r: int, col: int) -> None:
    pass


def circb(x: int, y: int, r: int, col: int) -> None:
    pass


def blt(
    x: int, y: int, img: int, u: int, v: int, w: int, h: int, colkey: int = None
) -> None:
    pass


def bltm(
    x: int, y: int, tm: int, u: int, v: int, w: int, h: int, colkey: int = None
) -> None:
    pass


def text(x: int, y: int, s: str, col: int):
    pass


#
# Audio
#


def sound(snd: int, *, system: bool = False) -> Sound:
    pass


def music(msc: int) -> Music:
    pass


def play(ch: int, snd: int, *, loop: bool = False) -> None:
    pass


def playm(msc: int, *, loop: bool = False) -> None:
    pass


def stop(ch: int = None) -> None:
    pass


#
# setup APIs
#


def _setup_apis():
    import sys

    from . import system_wrapper
    from . import resource_wrapper
    from . import input_wrapper
    from . import graphics_wrapper
    from . import audio_wrapper
    from . import image_wrapper
    from . import tilemap_wrapper
    from . import sound_wrapper
    from . import music_wrapper

    module = sys.modules[__name__]
    lib = core._lib

    system_wrapper.setup_apis(module, lib)
    resource_wrapper.setup_apis(module, lib)
    input_wrapper.setup_apis(module, lib)
    graphics_wrapper.setup_apis(module, lib)
    audio_wrapper.setup_apis(module, lib)
    image_wrapper.setup_apis(module, lib)
    tilemap_wrapper.setup_apis(module, lib)
    sound_wrapper.setup_apis(module, lib)
    music_wrapper.setup_apis(module, lib)


_setup_apis()
