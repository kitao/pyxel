# flake8: noqa
from typing import Any, Callable, Generic, List, Optional, Tuple, TypeVar, Union

# Constants
VERSION: str
BASE_DIR: str
WATCH_INFO_FILE_ENVVAR: str

APP_FILE_EXTENSION: str
APP_STARTUP_SCRIPT_FILE: str
RESOURCE_FILE_EXTENSION: str
PALETTE_FILE_EXTENSION: str

NUM_COLORS: int
NUM_IMAGES: int
IMAGE_SIZE: int
NUM_TILEMAPS: int
TILEMAP_SIZE: int
TILE_SIZE: int

COLOR_BLACK: int
COLOR_NAVY: int
COLOR_PURPLE: int
COLOR_GREEN: int
COLOR_BROWN: int
COLOR_DARK_BLUE: int
COLOR_LIGHT_BLUE: int
COLOR_WHITE: int
COLOR_RED: int
COLOR_ORANGE: int
COLOR_YELLOW: int
COLOR_LIME: int
COLOR_CYAN: int
COLOR_GRAY: int
COLOR_PINK: int
COLOR_PEACH: int

FONT_WIDTH: int
FONT_HEIGHT: int

NUM_CHANNELS: int
NUM_TONES: int
NUM_SOUNDS: int
NUM_MUSICS: int

TONE_TRIANGLE: int
TONE_SQUARE: int
TONE_PULSE: int
TONE_NOISE: int

EFFECT_NONE: int
EFFECT_SLIDE: int
EFFECT_VIBRATO: int
EFFECT_FADEOUT: int
EFFECT_HALF_FADEOUT: int
EFFECT_QUARTER_FADEOUT: int

# Keys
KEY_UNKNOWN: int
KEY_RETURN: int
KEY_ESCAPE: int
KEY_BACKSPACE: int
KEY_TAB: int
KEY_SPACE: int
KEY_EXCLAIM: int
KEY_QUOTEDBL: int
KEY_HASH: int
KEY_PERCENT: int
KEY_DOLLAR: int
KEY_AMPERSAND: int
KEY_QUOTE: int
KEY_LEFTPAREN: int
KEY_RIGHTPAREN: int
KEY_ASTERISK: int
KEY_PLUS: int
KEY_COMMA: int
KEY_MINUS: int
KEY_PERIOD: int
KEY_SLASH: int
KEY_0: int
KEY_1: int
KEY_2: int
KEY_3: int
KEY_4: int
KEY_5: int
KEY_6: int
KEY_7: int
KEY_8: int
KEY_9: int
KEY_COLON: int
KEY_SEMICOLON: int
KEY_LESS: int
KEY_EQUALS: int
KEY_GREATER: int
KEY_QUESTION: int
KEY_AT: int
KEY_LEFTBRACKET: int
KEY_BACKSLASH: int
KEY_RIGHTBRACKET: int
KEY_CARET: int
KEY_UNDERSCORE: int
KEY_BACKQUOTE: int
KEY_A: int
KEY_B: int
KEY_C: int
KEY_D: int
KEY_E: int
KEY_F: int
KEY_G: int
KEY_H: int
KEY_I: int
KEY_J: int
KEY_K: int
KEY_L: int
KEY_M: int
KEY_N: int
KEY_O: int
KEY_P: int
KEY_Q: int
KEY_R: int
KEY_S: int
KEY_T: int
KEY_U: int
KEY_V: int
KEY_W: int
KEY_X: int
KEY_Y: int
KEY_Z: int
KEY_CAPSLOCK: int
KEY_F1: int
KEY_F2: int
KEY_F3: int
KEY_F4: int
KEY_F5: int
KEY_F6: int
KEY_F7: int
KEY_F8: int
KEY_F9: int
KEY_F10: int
KEY_F11: int
KEY_F12: int
KEY_PRINTSCREEN: int
KEY_SCROLLLOCK: int
KEY_PAUSE: int
KEY_INSERT: int
KEY_HOME: int
KEY_PAGEUP: int
KEY_DELETE: int
KEY_END: int
KEY_PAGEDOWN: int
KEY_RIGHT: int
KEY_LEFT: int
KEY_DOWN: int
KEY_UP: int
KEY_NUMLOCKCLEAR: int
KEY_KP_DIVIDE: int
KEY_KP_MULTIPLY: int
KEY_KP_MINUS: int
KEY_KP_PLUS: int
KEY_KP_ENTER: int
KEY_KP_1: int
KEY_KP_2: int
KEY_KP_3: int
KEY_KP_4: int
KEY_KP_5: int
KEY_KP_6: int
KEY_KP_7: int
KEY_KP_8: int
KEY_KP_9: int
KEY_KP_0: int
KEY_KP_PERIOD: int
KEY_APPLICATION: int
KEY_POWER: int
KEY_KP_EQUALS: int
KEY_F13: int
KEY_F14: int
KEY_F15: int
KEY_F16: int
KEY_F17: int
KEY_F18: int
KEY_F19: int
KEY_F20: int
KEY_F21: int
KEY_F22: int
KEY_F23: int
KEY_F24: int
KEY_EXECUTE: int
KEY_HELP: int
KEY_MENU: int
KEY_SELECT: int
KEY_STOP: int
KEY_AGAIN: int
KEY_UNDO: int
KEY_CUT: int
KEY_COPY: int
KEY_PASTE: int
KEY_FIND: int
KEY_MUTE: int
KEY_VOLUMEUP: int
KEY_VOLUMEDOWN: int
KEY_KP_COMMA: int
KEY_KP_EQUALSAS400: int
KEY_ALTERASE: int
KEY_SYSREQ: int
KEY_CANCEL: int
KEY_CLEAR: int
KEY_PRIOR: int
KEY_RETURN2: int
KEY_SEPARATOR: int
KEY_OUT: int
KEY_OPER: int
KEY_CLEARAGAIN: int
KEY_CRSEL: int
KEY_EXSEL: int
KEY_KP_00: int
KEY_KP_000: int
KEY_THOUSANDSSEPARATOR: int
KEY_DECIMALSEPARATOR: int
KEY_CURRENCYUNIT: int
KEY_CURRENCYSUBUNIT: int
KEY_KP_LEFTPAREN: int
KEY_KP_RIGHTPAREN: int
KEY_KP_LEFTBRACE: int
KEY_KP_RIGHTBRACE: int
KEY_KP_TAB: int
KEY_KP_BACKSPACE: int
KEY_KP_A: int
KEY_KP_B: int
KEY_KP_C: int
KEY_KP_D: int
KEY_KP_E: int
KEY_KP_F: int
KEY_KP_XOR: int
KEY_KP_POWER: int
KEY_KP_PERCENT: int
KEY_KP_LESS: int
KEY_KP_GREATER: int
KEY_KP_AMPERSAND: int
KEY_KP_DBLAMPERSAND: int
KEY_KP_VERTICALBAR: int
KEY_KP_DBLVERTICALBAR: int
KEY_KP_COLON: int
KEY_KP_HASH: int
KEY_KP_SPACE: int
KEY_KP_AT: int
KEY_KP_EXCLAM: int
KEY_KP_MEMSTORE: int
KEY_KP_MEMRECALL: int
KEY_KP_MEMCLEAR: int
KEY_KP_MEMADD: int
KEY_KP_MEMSUBTRACT: int
KEY_KP_MEMMULTIPLY: int
KEY_KP_MEMDIVIDE: int
KEY_KP_PLUSMINUS: int
KEY_KP_CLEAR: int
KEY_KP_CLEARENTRY: int
KEY_KP_BINARY: int
KEY_KP_OCTAL: int
KEY_KP_DECIMAL: int
KEY_KP_HEXADECIMAL: int
KEY_LCTRL: int
KEY_LSHIFT: int
KEY_LALT: int
KEY_LGUI: int
KEY_RCTRL: int
KEY_RSHIFT: int
KEY_RALT: int
KEY_RGUI: int
KEY_MODE: int
KEY_AUDIONEXT: int
KEY_AUDIOPREV: int
KEY_AUDIOSTOP: int
KEY_AUDIOPLAY: int
KEY_AUDIOMUTE: int
KEY_MEDIASELECT: int
KEY_WWW: int
KEY_MAIL: int
KEY_CALCULATOR: int
KEY_COMPUTER: int
KEY_AC_SEARCH: int
KEY_AC_HOME: int
KEY_AC_BACK: int
KEY_AC_FORWARD: int
KEY_AC_STOP: int
KEY_AC_REFRESH: int
KEY_AC_BOOKMARKS: int
KEY_BRIGHTNESSDOWN: int
KEY_BRIGHTNESSUP: int
KEY_DISPLAYSWITCH: int
KEY_KBDILLUMTOGGLE: int
KEY_KBDILLUMDOWN: int
KEY_KBDILLUMUP: int
KEY_EJECT: int
KEY_SLEEP: int
KEY_APP1: int
KEY_APP2: int
KEY_AUDIOREWIND: int
KEY_AUDIOFASTFORWARD: int
KEY_NONE: int
KEY_SHIFT: int
KEY_CTRL: int
KEY_ALT: int
KEY_GUI: int

MOUSE_POS_X: int
MOUSE_POS_Y: int
MOUSE_WHEEL_X: int
MOUSE_WHEEL_Y: int
MOUSE_BUTTON_LEFT: int
MOUSE_BUTTON_MIDDLE: int
MOUSE_BUTTON_RIGHT: int
MOUSE_BUTTON_X1: int
MOUSE_BUTTON_X2: int
MOUSE_BUTTON_UNKNOWN: int

GAMEPAD1_AXIS_LEFTX: int
GAMEPAD1_AXIS_LEFTY: int
GAMEPAD1_AXIS_RIGHTX: int
GAMEPAD1_AXIS_RIGHTY: int
GAMEPAD1_AXIS_TRIGGERLEFT: int
GAMEPAD1_AXIS_TRIGGERRIGHT: int
GAMEPAD1_BUTTON_A: int
GAMEPAD1_BUTTON_B: int
GAMEPAD1_BUTTON_X: int
GAMEPAD1_BUTTON_Y: int
GAMEPAD1_BUTTON_BACK: int
GAMEPAD1_BUTTON_GUIDE: int
GAMEPAD1_BUTTON_START: int
GAMEPAD1_BUTTON_LEFTSTICK: int
GAMEPAD1_BUTTON_RIGHTSTICK: int
GAMEPAD1_BUTTON_LEFTSHOULDER: int
GAMEPAD1_BUTTON_RIGHTSHOULDER: int
GAMEPAD1_BUTTON_DPAD_UP: int
GAMEPAD1_BUTTON_DPAD_DOWN: int
GAMEPAD1_BUTTON_DPAD_LEFT: int
GAMEPAD1_BUTTON_DPAD_RIGHT: int

GAMEPAD2_AXIS_LEFTX: int
GAMEPAD2_AXIS_LEFTY: int
GAMEPAD2_AXIS_RIGHTX: int
GAMEPAD2_AXIS_RIGHTY: int
GAMEPAD2_AXIS_TRIGGERLEFT: int
GAMEPAD2_AXIS_TRIGGERRIGHT: int
GAMEPAD2_BUTTON_A: int
GAMEPAD2_BUTTON_B: int
GAMEPAD2_BUTTON_X: int
GAMEPAD2_BUTTON_Y: int
GAMEPAD2_BUTTON_BACK: int
GAMEPAD2_BUTTON_GUIDE: int
GAMEPAD2_BUTTON_START: int
GAMEPAD2_BUTTON_LEFTSTICK: int
GAMEPAD2_BUTTON_RIGHTSTICK: int
GAMEPAD2_BUTTON_LEFTSHOULDER: int
GAMEPAD2_BUTTON_RIGHTSHOULDER: int
GAMEPAD2_BUTTON_DPAD_UP: int
GAMEPAD2_BUTTON_DPAD_DOWN: int
GAMEPAD2_BUTTON_DPAD_LEFT: int
GAMEPAD2_BUTTON_DPAD_RIGHT: int

GAMEPAD3_AXIS_LEFTX: int
GAMEPAD3_AXIS_LEFTY: int
GAMEPAD3_AXIS_RIGHTX: int
GAMEPAD3_AXIS_RIGHTY: int
GAMEPAD3_AXIS_TRIGGERLEFT: int
GAMEPAD3_AXIS_TRIGGERRIGHT: int
GAMEPAD3_BUTTON_A: int
GAMEPAD3_BUTTON_B: int
GAMEPAD3_BUTTON_X: int
GAMEPAD3_BUTTON_Y: int
GAMEPAD3_BUTTON_BACK: int
GAMEPAD3_BUTTON_GUIDE: int
GAMEPAD3_BUTTON_START: int
GAMEPAD3_BUTTON_LEFTSTICK: int
GAMEPAD3_BUTTON_RIGHTSTICK: int
GAMEPAD3_BUTTON_LEFTSHOULDER: int
GAMEPAD3_BUTTON_RIGHTSHOULDER: int
GAMEPAD3_BUTTON_DPAD_UP: int
GAMEPAD3_BUTTON_DPAD_DOWN: int
GAMEPAD3_BUTTON_DPAD_LEFT: int
GAMEPAD3_BUTTON_DPAD_RIGHT: int

GAMEPAD4_AXIS_LEFTX: int
GAMEPAD4_AXIS_LEFTY: int
GAMEPAD4_AXIS_RIGHTX: int
GAMEPAD4_AXIS_RIGHTY: int
GAMEPAD4_AXIS_TRIGGERLEFT: int
GAMEPAD4_AXIS_TRIGGERRIGHT: int
GAMEPAD4_BUTTON_A: int
GAMEPAD4_BUTTON_B: int
GAMEPAD4_BUTTON_X: int
GAMEPAD4_BUTTON_Y: int
GAMEPAD4_BUTTON_BACK: int
GAMEPAD4_BUTTON_GUIDE: int
GAMEPAD4_BUTTON_START: int
GAMEPAD4_BUTTON_LEFTSTICK: int
GAMEPAD4_BUTTON_RIGHTSTICK: int
GAMEPAD4_BUTTON_LEFTSHOULDER: int
GAMEPAD4_BUTTON_RIGHTSHOULDER: int
GAMEPAD4_BUTTON_DPAD_UP: int
GAMEPAD4_BUTTON_DPAD_DOWN: int
GAMEPAD4_BUTTON_DPAD_LEFT: int
GAMEPAD4_BUTTON_DPAD_RIGHT: int

# Sequence class
T = TypeVar("T")

class Seq(Generic[T]):
    def __len__(self) -> None: ...
    def __getitem__(self, idx: int) -> T: ...
    def __setitem__(self, idx: int, value: T) -> T: ...
    def from_list(self, lst: List[T]) -> None: ...
    def to_list(self) -> List[T]: ...

# Font class
class Font:
    def __init__(self, filename: str) -> None: ...
    def text_width(self, s: str) -> int: ...

# Image class
class Image:
    width: int
    height: int

    def __init__(self, width: int, height: int) -> None: ...
    @staticmethod
    def from_image(filename: str, *, incl_colors: Optional[bool] = None) -> Image: ...
    def data_ptr(self) -> Any: ...
    def set(self, x: int, y: int, data: List[str]) -> None: ...
    def load(
        self, x: int, y: int, filename: str, *, incl_colors: Optional[bool] = None
    ) -> None: ...
    def save(self, filename: str, scale: int) -> None: ...
    def clip(
        self,
        x: Optional[float] = None,
        y: Optional[float] = None,
        w: Optional[float] = None,
        h: Optional[float] = None,
    ) -> None: ...
    def camera(
        self,
        x: Optional[float] = None,
        y: Optional[float] = None,
    ) -> None: ...
    def pal(self, col1: Optional[int] = None, col2: Optional[int] = None) -> None: ...
    def dither(self, alpha: float) -> None: ...
    def cls(self, col: int) -> None: ...
    def pget(self, x: float, y: float) -> int: ...
    def pset(self, x: float, y: float, col: int) -> None: ...
    def line(self, x1: float, y1: float, x2: float, y2: float, col: int) -> None: ...
    def rect(self, x: float, y: float, w: float, h: float, col: int) -> None: ...
    def rectb(self, x: float, y: float, w: float, h: float, col: int) -> None: ...
    def circ(self, x: float, y: float, r: float, col: int) -> None: ...
    def circb(self, x: float, y: float, r: float, col: int) -> None: ...
    def elli(self, x: float, y: float, w: float, h: float, col: int) -> None: ...
    def ellib(self, x: float, y: float, w: float, h: float, col: int) -> None: ...
    def tri(
        self, x1: float, y1: float, x2: float, y2: float, x3: float, y3: float, col: int
    ) -> None: ...
    def trib(
        self, x1: float, y1: float, x2: float, y2: float, x3: float, y3: float, col: int
    ) -> None: ...
    def fill(self, x: float, y: float, col: int) -> None: ...
    def blt(
        self,
        x: float,
        y: float,
        img: Union[int, Image],
        u: float,
        v: float,
        w: float,
        h: float,
        colkey: Optional[int] = None,
        *,
        rotate: Optional[float] = None,
        scale: Optional[float] = None,
    ) -> None: ...
    def bltm(
        self,
        x: float,
        y: float,
        tm: Union[int, Tilemap],
        u: float,
        v: float,
        w: float,
        h: float,
        colkey: Optional[int] = None,
        *,
        rotate: Optional[float] = None,
        scale: Optional[float] = None,
    ) -> None: ...
    def text(
        self, x: float, y: float, s: str, col: int, font: Optional[Font]
    ) -> None: ...

# Tilemap class
class Tilemap:
    width: int
    height: int
    imgsrc: Union[int, Image]

    def __init__(self, width: int, height: int, img: Union[int, Image]) -> None: ...
    @staticmethod
    def from_tmx(filename: str, layer: int) -> Image: ...
    def data_ptr(self) -> Any: ...
    def set(self, x: int, y: int, data: List[str]) -> None: ...
    def load(self, x: int, y: int, filename: str, layer: int) -> None: ...
    def clip(
        self,
        x: Optional[float] = None,
        y: Optional[float] = None,
        w: Optional[float] = None,
        h: Optional[float] = None,
    ) -> None: ...
    def camera(
        self,
        x: Optional[float] = None,
        y: Optional[float] = None,
    ) -> None: ...
    def cls(self, tile: Tuple[int, int]) -> None: ...
    def pget(self, x: float, y: float) -> Tuple[int, int]: ...
    def pset(self, x: float, y: float, tile: Tuple[int, int]) -> None: ...
    def line(
        self, x1: float, y1: float, x2: float, y2: float, tile: Tuple[int, int]
    ) -> None: ...
    def rect(
        self, x: float, y: float, w: float, h: float, tile: Tuple[int, int]
    ) -> None: ...
    def rectb(
        self, x: float, y: float, w: float, h: float, tile: Tuple[int, int]
    ) -> None: ...
    def circ(self, x: float, y: float, r: float, tile: Tuple[int, int]) -> None: ...
    def circb(self, x: float, y: float, r: float, tile: Tuple[int, int]) -> None: ...
    def elli(
        self, x: float, y: float, w: float, h: float, tile: Tuple[int, int]
    ) -> None: ...
    def ellib(
        self, x: float, y: float, w: float, h: float, tile: Tuple[int, int]
    ) -> None: ...
    def tri(
        self,
        x1: float,
        y1: float,
        x2: float,
        y2: float,
        x3: float,
        y3: float,
        tile: Tuple[int, int],
    ) -> None: ...
    def trib(
        self,
        x1: float,
        y1: float,
        x2: float,
        y2: float,
        x3: float,
        y3: float,
        tile: Tuple[int, int],
    ) -> None: ...
    def fill(self, x: float, y: float, tile: Tuple[int, int]) -> None: ...
    def blt(
        self,
        x: float,
        y: float,
        tm: Union[int, Tilemap],
        u: float,
        v: float,
        w: float,
        h: float,
        tilekey: Optional[Tuple[int, int]] = None,
        *,
        rotate: Optional[float] = None,
        scale: Optional[float] = None,
    ) -> None: ...

    # Deprecated field
    image: Image
    refimg: Optional[int]

# Channel class
class Channel:
    gain: float
    detune: int

    def __init__(self) -> None: ...
    def play(
        self,
        snd: Union[int, Seq[int], Sound, Seq[Sound]],
        *,
        tick: Optional[int] = None,
        loop: Optional[bool] = None,
        resume: Optional[bool] = None,
    ) -> None: ...
    def stop(self) -> None: ...
    def play_pos(self) -> Optional[Tuple[int, int]]: ...

# Tone class
class Tone:
    gain: float
    noise: int
    waveform: Seq[int]

    def __init__(self) -> None: ...

# Sound class
class Sound:
    notes: Seq[int]
    tones: Seq[int]
    volumes: Seq[int]
    effects: Seq[int]
    speed: int

    def __init__(self) -> None: ...
    def set(
        self,
        notes: str,
        tones: str,
        volumes: str,
        effects: str,
        speed: int,
    ) -> None: ...
    def set_notes(self, notes: str) -> None: ...
    def set_tones(self, tones: str) -> None: ...
    def set_volumes(self, volumes: str) -> None: ...
    def set_effects(self, effects: str) -> None: ...

# Music class
class Music:
    seqs: Seq[Seq[int]]

    def __init__(self) -> None: ...
    def set(
        self,
        *seqs: List[int],
    ) -> None: ...

    # Deprecated field
    snds_list: Seq[Seq[int]]

# System
width: int
height: int
frame_count: int

def init(
    width: int,
    height: int,
    *,
    title: Optional[str] = None,
    fps: Optional[int] = None,
    quit_key: Optional[int] = None,
    display_scale: Optional[int] = None,
    capture_scale: Optional[int] = None,
    capture_sec: Optional[int] = None,
) -> None: ...
def run(update: Callable[[], None], draw: Callable[[], None]) -> None: ...
def show() -> None: ...
def flip() -> None: ...
def quit() -> None: ...
def title(title: str) -> None: ...
def icon(data: List[str], scale: int, colkey: Optional[int]) -> None: ...
def perf_monitor(enabled: bool) -> None: ...
def integer_scale(enabled: bool) -> None: ...
def screen_mode(scr: int) -> None: ...
def fullscreen(enabled: bool) -> None: ...
def process_exists(pid: int) -> bool: ...

# Resource
def load(
    filename: str,
    *,
    excl_images: Optional[bool] = None,
    excl_tilemaps: Optional[bool] = None,
    excl_sounds: Optional[bool] = None,
    excl_musics: Optional[bool] = None,
    incl_colors: Optional[bool] = None,
    incl_channels: Optional[bool] = None,
    incl_tones: Optional[bool] = None,
) -> None: ...
def save(
    filename: str,
    *,
    excl_images: Optional[bool] = None,
    excl_tilemaps: Optional[bool] = None,
    excl_sounds: Optional[bool] = None,
    excl_musics: Optional[bool] = None,
    incl_colors: Optional[bool] = None,
    incl_channels: Optional[bool] = None,
    incl_tones: Optional[bool] = None,
) -> None: ...
def screenshot(scale: Optional[int] = None) -> None: ...
def screencast(scale: Optional[int] = None) -> None: ...
def reset_screencast() -> None: ...
def user_data_dir(vendor_name: str, app_name: str) -> str: ...

# Input
mouse_x: int
mouse_y: int
mouse_wheel: int
input_text: str
dropped_files: List[str]

def btn(key: int) -> bool: ...
def btnp(
    key: int, *, hold: Optional[int] = None, repeat: Optional[int] = None
) -> bool: ...
def btnr(key: int) -> bool: ...
def btnv(key: int) -> int: ...
def mouse(visible: bool) -> None: ...
def warp_mouse(x: float, y: float) -> None: ...

# Graphics
colors: Seq[int]
images: Seq[Image]
tilemaps: Seq[Tilemap]
screen: Image
cursor: Image
font: Image

def clip(
    x: Optional[float] = None,
    y: Optional[float] = None,
    w: Optional[float] = None,
    h: Optional[float] = None,
) -> None: ...
def camera(
    x: Optional[float] = None,
    y: Optional[float] = None,
) -> None: ...
def pal(col1: Optional[int] = None, col2: Optional[int] = None) -> None: ...
def dither(alpha: float) -> None: ...
def cls(col: int) -> None: ...
def pget(x: float, y: float) -> int: ...
def pset(x: float, y: float, col: int) -> None: ...
def line(x1: float, y1: float, x2: float, y2: float, col: int) -> None: ...
def rect(x: float, y: float, w: float, h: float, col: int) -> None: ...
def rectb(x: float, y: float, w: float, h: float, col: int) -> None: ...
def circ(x: float, y: float, r: float, col: int) -> None: ...
def circb(x: float, y: float, r: float, col: int) -> None: ...
def elli(self, x: float, y: float, w: float, h: float, col: int) -> None: ...
def ellib(self, x: float, y: float, w: float, h: float, col: int) -> None: ...
def tri(
    x1: float,
    y1: float,
    x2: float,
    y2: float,
    x3: float,
    y3: float,
    col: int,
) -> None: ...
def trib(
    x1: float,
    y1: float,
    x2: float,
    y2: float,
    x3: float,
    y3: float,
    col: int,
) -> None: ...
def fill(x: float, y: float, col: int) -> None: ...
def blt(
    x: float,
    y: float,
    img: Union[int, Image],
    u: float,
    v: float,
    w: float,
    h: float,
    colkey: Optional[int] = None,
    *,
    rotate: Optional[float] = None,
    scale: Optional[float] = None,
) -> None: ...
def bltm(
    x: float,
    y: float,
    tm: Union[int, Tilemap],
    u: float,
    v: float,
    w: float,
    h: float,
    colkey: Optional[int] = None,
    *,
    rotate: Optional[float] = None,
    scale: Optional[float] = None,
) -> None: ...
def text(x: float, y: float, s: str, col: int, font: Optional[Font]) -> None: ...

# Audio
channels: Seq[Channel]
tones: Seq[Tone]
sounds: Seq[Sound]
musics: Seq[Music]

def play(
    ch: int,
    snd: Union[int, Seq[int], Sound, Seq[Sound]],
    *,
    tick: Optional[int] = None,
    loop: Optional[bool] = None,
    resume: Optional[bool] = None,
) -> None: ...
def playm(
    msc: int,
    *,
    tick: Optional[int] = None,
    loop: Optional[bool] = None,
) -> None: ...
def stop(ch: Optional[int] = None) -> None: ...
def play_pos(ch: int) -> Optional[Tuple[int, int]]: ...

# Math
def ceil(x: float) -> int: ...
def floor(x: float) -> int: ...
def sgn(x: float) -> float: ...
def sqrt(x: float) -> float: ...
def sin(deg: float) -> float: ...
def cos(deg: float) -> float: ...
def atan2(y: float, x: float) -> float: ...
def rseed(seed: int) -> None: ...
def rndi(a: int, b: int) -> int: ...
def rndf(a: float, b: float) -> float: ...
def nseed(seed: int) -> None: ...
def noise(x: float, y: Optional[float] = None, z: Optional[float] = None) -> float: ...

# Deprecated functions
def image(img: int) -> Image: ...
def tilemap(tm: int) -> Tilemap: ...
def channel(ch: int) -> Channel: ...
def sound(snd: int) -> Sound: ...
def music(msc: int) -> Music: ...
