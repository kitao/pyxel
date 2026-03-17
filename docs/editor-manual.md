# Pyxel Editor Manual

*This document was auto-generated from the [Pyxel Editor Manual](https://kitao.github.io/pyxel/web/editor-manual/) web page, which also offers multilingual support.*

## Overview

Pyxel Editor is a built-in resource editing tool for Pyxel. It consists of four editors, which can be switched using the editor buttons at the top of the screen.

- **Image Editor:** Edit images (image banks) used for sprites and tiles
- **Tilemap Editor:** Arrange image bank images as tiles to compose maps
- **Sound Editor:** Edit sounds used for sound effects and melodies
- **Music Editor:** Arrange sounds in playback order to compose music such as BGM

## Getting Started

Launch Pyxel Editor with the following command:

pyxel edit [PYXEL_RESOURCE_FILE]

If the specified Pyxel resource file (.pyxres) exists, it will be loaded. If it does not exist, a new file with the specified name will be created. If the filename is omitted, `my_resource.pyxres` will be created.

## Common Operations

Operations common to all editors.

### Menu Bar

The menu bar at the top of the screen contains four editor switching buttons, Undo/Redo buttons, and a Save button. The help message area on the right displays context-sensitive information based on the cursor position.

![Menu Bar](images/ui_menubar.png)

| Shortcut | Action |
| --- | --- |
|  | Switch to Image Editor |
|  | Switch to Tilemap Editor |
|  | Switch to Sound Editor |
|  | Switch to Music Editor |
| Ctrl(Cmd)+Z | Undo |
| Ctrl(Cmd)+Y | Redo |
| Ctrl(Cmd)+S | Save |

### Other Shortcuts

| Shortcut | Action |
| --- | --- |
| Alt(Option)+Left/Right | Switch between editor types |
| Shift+Ctrl(Cmd)+C | Copy the entire editing target |
| Shift+Ctrl(Cmd)+X | Cut the entire editing target (copy and clear) |
| Shift+Ctrl(Cmd)+V | Paste the copied target |
| Shift+Click ([+]/[-] buttons) | Change number or value in increments of ±10 |

### Using from Programs

The created resource file can be loaded and used from a Pyxel application with the `pyxel.load()` function.

### Switching Resource Files

While the editor is running, you can drag and drop another .pyxres file onto the window to load its contents. This operation overwrites all current edits, so please be careful if you have unsaved changes.

## Image Editor

A mode for editing images in each image bank. You can create pixel art for sprites and tiles.

### Specifications

- **Image Banks:** 0–2 (3 banks)
- **Size:** 256×256 pixels each
- **Coordinate System:** Origin (0,0) at top-left, X+ rightward, Y+ downward

### Screen Layout

#### Image Canvas

The editing area selected in the image bank view is displayed magnified. Click and drag with the mouse to draw.

| Shortcut | Action |
| --- | --- |
| Right Click | Pick the color at the cursor position (color pick) |
| Right Drag | Pan the view |

#### Image Bank View

A thumbnail view of the entire image bank. The white frame indicates the current editing area.

| Shortcut | Action |
| --- | --- |
| Arrow Keys | Move the editing area (white frame) |
| Click | Move the editing area to the clicked position |
| Right Drag | Pan the view |

#### Tool Buttons

Select a tool for drawing.

#### Selection Tool Shortcuts

The following shortcut keys are available while using the selection tool.

| Shortcut | Action |
| --- | --- |
| Ctrl(Cmd)+A | Select the entire canvas |
| Ctrl(Cmd)+C | Copy the selection |
| Ctrl(Cmd)+X | Cut the selection (copy and clear) |
| Ctrl(Cmd)+V | Paste the copied area |
| H | Flip the selection horizontally |
| V | Flip the selection vertically |

#### Color Palette

Select a drawing color from the 16-color palette.

| Shortcut | Action |
| --- | --- |
| 1〜8 | Select colors 0–7 |
| Shift+1〜8 | Select colors 8–15 |

#### Bank Number

Switch the target image bank (0–2).

### Loading External Files

Drag and drop an image file such as PNG or JPG onto the Image Editor to load the image at the editing area position. The original image colors are automatically converted to Pyxel's color palette, and any parts exceeding the image bank boundaries are clipped.

## Tilemap Editor

A mode for arranging image bank images as tile patterns and editing tilemaps.

### Specifications

- **Tilemaps:** 0–7 (8 maps)
- **Size:** 256×256 tiles each
- **Tile Image:** 8×8 pixel region of the image bank
- **Reference Image Bank:** One per tilemap

### Screen Layout

#### Tilemap Canvas

The editing area selected in the tilemap view is displayed magnified. Place tile images selected in the tile image view. Drawing tools and selection shortcuts are shared with the Image Editor. Note that all cells default to tile (0,0), so it is recommended to keep image bank position (0,0) empty.

| Shortcut | Action |
| --- | --- |
| Right Click | Pick the tile at the cursor position (tile pick) |
| Right Drag | Pan the view |

#### Tilemap View

A thumbnail view of the entire tilemap. The white frame indicates the current editing area.

| Shortcut | Action |
| --- | --- |
| Arrow Keys | Move the editing area (white frame) |
| Click/Drag | Move the editing area to the clicked position |

#### Tile Image View

Displays the referenced image bank. Select tile images with the white frame.

| Shortcut | Action |
| --- | --- |
| Shift+Arrow Keys | Move the selection |
| Drag | Select multiple tiles in a rectangle |
| Right Drag | Pan the view |

#### Tilemap Number

Switch the target tilemap (0–7).

#### Ref. Image Bank Number

Switch the image bank (0–2) referenced for tile images.

#### Tool Buttons

Select a tool for drawing.

### Placing Multiple Tiles

When you draw with multiple tile images selected by dragging in the tile image view, the selected tiles are placed as a repeating pattern. For example, if you select two tiles vertically and draw with the pen, the two tiles are placed alternately in a repeating pattern. The same applies when selecting multiple tiles both horizontally and vertically. The drawing start position corresponds to the top-left of the selection.

### Loading TMX Files

Drag and drop a TMX file created with Tiled Map Editor ([Tiled](https://www.mapeditor.org/)) onto the Tilemap Editor to load the contents of layer 0 at the editing area position.

## Sound Editor

A mode for editing sounds used for melodies and sound effects.

### Specifications

- **Sounds:** 0–63 (64 sounds)
- **Pitch Range:** C0〜B4
- **Max Length:** 48 notes
- **Polyphony:** 1 note (or rest)
- **Per-Note Settings:** Tone, volume, effect

### Screen Layout

#### Sound Number

Switch the target sound (0–63).

#### Playback Speed

Sets the playback duration per note. This is a common setting for all notes, specified in the range 1–99. Smaller values result in faster playback (1 speed unit = 1/120 second; e.g., speed 60 = 0.5 seconds per note). For sound effects, 1–9 is typical; for melodies, 10 or higher is recommended.

#### Play / Stop / Loop

| Shortcut | Action |
| --- | --- |
| Space | Play |
| Space | Stop (press Space during playback to stop) |
| L | Toggle loop playback |

#### Piano Roll

Click to input note pitch. Red squares represent notes, blue squares represent rests. During piano roll editing, a blue cursor is displayed; during property area editing, a different cursor shape appears. Notes are played from left to right, and rests are automatically inserted in empty positions. Clicking an already-filled position overwrites it. Dragging inputs notes continuously with linear interpolation.

| Shortcut | Action |
| --- | --- |
| Up/Down | Switch between piano roll and property area |
| Left/Right | Move the cursor |
| Delete(fn+delete) | Delete the note at the current position |
| Backspace(delete) | Delete the note before the current position |
| Shift+Space | Play from the cursor position |
| Ctrl(Cmd)+A | Select the entire range |
| Shift+Left/Right | Start range selection |

#### Range Selection

| Shortcut | Action |
| --- | --- |
| Ctrl(Cmd)+C | Copy the selection |
| Ctrl(Cmd)+X | Cut the selection (copy and clear) |
| Ctrl(Cmd)+V | Paste the copied area |
| Ctrl(Cmd)+U | Raise the pitch of the selection by 1 |
| Ctrl(Cmd)+D | Lower the pitch of the selection by 1 |

#### Octave Bar

Displays the starting octave (0–3) for keyboard input. The keyboard covers two octaves from the selected octave.

| Shortcut | Action |
| --- | --- |
| PageUp(fn+Up) | Raise the octave by 1 |
| PageDown(fn+Down) | Lower the octave by 1 |

### Keyboard Input

You can input notes using the PC keyboard. The upper row (QWE row + number row) and the lower row (ZXC row + ASD row) each correspond to one octave of piano keys, covering two octaves simultaneously. White keys are assigned to natural notes and gray keys to sharps. Select the starting octave (0–3) with the octave bar.

| Shortcut | Action |
| --- | --- |
| Keyboard Key → Enter | Input the pressed keyboard note into the piano roll |
| 1 | Cycle the preview tone for keyboard input (4 types) |

#### Property Area (TON / VOL / EFX)

Set the tone (TON), volume (VOL), and effect (EFX) for each note. Press the ↓ key or click the TON row during piano roll editing to move to the property area. Cursor movement, range selection, copy & paste, and other operations are shared with the piano roll. Press a key on each row to input the corresponding value. For example, pressing P on the TON row sets Pulse. To change the tone partway through, set the value for all notes from the desired position onward. The same applies to VOL and EFX.

#### Tone

With the cursor on the TON row, press the following keys to set the tone. The default when not set is T (Triangle).

| Key | Name | Description |
| --- | --- | --- |
| `T` | Triangle | Triangle wave. Soft and gentle tone. Similar to a flute |
| `S` | Square | Square wave. Electronic and clear tone. Similar to a clarinet or organ |
| `P` | Pulse | Pulse wave. Bright and flashy tone. Similar to a trumpet |
| `N` | Noise | Noise. Unpitched sound. Suitable for percussion and explosion effects |

#### Volume

With the cursor on the VOL row, press number keys 0–7 to set the volume (0 = silent, 7 = maximum). The default when not set is 7. Using maximum volume on multiple channels simultaneously may result in excessive loudness.

#### Effect

With the cursor on the EFX row, press the following keys to set the effect. The default when not set is N (None).

| Key | Name | Description |
| --- | --- | --- |
| `N` | None | No effect |
| `S` | Slide | Smoothly glide the pitch from the previous note |
| `V` | Vibrato | Periodically oscillate the pitch |
| `F` | FadeOut | Gradually decrease the volume over the entire note. Also used to separate notes of the same pitch |
| `H` | Half-FadeOut | Gradually decrease the volume in the second half of the note. Longer sustain than F |
| `Q` | Quarter-FadeOut | Gradually decrease the volume in the last quarter of the note. Longer sustain than H |

## Music Editor

A mode for arranging sounds in playback order to compose music tracks.

### Specifications

- **Musics:** 0–7 (8 tracks)
- **Channels:** 4
- **Sounds per Channel:** Max 32

### Screen Layout

#### Music Number

Switch the target music (0–7).

#### Play / Stop / Loop

| Shortcut | Action |
| --- | --- |
| Space | Play |
| Space | Stop (press Space during playback to stop) |
| L | Toggle loop playback |

#### Sequence Editor (CH0–CH3)

Place sound numbers in playback order across four channels (CH0–CH3). Copy and paste between channels is also supported.

| Shortcut | Action |
| --- | --- |
| Up/Down | Switch the editing channel |
| Left/Right | Move the editing cursor |
| Delete(fn+delete) | Delete the sound at the current position |
| Backspace(delete) | Delete the sound before the current position |
| Shift+Space | Play from the cursor position |
| Ctrl(Cmd)+A | Select the entire range of the current channel |
| Shift+Arrow Keys | Select a range within the channel |
| Ctrl(Cmd)+C | Copy the selection |
| Ctrl(Cmd)+X | Cut the selection (copy and clear) |
| Ctrl(Cmd)+V | Paste the selection (also across channels) |
| Ctrl(Cmd)+U | Increment the sound number of the selection by 1 |
| Ctrl(Cmd)+D | Decrement the sound number of the selection by 1 |

#### Sound Buttons (0–63)

Buttons for inserting sound numbers (0–63). Pressing a button inserts the sound number at the editing cursor position. Created sounds are shown in blue, uncreated ones in gray. Hovering over a button previews the sound.
