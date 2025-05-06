# Pyxel FAQ

## Migrating to a New Version

<details>
<summary>How to migrate code to version 1.5</summary>

To make your code compatible with version 1.5, follow these steps:

- Rename the `caption` option in `init` to `title`
- Rename the `scale` option in `init` to `display_scale`
- Remove the `palette` option from `init` (you can modify the palette colors with the `colors` array after initialization)
- Remove the `fullscreen` option from `init` (you can toggle fullscreen mode using the `fullscreen` function after initialization)
- If an undefined key name error occurs, rename the key according to the [key definitions](https://github.com/kitao/pyxel/blob/main/python/pyxel/__init__.pyi)
- Change `get` and `set` in the `Image` and `Tilemap` classes to `pget` and `pset`, respectively
- Multiply the `u`, `v`, `w`, and `h` parameters of `bltm` by 8 (as `bltm` now operates in pixel units)
- Update the members and methods of the `Sound` and `Music` classes to their new names

</details>

<details>
<summary>I can’t use the <code>pyxeleditor</code> command in version 1.5+.</summary>

Starting from version 1.5, Pyxel's tools have been integrated into the `pyxel` command. To access the resource editor, use the following command: `pyxel edit [PYXEL_RESOURCE_FILE]`.

</details>

## Learning Pyxel

<details>
<summary>Where do I start to learn Pyxel?</summary>

It is recommended to try Pyxel's example code in the following order: 01, 05, 03, 04, 02.

</details>

<details>
<summary>Are there any books on Pyxel?</summary>

The official [book](https://gihyo.jp/book/2025/978-4-297-14657-3) is available in Japanese only.

</details>

## API Specification and Usage

<details>
<summary>What is the difference between the <code>update</code> and <code>draw</code> functions?</summary>

The `update` function is called every frame, but the `draw` function may be skipped if the processing time exceeds the allowable limit. This design in Pyxel reduces the impact of rendering load and OS interruptions, enabling smooth animation.

</details>

<details>
<summary>What are the types and usage of Pyxel's MML commands?</summary>

The following are the types of commands available for use with the `mml` method of the Sound class:

- `T`(1-900)<br>
  Specifies the tempo. The default is 100.<br>
  Note that there may be discrepancies in the specified tempo because it is converted using the formula `Sound.speed = 900/T`.<br>
  The tempo applies to the entire sound, and if specified multiple times, the last value will be used.
- `@`(0-3)<br>
  Specifies the tone. The default is 0.
- `O`(0-4)<br>
  Specifies the octave. The default is 2.
- `>`<br>
  Increases the octave by 1.
- `<`<br>
  Decreases the octave by 1.
- `Q`(1-8)<br>
  Specifies the quantization (length of the sound). At 8, there is no gap between notes; at 4, it is halved. The default is 7.
- `V`(0-7)<br>
  Specifies the volume. The default is 7.
- `X`(0-7)<br>
  Defines and specifies the volume envelope. This is an advanced command used instead of `V`.<br>
  For example, specifying `X2:345` switches to envelope 2 and changes the volume of each note to something like 34555... The unit of volume change is a sixteenth of a quarter note.<br>
  Specifying `X2` switches to envelope 2 and uses the volume envelope set for that number.
- `L`(1/2/4/8/16/32)<br>
  Specifies the length of notes and rests. `L8` is an eighth note. The default is 4.
- `CDEFGAB`<br>
  Plays the note for the specified pitch.<br>
  You can specify a length (1/2/4/8/16/32) after the note, like `F16`, to temporarily change the note's length.
- `R`<br>
  Plays a rest.<br>
  You can specify a length (1/2/4/8/16/32) after the rest, like `R8`, to temporarily change the rest's length.
- `#` or `+`<br>
  Written after a note, raises the pitch by a semitone.
- `-`<br>
  Written after a note, lowers the pitch by a semitone.
- `.`<br>
  Dotted note. Written after a note, extends its length by half.
- `~`<br>
  Written after a note, plays it with vibrato.
- `&`<br>
  Ties the next note if it has the same pitch, or slurs it if the pitch is different.

</details>

## File Operations and Data Management

<details>
<summary>File cannot be loaded. It may fail when the environment changes.</summary>

Make sure that the current directory is set as intended when loading files.<br>
When Pyxel's `init` function is called, the current directory is changed to the same location as the script file. After that, files can be specified using relative paths. However, loading may fail if you try to open a file before calling `init` or if the current directory is changed after calling `init`.

</details>

<details>
<summary>How can I save application-specific data like high scores or game progress?</summary>

Pass the developer name (`vendor_name`) and application name (`app_name`) to the `user_data_dir(vendor_name, app_name)` function. It will return the path to a directory suitable for data storage on the current platform. Use this directory to save and load your application's files.

</details>

## Using Pyxel Tools

<details>
<summary>Can I change the palette colors in Pyxel Editor?</summary>

By placing a Pyxel palette file (.pyxpal) in the same directory as the Pyxel resource file (.pyxres), you can match the palette colors used in Pyxel Editor to those in the resource file. For instructions on creating a Pyxel palette file, please refer to the README.

</details>

## Future Development Plans

<details>
<summary>What features are planned for future releases?</summary>

The following features and improvements are planned:

- Add a Pyxel app launcher
- Overhaul of sound functions and MML support
- Improve usability of Pyxel Editor
- Add Pyxel tutorials for children

</details>

## Licensing and Sponsorship

<details>
<summary>Can I use Pyxel for commercial purposes without the author's permission?</summary>

As long as you comply with the MIT License and clearly display the full text of the copyright and license in the source code or license file, you are free to sell or distribute it without the author’s permission. However, since Pyxel is developed by a single individual, it would be appreciated if you could contact the author or consider sponsoring their work if possible.

</details>
