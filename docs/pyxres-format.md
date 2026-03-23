# Pyxel Resource File Format (.pyxres)

This document describes the internal format of Pyxel resource files (`.pyxres`).

## Overview

A `.pyxres` file is a **ZIP archive** containing a single TOML file named `pyxel_resource.toml`. All resource data — images, tilemaps, sounds, and music — is stored as human-readable text in TOML format.

An optional **palette file** (`.pyxpal`) can accompany the resource file to define a custom color palette.

## File Structure

```
example.pyxres (ZIP archive)
└── pyxel_resource.toml
```

```
example.pyxpal (optional, same directory)
```

## TOML Format

The `pyxel_resource.toml` file has the following top-level structure:

```toml
format_version = 1

[[images]]
...

[[tilemaps]]
...

[[sounds]]
...

[[musics]]
...
```

The `format_version` field and all four section arrays (`images`, `tilemaps`, `sounds`, `musics`) are **required**. Empty resources are represented as empty arrays (e.g., `notes = []`). Pyxel always writes all entries (3 images, 8 tilemaps, 64 sounds, 8 musics), but files with fewer entries are accepted on load.

Pyxel currently writes `format_version = 1` for maximum backward compatibility. On load, files with format version up to **4** (the current maximum) are accepted. Files with version 1–3 that use the legacy archive layout (`pyxel_resource/version` + separate files) are loaded with automatic conversion.

## Images

Up to **3 image banks**, each **256×256 pixels**. Each pixel is a palette color index (`u8`, 0–15 for the default palette, up to 254 with extended palettes).

```toml
[[images]]
width = 256
height = 256
data = [[0, 0, 10, 12, 12, 0, ...], [0, 7, 7, ...], ...]
```

All fields are required.

| Field | Type | Description |
| --- | --- | --- |
| `width` | u32 | Image width in pixels |
| `height` | u32 | Image height in pixels |
| `data` | array of arrays of u8 | 2D pixel data, one sub-array per row. Each value is a color index |

**Compression:** Trailing elements in each row that repeat the last distinct value are omitted. Trailing rows that are identical to the last distinct row are also omitted. When loading, omitted values are filled by repeating the last value.

For example, a row `[0, 0, 5, 5, 5, 5]` is stored as `[0, 0, 5]`, and an image where the last 200 rows are all `[0]` stores only the first 56 rows plus one `[0]` row.

## Tilemaps

Up to **8 tilemaps**, each **256×256 tiles**. Each tile references an 8×8 pixel region in an image bank.

```toml
[[tilemaps]]
width = 256
height = 256
imgsrc = 0
data = [[10, 6, 11, 6, 12, 6, 0, 0, ...], ...]
```

All fields are required.

| Field | Type | Description |
| --- | --- | --- |
| `width` | u32 | Tilemap width in tiles |
| `height` | u32 | Tilemap height in tiles |
| `imgsrc` | u32 | Source image bank index (0–2) |
| `data` | array of arrays of u16 | 2D tile data in interleaved format (see below) |

Each tile in the tilemap is a coordinate pair `(tile_x, tile_y)` (`u16`, `u16`) pointing to the position of the 8×8 tile in the source image bank. In the TOML data, these pairs are **interleaved** within each row: `[tx0, ty0, tx1, ty1, ...]`, so each row has `width × 2` elements.

For example, a tilemap row of 3 tiles referencing image positions (10, 6), (11, 6), and (12, 6) is stored as `[10, 6, 11, 6, 12, 6]`.

The same trailing-value compression as images is applied.

## Sounds

Up to **64 sounds**. Each sound is a sequence of notes with per-note tone, volume, and effect settings.

```toml
[[sounds]]
notes = [28, 28, 24, 19, -1, -1]
tones = [2]
volumes = [6]
effects = [2, 3, 3, 0]
speed = 25
```

All fields are required. Empty sounds use empty arrays (e.g., `notes = []`).

| Field | Type | Description |
| --- | --- | --- |
| `notes` | array of i8 | Note values. -1 = rest, 0 (C0) to 59 (B4) |
| `tones` | array of u8 | 0 = Triangle, 1 = Square, 2 = Pulse, 3 = Noise |
| `volumes` | array of u8 | 0 (silent) to 7 (max) |
| `effects` | array of u8 | 0 = None, 1 = Slide, 2 = Vibrato, 3 = FadeOut, 4 = Half-FadeOut, 5 = Quarter-FadeOut |
| `speed` | u16 | Playback speed in ticks per note (1 tick = 1/120 second). Default: 30 |

The `tones`, `volumes`, and `effects` arrays may be shorter than `notes` due to trailing-value compression. When loading, the last value in each array is repeated to match the length of `notes`.

**Note mapping:** Notes are encoded as `base + octave × 12`, where C=0, D=2, E=4, F=5, G=7, A=9, B=11, and octave ranges from 0 to 4. For example, C0=0, A4=57, B4=59.

## Music

Up to **8 music tracks**. Each track arranges sounds across up to **4 channels** for sequential playback.

```toml
[[musics]]
seqs = [[0, 1], [2, 3], [4]]
```

The `seqs` field is required. Empty music uses an empty array (`seqs = []`).

| Field | Type | Description |
| --- | --- | --- |
| `seqs` | array of arrays of u32 | Sound sequences per channel. `seqs[0]` is channel 0, `seqs[1]` is channel 1, etc. Each value is a sound index (0–63) |

Trailing empty channels are omitted when saving.

## Palette File (.pyxpal)

An optional text file with one RGB color per line in 6-digit hexadecimal format. The file uses the same base name as the `.pyxres` file (e.g., `sample.pyxres` → `sample.pyxpal`).

```
000000
2b335f
7e2072
19959c
...
```

| Property | Value |
| --- | --- |
| Default colors | 16 |
| Maximum colors | 256 |
| Format | One `RRGGBB` hex value per line |

If no `.pyxpal` file is present, the built-in default palette is used:

| Index | Color | Hex |
| --- | --- | --- |
| 0 | Black | `000000` |
| 1 | Navy | `2b335f` |
| 2 | Purple | `7e2072` |
| 3 | Green | `19959c` |
| 4 | Brown | `8b4852` |
| 5 | Dark Blue | `395c98` |
| 6 | Light Blue | `a9c1ff` |
| 7 | White | `eeeeee` |
| 8 | Red | `d4186c` |
| 9 | Orange | `d38441` |
| 10 | Yellow | `e9c35b` |
| 11 | Lime | `70c6a9` |
| 12 | Cyan | `7696de` |
| 13 | Gray | `a3a3a3` |
| 14 | Pink | `ff9798` |
| 15 | Peach | `edc7b0` |
