from typing import Any, Callable, List

from . import _dll_path  # type: ignore
from . import constants  # type: ignore

_dll_path = _dll_path  # dummy for Flake8 F401 warning

#
# constants
#

DEFAULT_BORDER_COLOR: int = constants.DEFAULT_BORDER_COLOR
DEFAULT_BORDER_WIDTH: int = constants.DEFAULT_BORDER_WIDTH
DEFAULT_CAPTION: str = constants.DEFAULT_CAPTION
DEFAULT_FPS: int = constants.DEFAULT_FPS
DEFAULT_PALETTE: List[int] = constants.DEFAULT_PALETTE
DEFAULT_SCALE: int = constants.DEFAULT_SCALE
GAMEPAD_1_A: int = constants.GAMEPAD_1_A
GAMEPAD_1_B: int = constants.GAMEPAD_1_B
GAMEPAD_1_DOWN: int = constants.GAMEPAD_1_DOWN
GAMEPAD_1_LEFT: int = constants.GAMEPAD_1_LEFT
GAMEPAD_1_LEFT_SHOULDER: int = constants.GAMEPAD_1_LEFT_SHOULDER
GAMEPAD_1_RIGHT: int = constants.GAMEPAD_1_RIGHT
GAMEPAD_1_RIGHT_SHOULDER: int = constants.GAMEPAD_1_RIGHT_SHOULDER
GAMEPAD_1_SELECT: int = constants.GAMEPAD_1_SELECT
GAMEPAD_1_START: int = constants.GAMEPAD_1_START
GAMEPAD_1_UP: int = constants.GAMEPAD_1_UP
GAMEPAD_1_X: int = constants.GAMEPAD_1_X
GAMEPAD_1_Y: int = constants.GAMEPAD_1_Y
GAMEPAD_2_A: int = constants.GAMEPAD_2_A
GAMEPAD_2_B: int = constants.GAMEPAD_2_B
GAMEPAD_2_DOWN: int = constants.GAMEPAD_2_DOWN
GAMEPAD_2_LEFT: int = constants.GAMEPAD_2_LEFT
GAMEPAD_2_LEFT_SHOULDER: int = constants.GAMEPAD_2_LEFT_SHOULDER
GAMEPAD_2_RIGHT: int = constants.GAMEPAD_2_RIGHT
GAMEPAD_2_RIGHT_SHOULDER: int = constants.GAMEPAD_2_RIGHT_SHOULDER
GAMEPAD_2_SELECT: int = constants.GAMEPAD_2_SELECT
GAMEPAD_2_START: int = constants.GAMEPAD_2_START
GAMEPAD_2_UP: int = constants.GAMEPAD_2_UP
GAMEPAD_2_X: int = constants.GAMEPAD_2_X
GAMEPAD_2_Y: int = constants.GAMEPAD_2_Y
KEY_0: int = constants.KEY_0
KEY_1: int = constants.KEY_1
KEY_2: int = constants.KEY_2
KEY_3: int = constants.KEY_3
KEY_4: int = constants.KEY_4
KEY_5: int = constants.KEY_5
KEY_6: int = constants.KEY_6
KEY_7: int = constants.KEY_7
KEY_8: int = constants.KEY_8
KEY_9: int = constants.KEY_9
KEY_A: int = constants.KEY_A
KEY_ALT: int = constants.KEY_ALT
KEY_APOSTROPHE: int = constants.KEY_APOSTROPHE
KEY_B: int = constants.KEY_B
KEY_BACKSLASH: int = constants.KEY_BACKSLASH
KEY_BACKSPACE: int = constants.KEY_BACKSPACE
KEY_C: int = constants.KEY_C
KEY_CAPS_LOCK: int = constants.KEY_CAPS_LOCK
KEY_COMMA: int = constants.KEY_COMMA
KEY_CONTROL: int = constants.KEY_CONTROL
KEY_D: int = constants.KEY_D
KEY_DELETE: int = constants.KEY_DELETE
KEY_DOWN: int = constants.KEY_DOWN
KEY_E: int = constants.KEY_E
KEY_END: int = constants.KEY_END
KEY_ENTER: int = constants.KEY_ENTER
KEY_EQUAL: int = constants.KEY_EQUAL
KEY_ESCAPE: int = constants.KEY_ESCAPE
KEY_F: int = constants.KEY_F
KEY_F1: int = constants.KEY_F1
KEY_F2: int = constants.KEY_F2
KEY_F3: int = constants.KEY_F3
KEY_F4: int = constants.KEY_F4
KEY_F5: int = constants.KEY_F5
KEY_F6: int = constants.KEY_F6
KEY_F7: int = constants.KEY_F7
KEY_F8: int = constants.KEY_F8
KEY_F9: int = constants.KEY_F9
KEY_F10: int = constants.KEY_F10
KEY_F11: int = constants.KEY_F11
KEY_F12: int = constants.KEY_F12
KEY_F13: int = constants.KEY_F13
KEY_F14: int = constants.KEY_F14
KEY_F15: int = constants.KEY_F15
KEY_F16: int = constants.KEY_F16
KEY_F17: int = constants.KEY_F17
KEY_F18: int = constants.KEY_F18
KEY_F19: int = constants.KEY_F19
KEY_F20: int = constants.KEY_F20
KEY_F21: int = constants.KEY_F21
KEY_F22: int = constants.KEY_F22
KEY_F23: int = constants.KEY_F23
KEY_F24: int = constants.KEY_F24
KEY_F25: int = constants.KEY_F25
KEY_G: int = constants.KEY_G
KEY_GRAVE_ACCENT: int = constants.KEY_GRAVE_ACCENT
KEY_H: int = constants.KEY_H
KEY_HOME: int = constants.KEY_HOME
KEY_I: int = constants.KEY_I
KEY_INSERT: int = constants.KEY_INSERT
KEY_J: int = constants.KEY_J
KEY_K: int = constants.KEY_K
KEY_KP_0: int = constants.KEY_KP_0
KEY_KP_1: int = constants.KEY_KP_1
KEY_KP_2: int = constants.KEY_KP_2
KEY_KP_3: int = constants.KEY_KP_3
KEY_KP_4: int = constants.KEY_KP_4
KEY_KP_5: int = constants.KEY_KP_5
KEY_KP_6: int = constants.KEY_KP_6
KEY_KP_7: int = constants.KEY_KP_7
KEY_KP_8: int = constants.KEY_KP_8
KEY_KP_9: int = constants.KEY_KP_9
KEY_KP_ADD: int = constants.KEY_KP_ADD
KEY_KP_DECIMAL: int = constants.KEY_KP_DECIMAL
KEY_KP_DIVIDE: int = constants.KEY_KP_DIVIDE
KEY_KP_ENTER: int = constants.KEY_KP_ENTER
KEY_KP_EQUAL: int = constants.KEY_KP_EQUAL
KEY_KP_MULTIPLY: int = constants.KEY_KP_MULTIPLY
KEY_KP_SUBTRACT: int = constants.KEY_KP_SUBTRACT
KEY_L: int = constants.KEY_L
KEY_LEFT: int = constants.KEY_LEFT
KEY_LEFT_ALT: int = constants.KEY_LEFT_ALT
KEY_LEFT_BRACKET: int = constants.KEY_LEFT_BRACKET
KEY_LEFT_CONTROL: int = constants.KEY_LEFT_CONTROL
KEY_LEFT_SHIFT: int = constants.KEY_LEFT_SHIFT
KEY_LEFT_SUPER: int = constants.KEY_LEFT_SUPER
KEY_M: int = constants.KEY_M
KEY_MENU: int = constants.KEY_MENU
KEY_MINUS: int = constants.KEY_MINUS
KEY_N: int = constants.KEY_N
KEY_NUM_LOCK: int = constants.KEY_NUM_LOCK
KEY_O: int = constants.KEY_O
KEY_P: int = constants.KEY_P
KEY_PAGE_DOWN: int = constants.KEY_PAGE_DOWN
KEY_PAGE_UP: int = constants.KEY_PAGE_UP
KEY_PAUSE: int = constants.KEY_PAUSE
KEY_PERIOD: int = constants.KEY_PERIOD
KEY_PRINT_SCREEN: int = constants.KEY_PRINT_SCREEN
KEY_Q: int = constants.KEY_Q
KEY_R: int = constants.KEY_R
KEY_RIGHT: int = constants.KEY_RIGHT
KEY_RIGHT_ALT: int = constants.KEY_RIGHT_ALT
KEY_RIGHT_BRACKET: int = constants.KEY_RIGHT_BRACKET
KEY_RIGHT_CONTROL: int = constants.KEY_RIGHT_CONTROL
KEY_RIGHT_SHIFT: int = constants.KEY_RIGHT_SHIFT
KEY_RIGHT_SUPER: int = constants.KEY_RIGHT_SUPER
KEY_S: int = constants.KEY_S
KEY_SCROLL_LOCK: int = constants.KEY_SCROLL_LOCK
KEY_SEMICOLON: int = constants.KEY_SEMICOLON
KEY_SHIFT: int = constants.KEY_SHIFT
KEY_SLASH: int = constants.KEY_SLASH
KEY_SPACE: int = constants.KEY_SPACE
KEY_SUPER: int = constants.KEY_SUPER
KEY_T: int = constants.KEY_T
KEY_TAB: int = constants.KEY_TAB
KEY_U: int = constants.KEY_U
KEY_UNKNOWN: int = constants.KEY_UNKNOWN
KEY_UP: int = constants.KEY_UP
KEY_V: int = constants.KEY_V
KEY_W: int = constants.KEY_W
KEY_WORLD_1: int = constants.KEY_WORLD_1
KEY_WORLD_2: int = constants.KEY_WORLD_2
KEY_X: int = constants.KEY_X
KEY_Y: int = constants.KEY_Y
KEY_Z: int = constants.KEY_Z
MOUSE_LEFT_BUTTON: int = constants.MOUSE_LEFT_BUTTON
MOUSE_MIDDLE_BUTTON: int = constants.MOUSE_MIDDLE_BUTTON
MOUSE_RIGHT_BUTTON: int = constants.MOUSE_RIGHT_BUTTON
VERSION: str = constants.VERSION


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

    def set(self, x: int, y: int, data: Any, refimg: int = None) -> None:
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
