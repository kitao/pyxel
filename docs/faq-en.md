# Pyxel FAQ

## Migrating to a New Version

<details>
<summary>How to migrate code to version 2.4</summary>

In Pyxel 2.4, the sound engine and MML syntax have been revamped.<br>
To make your code compatible with version 2.4, please make the following changes:

- Rename the `waveform` field of the Tone class to `wavetable`
- Change the `tick` argument of the `play` and `playm` functions to `sec` (a float value in seconds)
- Update code to handle the return value of the `play_pos` function, which is now `(sound_no, sec)`
- Change the `count` argument of the `save` function in the Sound and Music classes to `sec`
- If you need the playback duration of a sound, use the `total_sec` function of the Sound class
- For the Sound class's `mml` function, use code that follows the new MML syntax
- To use the old MML syntax, use the Sound class's `old_mml` function
- Change the `excl_*` option in the `save` and `load` functions to `exclude_*`
- Remove the `incl_*` option from the `save` and `load` functions

For details on the new MML syntax, see "How to use Pyxel's MML" below.

</details>

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
<summary>How do I use Pyxel's MML?</summary>

You can use MML (Music Macro Language) in Pyxel by passing an MML string to the `mml` function of the Sound class. This switches the sound to MML mode, and the sound will be played according to the MML string.

In MML mode, normal parameters like `notes` and `speed` are ignored, and the sound is played according to the MML string. Calling `mml()` again resets the MML mode.

You can also play an MML string directly by passing it to the `play` function instead of a sound number.<br>
Example: `pyxel.play(0, "CDEFG")`

The available commands for Pyxel's MML can be found on the [Pyxel MML Commands](https://kitao.github.io/pyxel/wasm/mml-studio/mml-commands.html) page.

Examples of usage can be seen in the [demo](https://kitao.github.io/pyxel/wasm/showcase/examples/09-shooter.html) and [code](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py) of the 09_shooter.py example.

You can also create and share MML in your browser using [Pyxel MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/).

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
- Improve usability of Pyxel Editor
- Add Pyxel tutorials for children

</details>

## Licensing and Sponsorship

<details>
<summary>Can I use Pyxel for commercial purposes without the author's permission?</summary>

As long as you comply with the MIT License and clearly display the full text of the copyright and license in the source code or license file, you are free to sell or distribute it without the author’s permission. However, since Pyxel is developed by a single individual, it would be appreciated if you could contact the author or consider sponsoring their work if possible.

</details>
