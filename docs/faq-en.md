# Pyxel FAQ

## Learning Pyxel

<details>
<summary>Where do I start to learn Pyxel?</summary>

It is recommended to try Pyxel's example code in the following order.

1. 01_hello_pyxel — Pyxel basics
2. 05_color_palette — Color palette
3. 03_draw_api — Drawing API
4. 04_sound_api — Sound API
5. 02_jump_game — Game implementation

You can copy the examples with `pyxel copy_examples`, or run them in your browser on [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/).

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

MML (Music Macro Language) is a language for defining sounds by describing notes, tempo, and other parameters as a string.

Passing an MML string to the `mml` function of the Sound class causes that Sound to be played according to the MML content. Calling `mml()` with no arguments clears the MML setting.

```python
pyxel.sounds[0].mml("CDEFGAB>C")
```

You can also play an MML string directly by passing it to the `play` function instead of a sound number.

```python
pyxel.play(0, "CDEFG")
```

For available MML commands, see [Pyxel MML Commands](https://kitao.github.io/pyxel/wasm/mml-studio/mml-commands.html). For usage examples, see the [demo](https://kitao.github.io/pyxel/wasm/showcase/examples/09-shooter.html) and [code](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py) of the 09_shooter.py example.

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
<summary>Can I try Pyxel without installing it?</summary>

With [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/), you can create and run Pyxel apps in your browser. However, it does not support multi-file projects, so a local environment is recommended for full-scale development.

[Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/) lets you browse and run sample code and apps in your browser.

</details>

<details>
<summary>How do I publish my Pyxel app on the web?</summary>

There are three methods: Web Launcher, app2html, and Custom Tags. For details, see [How to Use Pyxel for Web](pyxel-web-en.md).

</details>

<details>
<summary>Can I change the palette colors in Pyxel Editor?</summary>

By placing a file with the same name but with the .pyxpal extension in the same directory as the Pyxel resource file (.pyxres), the palette display colors in Pyxel Editor will be updated. Palette files can be created with the `save_pal` function, or manually as a text file with one hex color code per line.

</details>

## Migration Guide

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

For details on the new MML syntax, see "[How do I use Pyxel's MML?](#api-specification-and-usage)" above.

</details>

## Licensing and Sponsorship

<details>
<summary>Can I use Pyxel for commercial purposes without the author's permission?</summary>

As long as you comply with the MIT License and clearly display the full text of the copyright and license in the source code or license file, you are free to sell or distribute it without the author's permission. However, since Pyxel is developed by a single individual, it would be appreciated if you could contact the author or consider sponsoring their work if possible.

</details>
