from typing import (
    Any,
    Callable,
    Generic,
    Iterator,
    TypeVar,
    overload,
)

T = TypeVar("T")

# Constants
VERSION: str
BASE_DIR: str
WINDOW_STATE_ENV: str
WATCH_STATE_FILE_ENV: str
WATCH_RESET_EXIT_CODE: int

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

DEFAULT_COLORS: list[int]
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

class Seq(Generic[T]):
    def __len__(self) -> int: ...
    @overload
    def __getitem__(self, idx: int) -> T: ...
    @overload
    def __getitem__(self, idx: slice) -> list[T]: ...
    @overload
    def __setitem__(self, idx: int, value: T) -> None: ...
    @overload
    def __setitem__(self, idx: slice, value: list[T]) -> None: ...
    def __delitem__(self, idx: int | slice) -> None: ...
    def __iter__(self) -> Iterator[T]: ...
    def __reversed__(self) -> Iterator[T]: ...
    def __contains__(self, value: T) -> bool: ...
    def __eq__(self, other: Any) -> bool: ...
    def __add__(self, other: list[T]) -> list[T]: ...
    def __mul__(self, n: int) -> list[T]: ...
    def __iadd__(self, other: list[T]) -> Seq[T]: ...
    def __bool__(self) -> bool: ...
    def __repr__(self) -> str: ...
    def append(self, value: T) -> None: ...
    def extend(self, values: list[T]) -> None: ...
    def insert(self, index: int, value: T) -> None: ...
    def pop(self, index: int | None = None) -> T: ...
    def clear(self) -> None: ...

# Font class
class Font:
    def __init__(self, filename: str, font_size: float = 10.0) -> None:
        """Create a Font instance from a font file (BDF/OTF/TTF/TTC).

        Args:
            filename: Font file name (BDF/OTF/TTF/TTC)
            font_size: Font size. Defaults to 10.0. Not used for BDF.

        Returns:
            New Font instance
        """
        ...
    def text_width(self, s: str) -> int:
        """Return the display width of the string s in this font.

        Args:
            s: String to measure

        Returns:
            Width in pixels
        """
        ...

# Image class
class Image:
    width: int
    """The width of the image."""
    height: int
    """The height of the image."""

    def __init__(self, width: int, height: int) -> None:
        """Create a new Image instance with the specified size.

        Args:
            width: Image width
            height: Image height

        Returns:
            New Image instance
        """
        ...
    @staticmethod
    def from_image(
        filename: str,
        include_colors: bool = False,
    ) -> Image:
        """Create an Image instance from an image file.

        Args:
            filename: Image file name
            include_colors: Include palette colors from file. Defaults to False.

        Returns:
            Image instance from file
        """
        ...
    def data_ptr(self) -> Any:
        """Return the raw data pointer of the image as a ctypes c_uint8 array.

        Returns:
            Raw data pointer
        """
        ...
    def set(self, x: int, y: int, data: list[str]) -> None:
        """Set the image at (x, y) using a list of hex-digit strings. Each character represents a color index (0-f).

        Args:
            x: X coordinate
            y: Y coordinate
            data: Image data as a list of strings

        Example::
            pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])
        """
        ...
    def load(
        self,
        x: int,
        y: int,
        filename: str,
        include_colors: bool = False,
    ) -> None:
        """Load an image file (PNG/GIF/JPEG) at (x, y).

        Args:
            x: X coordinate
            y: Y coordinate
            filename: Image file name (PNG/GIF/JPEG)
            include_colors: Include palette colors from file. Defaults to False.
        """
        ...
    def save(self, filename: str, scale: int) -> None:
        """Save the image to a file.

        Args:
            filename: Output file name
            scale: Scale factor
        """
        ...
    @overload
    def clip(self) -> None: ...
    @overload
    def clip(self, x: float, y: float, w: float, h: float) -> None: ...
    def clip(
        self,
        x: float | None = None,
        y: float | None = None,
        w: float | None = None,
        h: float | None = None,
    ) -> None:
        """Set the drawing area of the screen from (x, y) with a width of w and a height of h. Call clip() to reset to full screen. Call without arguments to reset the drawing area to full screen.

        Args:
            x: X coordinate of the upper-left corner
            y: Y coordinate of the upper-left corner
            w: Width of the clipping area
            h: Height of the clipping area
        """
        ...
    @overload
    def camera(self) -> None: ...
    @overload
    def camera(self, x: float, y: float) -> None: ...
    def camera(
        self,
        x: float | None = None,
        y: float | None = None,
    ) -> None:
        """Set the drawing offset to (x, y). All subsequent drawing operations will be shifted by (-x, -y). Call without arguments to reset the drawing offset to (0, 0).

        Args:
            x: X coordinate
            y: Y coordinate
        """
        ...
    @overload
    def pal(self) -> None: ...
    @overload
    def pal(self, col1: int, col2: int) -> None: ...
    def pal(self, col1: int | None = None, col2: int | None = None) -> None:
        """Replace color col1 with col2 when drawing. Call without arguments to reset the palette to the initial state.

        Args:
            col1: Color to replace
            col2: Replacement color
        """
        ...
    def dither(self, alpha: float) -> None:
        """Apply dithering (pseudo-transparency) when drawing. Set alpha in the range 0.0-1.0.

        Args:
            alpha: Opacity (0.0: transparent, 1.0: opaque)
        """
        ...
    def cls(self, col: int) -> None:
        """Clear the screen with color col.

        Args:
            col: Color
        """
        ...
    def pget(self, x: float, y: float) -> int:
        """Get the color of the pixel at (x, y).

        Args:
            x: X coordinate
            y: Y coordinate

        Returns:
            Color of the pixel
        """
        ...
    def pset(self, x: float, y: float, col: int) -> None:
        """Draw a pixel with the color col at (x, y).

        Args:
            x: X coordinate
            y: Y coordinate
            col: Color
        """
        ...
    def line(self, x1: float, y1: float, x2: float, y2: float, col: int) -> None:
        """Draw a line of color col from (x1, y1) to (x2, y2).

        Args:
            x1: Start X coordinate
            y1: Start Y coordinate
            x2: End X coordinate
            y2: End Y coordinate
            col: Color
        """
        ...
    def rect(self, x: float, y: float, w: float, h: float, col: int) -> None:
        """Draw a filled rectangle of width w, height h, and color col at (x, y).

        Args:
            x: X coordinate
            y: Y coordinate
            w: Width
            h: Height
            col: Color
        """
        ...
    def rectb(self, x: float, y: float, w: float, h: float, col: int) -> None:
        """Draw the outline of a rectangle of width w, height h, and color col at (x, y).

        Args:
            x: X coordinate
            y: Y coordinate
            w: Width
            h: Height
            col: Color
        """
        ...
    def circ(self, x: float, y: float, r: float, col: int) -> None:
        """Draw a filled circle of radius r and color col at (x, y).

        Args:
            x: Center X coordinate
            y: Center Y coordinate
            r: Radius
            col: Color
        """
        ...
    def circb(self, x: float, y: float, r: float, col: int) -> None:
        """Draw the outline of a circle of radius r and color col at (x, y).

        Args:
            x: Center X coordinate
            y: Center Y coordinate
            r: Radius
            col: Color
        """
        ...
    def elli(self, x: float, y: float, w: float, h: float, col: int) -> None:
        """Draw a filled ellipse of width w, height h, and color col at (x, y).

        Args:
            x: X coordinate
            y: Y coordinate
            w: Width
            h: Height
            col: Color
        """
        ...
    def ellib(self, x: float, y: float, w: float, h: float, col: int) -> None:
        """Draw the outline of an ellipse of width w, height h, and color col at (x, y).

        Args:
            x: X coordinate
            y: Y coordinate
            w: Width
            h: Height
            col: Color
        """
        ...
    def tri(
        self, x1: float, y1: float, x2: float, y2: float, x3: float, y3: float, col: int
    ) -> None:
        """Draw a filled triangle with vertices (x1, y1), (x2, y2), (x3, y3) and color col.

        Args:
            x1: Vertex 1 X
            y1: Vertex 1 Y
            x2: Vertex 2 X
            y2: Vertex 2 Y
            x3: Vertex 3 X
            y3: Vertex 3 Y
            col: Color
        """
        ...
    def trib(
        self, x1: float, y1: float, x2: float, y2: float, x3: float, y3: float, col: int
    ) -> None:
        """Draw the outline of a triangle with vertices (x1, y1), (x2, y2), (x3, y3) and color col.

        Args:
            x1: Vertex 1 X
            y1: Vertex 1 Y
            x2: Vertex 2 X
            y2: Vertex 2 Y
            x3: Vertex 3 X
            y3: Vertex 3 Y
            col: Color
        """
        ...
    def fill(self, x: float, y: float, col: int) -> None:
        """Fill the area connected with the same color as (x, y) with color col.

        Args:
            x: X coordinate
            y: Y coordinate
            col: Fill color
        """
        ...
    def blt(
        self,
        x: float,
        y: float,
        img: int | Image,
        u: float,
        v: float,
        w: float,
        h: float,
        colkey: int | None = None,
        rotate: float = 0.0,
        scale: float = 1.0,
    ) -> None:
        """Copy the region of size (w, h) from (u, v) of image bank img (0-2 or Image instance) to (x, y). Negative w/h flips the image. colkey sets the transparent color. rotate and scale apply transformations.

        Args:
            x: Destination X
            y: Destination Y
            img: Image bank number (0-2) or Image instance
            u: Source X in the image bank
            v: Source Y in the image bank
            w: Width (negative to flip)
            h: Height (negative to flip)
            colkey: Transparent color. If omitted, no transparency.
            rotate: Rotation angle in degrees (centered on the copy region). Defaults to 0.
            scale: Scale factor (centered on the copy region). Defaults to 1.
        """
        ...
    def bltm(
        self,
        x: float,
        y: float,
        tm: int | Tilemap,
        u: float,
        v: float,
        w: float,
        h: float,
        colkey: int | None = None,
        rotate: float = 0.0,
        scale: float = 1.0,
    ) -> None:
        """Copy the region of size (w, h) from (u, v) of tilemap tm (0-7 or Tilemap instance) to (x, y). Each tile is 8x8 pixels stored as (image_tx, image_ty). Negative w/h flips the image. colkey sets the transparent color. rotate and scale apply transformations.

        Args:
            x: Destination X
            y: Destination Y
            tm: Tilemap number (0-7) or Tilemap instance
            u: Source X in the tilemap
            v: Source Y in the tilemap
            w: Width (negative to flip)
            h: Height (negative to flip)
            colkey: Transparent color. If omitted, no transparency.
            rotate: Rotation angle in degrees (centered on the copy region). Defaults to 0.
            scale: Scale factor (centered on the copy region). Defaults to 1.
        """
        ...
    def blt3d(
        self,
        x: float,
        y: float,
        w: float,
        h: float,
        img: int | Image,
        pos: tuple[float, float, float],
        rot: tuple[float, float, float],
        fov: float = 60.0,
        colkey: int | None = None,
    ) -> None:
        """Draw the image bank img (0-2 or Image instance) with perspective projection onto the screen rectangle (x, y, w, h). pos is the camera position where x, y match 2D coordinates and z is height. rot is the rotation in degrees. fov sets the field of view in degrees. colkey sets the transparent color.

        Args:
            x: Destination X
            y: Destination Y
            w: Display width
            h: Display height
            img: Image bank number (0-2) or Image instance
            pos: Camera position (x, y, z). x, y match 2D coordinates, z is height
            rot: Rotation in degrees. rot_x is vertical, rot_y is horizontal, rot_z is tilt
            fov: Field of view in degrees. Defaults to 60.
            colkey: Transparent color. If omitted, no transparency.
        """
        ...
    def bltm3d(
        self,
        x: float,
        y: float,
        w: float,
        h: float,
        tm: int | Tilemap,
        pos: tuple[float, float, float],
        rot: tuple[float, float, float],
        fov: float = 60.0,
        colkey: int | None = None,
    ) -> None:
        """Draw the tilemap tm (0-7 or Tilemap instance) with perspective projection onto the screen rectangle (x, y, w, h). pos is the camera position where x, y match 2D coordinates and z is height. rot is the rotation in degrees. fov sets the field of view in degrees. colkey sets the transparent color.

        Args:
            x: Destination X
            y: Destination Y
            w: Display width
            h: Display height
            tm: Tilemap number (0-7) or Tilemap instance
            pos: Camera position (x, y, z). x, y match 2D coordinates, z is height
            rot: Rotation in degrees. rot_x is vertical, rot_y is horizontal, rot_z is tilt
            fov: Field of view in degrees. Defaults to 60.
            colkey: Transparent color. If omitted, no transparency.
        """
        ...
    def text(
        self, x: float, y: float, s: str, col: int, font: Font | None = None
    ) -> None:
        """Draw a string s in color col at (x, y).

        Args:
            x: X coordinate
            y: Y coordinate
            s: String to draw
            col: Color
            font: Custom font. If omitted, the standard font is used.
        """
        ...

# Tilemap class
class Tilemap:
    width: int
    """The width of the tilemap."""
    height: int
    """The height of the tilemap."""
    imgsrc: int | Image
    """The image bank (0-2) or Image instance referenced by the tilemap."""

    def __init__(self, width: int, height: int, img: int | Image) -> None:
        """Create a new Tilemap instance.

        Args:
            width: Tilemap width
            height: Tilemap height
            img: Image bank number (0-2) or Image instance

        Returns:
            New Tilemap instance
        """
        ...
    @staticmethod
    def from_tmx(filename: str, layer: int) -> Tilemap:
        """Create a Tilemap instance from a TMX file.

        Args:
            filename: TMX file name
            layer: Layer number (0-)

        Returns:
            Tilemap instance from TMX file
        """
        ...
    def data_ptr(self) -> Any:
        """Return the raw data pointer of the tilemap as a ctypes c_uint16 array (4 bytes per tile: image_tx, image_ty).

        Returns:
            Raw data pointer
        """
        ...
    def set(self, x: int, y: int, data: list[str]) -> None:
        """Set the tilemap at (x, y) using a list of strings. Each tile is a 4-digit hex value representing (image_tx, image_ty), separated by spaces.

        Args:
            x: X coordinate
            y: Y coordinate
            data: Tilemap data as a list of strings

        Example::
            pyxel.tilemaps[0].set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])
        """
        ...
    def load(self, x: int, y: int, filename: str, layer: int) -> None:
        """Load the layer (0-) from the TMX file at (x, y).

        Args:
            x: X coordinate
            y: Y coordinate
            filename: TMX file name
            layer: Layer number (0-)
        """
        ...
    @overload
    def clip(self) -> None: ...
    @overload
    def clip(self, x: float, y: float, w: float, h: float) -> None: ...
    def clip(
        self,
        x: float | None = None,
        y: float | None = None,
        w: float | None = None,
        h: float | None = None,
    ) -> None:
        """Set the drawing area of the tilemap from (x, y) with a width of w and a height of h. Call clip() to reset to the full tilemap. Call without arguments to reset the drawing area to the full tilemap.

        Args:
            x: X coordinate of the upper-left corner
            y: Y coordinate of the upper-left corner
            w: Width of the clipping area
            h: Height of the clipping area
        """
        ...
    @overload
    def camera(self) -> None: ...
    @overload
    def camera(self, x: float, y: float) -> None: ...
    def camera(
        self,
        x: float | None = None,
        y: float | None = None,
    ) -> None:
        """Set the drawing offset to (x, y). All subsequent drawing operations will be shifted by (-x, -y). Call without arguments to reset the drawing offset to (0, 0).

        Args:
            x: X coordinate
            y: Y coordinate
        """
        ...
    def cls(self, tile: tuple[int, int]) -> None:
        """Clear the tilemap with tile.

        Args:
            tile: Tile (image_x, image_y)
        """
        ...
    def pget(self, x: float, y: float) -> tuple[int, int]:
        """Get the tile at (x, y). A tile is a tuple of (image_tx, image_ty).

        Args:
            x: X coordinate
            y: Y coordinate

        Returns:
            (image_tx, image_ty)
        """
        ...
    def pset(self, x: float, y: float, tile: tuple[int, int]) -> None:
        """Set a tile at (x, y). A tile is a tuple of (image_tx, image_ty).

        Args:
            x: X coordinate
            y: Y coordinate
            tile: Tile as (image_tx, image_ty)
        """
        ...
    def line(
        self, x1: float, y1: float, x2: float, y2: float, tile: tuple[int, int]
    ) -> None:
        """Draw a line of tile from (x1, y1) to (x2, y2).

        Args:
            x1: Start X coordinate
            y1: Start Y coordinate
            x2: End X coordinate
            y2: End Y coordinate
            tile: Tile (image_x, image_y)
        """
        ...
    def rect(
        self, x: float, y: float, w: float, h: float, tile: tuple[int, int]
    ) -> None:
        """Draw a filled rectangle of width w, height h, and tile at (x, y).

        Args:
            x: X coordinate
            y: Y coordinate
            w: Width
            h: Height
            tile: Tile (image_x, image_y)
        """
        ...
    def rectb(
        self, x: float, y: float, w: float, h: float, tile: tuple[int, int]
    ) -> None:
        """Draw the outline of a rectangle of width w, height h, and tile at (x, y).

        Args:
            x: X coordinate
            y: Y coordinate
            w: Width
            h: Height
            tile: Tile (image_x, image_y)
        """
        ...
    def circ(self, x: float, y: float, r: float, tile: tuple[int, int]) -> None:
        """Draw a filled circle of radius r and tile at (x, y).

        Args:
            x: Center X coordinate
            y: Center Y coordinate
            r: Radius
            tile: Tile (image_x, image_y)
        """
        ...
    def circb(self, x: float, y: float, r: float, tile: tuple[int, int]) -> None:
        """Draw the outline of a circle of radius r and tile at (x, y).

        Args:
            x: Center X coordinate
            y: Center Y coordinate
            r: Radius
            tile: Tile (image_x, image_y)
        """
        ...
    def elli(
        self, x: float, y: float, w: float, h: float, tile: tuple[int, int]
    ) -> None:
        """Draw a filled ellipse of width w, height h, and tile at (x, y).

        Args:
            x: X coordinate
            y: Y coordinate
            w: Width
            h: Height
            tile: Tile (image_x, image_y)
        """
        ...
    def ellib(
        self, x: float, y: float, w: float, h: float, tile: tuple[int, int]
    ) -> None:
        """Draw the outline of an ellipse of width w, height h, and tile at (x, y).

        Args:
            x: X coordinate
            y: Y coordinate
            w: Width
            h: Height
            tile: Tile (image_x, image_y)
        """
        ...
    def tri(
        self,
        x1: float,
        y1: float,
        x2: float,
        y2: float,
        x3: float,
        y3: float,
        tile: tuple[int, int],
    ) -> None:
        """Draw a filled triangle with vertices (x1, y1), (x2, y2), (x3, y3) and tile.

        Args:
            x1: Vertex 1 X
            y1: Vertex 1 Y
            x2: Vertex 2 X
            y2: Vertex 2 Y
            x3: Vertex 3 X
            y3: Vertex 3 Y
            tile: Tile (image_x, image_y)
        """
        ...
    def trib(
        self,
        x1: float,
        y1: float,
        x2: float,
        y2: float,
        x3: float,
        y3: float,
        tile: tuple[int, int],
    ) -> None:
        """Draw the outline of a triangle with vertices (x1, y1), (x2, y2), (x3, y3) and tile.

        Args:
            x1: Vertex 1 X
            y1: Vertex 1 Y
            x2: Vertex 2 X
            y2: Vertex 2 Y
            x3: Vertex 3 X
            y3: Vertex 3 Y
            tile: Tile (image_x, image_y)
        """
        ...
    def fill(self, x: float, y: float, tile: tuple[int, int]) -> None:
        """Fill the area connected with the same tile as (x, y) with tile.

        Args:
            x: X coordinate
            y: Y coordinate
            tile: Tile (image_x, image_y)
        """
        ...
    def collide(
        self,
        x: float,
        y: float,
        w: float,
        h: float,
        dx: float,
        dy: float,
        walls: list[tuple[int, int]],
    ) -> tuple[float, float]:
        """Resolve collisions after applying pixel movement (dx, dy) to the pixel rectangle at (x, y) with pixel size (w, h), and return the adjusted (dx, dy). walls is a list of tile coordinates that act as obstacles.

        Args:
            x: Rectangle X position
            y: Rectangle Y position
            w: Rectangle width
            h: Rectangle height
            dx: Movement in X
            dy: Movement in Y
            walls: List of wall tiles (image_tx, image_ty)

        Returns:
            Adjusted (dx, dy)
        """
        ...
    def blt(
        self,
        x: float,
        y: float,
        tm: int | Tilemap,
        u: float,
        v: float,
        w: float,
        h: float,
        tilekey: tuple[int, int] | None = None,
        rotate: float = 0.0,
        scale: float = 1.0,
    ) -> None:
        """Copy the region of size (w, h) from (u, v) of tilemap tm (0-7 or Tilemap instance) to (x, y). Negative w/h flips the tilemap. tilekey sets the transparent tile. rotate and scale apply transformations.

        Args:
            x: Destination X
            y: Destination Y
            tm: Tilemap number (0-7) or Tilemap instance
            u: Source X in the image bank
            v: Source Y in the image bank
            w: Width (negative to flip)
            h: Height (negative to flip)
            tilekey: Transparent tile
            rotate: Rotation angle in degrees (centered on the copy region). Defaults to 0.
            scale: Scale factor (centered on the copy region). Defaults to 1.
        """
        ...

# Channel class
class Channel:
    gain: float
    """The gain (volume) of the channel. Defaults to 0.125."""
    detune: int
    """The detune value for pitch adjustment. Defaults to 0."""

    def __init__(self) -> None:
        """Create a new Channel instance.

        Returns:
            New Channel instance
        """
        ...
    def play(
        self,
        snd: int | Seq[int] | Sound | Seq[Sound] | str,
        sec: float = 0,
        loop: bool = False,
        resume: bool = False,
    ) -> None:
        """Play the sound snd on this channel. snd can be a sound number, a list, a Sound instance, a list of Sounds, or an MML string.

        Args:
            snd: Sound number (0-63), list of numbers, Sound instance, list of Sounds, or MML string
            sec: Playback start position in seconds. Defaults to 0.
            loop: Loop playback. Defaults to False.
            resume: Resume previous sound after playback ends. Defaults to False.
        """
        ...
    def stop(self) -> None:
        """Stop playback on this channel."""
        ...
    def play_pos(self) -> tuple[int, float] | None:
        """Get the playback position as a tuple of (sound_index, sec). Return None when playback has stopped.

        Returns:
            (sound_index, sec) or None
        """
        ...

# Tone class
class Tone:
    mode: int
    """Tone mode (0: Wavetable, 1: ShortPeriodNoise, 2: LongPeriodNoise)."""
    sample_bits: int
    """Sample bits for the wavetable. Defaults to 4."""
    wavetable: Seq[int]
    """Wavetable data as a list of sample values. Each value must be in range 0 to (2^sample_bits - 1)."""
    waveform: Seq[int]  # Deprecated: use wavetable
    """Deprecated alias of wavetable."""
    gain: float
    """Tone gain. Defaults to 1.0."""

    def __init__(self) -> None:
        """Create a new Tone instance.

        Returns:
            New Tone instance
        """
        ...

# Sound class
class Sound:
    notes: Seq[int]
    """List of notes (0-59). Higher values produce higher pitches. 33 = 'A2' (440 Hz). Rests are -1."""
    tones: Seq[int]
    """List of tones (0: Triangle, 1: Square, 2: Pulse, 3: Noise)."""
    volumes: Seq[int]
    """List of volumes (0-7)."""
    effects: Seq[int]
    """List of effects (0: None, 1: Slide, 2: Vibrato, 3: FadeOut, 4: Half-FadeOut, 5: Quarter-FadeOut)."""
    speed: int
    """Playback speed. 1 is the fastest, and the larger the number, the slower the playback speed. At 120, one note equals 1 second."""

    def __init__(self) -> None:
        """Create a new Sound instance.

        Returns:
            New Sound instance
        """
        ...
    def set(
        self,
        notes: str,
        tones: str,
        volumes: str,
        effects: str,
        speed: int,
    ) -> None:
        """Set notes, tones, volumes, and effects using strings. If the tones, volumes, or effects string is shorter than the notes, it repeats from the beginning.

        Args:
            notes: Note string
            tones: Tone string
            volumes: Volume string
            effects: Effect string
            speed: Playback speed
        """
        ...
    def set_notes(self, notes: str) -> None:
        """Set the notes using a string made of note names (CDEFGAB), optional sharp (#) or flat (-), octave (0-4), and rests (R). Case-insensitive, whitespace is ignored.

        Args:
            notes: Notes string

        Example::
            pyxel.sounds[0].set_notes("g2b-2d3r rf3f3f3")
        """
        ...
    def set_tones(self, tones: str) -> None:
        """Set the tones with a string of TSPN (or 0-9 for custom tone numbers). Case-insensitive, whitespace is ignored.

        Args:
            tones: Tones string

        Example::
            pyxel.sounds[0].set_tones("ttss pppn")
        """
        ...
    def set_volumes(self, volumes: str) -> None:
        """Set the volumes with a string of 01234567. Whitespace is ignored.

        Args:
            volumes: Volumes string

        Example::
            pyxel.sounds[0].set_volumes("7777 7531")
        """
        ...
    def set_effects(self, effects: str) -> None:
        """Set the effects with a string of NSVFHQ. Case-insensitive, whitespace is ignored.

        Args:
            effects: Effects string

        Example::
            pyxel.sounds[0].set_effects("nfnf nvvs")
        """
        ...
    def mml(self, code: str | None = None) -> None:
        """Switch to MML mode with the given MML string. In MML mode, normal parameters such as notes and speed are ignored. For available MML commands, see the Pyxel MML Commands page. Call without arguments to exit MML mode and return to normal mode.

        Args:
            code: MML string

        Example::
            pyxel.sounds[0].mml("T120 Q90 @1 V100 O5 L8 C4&C<G16R16>C.")
        """
        ...
    def pcm(self, filename: str | None = None) -> None:
        """Load an audio file (WAV/OGG) for playback. Call without arguments to exit PCM mode and return to normal mode.

        Args:
            filename: Audio file name (WAV/OGG)

        Example::
            pyxel.sounds[0].pcm("sounds/bgm.ogg")
        """
        ...
    def save(self, filename: str, sec: float, ffmpeg: bool = False) -> None:
        """Create a WAV file of the sound for the specified duration in seconds.

        Args:
            filename: Output WAV file name
            sec: Duration in seconds
            ffmpeg: Also create MP4 file (requires FFmpeg). Defaults to False.
        """
        ...
    def total_sec(self) -> float | None:
        """Return the playback time in seconds. Return None for infinite loops.

        Returns:
            Playback time in seconds, or None for infinite loops
        """
        ...

# Music class
class Music:
    seqs: Seq[Seq[int]]
    """A two-dimensional list of sounds (0-63) across multiple channels."""

    def __init__(self) -> None:
        """Create a new Music instance.

        Returns:
            New Music instance
        """
        ...
    def set(
        self,
        *seqs: list[int],
    ) -> None:
        """Set the lists of sounds (0-63) for each channel. An empty list means the channel is not used.

        Args:
            seq0, seq1, ...: Sound lists for each channel. Empty list = unused.

        Example::
            pyxel.musics[0].set([0, 1], [], [3])
        """
        ...
    def save(self, filename: str, sec: float, ffmpeg: bool = False) -> None:
        """Create a WAV file of the music for the specified duration in seconds.

        Args:
            filename: Output WAV file name
            sec: Duration in seconds
            ffmpeg: Also create MP4 file (requires FFmpeg). Defaults to False.
        """
        ...

# System
width: int
"""The width of the screen."""
height: int
"""The height of the screen."""
frame_count: int
"""The number of elapsed frames."""

def init(
    width: int,
    height: int,
    title: str = "Pyxel",
    fps: int = 30,
    quit_key: int = KEY_ESCAPE,
    display_scale: int | None = None,
    capture_scale: int = 2,
    capture_sec: int = 10,
    headless: bool = False,
) -> None:
    """Initialize the Pyxel application with the screen size (width, height).

    Args:
        width: Screen width
        height: Screen height
        title: Window title. Defaults to "Pyxel".
        fps: Frame rate. Defaults to 30.
        quit_key: Key to quit the application. Defaults to KEY_ESCAPE.
        display_scale: Display scale factor. If omitted, automatically determined.
        capture_scale: Screen capture scale factor. Defaults to 2.
        capture_sec: Maximum recording time for screen capture video. Defaults to 10.
        headless: Run without a window. Defaults to False.

    Example::
        pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)
    """
    ...

def run(update: Callable[[], None], draw: Callable[[], None]) -> None:
    """Start the Pyxel application and call the update function for frame update and the draw function for drawing.

    Args:
        update: Function to update game logic each frame
        draw: Function to draw the screen each frame
    """
    ...

def show() -> None:
    """Show the screen and wait until the quit key is pressed."""
    ...

def flip() -> None:
    """Refresh the screen by one frame. The application exits when the quit key is pressed. This function is not available in the web version."""
    ...

def quit() -> None:
    """Quit the Pyxel application."""
    ...

def reset() -> None:
    """Restart the Pyxel application from the beginning."""
    ...

def title(title: str) -> None:
    """Set the window title.

    Args:
        title: Window title
    """
    ...

def icon(data: list[str], scale: int, colkey: int | None = None) -> None:
    """Set the application icon. Specify the icon image as a list of strings.

    Args:
        data: Icon image as a list of strings
        scale: Scale factor
        colkey: Transparent color. If omitted, no transparency.
    """
    ...

def perf_monitor(enabled: bool) -> None:
    """Show or hide the performance monitor.

    Args:
        enabled: Enable the performance monitor
    """
    ...

def integer_scale(enabled: bool) -> None:
    """Enable integer scaling for the display.

    Args:
        enabled: Enable integer scaling
    """
    ...

def screen_mode(scr: int) -> None:
    """Set the screen mode (0: crisp, 1: smooth, 2: retro).

    Args:
        scr: Screen mode number (0: crisp, 1: smooth, 2: retro)
    """
    ...

def fullscreen(enabled: bool) -> None:
    """Set whether to run in fullscreen mode.

    Args:
        enabled: Enable fullscreen
    """
    ...

def resize(width: int, height: int) -> None:
    """Change the screen size at runtime.

    Args:
        width: New screen width in pixels
        height: New screen height in pixels
    """
    ...

# Resource
def load(
    filename: str,
    exclude_images: bool = False,
    exclude_tilemaps: bool = False,
    exclude_sounds: bool = False,
    exclude_musics: bool = False,
) -> None:
    """Load the resource file (.pyxres). If an option is set to True, the corresponding resource will be excluded from loading.

    Args:
        filename: Resource file name
        exclude_images: Exclude image banks. Defaults to False.
        exclude_tilemaps: Exclude tilemaps. Defaults to False.
        exclude_sounds: Exclude sounds. Defaults to False.
        exclude_musics: Exclude music tracks. Defaults to False.

    Note:
        If a palette file (.pyxpal) with the same name exists, the palette display colors will also be updated.
    """
    ...

def save(
    filename: str,
    exclude_images: bool = False,
    exclude_tilemaps: bool = False,
    exclude_sounds: bool = False,
    exclude_musics: bool = False,
) -> None:
    """Save the resource file (.pyxres). If an option is set to True, the corresponding resource will be excluded from saving.

    Args:
        filename: Resource file name
        exclude_images: Exclude image banks. Defaults to False.
        exclude_tilemaps: Exclude tilemaps. Defaults to False.
        exclude_sounds: Exclude sounds. Defaults to False.
        exclude_musics: Exclude music tracks. Defaults to False.
    """
    ...

def load_pal(filename: str) -> None:
    """Load a palette file (.pyxpal).

    Args:
        filename: Palette file name
    """
    ...

def save_pal(filename: str) -> None:
    """Save a palette file (.pyxpal).

    Args:
        filename: Palette file name
    """
    ...

def screenshot(filename: str | None = None, scale: int = 2) -> None:
    """Take a screenshot.

    Args:
        filename: File name. If omitted, saved to desktop.
        scale: Scale factor. Defaults to capture_scale.
    """
    ...

def screencast(filename: str | None = None, scale: int = 2) -> None:
    """Save the screen recording as a GIF file.

    Args:
        filename: File name. If omitted, saved to desktop.
        scale: Scale factor. Defaults to capture_scale.
    """
    ...

def reset_screencast() -> None:
    """Reset the screen recording buffer."""
    ...

def user_data_dir(vendor_name: str, app_name: str) -> str:
    """Return the user data directory created based on vendor_name and app_name. If the directory does not exist, it will be created automatically.

    Args:
        vendor_name: Vendor name
        app_name: Application name

    Returns:
        Path to the user data directory

    Example::
        pyxel.user_data_dir("Takashi Kitao", "Pyxel Shooter")
    """
    ...

# Input
mouse_x: int
"""The current x position of the mouse cursor."""
mouse_y: int
"""The current y position of the mouse cursor."""
mouse_wheel: int
"""The current value of the mouse wheel."""
input_keys: list[int]
"""List of keys input in the current frame."""
input_text: str
"""Text input in the current frame."""
dropped_files: list[str]
"""List of files dropped in the current frame."""

def btn(key: int) -> bool:
    """Return True if the key is pressed, otherwise return False.

    Args:
        key: Key code

    Returns:
        True if pressed
    """
    ...

def btnp(key: int, hold: int = 0, repeat: int = 0) -> bool:
    """Return True if the key is pressed in that frame. When hold and repeat are specified, after holding the key for hold frames, return True every repeat frames.

    Args:
        key: Key code
        hold: Frames to hold before repeat starts. Defaults to 0.
        repeat: Repeat interval in frames. If 0, no repeat.

    Returns:
        True if pressed in that frame
    """
    ...

def btnr(key: int) -> bool:
    """Return True if the key is released in that frame, otherwise return False.

    Args:
        key: Key code

    Returns:
        True if released in that frame
    """
    ...

def btnv(key: int) -> int:
    """Return the analog value of the specified key (e.g., gamepad axis value).

    Args:
        key: Key code

    Returns:
        Analog value of the key
    """
    ...

def mouse(visible: bool) -> None:
    """Show the mouse cursor if visible is True, and hide it if False. The cursor position continues to update even when hidden.

    Args:
        visible: Show or hide the cursor
    """
    ...

def set_btn(key: int, state: bool) -> None:
    """Set the press/release state of the specified key. Mainly for headless mode input simulation.

    Args:
        key: Target key
        state: True for press, False for release
    """
    ...

def set_btnv(key: int, val: int) -> None:
    """Set the analog value of the specified key. Mainly for headless mode input simulation.

    Args:
        key: Target key
        val: Analog value to set
    """
    ...

def set_mouse_pos(x: float, y: float) -> None:
    """Set the mouse cursor position. Mainly for headless mode input simulation.

    Args:
        x: X coordinate
        y: Y coordinate
    """
    ...

def set_input_text(text: str) -> None:
    """Set the text input for the current frame. Replaces any existing text. Mainly for headless mode input simulation.

    Args:
        text: Text input
    """
    ...

def set_dropped_files(files: list[str]) -> None:
    """Set the dropped file list for the current frame. Replaces any existing list. Mainly for headless mode input simulation.

    Args:
        files: List of file paths
    """
    ...

# Graphics
colors: Seq[int]
"""List of the palette display colors. Specified by 24-bit numerical value. Supports Python list operations."""
images: Seq[Image]
"""List of the image banks (instances of the Image class) (0-2)."""
tilemaps: Seq[Tilemap]
"""List of the tilemaps (instances of the Tilemap class) (0-7)."""
screen: Image
"""The screen image (Image class instance)."""
cursor: Image
"""The cursor image (Image class instance)."""
font: Image
"""The font image (Image class instance)."""

@overload
def clip() -> None: ...
@overload
def clip(x: float, y: float, w: float, h: float) -> None: ...
def clip(
    x: float | None = None,
    y: float | None = None,
    w: float | None = None,
    h: float | None = None,
) -> None:
    """Set the drawing area of the screen from (x, y) with a width of w and a height of h. Call clip() to reset to full screen. Call without arguments to reset the drawing area to full screen.

    Args:
        x: X coordinate of the upper-left corner
        y: Y coordinate of the upper-left corner
        w: Width of the clipping area
        h: Height of the clipping area
    """
    ...

@overload
def camera() -> None: ...
@overload
def camera(x: float, y: float) -> None: ...
def camera(
    x: float | None = None,
    y: float | None = None,
) -> None:
    """Set the drawing offset to (x, y). All subsequent drawing operations will be shifted by (-x, -y). Call without arguments to reset the drawing offset to (0, 0).

    Args:
        x: X coordinate
        y: Y coordinate
    """
    ...

@overload
def pal() -> None: ...
@overload
def pal(col1: int, col2: int) -> None: ...
def pal(col1: int | None = None, col2: int | None = None) -> None:
    """Replace color col1 with col2 when drawing. Call without arguments to reset the palette to the initial state.

    Args:
        col1: Color to replace
        col2: Replacement color
    """
    ...

def dither(alpha: float) -> None:
    """Apply dithering (pseudo-transparency) when drawing. Set alpha in the range 0.0-1.0.

    Args:
        alpha: Opacity (0.0: transparent, 1.0: opaque)
    """
    ...

def cls(col: int) -> None:
    """Clear the screen with color col.

    Args:
        col: Color
    """
    ...

def pget(x: float, y: float) -> int:
    """Get the color of the pixel at (x, y).

    Args:
        x: X coordinate
        y: Y coordinate

    Returns:
        Color of the pixel
    """
    ...

def pset(x: float, y: float, col: int) -> None:
    """Draw a pixel of color col at (x, y).

    Args:
        x: X coordinate
        y: Y coordinate
        col: Color
    """
    ...

def line(x1: float, y1: float, x2: float, y2: float, col: int) -> None:
    """Draw a line of color col from (x1, y1) to (x2, y2).

    Args:
        x1: Start X coordinate
        y1: Start Y coordinate
        x2: End X coordinate
        y2: End Y coordinate
        col: Color
    """
    ...

def rect(x: float, y: float, w: float, h: float, col: int) -> None:
    """Draw a filled rectangle of width w, height h, and color col at (x, y).

    Args:
        x: X coordinate
        y: Y coordinate
        w: Width
        h: Height
        col: Color
    """
    ...

def rectb(x: float, y: float, w: float, h: float, col: int) -> None:
    """Draw the outline of a rectangle of width w, height h, and color col at (x, y).

    Args:
        x: X coordinate
        y: Y coordinate
        w: Width
        h: Height
        col: Color
    """
    ...

def circ(x: float, y: float, r: float, col: int) -> None:
    """Draw a filled circle of radius r and color col at (x, y).

    Args:
        x: Center X coordinate
        y: Center Y coordinate
        r: Radius
        col: Color
    """
    ...

def circb(x: float, y: float, r: float, col: int) -> None:
    """Draw the outline of a circle of radius r and color col at (x, y).

    Args:
        x: Center X coordinate
        y: Center Y coordinate
        r: Radius
        col: Color
    """
    ...

def elli(x: float, y: float, w: float, h: float, col: int) -> None:
    """Draw a filled ellipse of width w, height h, and color col at (x, y).

    Args:
        x: X coordinate
        y: Y coordinate
        w: Width
        h: Height
        col: Color
    """
    ...

def ellib(x: float, y: float, w: float, h: float, col: int) -> None:
    """Draw the outline of an ellipse of width w, height h, and color col at (x, y).

    Args:
        x: X coordinate
        y: Y coordinate
        w: Width
        h: Height
        col: Color
    """
    ...

def tri(
    x1: float,
    y1: float,
    x2: float,
    y2: float,
    x3: float,
    y3: float,
    col: int,
) -> None:
    """Draw a filled triangle with vertices (x1, y1), (x2, y2), (x3, y3) and color col.

    Args:
        x1: Vertex 1 X
        y1: Vertex 1 Y
        x2: Vertex 2 X
        y2: Vertex 2 Y
        x3: Vertex 3 X
        y3: Vertex 3 Y
        col: Color
    """
    ...

def trib(
    x1: float,
    y1: float,
    x2: float,
    y2: float,
    x3: float,
    y3: float,
    col: int,
) -> None:
    """Draw the outline of a triangle with vertices (x1, y1), (x2, y2), (x3, y3) and color col.

    Args:
        x1: Vertex 1 X
        y1: Vertex 1 Y
        x2: Vertex 2 X
        y2: Vertex 2 Y
        x3: Vertex 3 X
        y3: Vertex 3 Y
        col: Color
    """
    ...

def fill(x: float, y: float, col: int) -> None:
    """Fill the area connected with the same color as (x, y) with color col.

    Args:
        x: X coordinate
        y: Y coordinate
        col: Fill color
    """
    ...

def blt(
    x: float,
    y: float,
    img: int | Image,
    u: float,
    v: float,
    w: float,
    h: float,
    colkey: int | None = None,
    rotate: float = 0.0,
    scale: float = 1.0,
) -> None:
    """Copy the region of size (w, h) from (u, v) of image bank img (0-2 or Image instance) to (x, y). Negative w/h flips the image. colkey sets the transparent color. rotate and scale apply transformations.

    Args:
        x: Destination X
        y: Destination Y
        img: Image bank number (0-2) or Image instance
        u: Source X in the image bank
        v: Source Y in the image bank
        w: Width (negative to flip)
        h: Height (negative to flip)
        colkey: Transparent color. If omitted, no transparency.
        rotate: Rotation angle in degrees (centered on the copy region). Defaults to 0.
        scale: Scale factor (centered on the copy region). Defaults to 1.
    """
    ...

def bltm(
    x: float,
    y: float,
    tm: int | Tilemap,
    u: float,
    v: float,
    w: float,
    h: float,
    colkey: int | None = None,
    rotate: float = 0.0,
    scale: float = 1.0,
) -> None:
    """Copy the region of size (w, h) from (u, v) of tilemap tm (0-7 or Tilemap instance) to (x, y). Each tile is 8x8 pixels stored as (image_tx, image_ty). Negative w/h flips the image. colkey sets the transparent color. rotate and scale apply transformations.

    Args:
        x: Destination X
        y: Destination Y
        tm: Tilemap number (0-7) or Tilemap instance
        u: Source X in the tilemap
        v: Source Y in the tilemap
        w: Width (negative to flip)
        h: Height (negative to flip)
        colkey: Transparent color. If omitted, no transparency.
        rotate: Rotation angle in degrees (centered on the copy region). Defaults to 0.
        scale: Scale factor (centered on the copy region). Defaults to 1.
    """
    ...

def blt3d(
    x: float,
    y: float,
    w: float,
    h: float,
    img: int | Image,
    pos: tuple[float, float, float],
    rot: tuple[float, float, float],
    fov: float = 60.0,
    colkey: int | None = None,
) -> None:
    """Draw the image bank img (0-2 or Image instance) with perspective projection onto the screen rectangle (x, y, w, h). pos is the camera position where x, y match 2D coordinates and z is height. rot is the rotation in degrees. fov sets the field of view in degrees. colkey sets the transparent color.

    Args:
        x: Destination X
        y: Destination Y
        w: Display width
        h: Display height
        img: Image bank number (0-2) or Image instance
        pos: Camera position (x, y, z). x, y match 2D coordinates, z is height
        rot: Rotation in degrees. rot_x is vertical, rot_y is horizontal, rot_z is tilt
        fov: Field of view in degrees. Defaults to 60.
        colkey: Transparent color. If omitted, no transparency.
    """
    ...

def bltm3d(
    x: float,
    y: float,
    w: float,
    h: float,
    tm: int | Tilemap,
    pos: tuple[float, float, float],
    rot: tuple[float, float, float],
    fov: float = 60.0,
    colkey: int | None = None,
) -> None:
    """Draw the tilemap tm (0-7 or Tilemap instance) with perspective projection onto the screen rectangle (x, y, w, h). pos is the camera position where x, y match 2D coordinates and z is height. rot is the rotation in degrees. fov sets the field of view in degrees. colkey sets the transparent color.

    Args:
        x: Destination X
        y: Destination Y
        w: Display width
        h: Display height
        tm: Tilemap number (0-7) or Tilemap instance
        pos: Camera position (x, y, z). x, y match 2D coordinates, z is height
        rot: Rotation in degrees. rot_x is vertical, rot_y is horizontal, rot_z is tilt
        fov: Field of view in degrees. Defaults to 60.
        colkey: Transparent color. If omitted, no transparency.
    """
    ...

def text(x: float, y: float, s: str, col: int, font: Font | None = None) -> None:
    """Draw a string s in color col at (x, y).

    Args:
        x: X coordinate
        y: Y coordinate
        s: String to draw
        col: Color
        font: Custom font. If omitted, the standard font is used.
    """
    ...

# Audio
channels: Seq[Channel]
"""List of the channels (instances of the Channel class) (0-3)."""
tones: Seq[Tone]
"""List of the tone definitions (instances of the Tone class) (0-3)."""
sounds: Seq[Sound]
"""List of the sounds (instances of the Sound class) (0-63)."""
musics: Seq[Music]
"""List of music tracks (instances of the Music class) (0-7)."""

def play(
    ch: int,
    snd: int | Seq[int] | Sound | Seq[Sound] | str,
    sec: float = 0,
    loop: bool = False,
    resume: bool = False,
) -> None:
    """Play the sound snd on channel ch (0-3). snd can be a sound number (0-63), a list of numbers, a Sound instance, a list of Sounds, or an MML string.

    Args:
        ch: Channel number (0-3)
        snd: Sound number (0-63), list of numbers, Sound instance, list of Sounds, or MML string
        sec: Playback start position in seconds. Defaults to 0.
        loop: Loop playback. Defaults to False.
        resume: Resume previous sound after playback ends. Defaults to False.
    """
    ...

def playm(
    msc: int,
    sec: float = 0,
    loop: bool = False,
) -> None:
    """Play the music msc (0-7).

    Args:
        msc: Music number (0-7)
        sec: Playback start position in seconds. Defaults to 0.
        loop: Loop playback. Defaults to False.
    """
    ...

def stop(ch: int | None = None) -> None:
    """Stop playback of the specified channel ch (0-3). Call without arguments to stop playback of all channels.

    Args:
        ch: Channel number (0-3)
    """
    ...

def play_pos(ch: int) -> tuple[int, float] | None:
    """Get the sound playback position of channel ch (0-3) as a tuple of (sound_index, sec). Return None when playback has stopped.

    Args:
        ch: Channel number (0-3)

    Returns:
        (sound_index, sec) or None
    """
    ...

def gen_bgm(
    preset: int,
    transp: int,
    instr: int,
    seed: int,
    play: bool = False,
) -> list[str]:
    """Generate a BGM MML list using an algorithm. preset (0-7) selects the preset, instr (0-3) selects the instrumentation.

    Args:
        preset: Preset number (0-7). 0-1: title, departure (medium tempo), 2-3: town, peaceful (slow tempo), 4-5: field, adventure (medium tempo), 6-7: battle, crisis (fast tempo)
        transp: Transpose in semitones (-5 to +5).
        instr: Instrumentation (0-3). 0: melody+reverb+bass (3ch), 1: melody+bass+drums (3ch), 2: melody+sub+bass (3ch), 3: melody+sub+bass+drums (4ch).
        seed: Random seed
        play: Play the generated MML. Defaults to False.

    Returns:
        List of MML strings
    """
    ...

# Math
def ceil(x: float) -> int:
    """Return the smallest integer greater than or equal to x.

    Args:
        x: Value

    Returns:
        Smallest integer >= x
    """
    ...

def floor(x: float) -> int:
    """Return the largest integer less than or equal to x.

    Args:
        x: Value

    Returns:
        Largest integer <= x
    """
    ...

def clamp(x: int | float, lower: int | float, upper: int | float) -> int | float:
    """Return x clamped between lower and upper.

    Args:
        x: Value to clamp
        lower: Minimum value
        upper: Maximum value

    Returns:
        Clamped value
    """
    ...

def sgn(x: int | float) -> int | float:
    """Return 1 when x is positive, 0 when it is 0, and -1 when it is negative.

    Args:
        x: Value

    Returns:
        Sign of the value (1, 0, or -1)
    """
    ...

def sqrt(x: float) -> float:
    """Return the square root of x.

    Args:
        x: Value

    Returns:
        Square root of x
    """
    ...

def sin(deg: float) -> float:
    """Return the sine of deg degrees.

    Args:
        deg: Angle in degrees

    Returns:
        Sine value
    """
    ...

def cos(deg: float) -> float:
    """Return the cosine of deg degrees.

    Args:
        deg: Angle in degrees

    Returns:
        Cosine value
    """
    ...

def atan2(y: float, x: float) -> float:
    """Return the arctangent of y/x in degrees.

    Args:
        y: Y value
        x: X value

    Returns:
        Angle in degrees
    """
    ...

def rseed(seed: int) -> None:
    """Set the seed of the random number generator.

    Args:
        seed: Seed value (non-negative integer)
    """
    ...

def rndi(a: int, b: int) -> int:
    """Return a random integer from a to b (inclusive).

    Args:
        a: Minimum value (inclusive)
        b: Maximum value (inclusive)

    Returns:
        Random integer from a to b
    """
    ...

def rndf(a: float, b: float) -> float:
    """Return a random float from a to b (inclusive).

    Args:
        a: Minimum value (inclusive)
        b: Maximum value (inclusive)

    Returns:
        Random float from a to b
    """
    ...

def nseed(seed: int) -> None:
    """Set the seed of Perlin noise.

    Args:
        seed: Seed value (non-negative integer)
    """
    ...

def noise(x: float, y: float = 0, z: float = 0) -> float:
    """Return the Perlin noise value for the specified coordinates.

    Args:
        x: X coordinate
        y: Y coordinate. Defaults to 0.
        z: Z coordinate. Defaults to 0.

    Returns:
        Perlin noise value
    """
    ...
