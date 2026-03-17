# Pyxel API Reference

*This document was auto-generated from the [Pyxel API Reference](https://kitao.github.io/pyxel/web/api-reference/) web page, which also offers multilingual support.*

## System

### `width` — variable

The width of the screen.

- **Type:** `int`

### `height` — variable

The height of the screen.

- **Type:** `int`

### `frame_count` — variable

The number of elapsed frames.

- **Type:** `int`

### `init(width, height, title="Pyxel", fps=30, quit_key=KEY_ESCAPE, display_scale=None, capture_scale=2, capture_sec=10, headless=False)` — function

Initialize the Pyxel application with the screen size (width, height).

**Parameters:**

- `width` (*int*) — Screen width
- `height` (*int*) — Screen height
- `title` (*str*) — Window title (default: "Pyxel")
- `fps` (*int*) — Frame rate (default: 30)
- `quit_key` (*int*) — Key to quit the application (default: KEY_ESCAPE)
- `display_scale` (*int*) — Display scale factor (None for auto)
- `capture_scale` (*int*) — Screen capture scale factor (default: 2)
- `capture_sec` (*int*) — Maximum recording time for screen capture video (default: 10)
- `headless` (*bool*) — Run without window, audio device, or display (default: False)

**Example:**

```python
pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)
```

### `run(update, draw)` — function

Start the Pyxel application and call the update function for frame update and the draw function for drawing.

**Parameters:**

- `update` (*callable*) — Function to update game logic each frame
- `draw` (*callable*) — Function to draw the screen each frame

### `show()` — function

Show the screen and wait until the quit key is pressed.

### `flip()` — function

Refresh the screen by one frame. The application exits when the quit key is pressed. This function is not available in the web version.

### `quit()` — function

Quit the Pyxel application.

### `reset()` — function

Restart the Pyxel application from the beginning.

### `title(title)` — function *(Advanced)*

Set the window title.

**Parameters:**

- `title` (*str*) — Window title

### `icon(data, scale, colkey=None)` — function *(Advanced)*

Set the application icon. Specify the icon image as a list of strings.

**Parameters:**

- `data` (*list[str]*) — Icon image as a list of strings
- `scale` (*int*) — Scale factor
- `colkey` (*int/None*) — Transparent color

### `fullscreen(enabled)` — function *(Advanced)*

Set whether to run in fullscreen mode.

**Parameters:**

- `enabled` (*bool*) — Enable fullscreen

### `screen_mode(scr)` — function *(Advanced)*

Set the screen mode (0: crisp, 1: smooth, 2: retro).

**Parameters:**

- `scr` (*int*) — Screen mode number (0: crisp, 1: smooth, 2: retro)

### `perf_monitor(enabled)` — function *(Advanced)*

Show or hide the performance monitor.

**Parameters:**

- `enabled` (*bool*) — Enable the performance monitor

### `integer_scale(enabled)` — function *(Advanced)*

Enable integer scaling for the display.

**Parameters:**

- `enabled` (*bool*) — Enable integer scaling

## Resource

### `load(filename, exclude_images=False, exclude_tilemaps=False, exclude_sounds=False, exclude_musics=False)` — function

Load the resource file (.pyxres). If an option is set to True, the corresponding resource will be excluded from loading.

**Parameters:**

- `filename` (*str*) — Resource file path
- `exclude_images` (*bool*) — Exclude image banks
- `exclude_tilemaps` (*bool*) — Exclude tilemaps
- `exclude_sounds` (*bool*) — Exclude sounds
- `exclude_musics` (*bool*) — Exclude music tracks

**Note:** If a palette file (.pyxpal) with the same name exists, the palette display colors will also be updated.

### `user_data_dir(vendor_name, app_name)` — function *(Advanced)*

Return the user data directory created based on vendor_name and app_name. If the directory does not exist, it will be created automatically.

**Parameters:**

- `vendor_name` (*str*) — Vendor name
- `app_name` (*str*) — Application name

**Returns:** `str` — Path to the user data directory

**Example:**

```python
pyxel.user_data_dir("Takashi Kitao", "Pyxel Shooter")
```

### `save(filename, exclude_images=False, exclude_tilemaps=False, exclude_sounds=False, exclude_musics=False)` — function *(Advanced)*

Save the resource file (.pyxres). If an option is set to True, the corresponding resource will be excluded from saving.

**Parameters:**

- `filename` (*str*) — Resource file path
- `exclude_images` (*bool*) — Exclude image banks
- `exclude_tilemaps` (*bool*) — Exclude tilemaps
- `exclude_sounds` (*bool*) — Exclude sounds
- `exclude_musics` (*bool*) — Exclude music tracks

### `screenshot(scale=2)` — function *(Advanced)*

Take a screenshot.

**Parameters:**

- `scale` (*int*) — Scale factor

### `screencast(scale=2)` — function *(Advanced)*

Save the screen recording as a GIF file.

**Parameters:**

- `scale` (*int*) — Scale factor

### `reset_screencast()` — function *(Advanced)*

Reset the screen recording buffer.

### `load_pal(filename)` — function *(Advanced)*

Load a palette file (.pyxpal).

**Parameters:**

- `filename` (*str*) — Palette file path

### `save_pal(filename)` — function *(Advanced)*

Save a palette file (.pyxpal).

**Parameters:**

- `filename` (*str*) — Palette file path

## Input

### `mouse_x` — variable

The current x position of the mouse cursor.

- **Type:** `int`

### `mouse_y` — variable

The current y position of the mouse cursor.

- **Type:** `int`

### `mouse_wheel` — variable

The current value of the mouse wheel.

- **Type:** `int`

### `input_keys` — variable *(Advanced)*

List of keys input in the current frame.

- **Type:** `list[int]`

### `input_text` — variable *(Advanced)*

Text input in the current frame.

- **Type:** `str`

### `dropped_files` — variable *(Advanced)*

List of files dropped in the current frame.

- **Type:** `list[str]`

### `btn(key)` — function

Return True if the key is pressed, otherwise return False.

**Parameters:**

- `key` (*int*) — Key code

**Returns:** `bool` — True if pressed

### `btnp(key, hold=0, repeat=0)` — function

Return True if the key is pressed in that frame. When hold and repeat are specified, after holding the key for hold frames, return True every repeat frames.

**Parameters:**

- `key` (*int*) — Key code
- `hold` (*int*) — Frames to hold before repeat starts
- `repeat` (*int*) — Repeat interval in frames

**Returns:** `bool` — True if pressed in that frame

### `btnr(key)` — function

Return True if the key is released in that frame, otherwise return False.

**Parameters:**

- `key` (*int*) — Key code

**Returns:** `bool` — True if released in that frame

### `mouse(visible)` — function

Show the mouse cursor if visible is True, and hide it if False. The cursor position continues to update even when hidden.

**Parameters:**

- `visible` (*bool*) — Show or hide the cursor

### `btnv(key)` — function *(Advanced)*

Return the analog value of the specified key (e.g., gamepad axis value).

**Parameters:**

- `key` (*int*) — Key code

**Returns:** `int` — Analog value of the key

### `warp_mouse(x, y)` — function *(Advanced)*

Move the mouse cursor to the specified position.

**Parameters:**

- `x` (*float*) — X coordinate
- `y` (*float*) — Y coordinate

### Key Constants

**Special Keys:**

`KEY_UNKNOWN` `KEY_RETURN` `KEY_ESCAPE` `KEY_BACKSPACE` `KEY_TAB` `KEY_SPACE` `KEY_NONE`

**Letters:**

`KEY_A` `KEY_B` `KEY_C` `KEY_D` `KEY_E` `KEY_F` `KEY_G` `KEY_H` `KEY_I` `KEY_J` `KEY_K` `KEY_L` `KEY_M` `KEY_N` `KEY_O` `KEY_P` `KEY_Q` `KEY_R` `KEY_S` `KEY_T` `KEY_U` `KEY_V` `KEY_W` `KEY_X` `KEY_Y` `KEY_Z`

**Numbers:**

`KEY_0` `KEY_1` `KEY_2` `KEY_3` `KEY_4` `KEY_5` `KEY_6` `KEY_7` `KEY_8` `KEY_9`

**Symbols:**

`KEY_EXCLAIM` `KEY_QUOTEDBL` `KEY_HASH` `KEY_PERCENT` `KEY_DOLLAR` `KEY_AMPERSAND` `KEY_QUOTE` `KEY_LEFTPAREN` `KEY_RIGHTPAREN` `KEY_ASTERISK` `KEY_PLUS` `KEY_COMMA` `KEY_MINUS` `KEY_PERIOD` `KEY_SLASH` `KEY_COLON` `KEY_SEMICOLON` `KEY_LESS` `KEY_EQUALS` `KEY_GREATER` `KEY_QUESTION` `KEY_AT` `KEY_LEFTBRACKET` `KEY_BACKSLASH` `KEY_RIGHTBRACKET` `KEY_CARET` `KEY_UNDERSCORE` `KEY_BACKQUOTE`

**Function:**

`KEY_F1` `KEY_F2` `KEY_F3` `KEY_F4` `KEY_F5` `KEY_F6` `KEY_F7` `KEY_F8` `KEY_F9` `KEY_F10` `KEY_F11` `KEY_F12` `KEY_F13` `KEY_F14` `KEY_F15` `KEY_F16` `KEY_F17` `KEY_F18` `KEY_F19` `KEY_F20` `KEY_F21` `KEY_F22` `KEY_F23` `KEY_F24`

**Navigation:**

`KEY_INSERT` `KEY_HOME` `KEY_PAGEUP` `KEY_DELETE` `KEY_END` `KEY_PAGEDOWN` `KEY_RIGHT` `KEY_LEFT` `KEY_DOWN` `KEY_UP`

**Modifiers:**

`KEY_CAPSLOCK` `KEY_LCTRL` `KEY_LSHIFT` `KEY_LALT` `KEY_LGUI` `KEY_RCTRL` `KEY_RSHIFT` `KEY_RALT` `KEY_RGUI` `KEY_MODE` `KEY_SHIFT` `KEY_CTRL` `KEY_ALT` `KEY_GUI`

**Numpad:**

`KEY_NUMLOCKCLEAR` `KEY_KP_DIVIDE` `KEY_KP_MULTIPLY` `KEY_KP_MINUS` `KEY_KP_PLUS` `KEY_KP_ENTER` `KEY_KP_1` `KEY_KP_2` `KEY_KP_3` `KEY_KP_4` `KEY_KP_5` `KEY_KP_6` `KEY_KP_7` `KEY_KP_8` `KEY_KP_9` `KEY_KP_0` `KEY_KP_PERIOD` `KEY_KP_EQUALS` `KEY_KP_COMMA` `KEY_KP_EQUALSAS400` `KEY_KP_00` `KEY_KP_000` `KEY_KP_LEFTPAREN` `KEY_KP_RIGHTPAREN` `KEY_KP_LEFTBRACE` `KEY_KP_RIGHTBRACE` `KEY_KP_TAB` `KEY_KP_BACKSPACE` `KEY_KP_A` `KEY_KP_B` `KEY_KP_C` `KEY_KP_D` `KEY_KP_E` `KEY_KP_F` `KEY_KP_XOR` `KEY_KP_POWER` `KEY_KP_PERCENT` `KEY_KP_LESS` `KEY_KP_GREATER` `KEY_KP_AMPERSAND` `KEY_KP_DBLAMPERSAND` `KEY_KP_VERTICALBAR` `KEY_KP_DBLVERTICALBAR` `KEY_KP_COLON` `KEY_KP_HASH` `KEY_KP_SPACE` `KEY_KP_AT` `KEY_KP_EXCLAM` `KEY_KP_MEMSTORE` `KEY_KP_MEMRECALL` `KEY_KP_MEMCLEAR` `KEY_KP_MEMADD` `KEY_KP_MEMSUBTRACT` `KEY_KP_MEMMULTIPLY` `KEY_KP_MEMDIVIDE` `KEY_KP_PLUSMINUS` `KEY_KP_CLEAR` `KEY_KP_CLEARENTRY` `KEY_KP_BINARY` `KEY_KP_OCTAL` `KEY_KP_DECIMAL` `KEY_KP_HEXADECIMAL`

**System & Media:**

`KEY_PRINTSCREEN` `KEY_SCROLLLOCK` `KEY_PAUSE` `KEY_APPLICATION` `KEY_POWER` `KEY_EXECUTE` `KEY_HELP` `KEY_MENU` `KEY_SELECT` `KEY_STOP` `KEY_AGAIN` `KEY_UNDO` `KEY_CUT` `KEY_COPY` `KEY_PASTE` `KEY_FIND` `KEY_MUTE` `KEY_VOLUMEUP` `KEY_VOLUMEDOWN` `KEY_ALTERASE` `KEY_SYSREQ` `KEY_CANCEL` `KEY_CLEAR` `KEY_PRIOR` `KEY_RETURN2` `KEY_SEPARATOR` `KEY_OUT` `KEY_OPER` `KEY_CLEARAGAIN` `KEY_CRSEL` `KEY_EXSEL` `KEY_THOUSANDSSEPARATOR` `KEY_DECIMALSEPARATOR` `KEY_CURRENCYUNIT` `KEY_CURRENCYSUBUNIT` `KEY_AUDIONEXT` `KEY_AUDIOPREV` `KEY_AUDIOSTOP` `KEY_AUDIOPLAY` `KEY_AUDIOMUTE` `KEY_MEDIASELECT` `KEY_WWW` `KEY_MAIL` `KEY_CALCULATOR` `KEY_COMPUTER` `KEY_AC_SEARCH` `KEY_AC_HOME` `KEY_AC_BACK` `KEY_AC_FORWARD` `KEY_AC_STOP` `KEY_AC_REFRESH` `KEY_AC_BOOKMARKS` `KEY_BRIGHTNESSDOWN` `KEY_BRIGHTNESSUP` `KEY_DISPLAYSWITCH` `KEY_KBDILLUMTOGGLE` `KEY_KBDILLUMDOWN` `KEY_KBDILLUMUP` `KEY_EJECT` `KEY_SLEEP` `KEY_APP1` `KEY_APP2` `KEY_AUDIOREWIND` `KEY_AUDIOFASTFORWARD`

### Mouse Constants

**Cursor & Wheel:**

`MOUSE_POS_X` `MOUSE_POS_Y` `MOUSE_WHEEL_X` `MOUSE_WHEEL_Y`

**Buttons:**

`MOUSE_BUTTON_LEFT` `MOUSE_BUTTON_MIDDLE` `MOUSE_BUTTON_RIGHT` `MOUSE_BUTTON_X1` `MOUSE_BUTTON_X2` `MOUSE_BUTTON_UNKNOWN`

### Gamepad Constants

**Gamepad 1:**

`GAMEPAD1_AXIS_LEFTX` `GAMEPAD1_AXIS_LEFTY` `GAMEPAD1_AXIS_RIGHTX` `GAMEPAD1_AXIS_RIGHTY` `GAMEPAD1_AXIS_TRIGGERLEFT` `GAMEPAD1_AXIS_TRIGGERRIGHT` `GAMEPAD1_BUTTON_A` `GAMEPAD1_BUTTON_B` `GAMEPAD1_BUTTON_X` `GAMEPAD1_BUTTON_Y` `GAMEPAD1_BUTTON_BACK` `GAMEPAD1_BUTTON_GUIDE` `GAMEPAD1_BUTTON_START` `GAMEPAD1_BUTTON_LEFTSTICK` `GAMEPAD1_BUTTON_RIGHTSTICK` `GAMEPAD1_BUTTON_LEFTSHOULDER` `GAMEPAD1_BUTTON_RIGHTSHOULDER` `GAMEPAD1_BUTTON_DPAD_UP` `GAMEPAD1_BUTTON_DPAD_DOWN` `GAMEPAD1_BUTTON_DPAD_LEFT` `GAMEPAD1_BUTTON_DPAD_RIGHT`

**Gamepad 2:**

`GAMEPAD2_AXIS_LEFTX` `GAMEPAD2_AXIS_LEFTY` `GAMEPAD2_AXIS_RIGHTX` `GAMEPAD2_AXIS_RIGHTY` `GAMEPAD2_AXIS_TRIGGERLEFT` `GAMEPAD2_AXIS_TRIGGERRIGHT` `GAMEPAD2_BUTTON_A` `GAMEPAD2_BUTTON_B` `GAMEPAD2_BUTTON_X` `GAMEPAD2_BUTTON_Y` `GAMEPAD2_BUTTON_BACK` `GAMEPAD2_BUTTON_GUIDE` `GAMEPAD2_BUTTON_START` `GAMEPAD2_BUTTON_LEFTSTICK` `GAMEPAD2_BUTTON_RIGHTSTICK` `GAMEPAD2_BUTTON_LEFTSHOULDER` `GAMEPAD2_BUTTON_RIGHTSHOULDER` `GAMEPAD2_BUTTON_DPAD_UP` `GAMEPAD2_BUTTON_DPAD_DOWN` `GAMEPAD2_BUTTON_DPAD_LEFT` `GAMEPAD2_BUTTON_DPAD_RIGHT`

**Gamepad 3:**

`GAMEPAD3_AXIS_LEFTX` `GAMEPAD3_AXIS_LEFTY` `GAMEPAD3_AXIS_RIGHTX` `GAMEPAD3_AXIS_RIGHTY` `GAMEPAD3_AXIS_TRIGGERLEFT` `GAMEPAD3_AXIS_TRIGGERRIGHT` `GAMEPAD3_BUTTON_A` `GAMEPAD3_BUTTON_B` `GAMEPAD3_BUTTON_X` `GAMEPAD3_BUTTON_Y` `GAMEPAD3_BUTTON_BACK` `GAMEPAD3_BUTTON_GUIDE` `GAMEPAD3_BUTTON_START` `GAMEPAD3_BUTTON_LEFTSTICK` `GAMEPAD3_BUTTON_RIGHTSTICK` `GAMEPAD3_BUTTON_LEFTSHOULDER` `GAMEPAD3_BUTTON_RIGHTSHOULDER` `GAMEPAD3_BUTTON_DPAD_UP` `GAMEPAD3_BUTTON_DPAD_DOWN` `GAMEPAD3_BUTTON_DPAD_LEFT` `GAMEPAD3_BUTTON_DPAD_RIGHT`

**Gamepad 4:**

`GAMEPAD4_AXIS_LEFTX` `GAMEPAD4_AXIS_LEFTY` `GAMEPAD4_AXIS_RIGHTX` `GAMEPAD4_AXIS_RIGHTY` `GAMEPAD4_AXIS_TRIGGERLEFT` `GAMEPAD4_AXIS_TRIGGERRIGHT` `GAMEPAD4_BUTTON_A` `GAMEPAD4_BUTTON_B` `GAMEPAD4_BUTTON_X` `GAMEPAD4_BUTTON_Y` `GAMEPAD4_BUTTON_BACK` `GAMEPAD4_BUTTON_GUIDE` `GAMEPAD4_BUTTON_START` `GAMEPAD4_BUTTON_LEFTSTICK` `GAMEPAD4_BUTTON_RIGHTSTICK` `GAMEPAD4_BUTTON_LEFTSHOULDER` `GAMEPAD4_BUTTON_RIGHTSHOULDER` `GAMEPAD4_BUTTON_DPAD_UP` `GAMEPAD4_BUTTON_DPAD_DOWN` `GAMEPAD4_BUTTON_DPAD_LEFT` `GAMEPAD4_BUTTON_DPAD_RIGHT`

## Graphics

### `colors` — variable

List of the palette display colors. Specified by 24-bit numerical value. Supports Python list operations.

- **Type:** `list[int]`

**Example:**

```python
old_colors = list(pyxel.colors)
pyxel.colors[15] = 0x112233
```

### `images` — variable

List of the image banks (instances of the Image class) (0-2).

- **Type:** `list[Image]`

**Example:**

```python
pyxel.images[0].load(0, 0, "title.png")
```

### `tilemaps` — variable

List of the tilemaps (instances of the Tilemap class) (0-7).

- **Type:** `list[Tilemap]`

### `screen` — variable *(Advanced)*

The screen image (Image class instance).

- **Type:** `Image`

### `cursor` — variable *(Advanced)*

The cursor image (Image class instance).

- **Type:** `Image`

### `font` — variable *(Advanced)*

The font image (Image class instance).

- **Type:** `Image`

### `clip(x, y, w, h)` — function

Set the drawing area of the screen from (x, y) with a width of w and a height of h. Call clip() to reset to full screen.

**Parameters:**

- `x` (*float*) — X coordinate of the upper-left corner
- `y` (*float*) — Y coordinate of the upper-left corner
- `w` (*float*) — Width of the clipping area
- `h` (*float*) — Height of the clipping area

### `clip()` — function

Reset the drawing area to full screen.

### `camera(x, y)` — function

Set the drawing offset to (x, y). All subsequent drawing operations will be shifted by (-x, -y).

**Parameters:**

- `x` (*float*) — X coordinate
- `y` (*float*) — Y coordinate

### `camera()` — function

Reset the drawing offset to (0, 0).

### `pal(col1, col2)` — function

Replace color col1 with col2 when drawing.

**Parameters:**

- `col1` (*int*) — Color to replace
- `col2` (*int*) — Replacement color

### `pal()` — function

Reset the palette to the initial state.

### `dither(alpha)` — function

Apply dithering (pseudo-transparency) when drawing. Set alpha in the range 0.0-1.0.

**Parameters:**

- `alpha` (*float*) — Opacity (0.0: transparent, 1.0: opaque)

### `cls(col)` — function

Clear the screen with color col.

**Parameters:**

- `col` (*int*) — Color

### `pget(x, y)` — function

Get the color of the pixel at (x, y).

**Parameters:**

- `x` (*float*) — X coordinate
- `y` (*float*) — Y coordinate

**Returns:** `int` — Color of the pixel

### `pset(x, y, col)` — function

Draw a pixel of color col at (x, y).

**Parameters:**

- `x` (*float*) — X coordinate
- `y` (*float*) — Y coordinate
- `col` (*int*) — Color

### `line(x1, y1, x2, y2, col)` — function

Draw a line of color col from (x1, y1) to (x2, y2).

**Parameters:**

- `x1` (*float*) — Start X coordinate
- `y1` (*float*) — Start Y coordinate
- `x2` (*float*) — End X coordinate
- `y2` (*float*) — End Y coordinate
- `col` (*int*) — Color

### `rect(x, y, w, h, col)` — function

Draw a filled rectangle of width w, height h, and color col at (x, y).

**Parameters:**

- `x` (*float*) — X coordinate
- `y` (*float*) — Y coordinate
- `w` (*float*) — Width
- `h` (*float*) — Height
- `col` (*int*) — Color

### `rectb(x, y, w, h, col)` — function

Draw the outline of a rectangle of width w, height h, and color col at (x, y).

**Parameters:**

- `x` (*float*) — X coordinate
- `y` (*float*) — Y coordinate
- `w` (*float*) — Width
- `h` (*float*) — Height
- `col` (*int*) — Color

### `circ(x, y, r, col)` — function

Draw a filled circle of radius r and color col at (x, y).

**Parameters:**

- `x` (*float*) — Center X coordinate
- `y` (*float*) — Center Y coordinate
- `r` (*float*) — Radius
- `col` (*int*) — Color

### `circb(x, y, r, col)` — function

Draw the outline of a circle of radius r and color col at (x, y).

**Parameters:**

- `x` (*float*) — Center X coordinate
- `y` (*float*) — Center Y coordinate
- `r` (*float*) — Radius
- `col` (*int*) — Color

### `elli(x, y, w, h, col)` — function

Draw a filled ellipse of width w, height h, and color col at (x, y).

**Parameters:**

- `x` (*float*) — X coordinate
- `y` (*float*) — Y coordinate
- `w` (*float*) — Width
- `h` (*float*) — Height
- `col` (*int*) — Color

### `ellib(x, y, w, h, col)` — function

Draw the outline of an ellipse of width w, height h, and color col at (x, y).

**Parameters:**

- `x` (*float*) — X coordinate
- `y` (*float*) — Y coordinate
- `w` (*float*) — Width
- `h` (*float*) — Height
- `col` (*int*) — Color

### `tri(x1, y1, x2, y2, x3, y3, col)` — function

Draw a filled triangle with vertices (x1, y1), (x2, y2), (x3, y3) and color col.

**Parameters:**

- `x1` (*float*) — Vertex 1 X
- `y1` (*float*) — Vertex 1 Y
- `x2` (*float*) — Vertex 2 X
- `y2` (*float*) — Vertex 2 Y
- `x3` (*float*) — Vertex 3 X
- `y3` (*float*) — Vertex 3 Y
- `col` (*int*) — Color

### `trib(x1, y1, x2, y2, x3, y3, col)` — function

Draw the outline of a triangle with vertices (x1, y1), (x2, y2), (x3, y3) and color col.

**Parameters:**

- `x1` (*float*) — Vertex 1 X
- `y1` (*float*) — Vertex 1 Y
- `x2` (*float*) — Vertex 2 X
- `y2` (*float*) — Vertex 2 Y
- `x3` (*float*) — Vertex 3 X
- `y3` (*float*) — Vertex 3 Y
- `col` (*int*) — Color

### `fill(x, y, col)` — function

Fill the area connected with the same color as (x, y) with color col.

**Parameters:**

- `x` (*float*) — X coordinate
- `y` (*float*) — Y coordinate
- `col` (*int*) — Fill color

### `blt(x, y, img, u, v, w, h, colkey=None, rotate=0, scale=1)` — function

Copy the region of size (w, h) from (u, v) of image bank img (0-2 or Image instance) to (x, y). Negative w/h flips the image. colkey sets the transparent color. rotate and scale apply transformations.

**Parameters:**

- `x` (*float*) — Destination X
- `y` (*float*) — Destination Y
- `img` (*int/Image*) — Image bank number (0-2) or Image instance
- `u` (*float*) — Source X in the image bank
- `v` (*float*) — Source Y in the image bank
- `w` (*float*) — Width (negative to flip)
- `h` (*float*) — Height (negative to flip)
- `colkey` (*int/None*) — Transparent color
- `rotate` (*float*) — Rotation angle in degrees (centered on the copy region)
- `scale` (*float*) — Scale factor (centered on the copy region)

### `bltm(x, y, tm, u, v, w, h, colkey=None, rotate=0, scale=1)` — function

Copy the region of size (w, h) from (u, v) of tilemap tm (0-7 or Tilemap instance) to (x, y). Each tile is 8x8 pixels stored as (image_tx, image_ty). Negative w/h flips the image. colkey sets the transparent color. rotate and scale apply transformations.

**Parameters:**

- `x` (*float*) — Destination X
- `y` (*float*) — Destination Y
- `tm` (*int/Tilemap*) — Tilemap number (0-7) or Tilemap instance
- `u` (*float*) — Source X in the tilemap
- `v` (*float*) — Source Y in the tilemap
- `w` (*float*) — Width (negative to flip)
- `h` (*float*) — Height (negative to flip)
- `colkey` (*int/None*) — Transparent color
- `rotate` (*float*) — Rotation angle in degrees (centered on the copy region)
- `scale` (*float*) — Scale factor (centered on the copy region)

### `blt3d(x, y, w, h, img, pos, rot, fov=60, colkey=None)` — function *(Advanced)*

Draw the image bank img (0-2 or Image instance) with perspective projection onto the screen rectangle (x, y, w, h). pos is the camera position where x, y match 2D coordinates and z is height. rot is the rotation in degrees. fov sets the field of view in degrees. colkey sets the transparent color.

**Parameters:**

- `x` (*float*) — Destination X
- `y` (*float*) — Destination Y
- `w` (*float*) — Display width
- `h` (*float*) — Display height
- `img` (*int/Image*) — Image bank number (0-2) or Image instance
- `pos` (*(float, float, float)*) — Camera position (x, y, z). x, y match 2D coordinates, z is height
- `rot` (*(float, float, float)*) — Rotation in degrees. rot_x is vertical, rot_y is horizontal, rot_z is tilt
- `fov` (*float*) — Field of view in degrees
- `colkey` (*int/None*) — Transparent color

### `bltm3d(x, y, w, h, tm, pos, rot, fov=60, colkey=None)` — function *(Advanced)*

Draw the tilemap tm (0-7 or Tilemap instance) with perspective projection onto the screen rectangle (x, y, w, h). pos is the camera position where x, y match 2D coordinates and z is height. rot is the rotation in degrees. fov sets the field of view in degrees. colkey sets the transparent color.

**Parameters:**

- `x` (*float*) — Destination X
- `y` (*float*) — Destination Y
- `w` (*float*) — Display width
- `h` (*float*) — Display height
- `tm` (*int/Tilemap*) — Tilemap number (0-7) or Tilemap instance
- `pos` (*(float, float, float)*) — Camera position (x, y, z). x, y match 2D coordinates, z is height
- `rot` (*(float, float, float)*) — Rotation in degrees. rot_x is vertical, rot_y is horizontal, rot_z is tilt
- `fov` (*float*) — Field of view in degrees
- `colkey` (*int/None*) — Transparent color

### `text(x, y, s, col, font=None)` — function

Draw a string s in color col at (x, y).

**Parameters:**

- `x` (*float*) — X coordinate
- `y` (*float*) — Y coordinate
- `s` (*str*) — String to draw
- `col` (*int*) — Color
- `font` (*Font*) — Custom font (optional)

### Color Constants

- `COLOR_BLACK` — 0 — Black (#000000)
- `COLOR_NAVY` — 1 — Navy (#2b335f)
- `COLOR_PURPLE` — 2 — Purple (#7e2072)
- `COLOR_GREEN` — 3 — Green (#19959c)
- `COLOR_BROWN` — 4 — Brown (#8b4852)
- `COLOR_DARK_BLUE` — 5 — Dark Blue (#395c98)
- `COLOR_LIGHT_BLUE` — 6 — Light Blue (#a9c1ff)
- `COLOR_WHITE` — 7 — White (#eeeeee)
- `COLOR_RED` — 8 — Red (#d4186c)
- `COLOR_ORANGE` — 9 — Orange (#d38441)
- `COLOR_YELLOW` — 10 — Yellow (#e9c35b)
- `COLOR_LIME` — 11 — Lime (#70c6a9)
- `COLOR_CYAN` — 12 — Cyan (#7696de)
- `COLOR_GRAY` — 13 — Gray (#a3a3a3)
- `COLOR_PINK` — 14 — Pink (#ff9798)
- `COLOR_PEACH` — 15 — Peach (#edc7b0)

### Resource Constants

- `NUM_COLORS` — 16 — Number of palette colors
- `NUM_IMAGES` — 3 — Number of image banks (0-2)
- `IMAGE_SIZE` — 256 — Image width and height
- `NUM_TILEMAPS` — 8 — Number of tilemaps (0-7)
- `TILEMAP_SIZE` — 256 — Tilemap width and height
- `TILE_SIZE` — 8 — Tile size in pixels
- `FONT_WIDTH` — 4 — Built-in font width
- `FONT_HEIGHT` — 6 — Built-in font height

## Audio

### `sounds` — variable

List of the sounds (instances of the Sound class) (0-63).

- **Type:** `list[Sound]`

**Example:**

```python
pyxel.sounds[0].speed = 60
```

### `musics` — variable

List of music tracks (instances of the Music class) (0-7).

- **Type:** `list[Music]`

### `tones` — variable *(Advanced)*

List of the tone definitions (instances of the Tone class) (0-3).

- **Type:** `list[Tone]`

### `channels` — variable *(Advanced)*

List of the channels (instances of the Channel class) (0-3).

- **Type:** `list[Channel]`

### `play(ch, snd, sec=0, loop=False, resume=False)` — function

Play the sound snd on channel ch (0-3). snd can be a sound number (0-63), a list of numbers, a Sound instance, a list of Sounds, or an MML string.

**Parameters:**

- `ch` (*int*) — Channel number (0-3)
- `snd` (*int/list/Sound/str*) — Sound number (0-63), list of numbers, Sound instance, list of Sounds, or MML string
- `sec` (*float*) — Playback start position in seconds
- `loop` (*bool*) — Loop playback
- `resume` (*bool*) — Resume previous sound after playback ends

### `playm(msc, sec=0, loop=False)` — function

Play the music msc (0-7).

**Parameters:**

- `msc` (*int*) — Music number (0-7)
- `sec` (*float*) — Playback start position in seconds
- `loop` (*bool*) — Loop playback

### `stop(ch)` — function

Stop playback of the specified channel ch (0-3).

**Parameters:**

- `ch` (*int*) — Channel number (0-3)

### `stop()` — function

Stop playback of all channels.

### `play_pos(ch)` — function

Get the sound playback position of channel ch (0-3) as a tuple of (sound_no, sec). Return None when playback has stopped.

**Parameters:**

- `ch` (*int*) — Channel number (0-3)

**Returns:** `tuple[int, float]/None` — (sound_index, sec) or None

### `gen_bgm(preset, instr, seed=None, play=False)` — function

Generate a BGM MML list using an algorithm. preset (0-7) selects the preset, instr (0-3) selects the instrumentation.

**Parameters:**

- `preset` (*int*) — Preset number (0-7). 0-1: title, departure (medium tempo), 2-3: town, peaceful (slow tempo), 4-5: field, adventure (medium tempo), 6-7: battle, crisis (fast tempo)
- `instr` (*int*) — Instrumentation (0-3). 0: melody+reverb+bass (3ch), 1: melody+bass+drums (3ch), 2: melody+sub+bass (3ch), 3: melody+sub+bass+drums (4ch)
- `seed` (*int*) — Random seed (omit for random)
- `play` (*bool*) — Play the generated MML

**Returns:** `list[str]` — List of MML strings

### Tone & Effect Constants

**Tones:**

- `TONE_TRIANGLE` — 0 — Triangle wave
- `TONE_SQUARE` — 1 — Square wave
- `TONE_PULSE` — 2 — Pulse wave
- `TONE_NOISE` — 3 — Noise

**Effects:**

- `EFFECT_NONE` — 0 — No effect
- `EFFECT_SLIDE` — 1 — Slide
- `EFFECT_VIBRATO` — 2 — Vibrato
- `EFFECT_FADEOUT` — 3 — Fade out
- `EFFECT_HALF_FADEOUT` — 4 — Half fade out
- `EFFECT_QUARTER_FADEOUT` — 5 — Quarter fade out

### Resource Constants

- `NUM_CHANNELS` — 4 — Number of audio channels (0-3)
- `NUM_TONES` — 4 — Number of tone types (0-3)
- `NUM_SOUNDS` — 64 — Number of sounds (0-63)
- `NUM_MUSICS` — 8 — Number of music tracks (0-7)

## Math

### `ceil(x)` — function

Return the smallest integer greater than or equal to x.

**Parameters:**

- `x` (*float*) — Value

**Returns:** `int` — Smallest integer >= x

### `floor(x)` — function

Return the largest integer less than or equal to x.

**Parameters:**

- `x` (*float*) — Value

**Returns:** `int` — Largest integer <= x

### `clamp(x, lower, upper)` — function

Return x clamped between lower and upper.

**Parameters:**

- `x` (*int/float*) — Value to clamp
- `lower` (*int/float*) — Minimum value
- `upper` (*int/float*) — Maximum value

**Returns:** `int/float` — Clamped value

### `sgn(x)` — function

Return 1 when x is positive, 0 when it is 0, and -1 when it is negative.

**Parameters:**

- `x` (*int/float*) — Value

**Returns:** `int/float` — Sign of the value (1, 0, or -1)

### `sqrt(x)` — function

Return the square root of x.

**Parameters:**

- `x` (*float*) — Value

**Returns:** `float` — Square root of x

### `sin(deg)` — function

Return the sine of deg degrees.

**Parameters:**

- `deg` (*float*) — Angle in degrees

**Returns:** `float` — Sine value

### `cos(deg)` — function

Return the cosine of deg degrees.

**Parameters:**

- `deg` (*float*) — Angle in degrees

**Returns:** `float` — Cosine value

### `atan2(y, x)` — function

Return the arctangent of y/x in degrees.

**Parameters:**

- `y` (*float*) — Y value
- `x` (*float*) — X value

**Returns:** `float` — Angle in degrees

### `rseed(seed)` — function

Set the seed of the random number generator.

**Parameters:**

- `seed` (*int*) — Seed value (non-negative integer)

### `rndi(a, b)` — function

Return a random integer from a to b (inclusive).

**Parameters:**

- `a` (*int*) — Minimum value (inclusive)
- `b` (*int*) — Maximum value (inclusive)

**Returns:** `int` — Random integer from a to b

### `rndf(a, b)` — function

Return a random float from a to b (inclusive).

**Parameters:**

- `a` (*float*) — Minimum value (inclusive)
- `b` (*float*) — Maximum value (inclusive)

**Returns:** `float` — Random float from a to b

### `nseed(seed)` — function

Set the seed of Perlin noise.

**Parameters:**

- `seed` (*int*) — Seed value (non-negative integer)

### `noise(x, y=0, z=0)` — function

Return the Perlin noise value for the specified coordinates.

**Parameters:**

- `x` (*float*) — X coordinate
- `y` (*float*) — Y coordinate
- `z` (*float*) — Z coordinate

**Returns:** `float` — Perlin noise value

## Font Class

### `Font(filename, font_size=10)` — class *(Advanced)*

Create a Font instance from a font file (BDF/OTF/TTF/TTC).

**Parameters:**

- `filename` (*str*) — Font file path (BDF/OTF/TTF/TTC)
- `font_size` (*float*) — Font size (default: 10.0, not used for BDF)

**Returns:** `Font` — New Font instance

### `Font.text_width(s)` — function *(Advanced)*

Return the display width of the string s in this font.

**Parameters:**

- `s` (*str*) — String to measure

**Returns:** `int` — Width in pixels

## Image Class

### `Image(width, height)` — class *(Advanced)*

Create a new Image instance with the specified size.

**Parameters:**

- `width` (*int*) — Image width
- `height` (*int*) — Image height

**Returns:** `Image` — New Image instance

### `Image.from_image(filename, include_colors=False)` — class *(Advanced)*

Create an Image instance from an image file.

**Parameters:**

- `filename` (*str*) — Image file path
- `include_colors` (*bool*) — Include palette colors from file (optional)

**Returns:** `Image` — Image instance from file

### `Image.width` — variable

The width of the image.

- **Type:** `int`

### `Image.height` — variable

The height of the image.

- **Type:** `int`

### `Image.set(x, y, data)` — function

Set the image at (x, y) using a list of hex-digit strings. Each character represents a color index (0-f).

**Parameters:**

- `x` (*int*) — X coordinate
- `y` (*int*) — Y coordinate
- `data` (*list[str]*) — Image data as a list of strings

**Example:**

```python
pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])
```

### `Image.load(x, y, filename, include_colors=False)` — function

Load an image file (PNG/GIF/JPEG) at (x, y).

**Parameters:**

- `x` (*int*) — X coordinate
- `y` (*int*) — Y coordinate
- `filename` (*str*) — Image file path (PNG/GIF/JPEG)
- `include_colors` (*bool*) — Include palette colors from file (optional)

### `Image.pget(x, y)` — function

Get the color of the pixel at (x, y).

**Parameters:**

- `x` (*float*) — X coordinate
- `y` (*float*) — Y coordinate

**Returns:** `int` — Color of the pixel

### `Image.pset(x, y, col)` — function

Draw a pixel with the color col at (x, y).

**Parameters:**

- `x` (*float*) — X coordinate
- `y` (*float*) — Y coordinate
- `col` (*int*) — Color

### `Image.{cls, line, rect, rectb, circ, circb, elli, ellib, tri, trib, fill, blt, bltm, blt3d, bltm3d, text, clip, camera, pal, dither}` — function

Image instances support all drawing functions from the Graphics section. They work the same way but draw on the image instead of the screen.

### `Image.data_ptr()` — function *(Advanced)*

Return the raw data pointer of the image as a ctypes c_uint8 array.

**Returns:** `Any` — Raw data pointer

### `Image.save(filename, scale)` — function *(Advanced)*

Save the image to a file.

**Parameters:**

- `filename` (*str*) — Output file path
- `scale` (*int*) — Scale factor

## Tilemap Class

### `Tilemap(width, height, img)` — class *(Advanced)*

Create a new Tilemap instance.

**Parameters:**

- `width` (*int*) — Tilemap width
- `height` (*int*) — Tilemap height
- `img` (*int/Image*) — Image bank number (0-2) or Image instance

**Returns:** `Tilemap` — New Tilemap instance

### `Tilemap.from_tmx(filename, layer)` — class *(Advanced)*

Create a Tilemap instance from a TMX file.

**Parameters:**

- `filename` (*str*) — TMX file path
- `layer` (*int*) — Layer number (0-)

**Returns:** `Tilemap` — Tilemap instance from TMX file

### `Tilemap.width` — variable

The width of the tilemap.

- **Type:** `int`

### `Tilemap.height` — variable

The height of the tilemap.

- **Type:** `int`

### `Tilemap.imgsrc` — variable

The image bank (0-2) or Image instance referenced by the tilemap.

- **Type:** `int/Image`

### `Tilemap.set(x, y, data)` — function

Set the tilemap at (x, y) using a list of strings. Each tile is a 4-digit hex value representing (image_tx, image_ty), separated by spaces.

**Parameters:**

- `x` (*int*) — X coordinate
- `y` (*int*) — Y coordinate
- `data` (*list[str]*) — Tilemap data as a list of strings

**Example:**

```python
pyxel.tilemaps[0].set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])
```

### `Tilemap.load(x, y, filename, layer)` — function

Load the layer (0-) from the TMX file at (x, y).

**Parameters:**

- `x` (*int*) — X coordinate
- `y` (*int*) — Y coordinate
- `filename` (*str*) — TMX file path
- `layer` (*int*) — Layer number (0-)

### `Tilemap.pget(x, y)` — function

Get the tile at (x, y). A tile is a tuple of (image_tx, image_ty).

**Parameters:**

- `x` (*float*) — X coordinate
- `y` (*float*) — Y coordinate

**Returns:** `tuple[int, int]` — (image_tx, image_ty)

### `Tilemap.pset(x, y, tile)` — function

Set a tile at (x, y). A tile is a tuple of (image_tx, image_ty).

**Parameters:**

- `x` (*float*) — X coordinate
- `y` (*float*) — Y coordinate
- `tile` (*tuple[int, int]*) — Tile as (image_tx, image_ty)

### `Tilemap.{cls, line, rect, rectb, circ, circb, elli, ellib, tri, trib, fill, blt, clip, camera}` — function

Tilemap instances support drawing functions from the Graphics section. They work the same way but draw tiles on the tilemap instead of pixels on the screen. Use a tile tuple (image_tx, image_ty) instead of a color value.

### `Tilemap.collide(x, y, w, h, dx, dy, walls)` — function

Resolve collisions after applying pixel movement (dx, dy) to the pixel rectangle at (x, y) with pixel size (w, h), and return the adjusted (dx, dy). walls is a list of tile coordinates that act as obstacles.

**Parameters:**

- `x` (*float*) — Rectangle X position
- `y` (*float*) — Rectangle Y position
- `w` (*float*) — Rectangle width
- `h` (*float*) — Rectangle height
- `dx` (*float*) — Movement in X
- `dy` (*float*) — Movement in Y
- `walls` (*list[tuple[int, int]]*) — List of wall tiles (image_tx, image_ty)

**Returns:** `tuple[float, float]` — Adjusted (dx, dy)

### `Tilemap.data_ptr()` — function *(Advanced)*

Return the raw data pointer of the tilemap as a ctypes c_uint8 array (2 bytes per tile: image_tx, image_ty).

**Returns:** `Any` — Raw data pointer

## Sound Class

### `Sound()` — class *(Advanced)*

Create a new Sound instance.

**Returns:** `Sound` — New Sound instance

### `Sound.notes` — variable

List of notes (0-59). Higher values produce higher pitches. 33 = 'A2' (440 Hz). Rests are -1.

- **Type:** `list[int]`

### `Sound.tones` — variable

List of tones (0: Triangle, 1: Square, 2: Pulse, 3: Noise).

- **Type:** `list[int]`

### `Sound.volumes` — variable

List of volumes (0-7).

- **Type:** `list[int]`

### `Sound.effects` — variable

List of effects (0: None, 1: Slide, 2: Vibrato, 3: FadeOut, 4: Half-FadeOut, 5: Quarter-FadeOut).

- **Type:** `list[int]`

### `Sound.speed` — variable

Playback speed. 1 is the fastest, and the larger the number, the slower the playback speed. At 120, one note equals 1 second.

- **Type:** `int`

### `Sound.set(notes, tones, volumes, effects, speed)` — function

Set notes, tones, volumes, and effects using strings. If the tones, volumes, or effects string is shorter than the notes, it repeats from the beginning.

**Parameters:**

- `notes` (*str*) — Note string
- `tones` (*str*) — Tone string
- `volumes` (*str*) — Volume string
- `effects` (*str*) — Effect string
- `speed` (*int*) — Playback speed

### `Sound.set_notes(notes)` — function

Set the notes using a string made of note names (CDEFGAB), optional sharp (#) or flat (-), octave (0-4), and rests (R). Case-insensitive, whitespace is ignored.

**Parameters:**

- `notes` (*str*) — Notes string

**Example:**

```python
pyxel.sounds[0].set_notes("g2b-2d3r rf3f3f3")
```

### `Sound.set_tones(tones)` — function

Set the tones with a string of TSPN (or 0-9 for custom tone numbers). Case-insensitive, whitespace is ignored.

**Parameters:**

- `tones` (*str*) — Tones string

**Example:**

```python
pyxel.sounds[0].set_tones("ttss pppn")
```

### `Sound.set_volumes(volumes)` — function

Set the volumes with a string of 01234567. Whitespace is ignored.

**Parameters:**

- `volumes` (*str*) — Volumes string

**Example:**

```python
pyxel.sounds[0].set_volumes("7777 7531")
```

### `Sound.set_effects(effects)` — function

Set the effects with a string of NSVFHQ. Case-insensitive, whitespace is ignored.

**Parameters:**

- `effects` (*str*) — Effects string

**Example:**

```python
pyxel.sounds[0].set_effects("nfnf nvvs")
```

### `Sound.mml(code)` — function

Switch to MML mode with the given MML string. In MML mode, normal parameters such as notes and speed are ignored.

**Parameters:**

- `code` (*str*) — MML string

**Example:**

```python
pyxel.sounds[0].mml("T120 Q90 @1 V100 O5 L8 C4&C<G16R16>C.")
```

### `Sound.mml()` — function

Exit MML mode and return to normal mode.

### `Sound.pcm(filename)` — function

Load an audio file (WAV/OGG) for playback.

**Parameters:**

- `filename` (*str*) — Audio file path (WAV/OGG)

**Example:**

```python
pyxel.sounds[0].pcm("sounds/bgm.ogg")
```

### `Sound.pcm()` — function

Exit PCM mode and return to normal mode.

### `Sound.save(filename, sec, ffmpeg=False)` — function *(Advanced)*

Create a WAV file of the sound for the specified duration in seconds.

**Parameters:**

- `filename` (*str*) — Output WAV file path
- `sec` (*float*) — Duration in seconds
- `ffmpeg` (*bool*) — Also create MP4 file (requires FFmpeg)

### `Sound.total_sec()` — function *(Advanced)*

Return the playback time in seconds. Return None for infinite loops.

**Returns:** `float/None` — Playback time in seconds, or None for infinite loops

## Music Class

### `Music()` — class *(Advanced)*

Create a new Music instance.

**Returns:** `Music` — New Music instance

### `Music.seqs` — variable

A two-dimensional list of sounds (0-63) across multiple channels.

- **Type:** `list[list[int]]`

### `Music.set(seq0, seq1, seq2, ...)` — function

Set the lists of sounds (0-63) for each channel. An empty list means the channel is not used.

**Parameters:**

- `seq0, seq1, ...` (*list[int]*) — Sound lists for each channel. Empty list = unused.

**Example:**

```python
pyxel.musics[0].set([0, 1], [], [3])
```

### `Music.save(filename, sec, ffmpeg=False)` — function *(Advanced)*

Create a WAV file of the music for the specified duration in seconds.

**Parameters:**

- `filename` (*str*) — Output WAV file path
- `sec` (*float*) — Duration in seconds
- `ffmpeg` (*bool*) — Also create MP4 file (requires FFmpeg)

## Channel Class

### `Channel()` — class *(Advanced)*

Create a new Channel instance.

**Returns:** `Channel` — New Channel instance

### `Channel.gain` — variable *(Advanced)*

The gain (volume) of the channel (default: 0.125).

- **Type:** `float`

### `Channel.detune` — variable *(Advanced)*

The detune value for pitch adjustment (default: 0).

- **Type:** `int`

### `Channel.play(snd, sec=0, loop=False, resume=False)` — function *(Advanced)*

Play the sound snd on this channel. snd can be a sound number, a list, a Sound instance, a list of Sounds, or an MML string.

**Parameters:**

- `snd` (*int/list/Sound/str*) — Sound number (0-63), list of numbers, Sound instance, list of Sounds, or MML string
- `sec` (*float*) — Playback start position in seconds
- `loop` (*bool*) — Loop playback (default: False)
- `resume` (*bool*) — Resume previous sound after playback ends (default: False)

### `Channel.stop()` — function *(Advanced)*

Stop playback on this channel.

### `Channel.play_pos()` — function *(Advanced)*

Get the playback position as a tuple of (sound_no, sec). Return None when playback has stopped.

**Returns:** `tuple[int, float]/None` — (sound_index, sec) or None

## Tone Class

### `Tone()` — class *(Advanced)*

Create a new Tone instance.

**Returns:** `Tone` — New Tone instance

### `Tone.mode` — variable *(Advanced)*

Tone mode (0: Wavetable, 1: ShortPeriodNoise, 2: LongPeriodNoise).

- **Type:** `int`

### `Tone.sample_bits` — variable *(Advanced)*

Sample bits for the wavetable (default: 4).

- **Type:** `int`

### `Tone.wavetable` — variable *(Advanced)*

Wavetable data as a list of sample values. Each value must be in range 0 to (2^sample_bits - 1).

- **Type:** `list[int]`

### `Tone.gain` — variable *(Advanced)*

Tone gain (default: 1.0).

- **Type:** `float`
