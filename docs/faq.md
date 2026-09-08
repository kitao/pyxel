# Pyxel FAQ

## Learning Pyxel

<details>
<summary>Where do I start to learn Pyxel?</summary>

We recommend trying Pyxel's example code in the following order.

1. 01_hello_pyxel — Pyxel basics
2. 05_color_palette — Color palette
3. 03_draw_api — Drawing API
4. 04_sound_api — Sound API
5. 02_jump_game — Game implementation

You can copy the examples by following the [User Guide](https://kitao.github.io/pyxel/web/user-guide/#s-usage), or run them in your browser on [Pyxel Showcase](https://kitao.github.io/pyxel/web/showcase/).

</details>

<details>
<summary>Are there any books on Pyxel?</summary>

The official [book](https://gihyo.jp/book/2025/978-4-297-14657-3) is available in Japanese only.

</details>

## API Specification and Usage

<details>
<summary>What is the difference between the <code>update</code> and <code>draw</code> functions?</summary>

`update` updates the state each frame; `draw` draws the screen. Pyxel may skip `draw` when processing falls behind, so put movement and collision logic in `update`.

</details>

<details>
<summary>How do I use Pyxel MML?</summary>

MML (Music Macro Language) specifies notes and tempo as a string.

Set it with a Sound's `mml` method; call `mml()` without arguments to clear it.

```python
pyxel.sounds[0].mml("CDEFGAB>C")
```

You can also play an MML string directly by passing it to the `play` function instead of a sound number.

```python
pyxel.play(0, "CDEFG")
```

For available MML commands, see [Pyxel MML Commands](https://kitao.github.io/pyxel/web/mml-studio/mml-commands.html). For usage examples, see the [demo](https://kitao.github.io/pyxel/web/showcase/examples/09-shooter.html) and [code](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py) of the 09_shooter.py example.

You can also create and share MML in your browser using [Pyxel MML Studio](https://kitao.github.io/pyxel/web/mml-studio/).

</details>

## File Operations and Data Management

<details>
<summary>Why does file loading fail when the environment changes?</summary>

Relative paths start from the current working directory. `pyxel.init()` changes it to the script's directory. Check this base path if you load files before `init` or change the working directory afterward.

</details>

<details>
<summary>How can I save application-specific data like high scores or game progress?</summary>

Pass the developer and app names to `pyxel.user_data_dir(vendor_name, app_name)`. It creates a storage directory for the current platform and returns its path. Save your data in this directory.

</details>

## Using Pyxel Tools

<details>
<summary>Can I try Pyxel without installing it?</summary>

With [Pyxel Code Maker](https://kitao.github.io/pyxel/web/code-maker/), you can create and run Pyxel apps in your browser. However, the code editor only edits `main.py`, so a local environment is recommended when editing projects with multiple Python files.

[Pyxel Showcase](https://kitao.github.io/pyxel/web/showcase/) lets you browse and run sample code and apps in your browser.

</details>

<details>
<summary>How do I publish my Pyxel app on the web?</summary>

There are four methods: Pyxel Code Maker, Pyxel Web Launcher, app2html, and HTML Custom Elements. For details, see [How to Use Pyxel for Web](https://kitao.github.io/pyxel/web/web-usage/).

</details>

<details>
<summary>Can I change the palette colors in Pyxel Editor?</summary>

If you place a file with the same name but the .pyxpal extension in the same directory as the Pyxel resource file (.pyxres), Pyxel Editor will display the palette using those colors. Palette files can be created with the `save_pal` function, or manually as a text file with one RRGGBB hex color per line.

</details>

## Migration Guide

<details>
<summary>How to migrate code to version 2.4</summary>

In Pyxel 2.4, the sound engine and MML syntax have been revamped.

To make your code compatible with version 2.4, please make the following changes:

- Rename the `waveform` field of the Tone class to `wavetable`
- Replace the `tick` argument of the `play` and `playm` functions with `sec`, converting its value to `tick / 120` seconds
- Update code to handle the return value of the `play_pos` function, which is now `(sound_index, sec)`
- For the Sound and Music classes' `save` function, replace the repeat count `count` with the duration in seconds, `sec`
- If you need the playback duration of a sound, use the `total_sec` function of the Sound class
- For the Sound class's `mml` function, rewrite the code in the new MML syntax
- Change the `excl_*` option in the `save` and `load` functions to `exclude_*`
- Remove the `incl_*` option from the `save` and `load` functions

For details on the new MML syntax, see "[How do I use Pyxel MML?](#api-specification-and-usage)" above.

</details>

## Licensing and Sponsorship

<details>
<summary>Can I use Pyxel for commercial purposes without the author's permission?</summary>

As long as you comply with the MIT License and clearly display the full text of the copyright and license in the source code or license file, you are free to sell or distribute your work without the author's permission. That said, Pyxel is developed by a single individual, so reaching out to the author or becoming a sponsor would be greatly appreciated.

</details>
