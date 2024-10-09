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

Currently, two books have been published in Japanese, but neither is authored by the developer of Pyxel. Additionally, there are no English versions available at this time.

</details>

## API Specification and Usage

<details>
<summary>What is the difference between the <code>update</code> and <code>draw</code> functions?</summary>

The `update` function is called every frame, but the `draw` function may be skipped if the processing time exceeds the allowable limit. This design in Pyxel reduces the impact of rendering load and OS interruptions, enabling smooth animation.

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
