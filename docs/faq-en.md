# Pyxel FAQ

## Migrating to a New Version

<details>
<summary>How to migrate code to version 1.5</summary>

To make your code compatible with version 1.5, follow these steps:

- Rename the `caption` option in `init` to `title`
- Rename the `scale` option in `init` to `display_scale`
- Remove the `palette` option from `init`. You can modify the palette colors via the `colors` array after initialization.
- Remove the `fullscreen` option from `init`. Use the `fullscreen` function to toggle fullscreen after initialization.
- If you encounter undefined errors, rename the key according to the [key definitions](https://github.com/kitao/pyxel/blob/main/python/pyxel/__init__.pyi).
- Change `get` and `set` in the `Image` and `Tilemap` classes to `pget` and `pset` respectively.
- Multiply the `u`, `v`, `w`, and `h` parameters of `bltm` by 8, as `bltm` now operates in pixel units.
- Update the members and methods in the `Sound` and `Music` classes based on the new naming conventions.
</details>

<details>
<summary>Why can’t I use the <code>pyxeleditor</code> command in version 1.5+?</summary>

Starting from version 1.5, Pyxel's tools have been integrated into the `pyxel` command. To access the resource editor, use the following command: `pyxel edit [PYXEL_RESOURCE_FILE]`.

</details>

## Learning Pyxel

<details>
<summary>Where do I start to learn Pyxel?</summary>

I recommend starting by experimenting with Pyxel's example code. Try the following examples in this order: 01, 05, 03, 04, and 02.

</details>

<details>
<summary>Are there any books on Pyxel?</summary>

There are currently two books available in Japanese, though neither is authored by Pyxel’s developer. Unfortunately, there are no English versions at the moment, but more Pyxel books, including English editions, are likely to be released in the future!

</details>

## API Specification and Usage

<details>
<summary>What is the difference between the <code>update</code> and <code>draw</code> functions?</summary>

The `update` function is called every frame, whereas the `draw` function may be skipped if the frame processing time exceeds the allowed limit. This design allows Pyxel to maintain smooth animations regardless of rendering load or interrupt handling.

</details>

## Using Pyxel Tools

## Future Development Plans

<details>
<summary>What features are planned for future Pyxel releases?</summary>

Upcoming features include:

- Improve the usability of Pyxel Editor
- Python and Pyxel tutorials aimed at children
</details>

## Licensing and Sponsorship

<details>
<summary>Can I use Pyxel for commercial purposes without permission?</summary>

Yes, you can use Pyxel for commercial purposes as long as you follow the MIT License and credit the developer. However, I’d greatly appreciate it if you consider sponsoring Pyxel!

</details>
